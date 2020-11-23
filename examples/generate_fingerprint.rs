use stegs::fingerprint::Fingerprint;

fn main() {
    let mut fp = Fingerprint::new().set_text("azbbrobinguignardperret");
    println!("{}", fp.render());
}
