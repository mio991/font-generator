use std::{
    cell::{RefCell, RefMut},
    io::Cursor,
    rc::Rc,
};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum LayoutError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
}

pub trait Layouted {
    fn reservation(&self) -> &Reservation;
    fn requires_another_pass(&self) -> bool;
    fn pass(&mut self, current_file: &[u8]) -> Result<(), LayoutError>;
}

pub trait Layoutable<L> {
    fn layout(&self, layouter: &mut Layouter) -> L;
}

impl<L, T: Fn(&mut Layouter) -> L> Layoutable<L> for T {
    fn layout(&self, layouter: &mut Layouter) -> L {
        self(layouter)
    }
}

#[derive(Debug)]
pub struct Layouter {
    alignment: usize,
    current_length: usize,
    buffer: Vec<Rc<RefCell<Cursor<Box<[u8]>>>>>,
}

impl Layouter {
    pub fn new(alignment: usize) -> Self {
        Self {
            alignment,
            current_length: 0,
            buffer: Vec::new(),
        }
    }

    pub fn reserve(&mut self, mut len: usize) -> Reservation {
        let padding = (self.alignment - (len % self.alignment)) % self.alignment;

        println!("Length: {}\tPadding: {}", len, padding);

        len += padding;

        let offset = self.current_length;
        self.current_length += len;

        let buffer = Rc::new(RefCell::new(Cursor::new(vec![0; len].into())));

        self.buffer.push(buffer.clone());

        Reservation {
            offset,
            len,
            buffer,
        }
    }

    pub fn get_result(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.current_length);

        for buffer in self.buffer.iter() {
            result.extend(buffer.borrow().get_ref().iter());
        }

        assert_eq!(result.len(), self.current_length);

        return result;
    }
}

#[derive(Debug)]
pub struct Reservation {
    offset: usize,
    len: usize,
    buffer: Rc<RefCell<Cursor<Box<[u8]>>>>,
}

impl Reservation {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn writer(&mut self) -> RefMut<impl std::io::Write + std::io::Seek> {
        let mut writer = self.buffer.borrow_mut();
        writer.set_position(0);
        writer
    }

    pub fn reader(&self) -> RefMut<impl std::io::Read + std::io::Seek> {
        let mut reader = self.buffer.borrow_mut();
        reader.set_position(0);
        reader
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use super::*;

    #[test]
    fn can_create_a_reservation() {
        let mut layouter = Layouter::new(1);

        layouter.reserve(8);
    }

    #[test]
    fn reservations_have_the_correct_offset() {
        let mut layouter = Layouter::new(1);

        layouter.reserve(8);

        let reservation = layouter.reserve(2);

        assert_eq!(8, reservation.offset());
    }

    #[test]
    fn can_write_to_a_reservation() -> std::io::Result<()> {
        let mut layouter = Layouter::new(4);

        let mut reservation = layouter.reserve(8);

        reservation.writer().write_all(b"abcdefgh")?;

        Ok(())
    }

    #[test]
    fn result_contains_what_was_written() -> std::io::Result<()> {
        let mut layouter = Layouter::new(1);

        layouter.reserve(4).writer().write_all(b"abcd")?;

        layouter.reserve(4).writer().write_all(b"efgh")?;

        let result = layouter.get_result();

        assert_eq!(result, b"abcdefgh".to_vec());

        Ok(())
    }

    #[test]
    fn can_not_write_more_than_reserved() {
        let mut layouter = Layouter::new(1);

        let result = layouter.reserve(4).writer().write_all(b"abcdefg");

        assert!(result.is_err());
    }
}
