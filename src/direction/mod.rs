/// [`Hex`] diagonal directions
mod diagonal_direction;
/// [`Hex`] neighbor directions
mod hex_direction;
/// Trait implementations
mod impls;
/// Test module
#[cfg(test)]
mod tests;

pub use diagonal_direction::DiagonalDirection;
pub use hex_direction::Direction;

/// Angle constants used for directions
pub mod angles {
    use std::f32::consts::PI;
    /// Angle in radian between *flat* and *pointy* top orientations.
    /// Equivalent to 30 degrees
    pub const DIRECTION_ANGLE_OFFSET: f32 = PI / 6.0;
    /// Angle in radian between *flat* and *pointy* top orientations.
    /// Equivalent to π / 6 in radians
    pub const DIRECTION_ANGLE_OFFSET_DEGREES: f32 = 30.0;
    /// Angle in radian between two adjacent directions counter clockwise.
    /// Equivalent to 60 degrees
    pub const DIRECTION_ANGLE_RAD: f32 = PI / 3.0;
    /// Angle in degrees between two adjacent directions counter clockwise.
    /// Equivalent to π / 3 in radians
    pub const DIRECTION_ANGLE_DEGREES: f32 = 60.0;
}