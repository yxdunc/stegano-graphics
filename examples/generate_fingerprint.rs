use stegs::fingerprint::Fingerprint;

fn main() {
    // let mut fp = Fingerprint::new().set_text("rmrmbzoq");
    let mut fp = Fingerprint::new().set_text("robin guignard perr");
    // println!("{}", fp.render_circular());
    println!("{}", fp.render());
}
