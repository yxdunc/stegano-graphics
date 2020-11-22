use stegs::fingerprint::Fingerprint;

fn main() {
    let mut fp = Fingerprint::new().set_text("abcdefghijklmnopqrstuvwxyz");
    println!("{}", fp.render());
}
