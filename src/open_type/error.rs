#[derive(Debug)]
pub enum WriteError {
    IoError(std::io::Error),
    Other(String),
}

impl From<std::io::Error> for WriteError {
    fn from(value: std::io::Error) -> Self {
        WriteError::IoError(value)
    }
}
