use std::{
    cell::{RefCell, RefMut},
    io::Cursor,
    rc::Rc,
};

pub trait Layouted {
    fn requires_another_pass(&self) -> bool;
    fn pass(&mut self);
}

pub trait Layoutable {
    fn layout(&self, layouter: &mut Layouter) -> Box<dyn Layouted>;
}
#[derive(Debug)]
pub struct Layouter {
    current_length: usize,
    buffer: Vec<Rc<RefCell<Cursor<Box<[u8]>>>>>,
}

impl Layouter {
    pub fn new() -> Self {
        Self {
            current_length: 0,
            buffer: Vec::new(),
        }
    }

    pub fn reserve(&mut self, size: usize) -> Reservation {
        let offset = self.current_length;
        self.current_length += size;

        let buffer = Rc::new(RefCell::new(Cursor::new(vec![0; size].into())));

        self.buffer.push(buffer.clone());

        Reservation { offset, buffer }
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
    buffer: Rc<RefCell<Cursor<Box<[u8]>>>>,
}

impl Reservation {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn writer(&mut self) -> RefMut<impl std::io::Write> {
        self.buffer.borrow_mut()
    }
}

#[cfg(test)]
mod test {
    use std::io::Write;

    use super::*;

    #[test]
    fn can_create_a_reservation() {
        let mut layouter = Layouter::new();

        layouter.reserve(8);
    }

    #[test]
    fn reservations_have_the_correct_offset() {
        let mut layouter = Layouter::new();

        layouter.reserve(8);

        let reservation = layouter.reserve(2);

        assert_eq!(8, reservation.offset());
    }

    #[test]
    fn can_write_to_a_reservation() -> std::io::Result<()> {
        let mut layouter = Layouter::new();

        let mut reservation = layouter.reserve(8);

        reservation.writer().write_all(b"abcdefgh")?;

        Ok(())
    }

    #[test]
    fn result_contains_what_was_written() -> std::io::Result<()> {
        let mut layouter = Layouter::new();

        layouter.reserve(4).writer().write_all(b"abcd")?;

        layouter.reserve(4).writer().write_all(b"efgh")?;

        let result = layouter.get_result();

        assert_eq!(result, b"abcdefgh".to_vec());

        Ok(())
    }

    #[test]
    fn can_not_write_more_than_reserved() {
        let mut layouter = Layouter::new();

        let result = layouter.reserve(4).writer().write_all(b"abcdefg");

        assert!(result.is_err());
    }
}
