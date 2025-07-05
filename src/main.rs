use config::Config;
use eyre::eyre;
use state::State;
use tokio::{net::TcpListener, task::JoinSet};
use tracing::info;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    tracing_subscriber::fmt::init();

    let config_file = std::env::args().skip(1).next().ok_or(eyre!(
        "Please supply path to a config file as the first argument"
    ))?;
    let config = Config::from_toml(config_file).await?;
    info!("Loaded config {config:?}");

    let state = State::new(config).await?;

    let mut task_set = JoinSet::new();
    start_proxy_tasks(state, &mut task_set).await?;

    task_set.join_all().await;
    Ok(())
}

async fn start_proxy_tasks(state: State, join_set: &mut JoinSet<()>) -> eyre::Result<()> {
    for listener in state.config().listeners.iter() {
        info!("Listening on {listener}");
        let tcp_listener = TcpListener::bind(listener).await?;
        join_set.spawn(tcp_service::handle_edge_incoming(
            tcp_listener,
            state.clone(),
        ));
    }

    Ok(())
}
