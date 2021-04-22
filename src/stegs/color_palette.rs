use svg_composer::element::attributes::{Color, ColorName, Paint};

#[derive(Debug)]
pub struct Palette {
    pub primary: Paint,
    pub secondary: Paint,
    pub background_0: Paint,
    pub background_1: Paint,
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
            background_0: Paint::from_color(Color::from_name(ColorName::Gray)),
            background_1: Paint::from_color(Color::from_name(ColorName::Black)),
            whitish: Paint::from_color(Color::from_name(ColorName::White)),
            warning_0: Paint::from_color(Color::from_name(ColorName::Yellow)),
            warning_1: Paint::from_color(Color::from_name(ColorName::Yellow)),
            error: Paint::from_color(Color::from_name(ColorName::Red)),
        }
    }
    pub fn stegano_default() -> Self {
        Palette {
            primary: Paint::from_color(Color::from_rgb(245, 194, 102)), // yellow
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
        Palette {
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
