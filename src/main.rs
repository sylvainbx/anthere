use anthere::Config;

mod routes;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let config = Config::new().expect("Unable to read initial configuration");

    let addr = format!("{}:{}", config.host, config.port);
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, routes::router()).await.unwrap();
}
