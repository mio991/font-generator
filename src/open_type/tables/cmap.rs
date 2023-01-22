use byteorder::{WriteBytesExt, BE};
use std::ops::Deref;

use super::super::{Tag, WriteError};

use super::{SearchData, Table};

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

    fn store_internal(&self, writer: &mut dyn std::io::Write) -> Result<(), WriteError> {
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

        let seg_count = self.len() as u16 + 1;
        let search_data = SearchData::for_length(seg_count);

        writer.write_u16::<BE>(seg_count * 2)?; // SegCountX2
        writer.write_u16::<BE>(search_data.search_range)?; // searchRange
        writer.write_u16::<BE>(search_data.entry_selector)?; // entrySelector
        writer.write_u16::<BE>(search_data.range_shift)?; // rangeShift

        let mut ranges = self.ranges.clone();
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

#[derive(Debug, Clone)]
pub struct CharacterRange {
    pub start: u16,
    pub end: u16,
    pub start_id: u16,
}
