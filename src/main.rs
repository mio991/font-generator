use std::{error::Error, io::Write};

use chrono::Utc;
use font_generator::{
    open_type::{tables::*, *},
    *,
};

fn main() -> Result<(), Box<dyn Error>> {
    let doc = File::new_with_tables(vec![
        Box::new(CMap::new_with_ranges(vec![CharacterRange {
            start: 'o' as u16,
            end: 'o' as u16,
            start_id: 0,
        }])),
        Box::new(Head {
            created: Utc::now(),
            modified: Utc::now(),
            revision: Fixed { major: 0, minor: 1 },
            flags: Flags::default(),
            min_x: -20,
            min_y: -20,
            max_x: 20,
            max_y: 20,
            units_per_em: 40,
            smalest_recocnizeable_size: 6,
        }),
        Box::new(OS2 {
            avg_glyph_width: 100,
            weight_class: 400,
            width_class: 5,
            subscript: Script {
                x_size: 60,
                y_size: 60,
                x_offset: 5,
                y_offset: 40,
            },
            superscript: Script {
                x_size: 60,
                y_size: 60,
                x_offset: 5,
                y_offset: -40,
            },
            strikeout_size: 10,
            strikeout_position: 45,
            panose: Panose::default(),
            typo_ascender: 20,
            typo_descender: 30,
            typo_line_gap: 120,
            win_ascent: 60,
            win_descent: 80,
            x_height: 60,
            cap_height: 80,
            default_cahr: 0,
            break_char: ' ' as u16,
            max_context: 1,
        }),
    ]);

    let mut file = Layouter::new(4);

    let mut doc = doc.layout(&mut file);

    let mut result = file.get_result();

    while doc.requires_another_pass() {
        println!("Execute another pass.");
        doc.pass(result.as_slice())?;
        result = file.get_result();
    }

    let path = std::env::args().nth(2).unwrap_or("./out.otf".to_string());
    let mut file = std::fs::File::create(path)?;

    file.write_all(&result)?;

    Ok(())
}
