#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    CommandLine,
}
