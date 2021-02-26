use svg_composer::element::attributes::{Color, ColorName};

pub struct Palette {
    pub primary: Color,
    pub secondary: Color,
    pub background_light: Color,
    pub background_dark: Color,
    pub whitish: Color,
    pub warning_0: Color,
    pub warning_1: Color,
    pub error: Color,
}

impl Palette {
    pub fn default() -> Self {
        Palette {
            primary: Color::from_name(ColorName::Aqua),
            secondary: Color::from_name(ColorName::Fuchsia),
            background_light: Color::from_name(ColorName::Gray),
            background_dark: Color::from_name(ColorName::Black),
            whitish: Color::from_name(ColorName::White),
            warning_0: Color::from_name(ColorName::Yellow),
            warning_1: Color::from_name(ColorName::Yellow),
            error: Color::from_name(ColorName::Red),
        }
    }
    pub fn default_stegano() -> Self {
        Palette {
            primary: Color::from_rgb(245, 194, 102),
            secondary: Color::from_rgb(178, 92, 34),
            background_light: Color::from_rgb(252, 226, 212),
            background_dark: Color::from_rgb(28, 53, 63),
            whitish: Color::from_rgb(241, 241, 241),
            warning_0: Color::from_name(ColorName::Olive),
            warning_1: Color::from_name(ColorName::Aqua),
            error: Color::from_name(ColorName::Fuchsia),
        }
    }
}
