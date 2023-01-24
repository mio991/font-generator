use std::{io::Write, ops::Deref};

use byteorder::{WriteBytesExt, BE};

use super::{SearchData, Table, WriteError};

#[derive(Debug)]
pub struct Document {
    pub sfnt_version: SFNTVersion,
    pub tables: Vec<Box<dyn Table>>,
}

impl Document {
    pub fn write(&self, writer: &mut dyn Write) -> Result<(), WriteError> {
        let num_tables = self.tables.len();
        let search_data = SearchData::for_length(num_tables as u16);

        let tables_offset = 12 + num_tables * 16;

        let mut tables: Vec<_> = self.tables.iter().map(|b| b.as_ref()).collect();
        tables.sort_by_key(|a| a.get_tag());

        let tables: Vec<_> = tables
            .into_iter()
            .scan(tables_offset, |offset, table| {
                let current = *offset;
                let size = table.get_size();
                *offset += size;
                Some((current, size, table))
            })
            .collect();

        writer.write_u32::<BE>(self.sfnt_version as u32)?; // +4
        writer.write_u16::<BE>(num_tables as u16)?; // +2
        writer.write_u16::<BE>(search_data.search_range)?; // +2
        writer.write_u16::<BE>(search_data.entry_selector)?; // +2
        writer.write_u16::<BE>(search_data.range_shift)?; // +2

        for (offset, size, table) in tables.clone() {
            writer.write_all(&table.get_tag())?;
            writer.write_u32::<BE>(0)?;
            writer.write_u32::<BE>(offset as u32)?;
            writer.write_u32::<BE>(size as u32)?;
        }

        for (_, _, table) in tables {
            table.write(writer)?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SFNTVersion {
    TrueType = 0x00010000,
    OpenType = 0x4F54544F,
}
