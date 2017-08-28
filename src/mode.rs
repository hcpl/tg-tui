#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Mode {
    Normal,
    Insert,
    Visual,
    CommandLine,
}
