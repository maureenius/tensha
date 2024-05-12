use crate::config::{initialize_rest_clients, initialize_service};
use crate::services::export_events_service::export;

mod models;
mod apis;
mod services;
mod config;
mod output;
mod utils;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let garoon_client = initialize_rest_clients().unwrap();
    let service = initialize_service(garoon_client).unwrap();

    let events = service.get_garoon_events().await?;
    output::print_results(&events);
    
    export(&events, "./events.csv")?;
    
    Ok(())
}
