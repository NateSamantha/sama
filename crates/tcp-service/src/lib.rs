use eyre::eyre;
use state::State;
use tokio::net::{TcpListener, TcpStream};
use tracing::{debug, error, field, info_span, Instrument};

pub async fn handle_edge_incoming(listener: TcpListener, state: State) {
    tokio::pin!(listener);

    loop {
        let incoming = match listener.accept().await {
            Ok((incoming, addr)) => {
                debug!("Accepted connection from {addr}");
                incoming
            }
            Err(e) => {
                error!(error = ?e, "Failed to accept connection!");
                // This is not _exactly_ great, why?
                continue;
            }
        };

        tokio::spawn(handle_tcp_conn(incoming, state.clone()));
    }
}

async fn handle_tcp_conn(conn: TcpStream, state: State) {
    if let Err(e) = handle_tcp_conn_inner(conn, state).await {
        error!(e = ?e, "Connection failed");
    }
}

async fn handle_tcp_conn_inner(mut conn: TcpStream, state: State) -> eyre::Result<()> {
    let peer_addr = conn.peer_addr()?;
    let local_addr = conn.local_addr()?;

    let span = info_span!("edge",
        peer_addr = %peer_addr,
        local_addr = %local_addr,
        app.id = field::Empty,
    );

    async move {
        let route = state
            .catalog()
            .route(local_addr)
            .ok_or(eyre!("Unable to route request!"))?;

        let balanced = state
            .balance(&route)
            .ok_or(eyre!("No service instance found!"))?;

        debug!(balanced = ?balanced, "Balanced connection");

        let mut egress = TcpStream::connect(balanced.0)
            .await
            .inspect_err(|e| error!(e = ?e, "Unable to connect to remote"))?;

        debug!("Transferring...");

        tokio::io::copy_bidirectional(&mut conn, &mut egress)
            .await
            .inspect_err(|e| error!(e = ?e, "Error during transfer"))?;

        Ok(())
    }
    .instrument(span)
    .await
}
