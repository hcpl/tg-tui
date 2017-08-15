use time;

use error;


pub fn strtime(time: &time::Tm) -> error::Result<String> {
    time::strftime("%H:%M:%S", time).map_err(Into::into)
}

pub fn strnow() -> error::Result<String> {
    strtime(&time::now())
}
