use stegs::fingerprint::Fingerprint;

fn main() {
    let mut fp = Fingerprint::new().set_text("poulet");
    // println!("{}", fp.render_circular());
    println!("{}", fp.render());
}
