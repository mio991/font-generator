use std::error::Error;

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
    ]);

    let mut file = Layouter::new();

    let mut doc = doc.layout(&mut file);

    let mut result = file.get_result();

    while doc.requires_another_pass() {
        println!("Execute another pass.");
        doc.pass(result.as_slice())?;
        result = file.get_result();
    }

    dbg!(result);

    Ok(())
}
