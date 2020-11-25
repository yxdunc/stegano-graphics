use crate::encoder::simple_latin_symbols;
use std::borrow::BorrowMut;
use std::f64::consts::PI;
use svg_composer::element::attributes::{Color, ColorName, Paint, Size, StrokeLineCap};
use svg_composer::element::circle::Circle;
use svg_composer::element::line::Line;
use svg_composer::element::path::command::CoordinateType::Absolute;
use svg_composer::element::path::command::{Arc, MoveTo};
use svg_composer::element::path::Command;
use svg_composer::element::rect::Rectangle;
use svg_composer::element::{Element, Path};
use svg_composer::Document;

static DEFAULT_NB_SECTIONS: i8 = 26;
static DEFAULT_MAX_RADIUS: f64 = 1000.;

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
            _sections_height: vec![1; _nb_sections as usize],
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
                    (0..section_2 + 1).collect::<Vec<i8>>(),
                ]
                .concat();
            } else {
                section_list = (section_1..section_2 + 1).collect::<Vec<i8>>();
            }
        } else {
            if section_1 < section_2 {
                section_list = [
                    (0..section_1 + 1).rev().collect::<Vec<i8>>(),
                    (section_2..self._nb_sections).rev().collect::<Vec<i8>>(),
                ]
                .concat();
            } else {
                section_list = (section_2..section_1 + 1).rev().collect::<Vec<i8>>();
            }
        }
        section_list
    }
    fn _compute_nosed_angle(&self, section: i8, clockwise: bool) -> f64 {
        let angular_len_nose = (2. * PI / self._nb_sections as f64) / self._nose_size;
        if clockwise {
            self._angle_from_section(section) - angular_len_nose
        } else {
            self._angle_from_section(section) + angular_len_nose
        }
    }
    fn _angle_from_section(&self, section: i8) -> f64 {
        section as f64 * (2. * PI / self._nb_sections as f64)
    }
    fn _new_compressed_arc(
        &mut self,
        section_1: i8,
        section_2: i8,
        clockwise: bool,
    ) -> Vec<Box<dyn Command>> {
        let mut sections = self._generate_section_list(section_1, section_2, clockwise);
        let mut compressed_arc: Vec<Box<dyn Command>> = Vec::new();

        println!(
            "section_1: {}, section_2: {}, clockwise: {}",
            section_1, section_2, clockwise
        );
        println!("sections: {:?}", sections);
        let mut previous_section = sections.remove(0);
        println!("popped ' ' sections: {:?}", sections);
        for i in 0..sections.len() {
            let section = sections[i];
            let radius =
                self._inner_circle_radius + self._sections_height[section as usize] as f64 * 50.;
            self._sections_height[previous_section as usize] += 1;
            println!(
                "[{}], radius: {}",
                previous_section, self._sections_height[section as usize]
            );
            let mut angle_1 = self._angle_from_section(previous_section);
            let mut angle_2 = self._angle_from_section(section);
            if i == sections.len() - 1 {
                angle_2 = self._compute_nosed_angle(section, clockwise);
            }
            let starting_point: (f64, f64) = (radius * (angle_1.cos()), radius * (angle_1.sin()));
            let end_point: (f64, f64) = (radius * (angle_2.cos()), radius * (angle_2.sin()));
            // compressed_arc.push(Box::new(MoveTo {
            //     point: starting_point,
            //     coordinate_type: Absolute,
            // }));
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
        self._sections_height[previous_section as usize] += 1;

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
    fn _new_nose_compressed(&self, section: i8, clockwise: bool) -> Box<Arc> {
        self._new_nose(
            self._compute_nosed_angle(section, clockwise),
            self._inner_circle_radius + self._sections_height[section as usize] as f64 * 50.,
            clockwise,
        )
    }
    fn _generate_rays(&self) -> Vec<Box<dyn Element>> {
        let mut result: Vec<Box<dyn Element>> = Vec::new();
        for i in 0..self._nb_sections {
            let angle = i as f64 * (2. * PI / self._nb_sections as f64);
            let point_2 = (
                angle.cos() * self._max_radius,
                angle.sin() * self._max_radius,
            );
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
            path = path.add_command(self._new_nose_compressed(current_section, clockwise));
            clockwise = !clockwise;
        }

        let rays: Vec<Box<dyn Element>> = self._generate_rays();
        self._svg_document.add_element(Box::new(
            Rectangle::new()
                .set_pos((-1000., -1000.))
                .set_size(Size::from_percentage(100.), Size::from_percentage(100.))
                .set_fill(Paint::from_color(Color::from_rgba(28, 53, 63, 255))),
        ));
        self._svg_document.add_elements(rays);
        self._svg_document.add_element(Box::new(path));
        self._svg_document
            .add_element(Box::new(Circle::new().set_pos((0., 0.)).set_radius(10.)));
        self._svg_document.render()
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_generate_section_list_clockwise_continuous() {
        let mut fp = Fingerprint::new();
        fp._nb_sections = 10;
        let sections = fp._generate_section_list(0, 5, true);
        assert_eq!(sections, vec![0, 1, 2, 3, 4, 5]);
    }
    #[test]
    fn should_generate_section_list_anti_clockwise_continuous() {
        let mut fp = Fingerprint::new();
        fp._nb_sections = 10;
        let sections = fp._generate_section_list(5, 0, false);
        assert_eq!(sections, vec![5, 4, 3, 2, 1, 0]);
    }
    #[test]
    fn should_generate_section_list_clockwise_not_continuous() {
        let mut fp = Fingerprint::new();
        fp._nb_sections = 10;
        let sections = fp._generate_section_list(8, 4, true);
        assert_eq!(sections, vec![8, 9, 0, 1, 2, 3, 4]);
    }
    #[test]
    fn should_generate_section_list_anti_clockwise_not_continuous() {
        let mut fp = Fingerprint::new();
        fp._nb_sections = 10;
        let sections = fp._generate_section_list(4, 8, false);
        assert_eq!(sections, vec![4, 3, 2, 1, 0, 9, 8]);
    }
}
