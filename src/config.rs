use std::env;
use dotenv::dotenv;
use crate::apis::garoon::GaroonClient;
use crate::apis::garoon_rest::{GaroonAuth, GaroonRestClient};
use crate::services::calendar_sync_service::CalendarSyncService;

pub fn initialize_rest_clients() -> Result<GaroonRestClient, anyhow::Error> {
    dotenv().ok();

    let base_url = env::var("GAROON_BASE_URL")?;
    let user_id = env::var("GAROON_USER_ID")?;
    let password = env::var("GAROON_PASSWORD")?;

    let auth = GaroonAuth { user_id, password };

    Ok(GaroonRestClient::new(base_url, auth))
}

pub fn initialize_service<G: GaroonClient + Send + Sync>(client: G) -> Result<CalendarSyncService<G>, anyhow::Error> {
    Ok(CalendarSyncService::<G>::new(client))
}
