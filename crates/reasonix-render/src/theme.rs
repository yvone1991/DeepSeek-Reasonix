use crate::scene::{Color, NamedColor};

pub fn hex(s: &'static str) -> Color {
    Color::Hex { hex: s.to_string() }
}

pub fn default_color() -> Color {
    Color::Named(NamedColor::Default)
}

pub mod palette {
    use super::hex;
    use crate::scene::Color;

    pub fn bg() -> Color {
        hex("#0f1018")
    }
    pub fn bg2() -> Color {
        hex("#161824")
    }
    pub fn fg() -> Color {
        hex("#e8e9f3")
    }
    pub fn fg1() -> Color {
        hex("#a8aabd")
    }
    pub fn fg2() -> Color {
        hex("#6b6e85")
    }
    pub fn fg3() -> Color {
        hex("#3d4055")
    }
    pub fn ds() -> Color {
        hex("#6b85ff")
    }
    pub fn ds_bright() -> Color {
        hex("#8b9fff")
    }
    pub fn ds_purple() -> Color {
        hex("#a78bfa")
    }
    pub fn ok() -> Color {
        hex("#5eead4")
    }
    pub fn warn() -> Color {
        hex("#fbbf24")
    }
    pub fn err() -> Color {
        hex("#fb7185")
    }
}
