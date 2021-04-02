use std::ops;

pub fn remap<
    T: ops::Add<Output = T>
        + ops::Sub<Output = T>
        + ops::Mul<Output = T>
        + ops::Div<Output = T>
        + Copy,
>(
    x: T,
    a: T,
    b: T,
    c: T,
    d: T,
) -> T {
    x / (b - a) * (d - c) + c
}

pub fn lerp<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy>(
    t: T,
    a: T,
    b: T,
) -> T {
    a + t * (b - a)
}

pub fn bezier<T: ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy>(
    t: T,
    a: T,
    b: T,
    c: T,
) -> T {
    lerp(t, lerp(t, a, b), lerp(t, b, c))
}
