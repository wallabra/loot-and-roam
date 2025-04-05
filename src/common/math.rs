//! # Mathematical utility functions

/// Linearly interpolate between two values.
pub fn lerp(from: f32, to: f32, alpha: f32) -> f32 {
    from + alpha * (to - from)
}

/// Interpolate between two values using smootherstep, Ken Perlin's optimized
/// version of the smoothstep interpolation spline.
pub fn smootherstep(from: f32, to: f32, alpha: f32) -> f32 {
    let alpha = alpha * alpha * alpha * (alpha * (6.0 * alpha - 15.0) + 10.0);
    lerp(from, to, alpha)
}
