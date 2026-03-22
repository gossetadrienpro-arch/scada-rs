use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};


#[tokio::main]
async fn main() {

        let mut stream = TcpStream::connect("127.0.0.1:5020").await.unwrap();
        println!("Connexion établie");
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        let raw: &[u8] = &[
    0x00, 0x01,  // transaction_id = 1
    0x00, 0x00,  // protocol_id = 0
    0x00, 0x06,  // length = 6
    0x01,        // unit_id = 1
    0x03,        // function_code = 3
    0x9C, 0x41,  // adresse 40001
    0x00, 0x01,  // nombre de registres = 1
];

let mut buf = [0u8; 256];
stream.write_all(raw).await.unwrap();
let n = stream.read(&mut buf).await.unwrap();
    println!("Réponse reçue : {:?}", &buf[..n]);
}