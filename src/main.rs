use anthere::App;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let server = App::new().await.expect("Unable to create server");
    server.serve().await.expect("Unable to serve app");
}
