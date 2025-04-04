/// Custom color structure, it will generate a true color in the result
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct CustomColor {
    /// Red
    pub r: u8,
    /// Green
    pub g: u8,
    /// Blue
    pub b: u8,
}

/// This only makes custom color creation easier.
impl CustomColor {
    /// Create a new custom color
    #[must_use]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

impl From<(u8, u8, u8)> for CustomColor {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self::new(r, g, b)
    }
}
