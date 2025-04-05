use std::borrow::Cow;

use crate::control::{ColorLevel, get_current_color_level};

const ANSI_16_COLORS: [(u8, u8, u8, Color); 16] = [
    (0, 0, 0, Color::Black),
    (128, 0, 0, Color::Red),
    (0, 128, 0, Color::Green),
    (128, 128, 0, Color::Yellow),
    (0, 0, 128, Color::Blue),
    (128, 0, 128, Color::Magenta),
    (0, 128, 128, Color::Cyan),
    (192, 192, 192, Color::White),
    (128, 128, 128, Color::BrightBlack),
    (255, 0, 0, Color::BrightRed),
    (0, 255, 0, Color::BrightGreen),
    (255, 255, 0, Color::BrightYellow),
    (0, 0, 255, Color::BrightBlue),
    (255, 0, 255, Color::BrightMagenta),
    (0, 255, 255, Color::BrightCyan),
    (255, 255, 255, Color::BrightWhite),
];

const CUBE_VALUES: [u8; 6] = [0, 0x5f, 0x87, 0xaf, 0xd7, 0xff];

/// The 16 standard colors, Ansi256 and TrueColor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
    Ansi256 { idx: u8 },
    TrueColor { r: u8, g: u8, b: u8 },
}

#[allow(missing_docs)]
impl Color {
    #[must_use]
    pub fn to_fg_str(&self) -> Cow<'static, str> {
        match *self {
            Self::Black => "30".into(),
            Self::Red => "31".into(),
            Self::Green => "32".into(),
            Self::Yellow => "33".into(),
            Self::Blue => "34".into(),
            Self::Magenta => "35".into(),
            Self::Cyan => "36".into(),
            Self::White => "37".into(),
            Self::BrightBlack => "90".into(),
            Self::BrightRed => "91".into(),
            Self::BrightGreen => "92".into(),
            Self::BrightYellow => "93".into(),
            Self::BrightBlue => "94".into(),
            Self::BrightMagenta => "95".into(),
            Self::BrightCyan => "96".into(),
            Self::BrightWhite => "97".into(),
            Self::Ansi256 { idx } => format!("38;5;{idx}").into(),
            Self::TrueColor { r, g, b } => match get_current_color_level() {
                ColorLevel::Ansi16 => self.fallback_to_ansi16().to_fg_str(),
                ColorLevel::Ansi256 => self.fallback_to_ansi256().to_fg_str(),
                _ => format!("38;2;{r};{g};{b}").into(),
            },
        }
    }

    #[must_use]
    pub fn to_bg_str(&self) -> Cow<'static, str> {
        match *self {
            Self::Black => "40".into(),
            Self::Red => "41".into(),
            Self::Green => "42".into(),
            Self::Yellow => "43".into(),
            Self::Blue => "44".into(),
            Self::Magenta => "45".into(),
            Self::Cyan => "46".into(),
            Self::White => "47".into(),
            Self::BrightBlack => "100".into(),
            Self::BrightRed => "101".into(),
            Self::BrightGreen => "102".into(),
            Self::BrightYellow => "103".into(),
            Self::BrightBlue => "104".into(),
            Self::BrightMagenta => "105".into(),
            Self::BrightCyan => "106".into(),
            Self::BrightWhite => "107".into(),
            Self::Ansi256 { idx } => match get_current_color_level() {
                ColorLevel::Ansi16 => self.fallback_to_ansi16().to_bg_str(),
                _ => format!("48;5;{idx}").into(),
            },
            Self::TrueColor { r, g, b } => match get_current_color_level() {
                ColorLevel::Ansi16 => self.fallback_to_ansi16().to_bg_str(),
                ColorLevel::Ansi256 => self.fallback_to_ansi256().to_bg_str(),
                _ => format!("48;2;{r};{g};{b}").into(),
            },
        }
    }

    /// Converts a `TrueColor` or `Ansi256` Color to the closest ANSI 16-color palette color.
    ///
    /// Returns self if not a `TrueColor` or `Ansi256` Color.
    #[must_use]
    pub fn fallback_to_ansi16(self) -> Self {
        let (r, g, b) = match self {
            Self::Ansi256 { idx } => ansi256_to_rgb(idx),
            Self::TrueColor { r, g, b } => (r, g, b),
            _ => return self,
        };
        let mut min_distance_sq = u32::MAX;
        let mut closest_color = self;

        for &(cr, cg, cb, color) in &ANSI_16_COLORS {
            let dr = (i32::from(r) - i32::from(cr)).pow(2) as u32;
            let dg = (i32::from(g) - i32::from(cg)).pow(2) as u32;
            let db = (i32::from(b) - i32::from(cb)).pow(2) as u32;
            let distance_sq = dr + dg + db;

            if distance_sq < min_distance_sq {
                min_distance_sq = distance_sq;
                closest_color = color;
            }
        }

        closest_color
    }

    /// Converts a `TrueColor` to the closest ANSI 256-color palette color.
    ///
    /// Returns self if not a TrueColor.
    #[must_use]
    pub fn fallback_to_ansi256(self) -> Self {
        let (r, g, b) = match self {
            Self::TrueColor { r, g, b } => (r, g, b),
            _ => return self,
        };
        let mut min_distance_sq = u32::MAX;
        let mut closest_idx = 0;

        for idx in 0u8..=255 {
            let (cr, cg, cb) = ansi256_to_rgb(idx);
            let dr = (i32::from(r) - i32::from(cr)).pow(2) as u32;
            let dg = (i32::from(g) - i32::from(cg)).pow(2) as u32;
            let db = (i32::from(b) - i32::from(cb)).pow(2) as u32;
            let distance_sq = dr + dg + db;

            if distance_sq < min_distance_sq {
                min_distance_sq = distance_sq;
                closest_idx = idx;
            }
        }

        Self::Ansi256 { idx: closest_idx }
    }
}

fn ansi256_to_rgb(idx: u8) -> (u8, u8, u8) {
    if idx < 16 {
        let (r, g, b, _) = ANSI_16_COLORS[idx as usize];
        (r, g, b)
    } else if idx <= 231 {
        let idx = idx - 16;
        let r = idx / 36;
        let rem = idx % 36;
        let g = rem / 6;
        let b = rem % 6;
        let r_val = CUBE_VALUES[r as usize];
        let g_val = CUBE_VALUES[g as usize];
        let b_val = CUBE_VALUES[b as usize];
        (r_val, g_val, b_val)
    } else {
        let gray_level = idx - 232;
        let gray_value = 8 + gray_level * 10;
        (gray_value, gray_value, gray_value)
    }
}
