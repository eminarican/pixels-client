pub mod color;
pub mod cooldown;
pub mod pixels;

pub mod prelude {
    pub use crate::{
        color::{Color, ColorMode},
        cooldown::Cooldown,
        pixels::{Pixels, PixelsIterator},
    };
}

#[macro_export]
macro_rules! from {
    ($from:ty, $to:ty, $body:expr $(,)?) => {
        impl From<$from> for $to {
            fn from(value: $from) -> Self {
                $body(value)
            }
        }
    };
}
