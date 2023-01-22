use byteorder::{WriteBytesExt, BE};

use super::super::{Tag, WriteError};

use super::Table;

#[derive(Debug)]
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

const TABLE_TAG: Tag = *b"OS/2";
const VENDOR_TAG: Tag = *b"    ";

impl Table for OS2 {
    fn get_tag(&self) -> Tag {
        TABLE_TAG
    }

    fn store_internal(&self, writer: &mut dyn std::io::Write) -> Result<(), WriteError> {
        writer.write_u16::<BE>(5)?;
        writer.write_i16::<BE>(self.avg_glyph_width)?;
        writer.write_u16::<BE>(self.weight_class)?;
        writer.write_u16::<BE>(self.width_class)?;

        // ToDo: Licensing
        writer.write_u16::<BE>(0)?;

        writer.write_script(&self.subscript)?;
        writer.write_script(&self.superscript)?;

        writer.write_i16::<BE>(self.strikeout_size)?;
        writer.write_i16::<BE>(self.strikeout_position)?;

        // ToDo: Family Class
        writer.write_i16::<BE>(0)?;

        writer.write_panose(&self.panose)?;

        writer.write_u32::<BE>(0)?; //ulUnicodeRange1 (Bits 0–31)
        writer.write_u32::<BE>(0)?; //ulUnicodeRange2 (Bits 32–63)
        writer.write_u32::<BE>(0)?; //ulUnicodeRange3 (Bits 64–95)
        writer.write_u32::<BE>(0)?; //ulUnicodeRange4 (Bits 96–127)

        writer.write_all(&VENDOR_TAG)?;

        // ToDo: fsSelection
        writer.write_u16::<BE>(0)?;

        writer.write_u16::<BE>(0xFFFF)?; // usFirstCharIndex
        writer.write_u16::<BE>(0xFFFF)?; // usLastCharIndex

        writer.write_i16::<BE>(self.typo_ascender)?;
        writer.write_i16::<BE>(self.typo_descender)?;
        writer.write_i16::<BE>(self.typo_line_gap)?;

        writer.write_u16::<BE>(self.win_ascent)?;
        writer.write_u16::<BE>(self.win_descent)?;

        writer.write_u32::<BE>(0)?; // ulCodePageRange1 Bits 0–31
        writer.write_u32::<BE>(0)?; // ulCodePageRange2 Bits 32–63

        writer.write_u16::<BE>(self.default_cahr)?;
        writer.write_u16::<BE>(self.break_char)?;
        writer.write_u16::<BE>(self.max_context)?;

        writer.write_u16::<BE>(0)?; // usLowerOpticalPointSize
        writer.write_u16::<BE>(0xFFFF)?; // usUpperOpticalPointSize

        return Ok(());
    }
}

#[derive(Debug)]
pub struct Script {
    pub x_size: i16,
    pub y_size: i16,
    pub x_offset: i16,
    pub y_offset: i16,
}

#[derive(Debug)]
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

trait WriteExt: std::io::Write {
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
