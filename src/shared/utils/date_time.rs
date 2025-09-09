use chrono::{DateTime, NaiveDate, Utc};

pub fn now() -> DateTime<Utc> {
    Utc::now()
}

pub fn today() -> NaiveDate {
    Utc::now().date_naive()
}

pub fn format_date(date: NaiveDate) -> String {
    date.format("%Y-%m-%d").to_string()
}

pub fn parse_date(date_str: &str) -> Result<NaiveDate, chrono::ParseError> {
    NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}
