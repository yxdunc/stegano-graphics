use std::io::Read;
use std::path::Path;
use stegs::stegs::color_palette::Palette;
use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;
use stegs::stegs::{RenderSpecs, Steg, StegError};
use svg_composer::element::attributes::{Color, ColorName, Paint};
use tiny_skia::Pixmap;

fn main() {
    // let message = "rmrmbzoq";
    // let message = "stegano graphics";
    // let message = "angeline robin virginie";
    // let message = "angeline combes";
    // let message = "omoabl";
    // let message = "welcome you smart";
    // let message = "the game";

    let message = "hello world";
    // let message = "squares";
    // let message = "two rad";
    // let message = "pi scale";
    // let message = "crates";

    let mut transparent_palette = Palette::stegano_default();
    transparent_palette.background_1 = Paint::new_empty();

    let mut steg = Fingerprint::new()
        .set_text(message)
        .set_color_palette(Palette::stegano_default());
    // .set_color_palette(transparent_palette);
    let mut steg = Spiral::new()
        .set_text(message)
        // .set_color_palette(Palette::stegano_variant());
        .set_color_palette(Palette::stegano_default());
    // .set_color_palette(transparent_palette);

    // steg = steg.set_render_debug(true);
    steg.render();
    println!("{}", steg.get_svg().render());
    let pixmap = steg.get_pixmap(RenderSpecs {
        antialiasing: true,
        transparent_background: false,
        max_stroke: 100.0,
        min_stroke: 14.0,
        width: 1000.0,
        height: 1000.0,
        margin: 0.0,
    });
    let pixmap = match pixmap {
        Ok(pxmp) => pxmp,
        Err(err) => {
            eprintln!("{}", err);
            panic!();
        }
    };
    // let raw_pixmap = pixmap.encode_png().unwrap();
    // eprintln!("{:?}", raw_pixmap);
    pixmap.save_png(Path::new("/tmp/steg.png"));
}
