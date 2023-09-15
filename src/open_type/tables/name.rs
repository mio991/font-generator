use crate::{
    layout::Reservation,
    open_type::{LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

#[derive(Debug, Clone)]
pub struct NameRecord {
    pub name_id: u16,
    pub content: String,
}

#[derive(Debug, Clone)]
pub struct Name {
    pub names: Vec<NameRecord>,
}

impl Layoutable<Box<dyn LayoutedTable>> for Name {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        let prepared_records: Vec<_> = self.names.iter().map(From::from).collect();

        let total_num_of_chars: usize = prepared_records
            .iter()
            .map(|r: &PreparedNameRecord| r.data.len())
            .sum();

        Box::new(LayoutedName {
            reservation: layouter
                .reserve(6 + (12 * prepared_records.len()) + (total_num_of_chars * 2)),
            requires_another_pass: true,
            names: prepared_records,
        })
    }
}

impl LayoutableTable for Name {
    fn tag(&self) -> [u8; 4] {
        *b"name"
    }
}

struct PreparedNameRecord {
    name_id: u16,
    data: Vec<u16>,
}

impl From<&NameRecord> for PreparedNameRecord {
    fn from(value: &NameRecord) -> Self {
        Self {
            name_id: value.name_id,
            data: str::encode_utf16(&value.content).collect(),
        }
    }
}

struct LayoutedName {
    reservation: Reservation,
    requires_another_pass: bool,
    names: Vec<PreparedNameRecord>,
}

impl Layouted for LayoutedName {
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

        writer.write_u16::<BE>(0)?; // Version
        writer.write_u16::<BE>(self.names.len() as u16)?;

        writer.write_u16::<BE>(12 * self.names.len() as u16)?; // StorageOffset

        let mut start_offset = 0;

        for record in self.names.iter() {
            writer.write_u16::<BE>(0)?; // PlatformID
            writer.write_u16::<BE>(4)?; // EncodingID
            writer.write_u16::<BE>(0)?; // LanguageID

            writer.write_u16::<BE>(record.name_id)?;

            let str_length = (record.data.len() * 2) as u16;
            writer.write_u16::<BE>(str_length)?;
            writer.write_u16::<BE>(start_offset)?;

            start_offset += str_length
        }

        for record in self.names.iter() {
            for char in record.data.iter() {
                writer.write_u16::<BE>(*char)?;
            }
        }

        Ok(())
    }
}

impl LayoutedTable for LayoutedName {
    fn tag(&self) -> [u8; 4] {
        *b"name"
    }
}
