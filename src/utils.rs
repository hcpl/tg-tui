use time;

use error;


pub fn now() -> error::Result<String> {
    time::strftime("%H:%M:%S", &time::now()).map_err(Into::into)
}
