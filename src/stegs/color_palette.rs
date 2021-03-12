use svg_composer::element::attributes::{Color, ColorName, Paint};

pub struct Palette {
    pub primary: Paint,
    pub secondary: Paint,
    pub background_light: Paint,
    pub background_dark: Paint,
    pub whitish: Paint,
    pub warning_0: Paint,
    pub warning_1: Paint,
    pub error: Paint,
}

impl Palette {
    pub fn default() -> Self {
        Palette {
            primary: Paint::from_color(Color::from_name(ColorName::Aqua)),
            secondary: Paint::from_color(Color::from_name(ColorName::Fuchsia)),
            background_light: Paint::from_color(Color::from_name(ColorName::Gray)),
            background_dark: Paint::from_color(Color::from_name(ColorName::Black)),
            whitish: Paint::from_color(Color::from_name(ColorName::White)),
            warning_0: Paint::from_color(Color::from_name(ColorName::Yellow)),
            warning_1: Paint::from_color(Color::from_name(ColorName::Yellow)),
            error: Paint::from_color(Color::from_name(ColorName::Red)),
        }
    }
    pub fn default_stegano() -> Self {
        Palette {
            primary: Paint::from_color(Color::from_rgb(245, 194, 102)),
            secondary: Paint::from_color(Color::from_rgb(178, 92, 34)),
            background_light: Paint::from_color(Color::from_rgb(252, 226, 212)),
            background_dark: Paint::from_color(Color::from_rgb(28, 53, 63)),
            whitish: Paint::from_color(Color::from_rgb(241, 241, 241)),
            warning_0: Paint::from_color(Color::from_name(ColorName::Olive)),
            warning_1: Paint::from_color(Color::from_name(ColorName::Aqua)),
            error: Paint::from_color(Color::from_name(ColorName::Fuchsia)),
        }
    }
}
