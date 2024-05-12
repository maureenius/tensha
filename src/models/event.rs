use chrono::{DateTime, Utc};
use crate::apis::garoon::{GaroonEvent};
use crate::utils::date_time_range::DateTimeRange;

#[derive(Debug)]
pub struct Event {
    pub(crate) title: Title,
    pub(crate) duration: DateTimeRange,
    attendees: Vec<Attendee>,
}
impl Event {
    pub fn new(title: Title, duration: DateTimeRange, attendees: Vec<Attendee>) -> Self {
        Self {
            title,
            duration,
            attendees,
        }
    }
}
impl From<GaroonEvent> for Event {
    fn from(value: GaroonEvent) -> Self {
        let title = Title::new(value.subject);
        let start = DateTime::parse_from_rfc3339(&value.start.dateTime)
            .unwrap()
            .with_timezone(&Utc);
        let end = DateTime::parse_from_rfc3339(&value.end.dateTime)
            .unwrap()
            .with_timezone(&Utc);
        let duration = DateTimeRange::new(start, end);
        let attendees = value
            .attendees
            .iter()
            .map(|attendee| Attendee {
                display_name: attendee.name.clone(),
            })
            .collect();

        Self::new(title, duration, attendees)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Title(String);
impl Title {
    pub fn new(title: String) -> Self {
        Self(title)
    }

    pub fn as_str(&self) -> String {
        self.0.clone()
    }
}
impl From<String> for Title {
    fn from(value: String) -> Self {
        Title::new(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attendee {
    display_name: String,
}
impl Attendee {
    pub fn new(display_name: String) -> Self {
        Self { display_name }
    }
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};
    use crate::apis::garoon::{GaroonAttendee, GaroonDateTime, GaroonEvent};
    use crate::models::event::{Event, Title};

    #[test]
    fn test_from_garoon_event() {
        let garoon_event = GaroonEvent {
            subject: "会議".to_string(),
            attendees: vec![
                GaroonAttendee {
                    name: "user1".to_string()
                }
            ],
            start: GaroonDateTime {
                dateTime: "2021-01-01T09:00:00+09:00".to_string(),
                timeZone: "Asia/Tokyo".to_string(),
            },
            end: GaroonDateTime {
                dateTime: "2021-01-01T10:00:00+09:00".to_string(),
                timeZone: "Asia/Tokyo".to_string(),
            },
        };
        let event = Event::from(garoon_event);
        assert_eq!(event.title, Title::new("会議".to_string()));
        assert_eq!(event.attendees[0].display_name, "user1".to_string());
        assert_eq!(event.duration.start, Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap());
        assert_eq!(event.duration.end, Utc.with_ymd_and_hms(2021, 1, 1, 1, 0, 0).unwrap());
    }
}
