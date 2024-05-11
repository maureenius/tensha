use chrono::{DateTime, Utc};
use crate::apis::garoon::GaroonEvent;

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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DateTimeRange {
    pub(crate) start: DateTime<Utc>,
    pub(crate) end: DateTime<Utc>,
}
impl DateTimeRange {
    pub fn new(start: DateTime<Utc>, end: DateTime<Utc>) -> Self {
        assert!(start <= end, "start must be before end");
        Self { start, end }
    }

    pub fn contains(&self, other: &DateTimeRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    pub fn overlaps(&self, other: &DateTimeRange) -> bool {
        self.start < other.end && self.end > other.start
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Attendee {
    display_name: String,
}

#[cfg(test)]
mod tests {
    mod date_time_range_test {
        use super::super::DateTimeRange;
        use chrono::{TimeZone, Utc};
        use crate::apis::garoon::{GaroonAttendee, GaroonDateTime, GaroonEvent};
        use crate::models::event::{Event, Title};

        #[test]
        fn test_new_datetime_range() {
            let start = Utc.with_ymd_and_hms(2024, 5, 10, 9, 0, 0).unwrap();
            let end = Utc.with_ymd_and_hms(2024, 5, 10, 17, 0, 0).unwrap();
            let range = DateTimeRange::new(start, end);
            assert_eq!(range.start, start);
            assert_eq!(range.end, end);
        }

        #[test]
        #[should_panic(expected = "start must be before end")]
        fn test_new_datetime_range_invalid() {
            let start = Utc.with_ymd_and_hms(2024, 5, 10, 17, 0, 0).unwrap();
            let end = Utc.with_ymd_and_hms(2024, 5, 10, 9, 0, 0).unwrap();
            let _range = DateTimeRange::new(start, end);
        }
        
        #[test]
        fn test_contains() {
            let start = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
            let end = Utc.with_ymd_and_hms(2021, 1, 2, 0, 0, 0).unwrap();
            let range = DateTimeRange::new(start, end);

            let contained_range = DateTimeRange::new(
                Utc.with_ymd_and_hms(2021, 1, 1, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2021, 1, 1, 13, 0, 0).unwrap(),
            );
            assert!(range.contains(&contained_range));

            let not_contained_range = DateTimeRange::new(
                Utc.with_ymd_and_hms(2021, 1, 1, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2021, 1, 2, 0, 0, 1).unwrap(),
            );
            assert!(!range.contains(&not_contained_range));
        }

        #[test]
        fn test_overlaps() {
            let start = Utc.with_ymd_and_hms(2021, 1, 1, 0, 0, 0).unwrap();
            let end = Utc.with_ymd_and_hms(2021, 1, 2, 0, 0, 0).unwrap();
            let range = DateTimeRange::new(start, end);

            let overlapping_range = DateTimeRange::new(
                Utc.with_ymd_and_hms(2021, 1, 1, 12, 0, 0).unwrap(),
                Utc.with_ymd_and_hms(2021, 1, 2, 0, 0, 0).unwrap(),
            );
            assert!(range.overlaps(&overlapping_range));

            let not_overlapping_range = DateTimeRange::new(
                Utc.with_ymd_and_hms(2021, 1, 2, 0, 0, 1).unwrap(),
                Utc.with_ymd_and_hms(2021, 1, 2, 12, 0, 0).unwrap(),
            );
            assert!(!range.overlaps(&not_overlapping_range));
        }
        
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
}
