use async_trait::async_trait;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::apis::garoon::{GaroonClient, GaroonEvent};


pub struct GaroonAuth {
    pub user_id: String,
    pub password: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GaroonEventResponse {
    events: Vec<GaroonEvent>,
}

pub struct GaroonRestClient {
    client: Client,
    base_url: String,
    auth: GaroonAuth,
}
impl GaroonRestClient {
    pub fn new(base_url: String, auth: GaroonAuth) -> Self {
        Self {
            client: Client::new(),
            base_url,
            auth,
        }
    }

    fn get_events_path(&self) -> &str {
        "/api/v1/schedule/events"
    }
}
#[async_trait]
impl GaroonClient for GaroonRestClient {
    async fn get_events(&self) -> Result<Vec<GaroonEvent>, reqwest::Error> {
        let url = format!("{}{}", self.base_url, self.get_events_path());
        let response = self.client
            .get(&url)
            .basic_auth(&self.auth.user_id, Some(&self.auth.password))
            .send()
            .await?
            .error_for_status()?
            .json::<GaroonEventResponse>()
            .await?;

        Ok(response.events)
    }
}

#[cfg(test)]
mod tests {
    use reqwest::StatusCode;
    use wiremock::{Mock, MockServer, ResponseTemplate};
    use wiremock::matchers::{method, path};
    use crate::apis::garoon_rest::*;
    use crate::apis::garoon::*;

    #[tokio::test]
    async fn get_events_正常系() {
        // Setup: モックサーバーを起動し、Garoon APIのレスポンスを設定する
        let mock_server = MockServer::start().await;
        let garoon_response = GaroonEventResponse {
            events: vec![GaroonEvent {
                subject: "会議".to_string(),
                attendees: vec![GaroonAttendee {
                    name: "山田太郎".to_string(),
                }],
                start: GaroonDateTime {
                    dateTime: "2024-05-10T09:00:00+09:00".to_string(),
                    timeZone: "Asia/Tokyo".to_string(),
                },
                end: GaroonDateTime {
                    dateTime: "2024-05-10T17:00:00+09:00".to_string(),
                    timeZone: "Asia/Tokyo".to_string(),
                },
            }],
        };
        
        let response = ResponseTemplate::new(200).set_body_json(&garoon_response);
        Mock::given(method("GET")).and(path("/api/v1/schedule/events")).respond_with(response).mount(&mock_server).await;
        
        let auth = GaroonAuth {
            user_id: "user".to_string(),
            password: "password".to_string(),
        };
        let client = GaroonRestClient::new(mock_server.uri(), auth);
        
        // Exercise: GaroonClient#get_eventsを実行する
        let result = client.get_events().await;
        
        // Asserts
        assert!(result.is_ok());
        let actual = result.unwrap();
        assert_eq!(actual, garoon_response.events);
    }
    
    #[tokio::test]
    async fn get_events_認証失敗() {
        // Setup: モックサーバーを起動し、Garoon APIのレスポンスを設定する
        let mock_server = MockServer::start().await;
        let response = ResponseTemplate::new(401);
        Mock::given(method("GET")).and(path("/api/v1/schedule/events")).respond_with(response).mount(&mock_server).await;
        
        let auth = GaroonAuth {
            user_id: "wrong_user".to_string(),
            password: "wrong_password".to_string(),
        };
        let client = GaroonRestClient::new(mock_server.uri(), auth);
        
        // Exercise: GaroonClient#get_eventsを実行する
        let result = client.get_events().await;
        
        // Asserts
        assert!(result.is_err());
        let actual = result.unwrap_err();
        assert_eq!(actual.status(), Some(StatusCode::UNAUTHORIZED));
    }
}
