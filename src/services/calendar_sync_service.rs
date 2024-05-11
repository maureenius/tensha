use anyhow::Result;

use crate::apis::garoon::GaroonClient;
use crate::models::event::Event;

pub struct CalendarSyncService<G: GaroonClient> {
    client: G,
}
impl<G> CalendarSyncService<G> where G: GaroonClient + Send + Sync {
    pub fn new(client: G) -> Self {
        CalendarSyncService { client }
    }
    
    pub async fn sync_events(&self) -> Result<(), anyhow::Error> {
        todo!()
    }
    
    pub async fn get_garoon_events(&self) -> Result<Vec<Event>, anyhow::Error> {
        let events = self.client
            .get_events()
            .await?
            .iter()
            .map(|garoon_event| Event::from(garoon_event.clone()))
            .collect();
        
        Ok(events)
    }
}

#[cfg(test)]
mod tests {
    use crate::apis::garoon::{GaroonDateTime, GaroonEvent, MockGaroonClient};
    use crate::models::event::Title;
    use crate::services::calendar_sync_service::CalendarSyncService;

    #[tokio::test]
    async fn test_sync_events_正常系() {
        // Setup: GaroonClientのモックを作成し、get_eventsメソッドの戻り値を設定する
        let mut garoon_client = MockGaroonClient::new();
        garoon_client.expect_get_events()
            .times(1)
            .return_once(|| Ok(vec![
                GaroonEvent {
                    subject: "会議".to_string(),
                    attendees: vec![],
                    start: GaroonDateTime {
                        dateTime: "2021-01-01T00:00:00+09:00".to_string(),
                        timeZone: "Asia/Tokyo".to_string(),
                    },
                    end: GaroonDateTime {
                        dateTime: "2021-01-01T01:00:00+09:00".to_string(),
                        timeZone: "Asia/Tokyo".to_string(),
                    },
                }
            ]));
        
        // Exercise: CalendarSyncServiceを作成し、sync_eventsメソッドを呼び出す
        let service = CalendarSyncService { client: garoon_client };
        let result = service.sync_events().await;
        
        // Assert: 戻り値がOkであることを検証する
        assert!(result.is_ok());
    }
    
    #[tokio::test]
    async fn test_get_garoon_events_正常系() {
        // Setup: GaroonClientのモックを作成し、get_eventsメソッドの戻り値を設定する
        let mut garoon_client = MockGaroonClient::new();
        garoon_client.expect_get_events()
            .times(1)
            .return_once(|| Ok(vec![
                GaroonEvent {
                    subject: "会議".to_string(),
                    attendees: vec![],
                    start: GaroonDateTime {
                        dateTime: "2021-01-01T09:00:00+09:00".to_string(),
                        timeZone: "Asia/Tokyo".to_string(),
                    },
                    end: GaroonDateTime {
                        dateTime: "2021-01-01T10:00:00+09:00".to_string(),
                        timeZone: "Asia/Tokyo".to_string(),
                    },
                }
            ]));
        
        // Exercise: CalendarSyncServiceを作成し、get_garoon_eventsメソッドを呼び出す
        let service = CalendarSyncService { client: garoon_client };
        let result = service.get_garoon_events().await;
        
        // Assert: 戻り値がOkであり、GaroonEventからEventに変換されていることを検証する
        assert!(result.is_ok());
        let events = result.unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].title, Title::new("会議".to_string()));
        assert_eq!(events[0].duration.start.to_rfc3339(), "2021-01-01T00:00:00+00:00");
        assert_eq!(events[0].duration.end.to_rfc3339(), "2021-01-01T01:00:00+00:00");
    }
}