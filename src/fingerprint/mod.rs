use svg_composer::Document;

pub struct Fingerprint {
    _inner_circle_size: f32,
    _nose_size: f32,
    _max_radius: f32,
    _stroke_width: f32,
    _text: String,
    _encoded_text: Vec<i8>,
    _svg_document: Document,
}
