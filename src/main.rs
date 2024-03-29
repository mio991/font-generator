use std::{error::Error, io::Write};

use chrono::Utc;
use font_generator::{
    open_type::{
        tables::*,
        true_type::{Contour, Instrution, Point},
        *,
    },
    *,
};

fn main() -> Result<(), Box<dyn Error>> {
    let doc = File::new_with_tables(vec![
        Box::new(Head {
            created: Utc::now(),
            modified: Utc::now(),
            revision: Fixed { major: 0, minor: 1 },
            flags: Flags::default(),
            min_x: 0,
            min_y: 0,
            max_x: 14,
            max_y: 40,
            units_per_em: 40,
            smalest_recocnizeable_size: 6,
        }),
        Box::new(Name {
            names: vec![
                NameRecord {
                    name_id: 1,
                    content: String::from("Test Font"),
                },
                NameRecord {
                    name_id: 2,
                    content: String::from("Default"),
                },
                NameRecord {
                    name_id: 8,
                    content: String::from("mio991"),
                },
                NameRecord {
                    name_id: 9,
                    content: String::from("mio991"),
                },
            ],
        }),
        Box::new(Post::default()),
        Box::new(Glyf {
            glyphs: vec![Glyph::Simple {
                contours: vec![Contour {
                    points: vec![
                        Point::on_curve(0, 0),
                        Point::on_curve(0, 40),
                        Point::on_curve(14, 40),
                        Point::on_curve(14, 0),
                    ],
                }],
                instructions: vec![Instrution::PushBytes(Box::new([5, 2]))],
            }],
        }),
        Box::new(Loca {
            offsets: vec![0, 0, 0x25],
        }),
        Box::new(CMap::new_with_ranges(vec![CharacterRange {
            start: 'o',
            end: 'o',
            start_index: 1,
        }])),
        Box::new(HHead {
            ascender: 1,
            descender: -1,
            line_gap: 0,
            advance_width_max: 0,
            min_left_side_bearing: 0,
            min_right_side_bearing: 0,
            x_max_extent: 14,
            caret_slope_rise: 1,
            caret_slope_run: 0,
            caret_offset: 0,
            metric_data_format: 0,
            number_of_hmetrics: 1,
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
            typo_descender: -30,
            typo_line_gap: 120,
            win_ascent: 60,
            win_descent: 80,
            x_height: 60,
            cap_height: 80,
            default_cahr: 0,
            break_char: ' ' as u16,
            max_context: 1,
        }),
        Box::new(MaxP {
            number_of_glyphs: 2,
        }),
        Box::new(Hmtx {
            horizontal_metrics: vec![HorizontalMetric {
                advance_width: 14,
                left_side_bearing: 0,
            }],
            left_side_bearings: vec![],
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
