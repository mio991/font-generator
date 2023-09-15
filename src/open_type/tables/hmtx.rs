use crate::{
    layout::Reservation,
    open_type::{LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

#[derive(Debug, Clone)]
pub struct Hmtx {
    pub horizontal_metrics: Vec<HorizontalMetric>,
    pub left_side_bearings: Vec<i16>,
}

impl Layoutable<Box<dyn LayoutedTable>> for Hmtx {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedHmtx {
            requires_another_pass: true,
            reservation: layouter
                .reserve(self.horizontal_metrics.len() * 4 + self.left_side_bearings.len() * 2),
            table: self.clone(),
        })
    }
}

impl LayoutableTable for Hmtx {
    fn tag(&self) -> [u8; 4] {
        *b"hmtx"
    }
}

#[derive(Debug, Clone)]
pub struct HorizontalMetric {
    /** Advance width, in font design units. */
    pub advance_width: u16,
    /** Glyph left side bearing, in font design units. */
    pub left_side_bearing: i16,
}

struct LayoutedHmtx {
    reservation: Reservation,
    requires_another_pass: bool,
    table: Hmtx,
}

impl Layouted for LayoutedHmtx {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::LayoutError> {
        use byteorder::{WriteBytesExt, BE};

        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();

        for metric in self.table.horizontal_metrics.iter() {
            writer.write_u16::<BE>(metric.advance_width)?;
            writer.write_i16::<BE>(metric.left_side_bearing)?;
        }

        for lsb in self.table.left_side_bearings.iter() {
            writer.write_i16::<BE>(*lsb)?;
        }

        Ok(())
    }
}

impl LayoutedTable for LayoutedHmtx {
    fn tag(&self) -> [u8; 4] {
        self.table.tag()
    }
}
