mod api;
mod tcp;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let (_, _) = tokio::join!(
        tcp::run("127.0.0.1:5020"),
        axum::serve(listener, api::router()),
    );
}
