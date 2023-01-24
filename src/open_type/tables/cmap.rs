use byteorder::{WriteBytesExt, BE};
use std::{io::Write, ops::Deref};

use crate::open_type::{Object, SearchData, Table};

use super::super::{Tag, WriteError};

#[derive(Debug)]
pub struct CMap {
    pub ranges: Vec<CharacterRange>,
}

impl Deref for CMap {
    type Target = Vec<CharacterRange>;
    fn deref(&self) -> &Self::Target {
        &self.ranges
    }
}

const TAG: Tag = *b"cmap";

impl Table for CMap {
    fn get_tag(&self) -> Tag {
        TAG
    }
}

impl Object for CMap {
    fn get_size(&self) -> usize {
        18 + self.ranges.get_size()
    }

    fn write(&self, writer: &mut dyn Write) -> Result<(), WriteError> {
        // Header
        writer.write_u16::<BE>(0)?;
        writer.write_u16::<BE>(1)?;

        writer.write_u16::<BE>(0)?; // PlatformId
        writer.write_u16::<BE>(3)?; // EncodingId

        writer.write_u32::<BE>(12)?; // SubTableOffset

        // Subtable
        writer.write_u16::<BE>(4)?; // Format
        writer.write_u16::<BE>(0)?; // Length
        writer.write_u16::<BE>(0)?; // Language

        self.ranges.write(writer)
    }
}

#[derive(Debug, Clone)]
pub struct CharacterRange {
    pub start: u16,
    pub end: u16,
    pub start_id: u16,
}

impl Object for Vec<CharacterRange> {
    fn get_size(&self) -> usize {
        8 * (self.len() + 1) + 10
    }

    fn write(&self, writer: &mut dyn Write) -> Result<(), WriteError> {
        let seg_count = self.len() as u16 + 1;
        let search_data = SearchData::for_length(seg_count);

        writer.write_u16::<BE>(seg_count * 2)?; // SegCountX2
        writer.write_u16::<BE>(search_data.search_range)?; // searchRange
        writer.write_u16::<BE>(search_data.entry_selector)?; // entrySelector
        writer.write_u16::<BE>(search_data.range_shift)?; // rangeShift

        let mut ranges = self.clone();
        ranges.push(CharacterRange {
            start: 0xffff,
            end: 0xffff,
            start_id: 0,
        });

        for range in ranges.iter() {
            writer.write_u16::<BE>(range.end)?; // endCode[i]
        }

        writer.write_u16::<BE>(0)?; // reservedPad

        for range in ranges.iter() {
            writer.write_u16::<BE>(range.start)?; // startCode[i]
        }

        for range in ranges.iter() {
            writer.write_u16::<BE>(range.start_id.wrapping_sub(range.start))?; // idDelta[i]
        }

        for _range in ranges.iter() {
            writer.write_u16::<BE>(0)?; // idRangeOffset[i]
        }

        return Ok(());
    }
}
