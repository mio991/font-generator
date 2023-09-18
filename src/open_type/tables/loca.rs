use crate::{
    layout::Reservation,
    open_type::{LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

pub struct Loca {
    pub offsets: Vec<u32>,
}

impl Layoutable<Box<dyn LayoutedTable>> for Loca {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(LayoutedLoca {
            requires_another_pass: true,
            reservation: layouter.reserve(self.offsets.len() * 4),
            offsets: self.offsets.clone(),
        })
    }
}

impl LayoutableTable for Loca {
    fn tag(&self) -> [u8; 4] {
        *b"loca"
    }
}

struct LayoutedLoca {
    reservation: Reservation,
    requires_another_pass: bool,
    offsets: Vec<u32>,
}

impl Layouted for LayoutedLoca {
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

        for offset in self.offsets.iter().copied() {
            writer.write_u32::<BE>(offset)?;
        }

        Ok(())
    }
}

impl LayoutedTable for LayoutedLoca {
    fn tag(&self) -> [u8; 4] {
        *b"loca"
    }
}
