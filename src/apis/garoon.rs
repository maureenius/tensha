use async_trait::async_trait;
use serde::{Deserialize, Serialize};
#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
#[async_trait]
pub trait GaroonGetEventsClient {
    async fn get(&self) -> Result<Vec<GaroonEvent>, reqwest::Error>;
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GaroonEvent {
    pub(crate) subject: String,
    pub(crate) attendees: Vec<GaroonAttendee>,
    pub(crate) start: GaroonDateTime,
    pub(crate) end: GaroonDateTime,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GaroonAttendee {
    pub(crate) name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct GaroonDateTime {
    pub(crate) dateTime: String,
    pub(crate) timeZone: String,
}
