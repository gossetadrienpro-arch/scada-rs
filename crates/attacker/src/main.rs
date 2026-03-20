use tokio::net::TcpStream;

#[tokio::main]
async fn main() {
    for i in 1..=6 {
        let _stream = TcpStream::connect("127.0.0.1:5020").await.unwrap();
        println!("Connexion {} établie", i);
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}