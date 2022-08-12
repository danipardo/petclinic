#[derive(Clone, Debug)]
pub struct Env {
    pub name: String,
    pub db_server: String,
    pub db_name: String,
    pub db_username: String,
    pub db_password: String,
    pub redis_server: String,
    pub redis_password: Option<String>,
    pub session_timeout: usize,
}

pub fn from_str(env: &str) -> Env {
    match env {
        "dev" => Env {
            name: "dev".to_string(),
            db_server: "localhost".to_string(),
            db_name: "petclinic".to_string(),
            db_username: "krabby".to_string(),
            db_password: "patty".to_string(),
            redis_server: "localhost".to_string(),
            redis_password: None,
            session_timeout: 108000, // 1h
        },
        "qa" => Env {
            name: "qa".to_string(),
            db_server: "localhost".to_string(),
            db_name: "petclinic".to_string(),
            db_username: "krabby".to_string(),
            db_password: "patty".to_string(),
            redis_server: "localhost".to_string(),
            redis_password: None,
            session_timeout: 108000, // 1h
        },
        "prod" => Env {
            name: "prod".to_string(),
            db_server: "localhost".to_string(),
            db_name: "petclinic".to_string(),
            db_username: "krabby".to_string(),
            db_password: "patty".to_string(),
            redis_server: "localhost".to_string(),
            redis_password: Some("redispass".to_string()),
            session_timeout: 108000, // 1h
        },
        _ => Env {
            name: "_".to_string(),
            db_server: "localhost".to_string(),
            db_name: "petclinic".to_string(),
            db_username: "krabby".to_string(),
            db_password: "patty".to_string(),
            redis_server: "localhost".to_string(),
            redis_password: None,
            session_timeout: 108000, // 1h
        },
    }
}
