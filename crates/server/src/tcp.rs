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

        let mut last_transaction_id: Option<u16> = None;

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

                if let Some(last_id) = last_transaction_id {
                    if last_id == frame.transaction_id {
                        warn!("Possible replay détecté — Transaction ID {} déjà vu", frame.transaction_id);
                    }
                }

                last_transaction_id = Some(frame.transaction_id);
    
                debug!("Valeur lue : {:?}", result);
            }
            Err(e) => {
                warn!("Trame invalide reçue : {} — connexion ignorée", e);
            }
        }

        }
    }
}