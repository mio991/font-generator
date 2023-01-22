use byteorder::{WriteBytesExt, BE};
use std::{fmt::Debug, io::Write};

use super::super::{Tag, WriteError};

pub trait Table: Debug {
    fn get_tag(&self) -> Tag;
    fn store_internal(&self, writer: &mut dyn Write) -> Result<(), WriteError>;
}

impl dyn Table {
    pub fn store(&self, writer: &mut dyn Write, offset: u32) -> Result<Vec<u8>, WriteError> {
        let mut buffer = Vec::<u8>::new();

        self.store_internal(&mut buffer)?;

        writer.write_all(&self.get_tag())?;
        writer.write_u32::<BE>(0);
        writer.write_u32::<BE>(offset);
        writer.write_u32::<BE>(buffer.len() as u32);

        return Ok(buffer);
    }
}
