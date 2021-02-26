use std::path::Path;
use stegs::stegs::color_palette::Palette;
use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;
use stegs::stegs::Steg;

fn main() {
    // let message = "rmrmbzoq";
    // let message = "stegano graphics";
    // let message = "angeline robin virginie";
    // let message = "angeline combes";
    // let message = "omoabl";
    // let message = "welcome you smart";
    // let message = "the game";

    // let message = "hello world";
    let message = "ba";

    let mut steg = Fingerprint::new()
        .set_text(message)
        .set_color_palette(Palette::default_stegano());
    // let mut steg = Spiral::new().set_text(message);

    // steg = steg.set_render_debug(true);
    steg.render();
    println!("{}", steg.get_svg().render());
    steg.get_pixmap(4200, 6000, 10, 60, 0, true)
        .unwrap()
        .save_png(Path::new("/tmp/steg.png"));
}
