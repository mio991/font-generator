use std::{
    collections::HashMap,
    io::{Read, Seek, SeekFrom, Write},
};

pub trait Buffer: Write + Seek + Read {}
impl<T: Write + Seek + Read> Buffer for T {}

pub trait WriteOp<const R: usize>: FnOnce(&mut dyn Buffer) -> std::io::Result<[u8; R]> {}
impl<const R: usize, F: FnOnce(&mut dyn Buffer) -> std::io::Result<[u8; R]>> WriteOp<R> for F {}

pub trait WriteOpInt: FnOnce(&mut dyn Buffer, u64) -> std::io::Result<()> {}
impl<F: FnOnce(&mut dyn Buffer, u64) -> std::io::Result<()>> WriteOpInt for F {}

pub struct WriteDefered<T> {
    inner: T,
    ops: HashMap<u64, Box<dyn WriteOpInt>>,
}

impl<T> WriteDefered<T> {
    pub fn new(inner: T) -> Self {
        Self {
            inner,
            ops: HashMap::new(),
        }
    }
}

impl<T: Buffer> WriteDefered<T> {
    pub fn resolve_pending(mut self) -> std::io::Result<T> {
        for (pos, op) in self.ops {
            op(&mut self.inner, pos)?;
        }

        return Ok(self.inner);
    }
}

pub trait DeferedWrite {
    /// `temp` must have the same size as the value writen in `operation`
    unsafe fn defer_write(
        &mut self,
        temp: &[u8],
        operation: Box<dyn WriteOpInt>,
    ) -> std::io::Result<()>;
}

impl<T: Buffer> DeferedWrite for WriteDefered<T> {
    unsafe fn defer_write(
        &mut self,
        temp: &[u8],
        operation: Box<dyn WriteOpInt>,
    ) -> std::io::Result<()> {
        let pos = self.inner.stream_position()?;

        self.inner.write_all(temp)?;
        self.ops.insert(pos, operation);

        return Ok(());
    }
}

impl dyn DeferedWrite + '_ {
    fn write_defered<const R: usize, Op: WriteOp<R> + 'static>(
        &mut self,
        operation: Op,
    ) -> std::io::Result<()> {
        let operation = Box::new(operation);

        unsafe {
            // This is safe because the compiler check the sizes
            self.defer_write(
                &[0; R],
                Box::new(move |writer, start| -> std::io::Result<()> {
                    let value = operation(writer)?;

                    writer.seek(SeekFrom::Start(start))?;
                    let _pos = writer.stream_position()?;
                    dbg!(_pos);
                    writer.write_all(&value)?;

                    return Ok(());
                }),
            )?;
        }
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
        test_dyn(&mut writer)?;
        writer.write_all(b"e")?;

        let result = (writer.resolve_pending()?).into_inner();

        assert_eq!(result, b"abcde");

        return Ok(());
    }

    fn test_dyn(writer: &mut dyn DeferedWrite) -> std::io::Result<()> {
        writer.write_defered(|_w| Ok(*b"bcd"))?;

        return Ok(());
    }
}
