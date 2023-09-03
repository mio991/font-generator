use byteorder::{ByteOrder, WriteBytesExt};

#[derive(Debug, Clone, Copy)]
pub struct Fixed {
    pub major: i16,
    pub minor: u16,
}

pub trait FixedWriteExt: std::io::Write {
    fn write_fixed<T: ByteOrder>(&mut self, fixed: &Fixed) -> std::io::Result<()> {
        self.write_i16::<T>(fixed.major)?;
        self.write_u16::<T>(fixed.minor)
    }
}

impl<W: std::io::Write + ?Sized> FixedWriteExt for W {}
