#[derive(Debug, Default, Clone, PartialEq)]
pub enum GameError {
    #[default]
    None,
    UnknownCommand,
    InvalidArgument,
}
