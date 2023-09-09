use crate::{
    layout::Reservation,
    open_type::{LayoutableTable, LayoutedTable},
    Layoutable, Layouted,
};

#[derive(Debug, Clone)]
pub struct Script {
    pub x_size: i16,
    pub y_size: i16,
    pub x_offset: i16,
    pub y_offset: i16,
}

#[derive(Debug, Clone)]
pub struct Panose {
    pub family_type: u8,
    pub serif_style: u8,
    pub weight: u8,
    pub proportion: u8,
    pub contrast: u8,
    pub stroke_variation: u8,
    pub arm_style: u8,
    pub letterform: u8,
    pub midline: u8,
    pub xheight: u8,
}

impl Default for Panose {
    fn default() -> Self {
        Self {
            family_type: 0,
            serif_style: 0,
            weight: 0,
            proportion: 0,
            contrast: 0,
            stroke_variation: 0,
            arm_style: 0,
            letterform: 0,
            midline: 0,
            xheight: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OS2 {
    pub avg_glyph_width: i16,
    /** 400 - Normal */
    pub weight_class: u16,
    /** 5 - Medium */
    pub width_class: u16,

    pub subscript: Script,
    pub superscript: Script,
    pub strikeout_size: i16,
    pub strikeout_position: i16,

    pub panose: Panose,

    pub typo_ascender: i16,
    pub typo_descender: i16,
    pub typo_line_gap: i16,

    pub win_ascent: u16,
    pub win_descent: u16,

    pub x_height: i16,
    pub cap_height: i16,

    pub default_cahr: u16,
    pub break_char: u16,

    pub max_context: u16,
}

impl Layoutable<Box<dyn LayoutedTable>> for OS2 {
    fn layout(&self, layouter: &mut crate::Layouter) -> Box<dyn LayoutedTable> {
        Box::new(OS2Layouted {
            reservation: layouter.reserve(70 + 2 * 8 + 10),
            requires_another_pass: true,
            table: self.clone(),
        })
    }
}

impl LayoutableTable for OS2 {
    fn tag(&self) -> [u8; 4] {
        *b"OS/2"
    }
}

struct OS2Layouted {
    table: OS2,
    reservation: Reservation,
    requires_another_pass: bool,
}

impl LayoutedTable for OS2Layouted {
    fn tag(&self) -> [u8; 4] {
        self.table.tag()
    }
}

impl Layouted for OS2Layouted {
    fn requires_another_pass(&self) -> bool {
        self.requires_another_pass
    }

    fn reservation(&self) -> &Reservation {
        &self.reservation
    }

    fn pass(&mut self, _current_file: &[u8]) -> Result<(), crate::LayoutError> {
        use byteorder::{WriteBytesExt, BE};
        use helpers::*;
        use std::io::Write;

        self.requires_another_pass = false;

        let mut writer = self.reservation.writer();

        writer.write_u16::<BE>(5)?;
        writer.write_i16::<BE>(self.table.avg_glyph_width)?;
        writer.write_u16::<BE>(self.table.weight_class)?;
        writer.write_u16::<BE>(self.table.width_class)?;

        // ToDo: Licensing
        writer.write_u16::<BE>(0)?;

        writer.write_script(&self.table.subscript)?;
        writer.write_script(&self.table.superscript)?;

        writer.write_i16::<BE>(self.table.strikeout_size)?;
        writer.write_i16::<BE>(self.table.strikeout_position)?;

        // ToDo: Family Class
        writer.write_i16::<BE>(0)?;

        writer.write_panose(&self.table.panose)?;

        writer.write_u32::<BE>(0)?; //ulUnicodeRange1 (Bits 0–31)
        writer.write_u32::<BE>(0)?; //ulUnicodeRange2 (Bits 32–63)
        writer.write_u32::<BE>(0)?; //ulUnicodeRange3 (Bits 64–95)
        writer.write_u32::<BE>(0)?; //ulUnicodeRange4 (Bits 96–127)

        writer.write_all(b"    ")?;

        // ToDo: fsSelection
        writer.write_u16::<BE>(0)?;

        writer.write_u16::<BE>(0xFFFF)?; // usFirstCharIndex
        writer.write_u16::<BE>(0xFFFF)?; // usLastCharIndex

        writer.write_i16::<BE>(self.table.typo_ascender)?;
        writer.write_i16::<BE>(self.table.typo_descender)?;
        writer.write_i16::<BE>(self.table.typo_line_gap)?;

        writer.write_u16::<BE>(self.table.win_ascent)?;
        writer.write_u16::<BE>(self.table.win_descent)?;

        writer.write_u32::<BE>(0)?; // ulCodePageRange1 Bits 0–31
        writer.write_u32::<BE>(0)?; // ulCodePageRange2 Bits 32–63

        writer.write_u16::<BE>(self.table.default_cahr)?;
        writer.write_u16::<BE>(self.table.break_char)?;
        writer.write_u16::<BE>(self.table.max_context)?;

        writer.write_u16::<BE>(0)?; // usLowerOpticalPointSize
        writer.write_u16::<BE>(0xFFFF)?; // usUpperOpticalPointSize

        Ok(())
    }
}

mod helpers {
    use byteorder::{WriteBytesExt, BE};

    use super::{Panose, Script};

    pub trait WriteExt: std::io::Write {
        fn write_script(&mut self, script: &Script) -> std::io::Result<()> {
            self.write_i16::<BE>(script.x_size)?;
            self.write_i16::<BE>(script.y_size)?;
            self.write_i16::<BE>(script.x_offset)?;
            self.write_i16::<BE>(script.y_offset)?;

            return Ok(());
        }

        fn write_panose(&mut self, panose: &Panose) -> std::io::Result<()> {
            self.write_u8(panose.family_type)?;
            self.write_u8(panose.serif_style)?;
            self.write_u8(panose.weight)?;
            self.write_u8(panose.proportion)?;
            self.write_u8(panose.contrast)?;
            self.write_u8(panose.stroke_variation)?;
            self.write_u8(panose.arm_style)?;
            self.write_u8(panose.letterform)?;
            self.write_u8(panose.midline)?;
            self.write_u8(panose.xheight)?;

            return Ok(());
        }
    }

    impl<W: std::io::Write + ?Sized> WriteExt for W {}
}
