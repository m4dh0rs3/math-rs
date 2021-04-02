mod angle;
mod bezier;
mod vec2d;

pub mod prelude {
    pub use crate::{angle::*, bezier::*, vec2d::Vec2D};
}

#[cfg(test)]
mod tests;
