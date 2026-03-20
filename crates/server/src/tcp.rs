use modbus::parse_frame;
use simulator::PlcSimulator;
use std::collections::HashMap;
use tokio::io::AsyncReadExt;
use tokio::net::TcpListener;
use tracing::{debug, info, warn};

pub async fn run(addr: &str) {
    let mut connection_count = HashMap::new();

    let listener = TcpListener::bind(addr).await.unwrap();
    tracing::info!("Serveur SCADA en écoute sur {}", addr);

    loop {
        let (mut socket, client_addr) = listener.accept().await.unwrap();
        info!("Nouvelle connexion : {}", client_addr);

        let ip = client_addr.ip();

        let count = {
            let c = connection_count.entry(ip).or_insert(0);
            *c += 1;
            *c
        };

        if count > 5 {
            warn!("Rate limit dépassé pour {} - connexion refusée", ip);
            continue;
        }

        let mut last_transaction_id: Option<u16> = None;

        loop {
            let mut buf = [0u8; 256];
            let n = socket.read(&mut buf).await.unwrap();

            debug!("Reçu {} octets : {:?}", n, &buf[..n]);

            let sim = PlcSimulator::new(1);

            if n == 0 {
                break;
            }

            match parse_frame(&buf[..n]) {
                Ok(frame) => {
                    let result = sim.process_request(&frame);

                    if let Some(last_id) = last_transaction_id {
                        if last_id == frame.transaction_id {
                            warn!(
                                "Possible replay détecté — Transaction ID {} déjà vu",
                                frame.transaction_id
                            );
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
