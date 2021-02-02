use stegs::stegs::steg_00_spiral::Spiral;
use stegs::stegs::steg_01_fingerprint::Fingerprint;

fn main() {
    // let mut fp = Fingerprint::new().set_text("rmrmbzoq");
    // let mut fp = Fingerprint::new().set_text("stegano graphics");
    // let mut fp = Fingerprint::new().set_text("angeline robin virginie");
    // let mut fp = Fingerprint::new().set_text("angeline combes");
    // let mut fp = Fingerprint::new().set_text("everything is a dildo if you're brave enough");
    // let mut fp = Fingerprint::new().set_text("omoabl");
    // let mut fp = Fingerprint::new().set_text("welcome you smart");
    // let mut fp = Fingerprint::new().set_text("the game");

    let message = "bitcoin mogul";

    let mut fp = Fingerprint::new().set_text(message);
    // let mut fp = Spiral::new().set_text(message);

    println!("{}", fp.render());
}
