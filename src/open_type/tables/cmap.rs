use byteorder::{WriteBytesExt, BE};

use crate::{
    layout::{Layoutable, Layouted, Reservation},
    open_type::{LayoutableTable, LayoutedTable},
};

#[derive(Debug, Clone)]
pub struct CharacterRange {
    pub start: char,
    pub end: char,
    pub start_index: u32,
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
        writer.write_u16::<BE>(0)?; // Version
        writer.write_u16::<BE>(1)?; // num entries

        writer.write_u16::<BE>(0)?; // PlatformId
        writer.write_u16::<BE>(4)?; // EncodingId

        writer.write_u32::<BE>(12)?; // SubTableOffset

        // Subtable
        writer.write_u16::<BE>(12)?; // Format
        writer.write_u16::<BE>(0)?; // Reserved

        writer.write_u32::<BE>((self.ranges.len() as u32 * 12) + 16)?; // Length
        writer.write_u32::<BE>(0)?; // Language

        writer.write_u32::<BE>(self.ranges.len() as u32)?; // NumGroups

        for range in self.ranges.iter() {
            writer.write_u32::<BE>(range.start as u32)?; // Start char
            writer.write_u32::<BE>(range.end as u32)?; // End Char
            writer.write_u32::<BE>(range.start_index)?; // start glyph index
        }

        Ok(())
    }
}
