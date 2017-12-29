use serde::{Serialize, Serializer, Deserialize, Deserializer};
use serde::de::{self, Error as DeError};


#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    CommandLine,
}

impl Serialize for Mode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = match *self {
            Mode::Normal => "normal",
            Mode::Insert => "insert",
            Mode::Visual => "visual",
            Mode::CommandLine => "command_line",
        };

        s.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Mode {
    fn deserialize<D>(deserializer: D) -> Result<Mode, D::Error>
        where D: Deserializer<'de>
    {
        let s: String = Deserialize::deserialize(deserializer)?;

        let mode = match s.as_str() {
            "normal" => Mode::Normal,
            "insert" => Mode::Insert,
            "visual" => Mode::Visual,
            "command_line" => Mode::CommandLine,
            x => return Err(D::Error::invalid_value(de::Unexpected::Str(x), &"values!!!")),
        };

        Ok(mode)
    }
}
