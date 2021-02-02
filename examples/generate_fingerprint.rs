use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;

fn main() {
    // let message = "rmrmbzoq";
    // let message = "stegano graphics";
    // let message = "angeline robin virginie";
    // let message = "angeline combes";
    // let message = "omoabl";
    // let message = "welcome you smart";
    // let message = "the game";

    let message = "hello world";

    // let mut fp = Fingerprint::new().set_text(message);
    let mut fp = Spiral::new().set_text(message);

    println!("{}", fp.render());
}
