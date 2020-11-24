use stegs::fingerprint::Fingerprint;

fn main() {
    let mut fp = Fingerprint::new().set_text("helloworld");
    println!("{}", fp.render_circular());
}
