use async_trait::async_trait;
use base64::prelude::*;
use reqwest::Client;
use reqwest::header::{ACCEPT, HeaderMap};
use serde::{Deserialize, Serialize};

use crate::apis::garoon::{GaroonGetEventsClient, GaroonEvent};

pub struct GaroonAuth {
    pub user_id: String,
    pub password: String,
}
impl GaroonAuth {
    pub fn cyboze_authorization(&self) -> String {
        let auth = format!("{}:{}", self.user_id, self.password);
        BASE64_STANDARD.encode(auth.as_bytes())
    }
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

    fn headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();
        headers.insert(ACCEPT, "application/json; charset=UTF-8".parse().unwrap());
        headers.insert("X-Cybozu-Authorization", self.auth.cyboze_authorization().parse().unwrap());

        headers
    }
}
#[async_trait]
impl GaroonGetEventsClient for GaroonRestClient {
    async fn get(&self) -> Result<Vec<GaroonEvent>, reqwest::Error> {
        let url = format!("{}{}", self.base_url, self.get_events_path());

        let response = self.client
            .get(&url)
            .headers(self.headers())
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
    use wiremock::matchers::{header, method, path};

    use crate::apis::garoon::*;
    use crate::apis::garoon_rest::*;

    #[tokio::test]
    async fn get_events_正常系() {
        // Setup: モックサーバーを起動し、Garoon APIのレスポンスを設定する
        let mock_server = MockServer::start().await;
        let expected_auth = "dXNlcjpwYXNzd29yZA==";  // base64("user:password")
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
        
        Mock::given(method("GET"))
            .and(path("/api/v1/schedule/events"))
            .and(header("X-Cybozu-Authorization", expected_auth))
            .respond_with(response)
            .mount(&mock_server)
            .await;

        let auth = GaroonAuth {
            user_id: "user".to_string(),
            password: "password".to_string(),
        };
        let client = GaroonRestClient::new(mock_server.uri(), auth);

        // Exercise: GaroonClient#get_eventsを実行する
        let result = client.get().await;

        // Asserts
        assert!(result.is_ok(), "Failed to get events: {:?}", result.err().unwrap());
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
        let result = client.get().await;

        // Asserts
        assert!(result.is_err());
        let actual = result.unwrap_err();
        assert_eq!(actual.status(), Some(StatusCode::UNAUTHORIZED));
    }
    
    #[test]
    fn test_headers() {
        let auth = GaroonAuth {
            user_id: "user".to_string(),
            password: "password".to_string(),
        };
        let client = GaroonRestClient::new("https://example.com".to_string(), auth);

        let headers = client.headers();
        assert_eq!(headers.get("Accept").unwrap(), "application/json; charset=UTF-8");
        assert_eq!(headers.get("X-Cybozu-Authorization").unwrap(), "dXNlcjpwYXNzd29yZA==");  // base64("user:password")
    }
}
