use stegs::fingerprint::Fingerprint;

fn main() {
    let mut fp = Fingerprint::new().set_text("robinguignardperret");
    println!("{}", fp.render());
}
