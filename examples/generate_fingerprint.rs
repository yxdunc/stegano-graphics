use std::io::Read;
use std::path::Path;
use stegs::stegs::color_palette::SteganoPalette;
use stegs::stegs::color_palette::UsagePalette as Palette;
use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;
use stegs::stegs::{RenderSpecs, Steg, StegError};
use svg_composer::element::attributes::{Color, ColorName, Paint};
use tiny_skia::Pixmap;

fn main() {
    // let message = "rmrmbzoq";
    // let message = "stegano graphics";
    // let message = "robin virginie";
    // let message = "angeline combes";
    // let message = "omoabl";
    //     let message = "welcome you smart";

    let message = (
        "generative art",
        SteganoPalette::Yellow00.to_paint(),
        SteganoPalette::Brown00.to_paint(),
    );

    // let message = "free access to computers"; // good
    // let message = "mistrust authority"; // very good
    // let message = "promote decentralization"; // good
    // let message = "you can create art on a computer"; // good

    let message = "all information should be free"; // not good
    let message = "all informatiok"; // not good
    let message = "your message";
    // let message = "only be judged by your hacking"; // not good
    // let message = "computers can change your life for the better"; // not good

    // let message = "pierre"; // bugging...
    // let message = "squares";
    // let message = "two rad";
    // let message = "pi scale";
    // let message = "crates";

    let mut transparent_palette = Palette::stegano_default();
    transparent_palette.background_1 = Paint::new_empty();

    let stegano_default = Palette::stegano_default();

    let mut custom_palette = Palette::stegano_default();
    custom_palette.primary = SteganoPalette::Yellow00.to_paint();
    custom_palette.background_1 = SteganoPalette::Brown00.to_paint();

    let mut steg = Fingerprint::new()
        // let mut steg = Spiral::new()
        .set_text(message)
        // .set_color_palette(custom_palette);
        // .set_color_palette(Palette::stegano_default());
        .set_color_palette(transparent_palette);
    // .set_color_palette(Palette::stegano_default());
    // .set_color_palette(transparent_palette);

    // steg = steg.set_render_debug(true);
    steg.render();
    println!("{}", steg.get_svg().render());
    let pixmap = steg.get_pixmap(
        RenderSpecs {
            antialiasing: false,
            transparent_background: false,
            max_stroke: 100.0,
            min_stroke: 1.0,
            width: 4200.0,
            height: 4200.0,
            margin: 0.0,
        },
        None,
    );
    let pixmap = match pixmap {
        Ok(pxmp) => pxmp,
        Err(err) => {
            eprintln!("{}", err);
            panic!();
        }
    };
    // let raw_pixmap = pixmap.encode_png().unwrap();
    // eprintln!("{:?}", raw_pixmap);
    // pixmap.save_png(Path::new("/tmp/steg.png"));
    pixmap.save_png(Path::new(&format!(
        "/Users/robin/Desktop/rendered_stegs/to_sort/{}.png",
        message
    )));
}
