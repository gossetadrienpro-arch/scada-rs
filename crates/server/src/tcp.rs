use tokio::net::TcpListener;
use tokio::io::AsyncReadExt;
use modbus::parse_frame;
use simulator::PlcSimulator;
use tracing::{info, warn, debug};

pub async fn run(addr: &str) {
    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Serveur SCADA en écoute sur {}", addr);

    loop {
        let (mut socket, addr) = listener.accept().await.unwrap();
        info!("Nouvelle connexion : {}", addr);

        loop{
        let mut buf = [0u8; 256];
        let n = socket.read(&mut buf).await.unwrap();

        debug!("Reçu {} octets : {:?}", n, &buf[..n]);

        let sim = PlcSimulator::new(1);

        if n == 0{
            break;
        } 

                match parse_frame(&buf[..n]) {
            Ok(frame) => {
                let result = sim.process_request(&frame);

                debug!("Valeur lue : {:?}", result);
            }
            Err(e) => {
                warn!("Trame invalide reçue : {} — connexion ignorée", e);
            }
        }

        }
    }
}