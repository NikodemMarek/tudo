use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[derive(Clone, Debug)]
pub enum TimestampType {
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
}

pub mod formatter {
    use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
    use tui::style::Color;

    use super::TimestampType;

    pub fn absolute(timestamp: &TimestampType) -> (String, Color) {
        match timestamp {
            TimestampType::Date(date) => (date.format("%F").to_string(), Color::White),
            TimestampType::Time(time) => (time.format("%F").to_string(), Color::White),
            TimestampType::DateTime(datetime) => (datetime.format("%F").to_string(), Color::White),
        }
    }

    pub fn relative(timestamp: &TimestampType) -> (String, Color) {
        match timestamp {
            TimestampType::Date(date) => relative_date(date),
            TimestampType::Time(time) => relative_time(time),
            TimestampType::DateTime(datetime) => relative_datetime(datetime),
        }
    }

    fn relative_date(date: &NaiveDate) -> (String, Color) {
        let now = chrono::Utc::now().naive_utc().date();
        let diff = date.to_owned() - now;

        let days = diff.num_days();

        (
            match days {
                ..=-14 => format!("{} weeks ago", (days / 7).abs()),
                -13..=-7 => "1 week ago".to_string(),
                -6..=-2 => format!("{} days ago", days.abs()),
                -1 => "yesterday".to_string(),
                0 => "today".to_string(),
                1 => "tomorrow".to_string(),
                2..=6 => format!("in {} days", days),
                7..=13 => "in 1 week".to_string(),
                14.. => format!("in {} weeks", (days / 7)),
            },
            match days {
                ..=-1 => Color::Red,
                0 => Color::Yellow,
                0.. => Color::Green,
            },
        )
    }

    fn relative_time(time: &NaiveTime) -> (String, Color) {
        let now = chrono::Utc::now().naive_utc().time();
        let diff = time.to_owned() - now;

        let minutes = diff.num_minutes();

        (
            match minutes {
                ..=-120 => format!("{} hours ago", (minutes / 60).abs()),
                -119..=-60 => "1 hour ago".to_string(),
                -59..=-2 => format!("{} minutes ago", minutes.abs()),
                -1 => "1 minute ago".to_string(),
                0 => "now".to_string(),
                1 => "in 1 minute".to_string(),
                2..=59 => format!("in {} minutes", minutes),
                60..=119 => "in 1 hour".to_string(),
                120.. => format!("in {} hours", (minutes / 60)),
            },
            match minutes {
                ..=-1 => Color::Red,
                0 => Color::Yellow,
                0.. => Color::Green,
            },
        )
    }

    fn relative_datetime(datetime: &NaiveDateTime) -> (String, Color) {
        let now = chrono::Utc::now().naive_utc();
        let diff = datetime.to_owned() - now;

        let days = diff.num_days();

        if days != 0 {
            relative_date(&(datetime.date()))
        } else {
            relative_time(&(datetime.time()))
        }
    }
}
