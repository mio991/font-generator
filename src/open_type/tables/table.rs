use byteorder::{WriteBytesExt, BE};
use std::{
    fmt::Debug,
    io::{Read, Seek, Write},
};

use crate::open_type::{Sink, WriteDefered};

use super::super::{Tag, WriteError};

pub trait Table: Debug {
    fn get_tag(&self) -> Tag;
    fn store_internal(&self, writer: &mut Box<dyn Sink>) -> Result<(u64, u64), WriteError>;
}

impl dyn Table {
    pub fn store(&self, writer: &mut dyn Sink, offset: u32) -> Result<Vec<u8>, WriteError> {
        let mut buffer = WriteDefered::new(std::io::Cursor::new(Vec::new()));

        self.store_internal(&mut buffer)?;

        let buffer = buffer.into_inner();

        writer.write_all(&self.get_tag())?;
        writer.write_u32::<BE>(0)?;
        writer.write_u32::<BE>(offset)?;
        writer.write_u32::<BE>(buffer.len() as u32)?;

        return Ok(buffer);
    }
}
