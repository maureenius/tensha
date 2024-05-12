use std::fs;
use std::path::Path;

use chrono::Local;
use serde::Serialize;

use crate::models::event::Event;

#[derive(Debug, Serialize)]
pub struct ExportedEvent {
    subject: String,
    start_date: String,
    start_time: String,
    end_date: String,
    end_time: String,
}

pub fn export(events: &Vec<Event>, path: impl AsRef<Path>) -> Result<(), anyhow::Error> {
    let csv_content = to_csv(events)?;
    fs::write(path, csv_content)?;

    Ok(())
}

fn to_csv(events: &Vec<Event>) -> Result<String, anyhow::Error> {
    let mut csv = String::new();
    csv.push_str("Subject,Start Date,Start Time,End Date,End Time\n");

    for event in events {
        let exported_event = ExportedEvent {
            subject: event.title.as_str(),
            start_date: event.duration.start.with_timezone(&Local).date_naive().to_string(),
            start_time: event.duration.start.with_timezone(&Local).time().to_string(),
            end_date: event.duration.end.with_timezone(&Local).date_naive().to_string(),
            end_time: event.duration.end.with_timezone(&Local).time().to_string(),
        };

        csv.push_str(&format!(
            "{},{},{},{},{}\n",
            exported_event.subject,
            exported_event.start_date,
            exported_event.start_time,
            exported_event.end_date,
            exported_event.end_time,
        ));
    }

    Ok(csv)
}

#[cfg(test)]
mod tests {
    use chrono::TimeZone;

    use crate::models::event::{Attendee, Event, Title};
    use crate::services::export_events_service::to_csv;
    use crate::utils::date_time_range::DateTimeRange;

    #[test]
    fn test_to_csv() {
        let events = vec![
            Event::new(
                Title::new("会議".to_string()),
                DateTimeRange::new(
                    chrono::Utc.with_ymd_and_hms(2021, 1, 1,0, 0, 0).unwrap(),
                    chrono::Utc.with_ymd_and_hms(2021, 1, 1, 1, 0, 0).unwrap(),
                ),
                vec![Attendee::new("user1".to_string())],
            ),
            Event::new(
                Title::new("打ち合わせ".to_string()),
                DateTimeRange::new(
                    chrono::Utc.with_ymd_and_hms(2021, 1, 2, 15, 0, 0).unwrap(),
                    chrono::Utc.with_ymd_and_hms(2021, 1, 2, 16, 0, 0).unwrap(),
                ),
                vec![Attendee::new("user2".to_string())],
            ),
        ];

        let csv = to_csv(&events).unwrap();

        assert_eq!(
            csv,
            "Subject,Start Date,Start Time,End Date,End Time\n\
            会議,2021-01-01,09:00:00,2021-01-01,10:00:00\n\
            打ち合わせ,2021-01-03,00:00:00,2021-01-03,01:00:00\n"
        );
    }
}
