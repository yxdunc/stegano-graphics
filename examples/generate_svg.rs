use std::path::Path;

use stegs::{generate_spiral_svg, StegOptions};

fn main() {
    let options = StegOptions::default();
    let generated = generate_spiral_svg("mistrust authority", &options);

    std::fs::write(Path::new("steg.svg"), generated.svg).expect("write steg.svg");
}
