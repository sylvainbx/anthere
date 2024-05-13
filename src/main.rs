use std::{env, fmt};

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


fn main() {
    let app_env = match env::var("APP_ENV") {
        Ok(v) if v == "prod" => AppEnv::Prod,
        _ => AppEnv::Dev,
    };

    println!("Running in {app_env} mode");

    if app_env == AppEnv::Dev {
        match dotenvy::dotenv() {
            Ok(path) => println!(".env read successfully from {}", path.display()),
            Err(e) => panic!("Could not load .env file: {e}"),
        };
    }

    // let host = env::var("HOST").expect("HOST not set");
    // println!("Host: {host}");

    println!("Hello, world!");
}
