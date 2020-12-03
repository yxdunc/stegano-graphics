use stegs::fingerprint::Fingerprint;

fn main() {
    // let mut fp = Fingerprint::new().set_text("rmrmbzoq");
    let mut fp = Fingerprint::new().set_text("a");
    // println!("{}", fp.render_circular());
    println!("{}", fp.render());
}
