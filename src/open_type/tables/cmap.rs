use byteorder::{WriteBytesExt, BE};

use crate::{
    layout::{Layoutable, Layouted, Reservation},
    open_type::{search::SearchData, LayoutableTable, LayoutedTable},
};

#[derive(Debug, Clone)]
pub struct CharacterRange {
    pub start: u16,
    pub end: u16,
    pub start_id: u16,
}

#[derive(Debug)]
pub struct CMap {
    ranges: Vec<CharacterRange>,
}

impl CMap {
    pub fn new_with_ranges(ranges: Vec<CharacterRange>) -> Self {
        Self { ranges }
    }
}

impl LayoutableTable for CMap {
    fn tag(&self) -> [u8; 4] {
        *b"cmap"
    }
}

impl Layoutable<Box<dyn LayoutedTable>> for CMap {
    fn layout(&self, layouter: &mut crate::layout::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedCMap {
            requires_another_pass: true,
            reservation: layouter.reserve(18 + 8 * (self.ranges.len() + 1) + 10),
            ranges: self.ranges.clone(),
        })
    }
}

struct LayoutedCMap {
    requires_another_pass: bool,
    reservation: Reservation,
    ranges: Vec<CharacterRange>,
}

impl LayoutedTable for LayoutedCMap {
    fn tag(&self) -> [u8; 4] {
        *b"cmap"
    }
}

impl Layouted for LayoutedCMap {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::layout::LayoutError> {
        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();
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

        let mut ranges = self.ranges.clone();
        ranges.push(CharacterRange {
            start: 0xffff,
            end: 0xffff,
            start_id: 0,
        });

        let seg_count = ranges.len() as u16;
        let search_data = SearchData::for_length(seg_count);

        writer.write_u16::<BE>(seg_count * 2)?; // SegCountX2
        writer.write_u16::<BE>(search_data.search_range)?; // searchRange
        writer.write_u16::<BE>(search_data.entry_selector)?; // entrySelector
        writer.write_u16::<BE>(search_data.range_shift)?; // rangeShift

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

        Ok(())
    }
}
