use std::fmt;

use chrono::prelude::{DateTime, TimeZone, Local};


pub fn strtime<Tz>(date_time: &DateTime<Tz>) -> String
    where
        Tz: TimeZone,
        Tz::Offset: fmt::Display,
{
    date_time.format("%H:%M:%S").to_string()
}

pub fn local_strnow() -> String {
    strtime(&Local::now())
}
