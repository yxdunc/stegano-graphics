use stegs::fingerprint::Fingerprint;

fn main() {
    let mut fp = Fingerprint::new().set_text("aabc");
    println!("{}", fp.render());
}
