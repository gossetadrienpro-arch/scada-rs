mod tcp;
mod api;



#[tokio::main]
async fn main() {
let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    tokio::join!(
        tcp::run("127.0.0.1:5020"),
        axum::serve(listener, api::router()),
    );
}