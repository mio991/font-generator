use crate::{
    layout::Reservation,
    open_type::{LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

pub struct MaxP {
    pub number_of_glyphs: u16,
}

impl LayoutableTable for MaxP {
    fn tag(&self) -> [u8; 4] {
        *b"maxp"
    }
}

impl Layoutable<Box<dyn LayoutedTable>> for MaxP {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedMaxP {
            requires_another_pass: true,
            number_of_glyphs: self.number_of_glyphs,
            reservation: layouter.reserve(6),
        })
    }
}

struct LayoutedMaxP {
    requires_another_pass: bool,
    reservation: Reservation,
    number_of_glyphs: u16,
}

impl LayoutedTable for LayoutedMaxP {
    fn tag(&self) -> [u8; 4] {
        *b"maxp"
    }
}

impl Layouted for LayoutedMaxP {
    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::LayoutError> {
        use byteorder::{WriteBytesExt, BE};

        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();

        writer.write_i32::<BE>(0x00005000)?;
        writer.write_u16::<BE>(self.number_of_glyphs)?;

        Ok(())
    }
}
