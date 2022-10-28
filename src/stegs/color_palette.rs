use svg_composer::element::attributes::{Color, ColorName, Paint};

pub enum SteganoPalette {
    Yellow00,
    Brown00,
    LightPink00,
    Grey,
    LightBlue00,
    DarkBlue00,
    Whitish00,
    Transparent,
}

impl SteganoPalette {
    pub fn to_paint(&self) -> Paint {
        match *self {
            SteganoPalette::Yellow00 => Paint::from_color(Color::from_rgb(245, 194, 102)),
            SteganoPalette::Brown00 => Paint::from_color(Color::from_rgb(178, 92, 34)),
            SteganoPalette::LightPink00 => Paint::from_color(Color::from_rgb(252, 226, 212)),
            SteganoPalette::Grey => Paint::from_color(Color::from_rgb(90, 118, 126)),
            SteganoPalette::LightBlue00 => Paint::from_color(Color::from_rgb(146, 189, 193)),
            SteganoPalette::DarkBlue00 => Paint::from_color(Color::from_rgb(28, 53, 63)),
            SteganoPalette::Whitish00 => Paint::from_color(Color::from_rgb(241, 241, 241)),
            SteganoPalette::Transparent => Paint::new_empty(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct UsagePalette {
    pub primary: Paint,
    pub secondary: Paint,
    pub background_0: Paint,
    pub background_1: Paint,
    pub whitish: Paint,
    pub warning_0: Paint,
    pub warning_1: Paint,
    pub error: Paint,
}

impl UsagePalette {
    pub fn default() -> Self {
        UsagePalette {
            primary: Paint::from_color(Color::from_name(ColorName::Aqua)),
            secondary: Paint::from_color(Color::from_name(ColorName::Fuchsia)),
            background_0: Paint::from_color(Color::from_name(ColorName::Gray)),
            background_1: Paint::from_color(Color::from_name(ColorName::Black)),
            whitish: Paint::from_color(Color::from_name(ColorName::White)),
            warning_0: Paint::from_color(Color::from_name(ColorName::Yellow)),
            warning_1: Paint::from_color(Color::from_name(ColorName::Yellow)),
            error: Paint::from_color(Color::from_name(ColorName::Red)),
        }
    }
    pub fn stegano_default() -> Self {
        UsagePalette {
            primary: SteganoPalette::Yellow00.to_paint(),
            secondary: Paint::from_color(Color::from_rgb(178, 92, 34)), // brown
            background_0: Paint::from_color(Color::from_rgb(252, 226, 212)), // light pink
            background_1: Paint::from_color(Color::from_rgb(28, 53, 63)), // dark blue
            whitish: Paint::from_color(Color::from_rgb(241, 241, 241)), //  white
            warning_0: Paint::from_color(Color::from_name(ColorName::Olive)),
            warning_1: Paint::from_color(Color::from_name(ColorName::Aqua)),
            error: Paint::from_color(Color::from_name(ColorName::Fuchsia)),
        }
    }
    pub fn stegano_variant() -> Self {
        UsagePalette {
            primary: Paint::from_color(Color::from_rgb(178, 92, 34)), // brown
            secondary: Paint::from_color(Color::from_rgb(245, 194, 102)), // yellow
            background_0: Paint::from_color(Color::from_rgb(28, 53, 63)), // dark blue
            background_1: Paint::from_color(Color::from_rgb(252, 226, 212)), // light pink
            whitish: Paint::from_color(Color::from_rgb(241, 241, 241)), //  white
            warning_0: Paint::from_color(Color::from_name(ColorName::Olive)),
            warning_1: Paint::from_color(Color::from_name(ColorName::Aqua)),
            error: Paint::from_color(Color::from_name(ColorName::Fuchsia)),
        }
    }
}
