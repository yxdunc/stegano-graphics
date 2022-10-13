use std::io::Read;
use std::path::Path;
use stegs::stegs::color_palette::UsagePalette as Palette;
use stegs::stegs::color_palette::{SteganoPalette, UsagePalette};
use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;
use stegs::stegs::{RenderSpecs, Steg, StegError};
use svg_composer::element::attributes::{Color, ColorName, Paint};
use tiny_skia::Pixmap;

fn generate_steg(
    regular: bool,
    msg: &str,
    palette: UsagePalette,
    antialiasing: bool,
    save_in_tmp: bool,
) {
    let mut steg: Box<dyn Steg> = match regular {
        true => Box::new(Spiral::new().set_text(msg).set_color_palette(palette)),
        false => Box::new(Fingerprint::new().set_text(msg).set_color_palette(palette)),
    };

    steg.render();

    println!("{}", steg.get_svg().render());

    let pixmap = steg.get_pixmap(
        RenderSpecs {
            antialiasing,
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

    match save_in_tmp {
        true => {
            pixmap.save_png(Path::new("/tmp/steg.png"));
        }
        false => {
            pixmap.save_png(Path::new(&format!(
                "/Users/robin/Desktop/rendered_stegs/to_sort/{}.png",
                msg
            )));
        }
    }
}

fn main() {
    // let message = "free access to computers"; // good
    // let message = "mistrust authority"; // very good
    // let message = "promote decentralization"; // good
    // let message = "you can create art on a computer"; // good
    // let message = "all information should be free"; // good
    // let message = "computers can change your life for the better"; // good
    // let message = "i m a hacker"; // good

    // let message = "only be judged by your hacking"; // not good

    // let message = "pierre"; // bugging...

    let steg_descriptions = vec![("music", SteganoPalette::LightPink00, SteganoPalette::Grey)];

    for steg_description in steg_descriptions {
        let mut palette = UsagePalette::default();
        palette.primary = steg_description.1.to_paint();
        palette.background_1 = steg_description.2.to_paint();
        generate_steg(false, steg_description.0, palette, true, false);
    }
}
