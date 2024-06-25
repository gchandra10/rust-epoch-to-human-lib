use chrono::{NaiveDateTime, Utc, TimeZone};

/// Converts a human-readable date string to epoch time.
/// The input date format should be "YYYY-MM-DD HH:MM:SS".
/// Returns a Result with the epoch time in seconds or an error if parsing fails.
pub fn date_to_epoch(date_str: &str) -> Result<i64, chrono::ParseError> {
    let naive_datetime = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%d %H:%M:%S")?;
    let datetime = Utc.from_utc_datetime(&naive_datetime);
    Ok(datetime.timestamp())
}

/// Converts epoch time to a human-readable date string.
/// The output date format will be "YYYY-MM-DD HH:MM:SS".
pub fn epoch_to_date(epoch: i64) -> Result<String, &'static str> {
    match Utc.timestamp_opt(epoch, 0) {
        chrono::LocalResult::None => Err("Invalid epoch time"),
        chrono::LocalResult::Single(datetime) => Ok(datetime.format("%Y-%m-%d %H:%M:%S").to_string()),
        chrono::LocalResult::Ambiguous(_, _) => Err("Ambiguous epoch time"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_to_epoch() {
        let date_str = "2024-06-25 11:34:56";
        let epoch = date_to_epoch(date_str).unwrap();
        assert_eq!(epoch, 1719315296);
    }

    #[test]
    fn test_epoch_to_date() {
        let epoch = 1719315296;
        let date_str = epoch_to_date(epoch).unwrap();
        assert_eq!(date_str, "2024-06-25 11:34:56");
    }
}