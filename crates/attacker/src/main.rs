use tokio::{net::TcpStream};
use tokio::io::AsyncWriteExt;


#[tokio::main]
async fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5020").await.unwrap();
    println!("Connecté au serveur SCADA !");

        let raw: &[u8] = &[

            0x00, 0x01, //transaction_id = 1
            0x00, 0x00, // protocol_id = 0
            0x01, 0x06, // length = 6
            0x01,       // unit_id = 1
            0x03,       // function_code =3 
            0x9c, 0x41,  // adresse 40001
            0x00, 0x01, // nombre de registres = 1
        ];

        stream.write_all(raw).await.unwrap();

        
        println!("Trame 1 envoyée — lecture registre 40001");

        let raw2:&[u8] = &[

            0x00, 0x02, //transaction_id = 1
            0x00, 0x00, //protocol_id = 0
            0x00, 0x06, //Length
            0x01,       //Unit_ID
            0x06,       //function_code
            0x9c, 0x41, //address 40001
            0x27, 0x0F, //value 9999
        ];

        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        stream.write_all(raw2).await.unwrap();
        println!("Trame 2 envoyée — écriture 9999 dans registre 40001");


}