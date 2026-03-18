mod tcp;

#[tokio::main]
async fn main() {
    tcp::run("127.0.0.1:5020").await;
}