use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use modbus::parse_frame;
use simulator::PlcSimulator;

pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Serveur SCADA en écoute sur {}", addr);

    loop{
        let (mut socket, addr) = listener.accept().await.unwrap();
        println!("Nouvelle connexion : {}", addr);

        let mut buf = [0u8; 256];           // buffer de 256 octets
        let n = socket.read(&mut buf).await.unwrap();  // lit les octets, retourne le nombre lu
        println!("Reçu {} octets : {:?}", n, &buf[..n]);

       let frame = parse_frame(&buf[..n]).unwrap();
       let sim = PlcSimulator::new(1);
       let result = sim.process_request(&frame);
       println!("Valeur lue : {:?}", result);

    }
}