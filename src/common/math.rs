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

/// The four corners of a square.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum QuadCorner {
    NW,
    NE,
    SW,
    SE,
}

impl QuadCorner {
    /// Unit X coordinate of the square corner.
    pub fn x(&self) -> usize {
        use QuadCorner::*;
        match self {
            NW => 0,
            SW => 0,
            NE => 1,
            SE => 1,
        }
    }

    /// Unit Y coordinate of the square corner.
    pub fn y(&self) -> usize {
        use QuadCorner::*;
        match self {
            NW => 0,
            NE => 0,
            SW => 1,
            SE => 1,
        }
    }

    /// Unit X coordinate of the square corner, as a f32.
    pub fn xf(&self) -> f32 {
        use QuadCorner::*;
        match self {
            NW => 0.0,
            SW => 0.0,
            NE => 1.0,
            SE => 1.0,
        }
    }

    /// Unit Y coordinate of the square corner, as a f32.
    pub fn yf(&self) -> f32 {
        use QuadCorner::*;
        match self {
            NW => 0.0,
            NE => 0.0,
            SW => 1.0,
            SE => 1.0,
        }
    }
}
