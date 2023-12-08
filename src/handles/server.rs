use crate::config::ServerConfig;
use log::{debug, error, warn};
use std::{
    error::Error,
    io::{BufReader, BufWriter},
    net::TcpListener,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    thread,
};

pub fn server_handle(server: TcpListener, config: Arc<ServerConfig>) -> Result<(), Box<dyn Error>> {
    let connections = Arc::new(AtomicUsize::new(0));

    for connection in server.incoming() {
        let (client_reader, client_writer) = match connection {
            Ok(socket) => {
                let writer = socket.try_clone().unwrap();

                (BufReader::new(socket), BufWriter::new(writer))
            }
            Err(why) => {
                error!("Failed to accept connection: {}", why.to_string());
                continue;
            }
        };

        if connections.load(Ordering::Relaxed) == config.max_conns {
            warn!("Maximum connections reached");
            continue;
        }

        {
            let config = config.clone();
            let connections = connections.clone();

            thread::spawn(move || {
                let addr = client_reader.get_ref().peer_addr().unwrap();
                debug!("New connection from {addr}");

                if let Err(why) = super::client::handle_client(client_reader, client_writer, config)
                {
                    error!("Failed to handle {addr}: {why}");
                }
                debug!("Handled {addr} successfully");
                connections.fetch_sub(1, Ordering::Relaxed);
            });
        }

        connections.fetch_add(1, Ordering::Relaxed);
    }

    Ok(())
}
