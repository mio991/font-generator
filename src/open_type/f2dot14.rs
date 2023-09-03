use std::{cmp::Ordering, ops::Add};

#[derive(Debug, PartialEq, Eq, Ord)]
pub struct F2Dot14 {
    int: i8,
    fract: u16,
}

const MAX_FRACT: u16 = 0x3fff;

impl F2Dot14 {
    pub fn try_create(int: i8, fract: u16) -> Option<Self> {
        if int >= -2 && int < 2 && fract <= MAX_FRACT {
            Some(Self { int, fract })
        } else {
            None
        }
    }

    pub fn clamped(int: i8, fract: u16) -> Self {
        Self {
            int: int.clamp(-2, 1),
            fract: fract.clamp(0, MAX_FRACT),
        }
    }
}

impl PartialOrd for F2Dot14 {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.int < other.int {
            Some(Ordering::Less)
        } else if self.int > other.int {
            Some(Ordering::Greater)
        } else {
            if self.fract < other.fract {
                Some(Ordering::Less)
            } else if self.fract > other.fract {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Equal)
            }
        }
    }
}

impl Add for F2Dot14 {
    type Output = F2Dot14;
    fn add(self, other: Self) -> Self::Output {
        let mut fract = self.fract + other.fract;
        let mut int = self.int + other.int;

        if fract > MAX_FRACT {
            fract -= MAX_FRACT;
            int += 1
        }

        Self::clamped(int, fract)
    }
}

/*
impl Neg for F2Dot14 {
    type Output = F2Dot14;

    fn neg(self) -> Self::Output {}
}

impl Sub for F2Dot14 {}
*/
