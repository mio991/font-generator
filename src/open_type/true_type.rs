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
pub enum Instrution {}
