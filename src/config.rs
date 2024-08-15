use std::{env, fmt};
use axum_csrf::CsrfConfig;

#[derive(PartialEq)]
enum AppEnv {
    Dev,
    Prod,
}
impl fmt::Display for AppEnv {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppEnv::Dev => write!(f, "dev"),
            AppEnv::Prod => write!(f, "prod"),
        }
    }
}

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub host: std::net::Ipv4Addr,
    pub csrf_config: CsrfConfig,
}


impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let app_env = match env::var("APP_ENV") {
            Ok(v) if v == "prod" => AppEnv::Prod,
            _ => AppEnv::Dev,
        };

        tracing::debug!("Running in {app_env} mode");

        if app_env == AppEnv::Dev {
            match dotenvy::dotenv() {
                Ok(path) => tracing::debug!(".env read successfully from {}", path.display()),
                Err(e) => panic!("Could not load .env file: {}", e),
            };
        }
        let database_url = env::var("DATABASE_URL")
            .or(Err("DATABASE_URL not set"))?;
        
        let port = env::var("PORT")
            .unwrap_or(String::from("7878"))
            .parse::<u16>()
            .or(Err("PORT is not a valid u16"))?;
        
        let host = env::var("HOST")
            .unwrap_or(String::from("127.0.0.1"))
            .parse::<std::net::Ipv4Addr>()
            .or(Err("HOST is not a valid IP V4 address"))?;
        
        let csrf_config = CsrfConfig::default();

        Ok(Config { database_url, port, host, csrf_config })
    }
}
