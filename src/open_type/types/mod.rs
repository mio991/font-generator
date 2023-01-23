mod fixed;
mod write_defered;

use std::ops::DerefMut;

pub use fixed::{Fixed, FixedWriteExt};
pub use write_defered::*;

pub type Tag = [u8; 4];

pub trait Sink: Buffer + DeferedWrite {}
impl<T: Buffer + DeferedWrite> Sink for T {}
