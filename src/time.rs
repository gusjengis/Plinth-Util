use chrono::{Local, NaiveDateTime, TimeZone};
use js_sys::Date;

pub fn now() -> f64 {
    Date::now()
}

pub fn format_timestamp(timestamp_ms: i64) -> String {
    let datetime = NaiveDateTime::from_timestamp_millis(timestamp_ms).expect("Invalid timestamp");
    let local_datetime = Local.from_utc_datetime(&datetime);

    let formatted = local_datetime.format("%m-%d-%Y: %I:%M %P").to_string();
    formatted.replace("am", "a").replace("pm", "p")
}
