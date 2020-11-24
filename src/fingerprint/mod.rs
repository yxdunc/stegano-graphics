use crate::encoder::simple_latin_symbols;
use std::f64::consts::PI;
use svg_composer::element::attributes::{Color, ColorName, Paint, Size, StrokeLineCap};
use svg_composer::element::circle::Circle;
use svg_composer::element::path::command::CoordinateType::Absolute;
use svg_composer::element::path::command::{Arc, MoveTo};
use svg_composer::element::path::Command;
use svg_composer::element::rect::Rectangle;
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
        let _nose_size = 2.;
        Fingerprint {
            _nb_sections,
            _sections_height: vec![0, _nb_sections as i32],
            _inner_circle_radius: Self::_compute_inner_circle_radius(_nb_sections, _nose_size),
            _nose_size,
            _max_radius: DEFAULT_MAX_RADIUS,
            _stroke_width: 1.,
            _text: "".to_string(),
            _encoded_text: vec![],
            _svg_document: Document::new(Vec::new(), Some([-1000., -1000., 2000., 2000.])),
            _position: (0., 0.),
        }
    }
    pub fn set_text(mut self, text: &str) -> Self {
        self._text = text.to_string();
        self
    }
    fn _new_arc(angle_1: f64, angle_2: f64, radius: f64, clockwise: bool) -> Box<Arc> {
        let mut arc_angle;
        let end_point = (radius * (angle_2.cos()), radius * (angle_2.sin()));
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
    fn _generate_section_list(&self, section_1: i8, section_2: i8, clockwise: bool) -> Vec<i8> {
        let mut section_list: Vec<i8> = Vec::new();
        if clockwise {
            if section_1 > section_2 {
                section_list = [
                    (section_1..self._nb_sections).collect::<Vec<i8>>(),
                    (0..section_2).collect::<Vec<i8>>(),
                ]
                .concat();
            } else {
                section_list = (section_1..section_2).collect::<Vec<i8>>();
            }
        } else {
            if section_1 < section_2 {
                section_list = [
                    (0..section_1).rev().collect::<Vec<i8>>(),
                    (section_2..self._nb_sections).rev().collect::<Vec<i8>>(),
                ]
                .concat();
            } else {
                section_list = (section_2..section_1).rev().collect::<Vec<i8>>();
            }
        }
        section_list
    }
    fn _new_compressed_arc(
        &mut self,
        section_1: i8,
        section_2: i8,
        clockwise: bool,
    ) -> Vec<Box<dyn Command>> {
        let mut sections = self._generate_section_list(section_1, section_2, clockwise);
        let mut compressed_arc: Vec<Box<dyn Command>> = Vec::new();

        let mut previous_section = sections.remove(0);
        for section in sections {
            let radius = self._sections_height[section as usize] as f64 * self._nose_size;
            let angle_1 = previous_section as f64 * (2. * PI / self._nb_sections as f64);
            let angle_2 = section as f64 * (2. * PI / self._nb_sections as f64);
            let starting_point: (f64, f64) = (radius * (angle_1.cos()), radius * (angle_1.sin()));
            let end_point: (f64, f64) = (radius * (angle_2.cos()), radius * (angle_2.sin()));
            compressed_arc.push(Box::new(MoveTo {
                point: starting_point,
                coordinate_type: Absolute,
            }));
            compressed_arc.push(Box::new(Arc {
                radius: (radius, radius),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: clockwise,
                point: end_point,
                coordinate_type: Absolute,
            }));
            previous_section = section;
        }

        compressed_arc
    }
    fn _new_nose(&self, angle: f64, radius: f64, clockwise: bool) -> Box<Arc> {
        let end_point = (radius * (angle.cos()), radius * (angle.sin()));

        Box::new(Arc {
            radius: (self._nose_size, self._nose_size),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: !clockwise,
            point: end_point,
            coordinate_type: Absolute,
        })
    }
    pub fn render(&mut self) -> String {
        let mut path = Path::new();
        let mut current_section: i8 = 0;
        let mut clockwise = true;
        let mut current_dist_to_center = self._inner_circle_radius;
        self._encoded_text = simple_latin_symbols::encode(&self._text);

        path = path
            .set_fill(Paint::from_color(Color::from_rgba(0, 0, 0, 0)))
            .set_stroke(Paint::from_color(Color::from_rgba(245, 194, 102, 255)))
            .set_stroke_width(Size::from_length(self._compute_stroke_width()))
            .set_stroke_linecap(StrokeLineCap::Round)
            .add_commands(vec![Box::new(MoveTo {
                point: (
                    (current_section as f64 * (2. * PI / self._nb_sections as f64)).cos()
                        * current_dist_to_center,
                    (current_section as f64 * (2. * PI / self._nb_sections as f64)).sin()
                        * current_dist_to_center,
                ),
                coordinate_type: Absolute,
            })]);

        for current_char in &self._encoded_text.clone() {
            let previous_section = current_section;
            current_section = *current_char;

            let compressed_arc =
                self._new_compressed_arc(previous_section, current_section, clockwise);
            path = path.add_commands(compressed_arc);
            clockwise = !clockwise;
        }

        self._svg_document
            .add_elements(vec![
                Box::new(
                    Rectangle::new()
                        .set_pos((-1000., -1000.))
                        .set_size(Size::from_percentage(100.), Size::from_percentage(100.))
                        .set_fill(Paint::from_color(Color::from_rgba(28, 53, 63, 255))),
                ),
                Box::new(path),
                Box::new(Circle::new().set_pos((0., 0.)).set_radius(10.)),
            ])
            .render()
    }
    pub fn render_circular(&mut self) -> String {
        let mut path = Path::new();
        let mut current_angle: f64 = 0.;
        let mut clockwise = true;
        let mut current_dist_to_center = self._inner_circle_radius;
        self._encoded_text = simple_latin_symbols::encode(&self._text);

        path = path
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
            let previous_angle = current_angle;
            current_angle = *current_char as f64 * (2. * PI / self._nb_sections as f64);

            let arc = Self::_new_arc(
                previous_angle,
                current_angle,
                current_dist_to_center,
                clockwise,
            );
            current_dist_to_center += 50.;
            let nose = self._new_nose(current_angle, current_dist_to_center, clockwise);
            path = path.add_commands(vec![arc, nose]);
            clockwise = !clockwise;
        }

        self._svg_document
            .add_elements(vec![
                Box::new(
                    Rectangle::new()
                        .set_pos((-1000., -1000.))
                        .set_size(Size::from_percentage(100.), Size::from_percentage(100.))
                        .set_fill(Paint::from_color(Color::from_rgba(28, 53, 63, 255))),
                ),
                Box::new(path),
                Box::new(Circle::new().set_pos((0., 0.)).set_radius(10.)),
            ])
            .render()
    }
    fn _compute_inner_circle_radius(nb_sections: i8, nose_size: f64) -> f64 {
        (nb_sections as f64 * nose_size * 2.) / 2. * PI
    }
    fn _compute_stroke_width(&self) -> f64 {
        // TODO
        20.
    }
}
