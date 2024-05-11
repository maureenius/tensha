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

#[cfg(test)]
mod tests {
    #[test]
    fn test_initialize_rest_clients() {
        use super::initialize_rest_clients;
        use std::env;

        env::set_var("GAROON_BASE_URL", "https://example.com");
        env::set_var("GAROON_USER_ID", "user");
        env::set_var("GAROON_PASSWORD", "password");

        let result = initialize_rest_clients();
        assert!(result.is_ok());
    }
}
