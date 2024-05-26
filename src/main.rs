use anthere::Server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let server = Server::new().await.expect("Unable to create server");
    server.serve().await.expect("Unable to serve app");
}
