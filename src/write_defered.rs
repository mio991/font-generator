use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom, Write},
};

pub trait WriteOp<const R: usize, T: Write + Seek + Read>:
    FnOnce(&mut T) -> std::io::Result<[u8; R]>
{
}
impl<const R: usize, T: Write + Seek + Read, F: FnOnce(&mut T) -> std::io::Result<[u8; R]>>
    WriteOp<R, T> for F
{
}

pub trait DeferedWrite {
    type Buffer: Write + Seek + Read;

    fn write_defered<const R: usize, Op: WriteOp<R, Self::Buffer> + 'static>(
        &mut self,
        operation: Op,
    ) -> std::io::Result<()>;
}

trait WriteOpInt<T: Write + Seek + Read>: FnOnce(&mut T) -> std::io::Result<()> {}
impl<T: Write + Seek + Read, F: FnOnce(&mut T) -> std::io::Result<()>> WriteOpInt<T> for F {}

pub struct WriteDefered<T: Write + Seek + Read> {
    inner: Box<T>,
    ops: HashMap<u64, Box<dyn WriteOpInt<T>>>,
}

impl<T: Write + Seek + Read> WriteDefered<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner: Box::new(inner),
            ops: HashMap::new(),
        }
    }

    pub fn resolve_pending(mut self) -> std::io::Result<T> {
        for (pos, op) in self.ops {
            op(&mut self.inner)?;
        }

        return Ok(*self.inner);
    }
}

impl<T: Write + Seek + Read> DeferedWrite for WriteDefered<T> {
    type Buffer = T;
    fn write_defered<const R: usize, Op: WriteOp<R, Self::Buffer> + 'static>(
        &mut self,
        operation: Op,
    ) -> std::io::Result<()> {
        let pos = self.inner.stream_position()?;
        dbg!(pos);
        let operation = Box::new(operation);

        self.inner.write_all(&[0; R])?;

        self.ops.insert(
            pos,
            Box::new(move |writer| -> std::io::Result<()> {
                let value = operation(writer)?;

                writer.seek(SeekFrom::Start(pos))?;
                let _pos = writer.stream_position()?;
                dbg!(_pos);
                writer.write_all(&value)?;

                return Ok(());
            }),
        );

        return Ok(());
    }
}

impl<T: Write + Seek + Read> std::ops::Deref for WriteDefered<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: Write + Seek + Read> std::ops::DerefMut for WriteDefered<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use crate::write_defered::DeferedWrite;

    use super::WriteDefered;

    #[test]
    fn test() -> std::io::Result<()> {
        let mut writer = WriteDefered::new(std::io::Cursor::new(Vec::new()));

        writer.write_all(b"a")?;
        writer.write_defered(|w| Ok(*b"bcd"))?;
        writer.write_all(b"e")?;

        let result = (writer.resolve_pending()?).into_inner();

        assert_eq!(result, b"abcde");

        return Ok(());
    }
}
