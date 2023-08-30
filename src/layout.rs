use std::{
    cell::{RefCell, RefMut},
    sync::Arc,
};

#[derive(Debug)]
pub struct Layouter {
    current_length: usize,
    buffer: Vec<Arc<RefCell<[u8]>>>,
}

impl Layouter {
    pub fn new() -> Self {
        Self {
            current_length: 0,
            buffer: Vec::new(),
        }
    }

    pub fn reserve<const N: usize>(&mut self) -> Reservation<N> {
        let offset = self.current_length;
        let buffer = Arc::new(RefCell::new([0; N]));

        self.current_length += N;
        self.buffer.push(buffer.clone());

        Reservation { offset, buffer }
    }

    pub fn get_result(&self) -> Vec<u8> {
        let mut result = Vec::with_capacity(self.current_length);

        for arc in self.buffer.iter() {
            result.extend(arc.borrow().to_vec());
        }

        assert_eq!(result.len(), self.current_length);

        return result;
    }
}

#[derive(Debug)]
pub struct Reservation<const N: usize> {
    offset: usize,
    buffer: Arc<RefCell<[u8; N]>>,
}

impl<const N: usize> Reservation<N> {
    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn buffer(&mut self) -> RefMut<[u8; N]> {
        let cell = Arc::as_ref(&self.buffer);
        RefCell::borrow_mut(cell)
    }
}

pub trait Layouted {
    fn requires_another_pass(&self) -> bool;
    fn pass(&mut self);
}

pub trait Layoutable {
    fn layout(&self, layouter: &mut Layouter) -> Box<dyn Layouted>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_create_a_reservation() {
        let mut layouter = Layouter::new();

        layouter.reserve::<8>();
    }

    #[test]
    fn reservations_have_the_correct_offset() {
        let mut layouter = Layouter::new();

        layouter.reserve::<8>();

        let reservation = layouter.reserve::<2>();

        assert_eq!(8, reservation.offset());
    }

    #[test]
    fn can_write_to_a_reservation() {
        let mut layouter = Layouter::new();

        let mut reservation = layouter.reserve::<8>();

        *reservation.buffer() = *b"abcdefgh";
    }

    #[test]
    fn result_contains_what_was_written() {
        let mut layouter = Layouter::new();

        *layouter.reserve::<4>().buffer() = *b"abcd";

        *layouter.reserve::<4>().buffer() = *b"efgh";

        assert_eq!(layouter.get_result(), b"abcdefgh".to_vec());
    }
}
