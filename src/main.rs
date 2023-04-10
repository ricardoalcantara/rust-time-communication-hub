use dotenvy::dotenv;
use rust_time_communication_hub::{
    error::AppResult,
    hub::hub_server::HubServer,
    repository::repository_base::RepositoryBase,
    servers::{server_admin, server_sse},
};

#[tokio::main]
async fn main() -> AppResult {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let repository = RepositoryBase::new().await?;
    let hub_connection = HubServer::spawn(repository.clone());

    tokio::select! {
        _ = server_sse::start(hub_connection.clone(), repository.clone()) => {},
        _ = server_admin::start(hub_connection, repository) => {},
    }

    Ok(())
}
