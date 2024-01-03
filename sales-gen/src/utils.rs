use chrono::{Utc, DateTime, Duration, TimeZone};

#[derive(Clone)]
pub struct DateRange(pub DateTime<Utc>, pub DateTime<Utc>);
impl Iterator for DateRange {
    type Item = DateTime<Utc>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.0 <= self.1 {
            let next = self.0 + Duration::days(1);
            Some(std::mem::replace(&mut self.0, next))
        } else {
            None
        }
    }
}

pub fn format_date<Tz: TimeZone>(date: DateTime<Tz>) -> String {
    date.to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
}