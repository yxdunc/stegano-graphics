use std::path::Path;
use stegs::stegs::color_palette::Palette;
use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;
use stegs::stegs::Steg;
use svg_composer::element::attributes::{Color, ColorName, Paint};

fn main() {
    // let message = "rmrmbzoq";
    // let message = "stegano graphics";
    // let message = "angeline robin virginie";
    // let message = "angeline combes";
    // let message = "omoabl";
    // let message = "welcome you smart";
    // let message = "the game";

    // let message = "hello world";
    // let message = "squares";
    // let message = "two rad";
    // let message = "pi scale";
    let message = "voxal";

    // let message = "crates";

    let mut transparent_palette = Palette::default_stegano();
    transparent_palette.background_1 = Paint::new_empty();

    // let mut steg = Fingerprint::new()
    // .set_text(message)
    // .set_color_palette(Palette::default_stegano());
    // .set_color_palette(transparent_palette);
    let mut steg = Spiral::new()
        .set_text(message)
        .set_color_palette(Palette::default_stegano());
    // .set_color_palette(transparent_palette);

    // steg = steg.set_render_debug(true);
    steg.render();
    println!("{}", steg.get_svg().render());
    steg.get_pixmap(1000, 1000, 0, 100, 0, true)
        .unwrap()
        .save_png(Path::new("/tmp/steg.png"));
}
