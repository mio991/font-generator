use std::io::Seek;

use byteorder::ReadBytesExt;

use crate::layout::{LayoutError, Layoutable, Layouted, Layouter, Reservation};

use super::search::SearchData;

pub trait LayoutedTable: Layouted {
    fn tag(&self) -> [u8; 4];
}

pub trait LayoutableTable: Layoutable<Box<dyn LayoutedTable>> {
    fn tag(&self) -> [u8; 4];
}

pub struct File {
    tables: Vec<Box<dyn LayoutableTable>>,
}

impl File {
    pub fn new_with_tables(mut tables: Vec<Box<dyn LayoutableTable>>) -> Self {
        tables.sort_by_key(|t| String::from(std::str::from_utf8(&t.tag()).unwrap_or("_MISSING_")));
        Self { tables }
    }
}

impl Layoutable<Box<dyn Layouted>> for File {
    fn layout(&self, layouter: &mut Layouter) -> Box<dyn Layouted> {
        let reservation = layouter.reserve(self.tables.len() * 16 + 12);

        let tables: Vec<_> = self
            .tables
            .iter()
            .map(|tabel| {
                println!(
                    "Layout Table: {:?}",
                    std::str::from_utf8(&tabel.tag()).unwrap_or("MISSING")
                );
                tabel.layout(layouter)
            })
            .collect();

        Box::new(LayoutedFile {
            requires_pass: true,
            reservation,
            tables,
        })
    }
}

struct LayoutedFile {
    requires_pass: bool,
    reservation: Reservation,
    tables: Vec<Box<dyn LayoutedTable>>,
}

impl Layouted for LayoutedFile {
    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn requires_another_pass(&self) -> bool {
        self.requires_pass || self.tables.iter().any(|l| l.requires_another_pass())
    }

    fn pass(&mut self, current_file: &[u8]) -> Result<(), LayoutError> {
        use byteorder::{WriteBytesExt, BE};
        use std::io::Write;

        for table in self.tables.iter_mut() {
            table.pass(current_file)?;
        }

        let search_data = SearchData::for_length(self.tables.len() as u16);
        let mut writer = self.reservation.writer();

        writer.write_all(b"OTTO")?; // Magic Number
        writer.write_u16::<BE>(self.tables.len() as u16)?;

        writer.write_u16::<BE>(search_data.search_range)?;
        writer.write_u16::<BE>(search_data.entry_selector)?;
        writer.write_u16::<BE>(search_data.range_shift)?;

        for table in self.tables.iter() {
            writer.write_all(&table.tag())?;
            writer.write_u32::<BE>(checksum(table.reservation())?)?;
            writer.write_u32::<BE>(table.reservation().offset() as u32)?;
            writer.write_u32::<BE>(table.reservation().len() as u32)?;
        }

        self.requires_pass = false;

        Ok(())
    }
}

fn checksum(reservation: &Reservation) -> std::io::Result<u32> {
    use byteorder::BE;
    use std::io::Read;

    let mut reader = reservation.reader();

    let mut sum: u32 = 0;
    let mut remaining = reader.seek(std::io::SeekFrom::End(0))?;

    reader.seek(std::io::SeekFrom::Start(0))?;

    while remaining >= 4 {
        remaining -= 4;

        sum = sum.wrapping_add(reader.read_u32::<BE>()?);
    }

    if remaining > 0 {
        let mut buffer = Vec::with_capacity(4);
        reader.read_to_end(&mut buffer)?;

        buffer.resize(4, 0);

        sum += buffer.as_slice().read_u32::<BE>()?;
    }

    Ok(sum)
}
