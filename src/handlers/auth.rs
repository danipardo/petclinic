use crate::{
    logic::users::{self},
    AppError, Context,
};
use axum::{
    extract::{Extension, Form},
    response::{Html, IntoResponse, Redirect},
};
use axum::extract::Query;

use axum_extra::extract::{cookie::Cookie, CookieJar};
use redis::{Commands, RedisError};
use serde::Deserialize;
use std::sync::Arc;
use tera::Tera;
use tracing::info;

#[derive(Deserialize, Debug)]
pub struct LoginForm {
    username: String,
    password: String,
}
#[derive(Deserialize, Debug)]
pub struct LoginParams {
    error: Option<String>
}

pub async fn logout(jar: CookieJar) -> Result<impl IntoResponse, AppError> {
    let session_cookie = Cookie::build("axum_session", "").finish();
    let updated_jar = jar.remove(session_cookie);

    Ok((updated_jar, Redirect::to("/")))
}
pub async fn login(
    Extension(tera): Extension<Tera>,
    Extension(_state): Extension<Arc<Context>>,
    params: Query<LoginParams>
) -> Html<String> {
    let mut c = tera::Context::new();

    if let Some(_error) = &params.error {
        c.insert("error", "Invalid credentials (try with admin/admin)");
    }
    let r = tera.render("auth/login.html", &c).unwrap();

    Html::from(r)
}

pub async fn post_login(
    Extension(state): Extension<Arc<Context>>,
    Form(login): Form<LoginForm>,
    jar: CookieJar,
) -> Result<impl IntoResponse, AppError> {
    let session_cookie = Cookie::build("axum_session", users::session_key()).finish();


    let user = users::authenticate(&state.rb, &login.username, &login.password).await?;

    if let Some(_u) = user {
        // Add a session to redis, where the key is the cookie value
        // And the redis value is the user info
        let mut conn = state.redis_connection.lock().unwrap();

        let redis_key = session_cookie.value();
        let redis_value = serde_json::to_string(&_u).unwrap();
        info!("Redis key {} Value: {}", &redis_key, &redis_value);
        let redis_response: Result<(), RedisError> = conn.set_ex(redis_key, redis_value, 10);
        if redis_response.is_err() {
            let x = redis_response.err().unwrap();
            tracing::error!("Cannot write into redis: {}", x.to_string());
        }
        let updated_jar = jar.add(session_cookie);
        return Ok((updated_jar, Redirect::to("/pets")));
    }

    Ok((jar, Redirect::to("/login?error")))
}
