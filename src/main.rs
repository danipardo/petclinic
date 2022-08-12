use std::{
    error::Error,
    sync::{Arc, Mutex},
};

use axum::{
    async_trait,
    extract::{Extension, FromRequest, RequestParts},
    http::StatusCode,
    middleware::from_extractor,
    response::{Html, IntoResponse, Redirect, Response},
    routing::{get, get_service, post},
    Router,
};

use axum_extra::extract::cookie::CookieJar;

use handlers::*;
use petclinic::Env;

use argh::FromArgs;
use logic::users::User;

use rbatis::rbatis::Rbatis;
use redis::{Commands, Connection, RedisError};

use serde_json::Value;
use tera::Tera;

use tower_http::{services::ServeDir, trace::TraceLayer};
use tracing::{debug, info};
mod handlers;
mod logic;

#[derive(FromArgs)]
/// Unipromos webservice for cart
struct Args {
    /// environment (dev,testing,live)
    #[argh(option, default = "String::from(\"dev\")")]
    env: String,

    /// web service port to bind to
    #[argh(option, default = "3000")]
    port: u16,
}

pub struct Context {
    pub rb: Rbatis,
    pub env: Env,
    pub redis_connection: Mutex<Connection>,
}

#[derive(Debug)]
pub struct AppError {
    inner: Box<dyn Error>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        Html::from(format!("Oh, something bad happened: {}", &self.inner)).into_response()
    }
}

impl From<rbatis::Error> for AppError {
    fn from(e: rbatis::Error) -> Self {
        AppError { inner: Box::new(e) }
    }
}
impl From<Box<dyn Error>> for AppError {
    fn from(e: Box<dyn Error>) -> Self {
        AppError { inner: e }
    }
}

#[tokio::main]
async fn main() {
    let args: Args = argh::from_env();

    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "warn,petclinic=debug")
    }

    tracing_subscriber::fmt::init();

    let env_name = args.env.as_str();

    info!("Env: {env_name}");
    let env = petclinic::from_str(env_name);
    let state = create_context(env.clone()).await;

    let app = get_public_routes()
        .merge(get_protected_routes())
        .fallback(get(|| async { "fallback route?" }))
        .layer(TraceLayer::new_for_http())
        .route_layer(Extension(Arc::new(state)))
        .route_layer(Extension(Arc::new(env)))
        .route_layer(Extension(get_tera_instance()));

    axum::Server::bind(&format!("0.0.0.0:{}", args.port).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    info!("Server started");
}

async fn create_context(env: Env) -> Context {
    let rb = Rbatis::new();
    let dsn = format!(
        "mysql://{}:{}@{}/{}",
        env.db_username, env.db_password, env.db_server, env.db_name
    );
    rb.link(dsn.as_str()).await.unwrap();

    let redis_url = match &env.redis_password {
        Some(password) => format!("redis://:{}@{}", password, env.redis_server),
        None => format!("redis://{}", env.redis_server),
    };

    let client = redis::Client::open(redis_url).unwrap();
    let redis_connection = client.get_connection().unwrap();

    Context {
        rb,
        env,
        redis_connection: Mutex::new(redis_connection),
    }
}
fn get_public_routes() -> Router {
    Router::new()
        .route("/", get(home::home))
        .route("/logout", get(auth::logout))
        .route("/login", get(auth::login).post(auth::post_login))
        .nest(
            "/static",
            get_service(ServeDir::new("static")).handle_error(|_| async move {}),
        )
}
fn get_protected_routes() -> Router {
    Router::new()
        .route("/vets", get(vets::list))
        .route("/vets/save", post(vets::save))
        .route("/vets/:id", get(vets::get))
        .route("/pets", get(pets::list))
        .route("/pets/save", post(pets::save))
        .route("/pets/:id", get(pets::get))
        .route("/vets/delete/:id", get(vets::delete))
        .route("/pets/delete/:id", get(pets::delete))
        .route_layer(from_extractor::<User>())
}

struct Principal {
    user: Option<User>,
}

impl tera::Function for Principal {
    fn call(
        &self,
        _args: &std::collections::HashMap<String, serde_json::Value>,
    ) -> tera::Result<Value> {
        debug!("User in tera: {:?}", &self.user);
        if let Some(username) = &self.user {
            tera::Result::Ok(Value::String(username.username.clone()))
        } else {
            tera::Result::Ok(Value::String(String::from("Not Logged in")))
        }
    }
}

fn get_tera_instance() -> Tera {
    debug!("Creating Tera instance");
    let mut tera = match Tera::new("templates/**/*") {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec![".html", ".sql"]);
    tera
}

#[async_trait]
impl<B> FromRequest<B> for User
where
    B: Send,
{
    type Rejection = (StatusCode, Redirect);

    async fn from_request(req: &mut RequestParts<B>) -> Result<Self, Self::Rejection> {
        let Extension(context) = Extension::<Arc<Context>>::from_request(req).await.unwrap();
        let Extension(env) = Extension::<Arc<Env>>::from_request(req).await.unwrap();
        let cookiejar = Option::<CookieJar>::from_request(req)
            .await
            .unwrap()
            .unwrap();

        let tera = req.extensions_mut().get_mut::<Tera>().unwrap();

        if cookiejar.get("axum_session").is_none() {
            debug!("Session cookie not found, redirecting to login url");
            return Err((StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login")));
        }

        // check if the session cookie is valid against  redis
        let mut connection = context.redis_connection.lock().unwrap();
        let cookie = cookiejar.get("axum_session").unwrap();

        let valid_session: Result<User, RedisError> = connection.get(cookie.value());

        match valid_session {
            Ok(user) => {
                // refresh the key ttl
                let _redis_response: Result<(), RedisError> =
                    connection.expire(cookie.value(), context.env.session_timeout);
                tera.register_function(
                    "principal",
                    Principal {
                        user: Some(user.clone()),
                    },
                );
                if env.name == "dev" {
                    tera.full_reload().unwrap();
                }

                return Ok(user);
            }
            Err(_error) => {
                // redis query failed
                return Err((StatusCode::TEMPORARY_REDIRECT, Redirect::to("/login")));
            }
        }
    }
}
