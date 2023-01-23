use byteorder::{WriteBytesExt, BE};

use crate::open_type::{Fixed, FixedWriteExt, Sink, WriteDefered};

use super::super::Tag;

use super::Table;

#[derive(Debug)]
pub struct Head {
    revision: Fixed,
}

const TAG: Tag = *b"head";

impl Table for Head {
    fn get_tag(&self) -> crate::open_type::Tag {
        TAG
    }

    fn store_internal<S: Sink>(
        &self,
        writer: &mut WriteDefered<S>,
    ) -> Result<(), crate::open_type::WriteError> {
        writer.write_u16::<BE>(1)?; // Major Vaersion
        writer.write_u16::<BE>(0)?; // Minor Vaersion

        writer.write_fixed::<BE>(&self.revision)?;

        return Ok(());
    }
}
