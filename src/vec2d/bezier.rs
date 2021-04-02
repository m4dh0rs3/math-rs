use crate::prelude::*;
use std::ops;

impl<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy> Vec2D<T> {
    pub fn lerp(t: T, a: Self, b: Self) -> Self {
        Self {
            x: lerp(t, a.x, b.x),
            y: lerp(t, a.y, b.y),
        }
    }

    pub fn bezier(t: T, a: Self, b: Self, c: Self) -> Self {
        Self {
            x: bezier(t, a.x, b.x, c.x),
            y: bezier(t, a.y, b.y, c.y),
        }
    }
}
