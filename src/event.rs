#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventType {
    View,
    Share,
    Like
}

#[derive(Debug, Clone)]
pub struct Event {
    user_id: String,
    article_id: String,
    timestamp: chrono::DateTime<chrono::Utc>,
    event_type: EventType,
}

impl Event {
    fn new(user_id: String, article_id: String, timestamp: chrono::DateTime<chrono::Utc>, event_type: EventType) -> Self {
        Event {
            user_id,
            article_id,
            timestamp,
            event_type,
        }
    }

    pub fn from_csv_record(record: &csv::StringRecord) -> Result<Self, String> {
        if record.len() != 4 {
            return Err("Record does not have exactly 4 fields".to_string());
        }

        let user_id = record.get(0).ok_or("Missing user_id")?.to_string();
        let article_id = record.get(1).ok_or("Missing article_id")?.to_string();
        let timestamp_str = record.get(2).ok_or("Missing timestamp")?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp_str)
            .map_err(|e| format!("Invalid timestamp format: {}", e))?
            .with_timezone(&chrono::Utc);
        let event_type_str = record.get(3).ok_or("Missing event_type")?;
        let event_type = match event_type_str {
            "view" => EventType::View,
            "share" => EventType::Share,
            "like" => EventType::Like,
            _ => return Err(format!("Unknown event type: {}", event_type_str)),
        };

        Ok(Event::new(user_id, article_id, timestamp, event_type))
    }
}

mod tests {
    use super::*;
    use chrono::TimeZone;

    #[test]
    fn test_event_from_csv_record() {
        let record = csv::StringRecord::from(vec![
            "user123",
            "article456",
            "2025-05-30T12:34:56Z",
            "view"
        ]);
        let event = Event::from_csv_record(&record).unwrap();
        assert_eq!(event.user_id, "user123");
        assert_eq!(event.article_id, "article456");
        assert_eq!(
            event.timestamp,
            chrono::Utc.with_ymd_and_hms(2025, 5, 30, 12, 34, 56).single().unwrap()
        );
        assert_eq!(event.event_type, EventType::View);
    }

    #[test]
    fn test_event_from_csv_record_invalid_timestamp() {
        let record = csv::StringRecord::from(vec![
            "user123",
            "article456",
            "invalid-timestamp",
            "view"
        ]);
        let result = Event::from_csv_record(&record);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Invalid timestamp format: input contains invalid characters");
    }
    #[test]
    fn test_event_from_csv_record_missing_fields() {
        let record = csv::StringRecord::from(vec![
            "user123",
            "article456",
            "2025-05-30T12:34:56Z"
        ]);
        let result = Event::from_csv_record(&record);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Record does not have exactly 4 fields");
    }
    #[test]
    fn test_event_from_csv_record_unknown_event_type() {
        let record = csv::StringRecord::from(vec![
            "user123",
            "article456",
            "2025-05-30T12:34:56Z",
            "unknown"
        ]);
        let result = Event::from_csv_record(&record);
        assert!(result.is_err());
        assert_eq!(result.err().unwrap(), "Unknown event type: unknown");
    }
}
