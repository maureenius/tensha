use chrono::{DateTime, Utc};

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

#[cfg(test)]
mod tests {
    mod date_time_range_test {
        use chrono::{TimeZone, Utc};

        use super::super::DateTimeRange;

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
    }
}
