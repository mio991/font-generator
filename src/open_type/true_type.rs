#[derive(Debug, Clone)]
pub struct Contour {
    pub points: Vec<Point>,
}

/** */
#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub is_on_curve: bool,
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub fn on_curve(x: i16, y: i16) -> Self {
        Self {
            is_on_curve: true,
            x,
            y,
        }
    }

    pub fn off_curve(x: i16, y: i16) -> Self {
        Self {
            is_on_curve: true,
            x,
            y,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instrution {
    PushBytes(Box<[u8]>),
}

pub trait InstrutionWriteExt: std::io::Write {
    fn write_instruction(&mut self, instruction: &Instrution) -> std::io::Result<()> {
        use byteorder::WriteBytesExt;
        use Instrution as I;

        match instruction {
            I::PushBytes(bytes) => {
                self.write_u8(0xB0 + bytes.len() as u8)?;
                self.write_all(bytes.as_ref())?;
            }
        }

        Ok(())
    }
}

impl<W: std::io::Write + ?Sized> InstrutionWriteExt for W {}
