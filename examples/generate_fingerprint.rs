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
    let message = "alice";

    let mut steg = Fingerprint::new().set_text(message);
    // let mut steg = Spiral::new().set_text(message);
    steg._render();
    println!("{}", steg.get_svg().render());
}
