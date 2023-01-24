mod document;
mod search;
mod tables;
mod types;

pub use document::*;
pub use search::*;
pub use tables::*;
pub use types::*;

use std::{fmt::Debug, io::Write};

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

pub trait Object: Debug {
    fn get_size(&self) -> usize;
    fn write(&self, writer: &mut dyn Write) -> Result<(), WriteError>;
}

pub trait Table: Object {
    fn get_tag(&self) -> Tag;
}
