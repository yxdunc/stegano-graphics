// std
use std::f64::consts::PI;

// svg composer
use svg_composer::element::attributes::{ClassName, Color, ColorName, Paint, Size, StrokeLineCap};
use svg_composer::element::circle::Circle;
use svg_composer::element::line::Line;
use svg_composer::element::path::command::CoordinateType::Absolute;
use svg_composer::element::path::command::{Arc, MoveTo};
use svg_composer::element::path::Command;
use svg_composer::element::rect::Rectangle;
use svg_composer::element::text::Text;
use svg_composer::element::{Element, Path};
use svg_composer::Document;

// encoder
use crate::encoder::simple_latin_symbols;
use crate::encoder::simple_latin_symbols::CHAR_LIST;
use crate::geometry::Dimensions2D;
use crate::stegs::Steg;
use std::error::Error;

static DEFAULT_NB_SECTIONS: i8 = 26;
static DEFAULT_MAX_RADIUS: f64 = 1000.;

pub struct Spiral {
    _nb_sections: i8,
    _sections_height: Vec<i32>,
    _inner_circle_radius: f64,
    _nose_size: f64,
    _max_radius: f64,
    _stroke_width: f64,
    _text: String,
    _should_render_debug: bool,
    _encoded_text: Vec<i8>,
    _svg_document: Document,
    _position: (f64, f64),
    _draw_explainer: bool,
}

impl Spiral {
    pub fn new() -> Self {
        let _nb_sections = 28;
        let _nose_size = 50.;
        Spiral {
            _nb_sections,
            _sections_height: vec![0; _nb_sections as usize * 2],
            _inner_circle_radius: Self::_compute_inner_circle_radius(_nb_sections, _nose_size),
            _nose_size,
            _max_radius: DEFAULT_MAX_RADIUS,
            _stroke_width: 1.,
            _should_render_debug: false,
            _text: "".to_string(),
            _encoded_text: vec![],
            _svg_document: Document::new(Vec::new(), Some([-1000., -1000., 2000., 2000.])),
            _position: (0., 0.),
            _draw_explainer: false,
        }
    }

    // Setters
    pub fn draw_explainer(mut self, v: bool) -> Self {
        self._draw_explainer = v;
        self
    }

    // Renderer
    pub fn _render(&mut self) {
        let section_angular_len = (2. * PI / self._nb_sections as f64);
        let mut path = Path::new();
        let mut current_angle: f64 = 0.;
        let mut clockwise = true;
        let mut current_dist_to_center = self._inner_circle_radius;
        self._encoded_text = simple_latin_symbols::encode(&self._text);

        path = path
            .set_classes(vec![
                ClassName::from_string("main_path".to_string()).unwrap()
            ])
            .set_fill(Paint::from_color(Color::from_rgba(0, 0, 0, 0)))
            .set_stroke(Paint::from_color(Color::from_rgba(245, 194, 102, 255)))
            .set_stroke_width(Size::from_length(self._compute_stroke_width()))
            .set_stroke_linecap(StrokeLineCap::Round)
            .add_commands(vec![Box::new(MoveTo {
                point: (
                    current_angle.cos() * current_dist_to_center,
                    current_angle.sin() * current_dist_to_center,
                ),
                coordinate_type: Absolute,
            })]);

        for current_char in &self._encoded_text {
            let mut new_commands: Vec<Box<dyn Command>> = Vec::new();
            let previous_angle = current_angle;
            current_angle = *current_char as f64 * section_angular_len;

            if f64::abs(previous_angle - current_angle) > section_angular_len / 2. {
                new_commands.push(Self::_new_arc(
                    previous_angle,
                    current_angle,
                    current_dist_to_center,
                    clockwise,
                ));
            }
            current_dist_to_center += self._nose_size;
            new_commands.push(self._new_nose(current_angle, current_dist_to_center, clockwise));
            path = path.add_commands(new_commands);
            clockwise = !clockwise;
        }

        self._svg_document.add_elements(vec![
            Box::new(
                Rectangle::new()
                    .set_pos((-1000., -1000.))
                    .set_size(Size::from_percentage(100.), Size::from_percentage(100.))
                    .set_fill(Paint::from_color(Color::from_rgba(28, 53, 63, 255))),
            ),
            Box::new(path),
            Box::new(Circle::new().set_pos((0., 0.)).set_radius(10.)),
        ]);
    }

    // Drawing methods
    fn _new_arc(angle_1: f64, angle_2: f64, radius: f64, clockwise: bool) -> Box<Arc> {
        let mut arc_angle;
        let end_point = (radius * (angle_2.cos()), radius * (angle_2.sin()));
        eprintln!("Drawing arc from {} to {}", angle_1, angle_2);
        if clockwise {
            if angle_1 > angle_2 {
                arc_angle = angle_2 + (2. * PI - angle_1);
            } else {
                arc_angle = angle_2 - angle_1;
            }
        } else {
            if angle_1 < angle_2 {
                arc_angle = angle_1 + (2. * PI - angle_2);
            } else {
                arc_angle = angle_1 - angle_2;
            }
        }
        let is_large = arc_angle > PI;

        Box::new(Arc {
            radius: (radius, radius),
            x_axis_rotation: 0.0,
            large_arc_flag: is_large,
            sweep_flag: clockwise,
            point: end_point,
            coordinate_type: Absolute,
        })
    }
    fn _new_nose(&self, angle: f64, radius: f64, clockwise: bool) -> Box<Arc> {
        let end_point = (radius * (angle.cos()), radius * (angle.sin()));

        Box::new(Arc {
            radius: (self._nose_size / 2., self._nose_size / 2.),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: !clockwise,
            point: end_point,
            coordinate_type: Absolute,
        })
    }
    fn _add_explainer(&self) -> Vec<Box<dyn Element>> {
        let mut result: Vec<Box<dyn Element>> = Vec::new();
        for i in 0..self._nb_sections {
            let angle = i as f64 * (2. * PI / (self._nb_sections as f64 * 2.));
            let point_2 = (
                angle.cos() * (self._max_radius - 25.),
                angle.sin() * (self._max_radius - 25.),
            );
            let letter_pos = (
                angle.cos() * (self._max_radius - 10.),
                angle.sin() * (self._max_radius - 10.),
            );
            if i % 2 == 0 {
                result.push(Box::new(
                    Text::new(CHAR_LIST[(i / 2) as usize].to_string()).set_pos(letter_pos),
                ));
            }
            result.push(Box::new(
                Line::new()
                    .set_point_1((0.0, 0.0))
                    .set_point_2(point_2)
                    .set_stroke_width(Size::from_length(5.))
                    .set_stroke(Paint::from_color(Color::from_name(ColorName::Olive))),
            ));
        }
        result
    }

    // Compute methods
    fn _compute_inner_circle_radius(nb_sections: i8, nose_size: f64) -> f64 {
        (nb_sections as f64 * (nose_size / 2. + 20.)) / (2. * PI)
    }
    fn _compute_stroke_width(&self) -> f64 {
        // TODO
        20.
    }
}

impl Steg for Spiral {
    fn set_text(mut self, text: &str) -> Self {
        self._text = text.to_string();
        self
    }

    fn set_render_debug(mut self, should_render_debug: bool) -> Self {
        self._should_render_debug = should_render_debug;
        self
    }

    fn get_stroke_width(&self) -> f64 {
        self._stroke_width
    }

    fn get_shape_dimensions(&self) -> Dimensions2D {
        let radius = self._text.len() as f64 * self._nose_size + self._inner_circle_radius;
        Dimensions2D {
            width: radius,
            height: radius,
        }
    }

    fn render(&mut self) {
        self._render();
    }

    fn get_svg(&self) -> &Document {
        &self._svg_document
    }
}
