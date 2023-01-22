use byteorder::{WriteBytesExt, BE};
use std::io::Write;

use super::{SearchData, Table, WriteError};

#[derive(Debug)]
pub struct Document {
    pub sfnt_version: SFNTVersion,
    pub tables: Vec<Box<dyn Table>>,
}

impl Document {
    pub fn store(&mut self, writer: &mut dyn Write) -> Result<(), WriteError> {
        // +4
        writer.write_u32::<BE>(self.sfnt_version as u32)?;

        let num_tables = self.tables.len() as u16;
        let search_data = SearchData::for_length(num_tables);

        writer.write_u16::<BE>(num_tables)?; // +2
        writer.write_u16::<BE>(search_data.search_range)?; // +2
        writer.write_u16::<BE>(search_data.entry_selector)?; // +2
        writer.write_u16::<BE>(search_data.range_shift)?; // +2

        let mut tables = Vec::<Vec<u8>>::new();
        let tables_offset = 12 + (num_tables as u32) * 16;

        self.tables.sort_by_key(|a| a.get_tag());

        for table in self.tables.iter() {
            let offset = tables_offset + tables.iter().map(|t| t.len() as u32).sum::<u32>();
            // writes 16 Bytes
            let mut table = table.store(writer, offset)?;

            // Pad
            while table.len() % 4 > 0 {
                table.push(0);
            }

            tables.push(table)
        }

        for table in tables {
            writer.write_all(&table)?;
        }

        return Ok(());
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SFNTVersion {
    TrueType = 0x00010000,
    OpenType = 0x4F54544F,
}
