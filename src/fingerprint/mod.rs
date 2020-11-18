use crate::encoder::simple_latin_symbols;
use std::f64::consts::PI;
use svg_composer::element::attributes::{Color, ColorName, Paint};
use svg_composer::element::path::command::CoordinateType::Absolute;
use svg_composer::element::path::command::{Arc, MoveTo};
use svg_composer::element::{Element, Path};
use svg_composer::Document;

static DEFAULT_NB_SECTIONS: i8 = 26;
static DEFAULT_MAX_RADIUS: f64 = 200.;

pub struct Fingerprint {
    _nb_sections: i8,
    _sections_height: Vec<i32>,
    _inner_circle_radius: f64,
    _nose_size: f64,
    _max_radius: f64,
    _stroke_width: f64,
    _text: String,
    _encoded_text: Vec<i8>,
    _svg_document: Document,
    _position: (f64, f64),
}

impl Fingerprint {
    pub fn new() -> Self {
        let _nb_sections = 26;
        let _nose_size = 10.;
        Fingerprint {
            _nb_sections,
            _sections_height: vec![0, _nb_sections as i32],
            _inner_circle_radius: Self::_compute_inner_circle_radius(_nb_sections, _nose_size),
            _nose_size,
            _max_radius: DEFAULT_MAX_RADIUS,
            _stroke_width: 1.,
            _text: "".to_string(),
            _encoded_text: vec![],
            _svg_document: Document::new(Vec::new(), Some([-500., -500., 5000., 1000.])),
            _position: (0., 0.),
        }
    }
    pub fn set_text(mut self, text: &str) -> Self {
        self._text = text.to_string();
        self
    }
    pub fn render(&mut self) -> String {
        let mut path = Path::new();
        let mut current_end_path = (self._inner_circle_radius, 0.);
        let mut current_dist_to_center = self._inner_circle_radius;
        self._encoded_text = simple_latin_symbols::encode(&self._text);

        path = path
            .set_fill(Paint::from_color(Color::from_rgba(0, 0, 0, 0)))
            .set_stroke(Paint::from_color(Color::from_rgba(0, 0, 0, 255)))
            .add_commands(vec![Box::new(MoveTo {
                point: (self._position.0, self._position.1),
                coordinate_type: Absolute,
            })]);

        for current_char in &self._encoded_text {
            let current_angle: f64 = *current_char as f64 * (2. * PI / self._nb_sections as f64);
            println!("curr angle: {} ({})", current_angle, current_char);
            let move_to = Box::new(MoveTo {
                point: (current_dist_to_center, 0.0),
                coordinate_type: Absolute,
            });
            current_end_path = (
                current_end_path.0 + current_dist_to_center * (current_angle.cos() / 2.),
                current_end_path.1 + current_dist_to_center * (current_angle.sin() / 2.),
            );
            let arc = Box::new(Arc {
                radius: (current_dist_to_center, current_dist_to_center),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: false,
                point: current_end_path,
                coordinate_type: Absolute,
            });
            path = path.add_commands(vec![move_to, arc]);
            current_dist_to_center += 10.;
        }

        self._svg_document.add_element(Box::new(path)).render()
    }
    fn _compute_inner_circle_radius(nb_sections: i8, nose_size: f64) -> f64 {
        (nb_sections as f64 * nose_size * 2.) / 2. * PI
    }
    fn _compute_stroke_width(nb_sections: f64, nose_size: f64) -> f64 {
        // TODO
        1.
    }
}
