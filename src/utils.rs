use std::fmt;

use chrono::DateTime;
use chrono::format::{Item as FormatItem, StrftimeItems};
use chrono::offset::{TimeZone, Local};

use error::{self, TgTuiError};


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

fn check_strftime_format(format: &str) -> error::Result<()> {
    for item in StrftimeItems::new(format) {
        match item {
            FormatItem::Error => bail_err!(TgTuiError::ChronoFormat { format: format.to_owned() }),
            _ => (),
        }
    }

    Ok(())
}

// Initially, we wanted a custom enum that contains all variants from `FormatItem`, but
// unfortunately this will not work with methods that expect an `Iterator` of `FormatItem` to be
// passed to them.
//
// So we are going with the `chrono`-provided `FormatItem`.
fn checked_strftime_items(format: &str) -> error::Result<StrftimeItems> {
    check_strftime_format(format)?;
    Ok(StrftimeItems::new(format))
}
