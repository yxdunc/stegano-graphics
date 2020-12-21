use crate::encoder::simple_latin_symbols;
use crate::encoder::simple_latin_symbols::CHAR_LIST;
use std::borrow::BorrowMut;
use std::cmp::min;
use std::f64::consts::PI;
use std::panic::resume_unwind;
use svg_composer::element::attributes::{Color, ColorName, Paint, Size, StrokeLineCap};
use svg_composer::element::circle::Circle;
use svg_composer::element::line::Line;
use svg_composer::element::path::command::CoordinateType::Absolute;
use svg_composer::element::path::command::{Arc, CubicBezierCurve, LineTo, LineToOption, MoveTo};
use svg_composer::element::path::Command;
use svg_composer::element::rect::Rectangle;
use svg_composer::element::text::Text;
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
        let _nb_sections = 28;
        let _nose_size = 50.;
        Fingerprint {
            _nb_sections,
            _sections_height: vec![0; _nb_sections as usize * 2],
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

    // Setters
    pub fn set_text(mut self, text: &str) -> Self {
        self._text = text.to_string();
        self
    }

    // Renderers
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
                    (0.5 * (2. * PI / self._nb_sections as f64)).cos() * current_dist_to_center,
                    (0.5 as f64 * (2. * PI / self._nb_sections as f64)).sin()
                        * current_dist_to_center,
                ),
                coordinate_type: Absolute,
            })]);

        // let last_section = self._sections_height.len() - 1 as usize;
        // self._sections_height[last_section] = 1;
        for current_char in &self._encoded_text.clone() {
            let previous_section = current_section;
            current_section = *current_char;

            let compressed_arc =
                self._new_compressed_arc(previous_section, current_section, clockwise);
            path = path.add_commands(compressed_arc);
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
            current_dist_to_center += self._nose_size;
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

    // Drawing methods
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
    fn _new_downward_nose(
        &self,
        section_0: i8,
        section_1: i8,
        clockwise: bool,
    ) -> Vec<Box<dyn Command>> {
        let split_arc = self._sections_height[section_0 as usize] > 3;
        eprintln!("-----> downward_nose");
        eprintln!(
            "-----> section_0 height: {}",
            self._sections_height[section_0 as usize]
        );
        eprintln!("-----> splitting arc: {}", split_arc);
        let mut downward_nose: Vec<Box<dyn Command>> = Vec::new();

        let angle_section_1 =
            Self::_compute_angle_from_section(section_1, self._nb_sections as i32 * 2);
        let section_angle_delta =
            Self::_compute_angle_from_section(1, self._nb_sections as i32 * 2);
        let mut nose_radius = Self::_compute_size_from_angular(
            section_angle_delta / 2.,
            self._compute_distance_to_center(section_0),
        );
        let distance_to_center = self._compute_distance_to_center(section_0);
        let arc_2_end_point = (
            (distance_to_center + self._nose_size / 2.) * (angle_section_1.cos()),
            (distance_to_center + self._nose_size / 2.) * (angle_section_1.sin()),
        );
        if split_arc {
            eprintln!("-----> Splitting arc");
            nose_radius = self._nose_size / 2.;
            let nose_angular_size = self._compute_nose_angular_size(distance_to_center)
                * (-2. * (!clockwise) as i8 as f64 + 1.);
            let angle_section_0 =
                Self::_compute_angle_from_section(section_0, self._nb_sections as i32 * 2)
                    + nose_angular_size;
            let arc_1_end_point = (
                (distance_to_center) * (angle_section_0.cos()),
                (distance_to_center) * (angle_section_0.sin()),
            );
            let angle_arc_2_start = angle_section_1 - nose_angular_size;
            let arc_2_start_point = (
                (distance_to_center) * (angle_arc_2_start.cos()),
                (distance_to_center) * (angle_arc_2_start.sin()),
            );
            downward_nose.push(Box::new(Arc {
                radius: (nose_radius, nose_radius),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: !clockwise,
                point: arc_1_end_point,
                coordinate_type: Absolute,
            }));
            downward_nose.push(Box::new(LineTo {
                point: arc_2_start_point,
                option: LineToOption::Default,
                coordinate_type: Absolute,
            }));
        }
        downward_nose.push(Box::new(Arc {
            radius: (nose_radius, nose_radius),
            x_axis_rotation: 0.0,
            large_arc_flag: false,
            sweep_flag: !clockwise,
            point: arc_2_end_point,
            coordinate_type: Absolute,
        }));

        downward_nose
    }
    fn _new_compressed_arc(
        &mut self,
        section_start: i8,
        section_end: i8,
        clockwise: bool,
    ) -> Vec<Box<dyn Command>> {
        let mut sections =
            self._generate_section_list(section_start * 2, section_end * 2, clockwise);
        let mut compressed_arc: Vec<Box<dyn Command>> = Vec::new();

        let mut height_increment_to_apply_after_arc: Vec<i32> =
            vec![0; self._nb_sections as usize * 2];
        eprintln!(
            "> fn _new_compressed_arc({}, {}, {})",
            section_start, section_end, clockwise
        );
        eprintln!("-> sections {:?}", sections);
        let mut touchy_nose = false;
        let mut i = 0;
        while i < sections.len() {
            let section_0 = sections[i];
            let section_1 = self._change_section(section_0, 1, clockwise);
            let section_2 = self._change_section(section_1, 1, clockwise);
            let section_3 = self._change_section(section_2, 1, clockwise);
            let section_minus_1 = self._change_section(section_0, -1, clockwise);
            let section_minus_2 = self._change_section(section_minus_1, -1, clockwise);
            let section_minus_3 = self._change_section(section_minus_2, -1, clockwise);

            let radius = self._compute_distance_to_center(section_0);
            let mut angle_1 =
                Self::_compute_angle_from_section(section_0, self._nb_sections as i32 * 2);
            let mut angle_2 =
                Self::_compute_angle_from_section(section_1, self._nb_sections as i32 * 2);
            if i == sections.len() - 2 {
                angle_2 = self._compute_nosed_angle(section_1, clockwise);
            }
            let starting_point: (f64, f64) = (radius * (angle_1.cos()), radius * (angle_1.sin()));
            let end_point: (f64, f64) = (radius * (angle_2.cos()), radius * (angle_2.sin()));

            // {
            //     eprintln!("--> current sections");
            //     eprintln!(
            //         "--> {}, {}, ({}), {}, {}, {}",
            //         section_minus_2, section_minus_1, section_0, section_1, section_2, section_3
            //     );
            //     eprintln!("--> sections heights");
            //     eprintln!(
            //         "--> {}, {}, ({}), {}, {}, {}",
            //         self._sections_height[section_minus_2 as usize],
            //         self._sections_height[section_minus_1 as usize],
            //         self._sections_height[section_0 as usize],
            //         self._sections_height[section_1 as usize],
            //         self._sections_height[section_2 as usize],
            //         self._sections_height[section_3 as usize],
            //     );
            // } // debug prints

            if self._sections_height[section_0 as usize]
                != self._sections_height[section_1 as usize]
                && i < sections.len() - 1
            {
                eprintln!("---> height transition");

                if self._sections_height[section_minus_2 as usize]
                    > self._sections_height[section_minus_1 as usize]
                    && self._sections_height[section_minus_1 as usize] - 1
                        == self._sections_height[section_0 as usize]
                    && self._sections_height[section_0 as usize]
                        < self._sections_height[section_1 as usize]
                {
                    eprintln!("----> getting out of pit");
                    let section_angle_delta =
                        Self::_compute_angle_from_section(1, self._nb_sections as i32 * 2);
                    let tmp_radius = Self::_compute_size_from_angular(
                        section_angle_delta / 2.,
                        self._compute_distance_to_center(section_0),
                    );
                    let tmp_start_point = (
                        (radius + self._nose_size / 2.) * (angle_1.cos()),
                        (radius + self._nose_size / 2.) * (angle_1.sin()),
                    );
                    compressed_arc.pop();
                    compressed_arc.pop();
                    compressed_arc.push(Box::new(Arc {
                        radius: (tmp_radius, tmp_radius),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: !clockwise,
                        point: tmp_start_point,
                        coordinate_type: Absolute,
                    }));
                    let mut height_transition =
                        self._new_height_transition(section_0, section_1, clockwise, true);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    compressed_arc.append(&mut height_transition);
                    compressed_arc.pop();
                    let tmp_angle_2 = self._compute_nosed_angle(section_1, clockwise);
                    let tmp_radius = self._compute_distance_to_center(section_1);
                    let tmp_start_nose_point: (f64, f64) = (
                        (tmp_radius) * (tmp_angle_2.cos()),
                        (tmp_radius) * (tmp_angle_2.sin()),
                    );
                    compressed_arc.push(Box::new(Arc {
                        radius: (
                            self._compute_distance_to_center(section_1),
                            self._compute_distance_to_center(section_1),
                        ),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: clockwise,
                        point: tmp_start_nose_point,
                        coordinate_type: Absolute,
                    }));
                } else if i == sections.len() - 3
                    && self._sections_height[section_0 as usize]
                        > self._sections_height[section_1 as usize]
                {
                    eprintln!("----> lowering before nose");
                    i += 1;
                    touchy_nose = true;
                    compressed_arc.append(
                        &mut self._new_height_transition(section_0, section_1, clockwise, false),
                    );
                    let angle_3 =
                        Self::_compute_angle_from_section(section_2, self._nb_sections as i32 * 2);
                    let section_angle_delta =
                        Self::_compute_angle_from_section(1, self._nb_sections as i32 * 2);
                    let tmp_radius = Self::_compute_size_from_angular(
                        section_angle_delta / 2.,
                        self._compute_distance_to_center(section_1),
                    );
                    let radius_0 = self._compute_distance_to_center(section_1);
                    let tmp_start_point = (
                        (radius_0 + self._nose_size / 2.) * (angle_3.cos()),
                        (radius_0 + self._nose_size / 2.) * (angle_3.sin()),
                    );
                    compressed_arc.pop();
                    compressed_arc
                        .append(&mut self._new_downward_nose(section_1, section_2, clockwise));
                    self._sections_height[section_0 as usize] += 1;
                    self._sections_height[section_1 as usize] =
                        self._sections_height[section_0 as usize];
                    let mut height_transition =
                        self._new_height_transition(section_2, section_1, !clockwise, false);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    height_transition.pop();
                    compressed_arc.append(&mut height_transition);
                    self._sections_height[section_2 as usize] =
                        self._sections_height[section_0 as usize];
                } else if i < sections.len() - 2
                    && self._sections_height[section_0 as usize]
                        > self._sections_height[section_1 as usize]
                    && self._sections_height[section_0 as usize]
                        <= self._sections_height[section_2 as usize]
                {
                    eprintln!("----> tight pit");
                    compressed_arc.push(Box::new(Arc {
                        radius: (radius, radius),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: clockwise,
                        point: end_point,
                        coordinate_type: Absolute,
                    }));
                    self._sections_height[section_1 as usize] =
                        self._sections_height[section_0 as usize];
                } else if i == sections.len() - 2
                    && self._sections_height[section_0 as usize]
                        > self._sections_height[section_1 as usize]
                {
                    eprintln!("----> lowering in nose");
                    i += 1;
                    touchy_nose = true;
                    compressed_arc.append(
                        &mut self._new_height_transition(section_0, section_1, clockwise, false),
                    );
                    let angle_3 =
                        Self::_compute_angle_from_section(section_2, self._nb_sections as i32 * 2);
                    let section_angle_delta =
                        Self::_compute_angle_from_section(1, self._nb_sections as i32 * 2);
                    let tmp_radius = Self::_compute_size_from_angular(
                        section_angle_delta / 2.,
                        self._compute_distance_to_center(section_1),
                    );
                    let radius_0 = self._compute_distance_to_center(section_1);
                    let tmp_start_point = (
                        (radius_0 + self._nose_size / 2.) * (angle_3.cos()),
                        (radius_0 + self._nose_size / 2.) * (angle_3.sin()),
                    );
                    compressed_arc.pop();
                    compressed_arc.push(Box::new(Arc {
                        radius: (tmp_radius, tmp_radius),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: !clockwise,
                        point: tmp_start_point,
                        coordinate_type: Absolute,
                    }));
                    self._sections_height[section_0 as usize] += 1;
                    self._sections_height[section_1 as usize] =
                        self._sections_height[section_0 as usize];
                    let mut height_transition =
                        self._new_height_transition(section_2, section_1, !clockwise, false);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    height_transition.pop();
                    compressed_arc.append(&mut height_transition);
                    self._sections_height[section_2 as usize] =
                        self._sections_height[section_0 as usize] + 1;
                } else if i == sections.len() - 4
                    && self._sections_height[section_0 as usize]
                        > self._sections_height[section_1 as usize]
                    && self._sections_height[section_1 as usize] < 4
                {
                    eprintln!("----> lowering in close to center 3 before nose");
                    i += 2;
                    touchy_nose = true;
                    let section_angle_delta =
                        Self::_compute_angle_from_section(1, self._nb_sections as i32 * 2);
                    let radius_0 = self._compute_distance_to_center(section_1);
                    let tmp_radius = Self::_compute_size_from_angular(
                        section_angle_delta / 2.,
                        self._compute_distance_to_center(section_1),
                    );
                    self._sections_height[section_1 as usize] =
                        self._sections_height[section_0 as usize];
                    compressed_arc.append(
                        &mut self._new_height_transition(section_1, section_2, clockwise, false),
                    );
                    let angle_3 =
                        Self::_compute_angle_from_section(section_3, self._nb_sections as i32 * 2);

                    let tmp_start_point = (
                        (radius_0 + self._nose_size / 2.) * (angle_3.cos()),
                        (radius_0 + self._nose_size / 2.) * (angle_3.sin()),
                    );
                    compressed_arc.pop();
                    compressed_arc.pop();
                    compressed_arc.push(Box::new(Arc {
                        radius: (tmp_radius, tmp_radius),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: !clockwise,
                        point: tmp_start_point,
                        coordinate_type: Absolute,
                    }));
                    self._sections_height[section_0 as usize] += 1;
                    self._sections_height[section_2 as usize] =
                        self._sections_height[section_0 as usize];
                    let mut height_transition =
                        self._new_height_transition(section_3, section_2, !clockwise, false);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    height_transition.pop();
                    compressed_arc.append(&mut height_transition);
                    self._sections_height[section_3 as usize] =
                        self._sections_height[section_0 as usize];
                    self._sections_height[section_2 as usize] =
                        self._sections_height[section_0 as usize];
                    self._sections_height[section_1 as usize] =
                        self._sections_height[section_0 as usize];
                } else if i == sections.len() - 2
                    && self._sections_height[section_0 as usize]
                        < self._sections_height[section_1 as usize]
                {
                    eprintln!("----> raise before nose");
                    touchy_nose = true;
                    compressed_arc.pop();
                    compressed_arc.pop();
                    let orig_section_height_minus_2 =
                        self._sections_height[section_minus_2 as usize];
                    let orig_section_height_minus_3 =
                        self._sections_height[section_minus_3 as usize];
                    self._sections_height[section_minus_1 as usize] -= 1;
                    self._sections_height[section_0 as usize] =
                        self._sections_height[section_1 as usize];

                    let mut height_transition =
                        self._new_height_transition(section_minus_1, section_0, clockwise, false);
                    if orig_section_height_minus_3 - 1 > orig_section_height_minus_2 {
                        eprintln!("down before up to nose");
                        compressed_arc.pop();
                        compressed_arc.pop();
                        height_transition.remove(0);
                        height_transition.remove(0);
                        let section_angle_delta =
                            Self::_compute_angle_from_section(1, self._nb_sections as i32 * 2);
                        let tmp_radius = Self::_compute_size_from_angular(
                            section_angle_delta / 2.,
                            self._compute_distance_to_center(section_minus_1),
                        );
                        let tmp_angle = Self::_compute_angle_from_section(
                            section_minus_1,
                            self._nb_sections as i32 * 2,
                        );
                        let tmp_start_point = (
                            (radius + self._nose_size / 2.) * (tmp_angle.cos()),
                            (radius + self._nose_size / 2.) * (tmp_angle.sin()),
                        );
                        compressed_arc.push(Box::new(Arc {
                            radius: (tmp_radius, tmp_radius),
                            x_axis_rotation: 0.0,
                            large_arc_flag: false,
                            sweep_flag: !clockwise,
                            point: tmp_start_point,
                            coordinate_type: Absolute,
                        }));
                    }

                    compressed_arc.append(&mut height_transition);
                    self._sections_height[section_minus_1 as usize] =
                        self._sections_height[section_1 as usize] + 1;
                    self._sections_height[section_0 as usize] += 1;
                    let tmp_angle_2 = self._compute_nosed_angle(section_1, clockwise);
                    let tmp_radius = self._compute_distance_to_center(section_0);
                    let tmp_start_nose_point: (f64, f64) = (
                        (tmp_radius - self._nose_size) * (tmp_angle_2.cos()),
                        (tmp_radius - self._nose_size) * (tmp_angle_2.sin()),
                    );
                    compressed_arc.push(Box::new(Arc {
                        radius: (
                            self._compute_distance_to_center(section_0) - self._nose_size,
                            self._compute_distance_to_center(section_0) - self._nose_size,
                        ),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: clockwise,
                        point: tmp_start_nose_point,
                        coordinate_type: Absolute,
                    }));
                    self._sections_height[section_1 as usize] += 1;
                    compressed_arc.push(self._new_nose_compressed(section_end, clockwise, false));
                } else if i == sections.len() - 3
                    && self._sections_height[section_0 as usize]
                        < self._sections_height[section_1 as usize]
                {
                    eprintln!("----> raising");
                    compressed_arc.push(Box::new(Arc {
                        radius: (radius, radius),
                        x_axis_rotation: 0.0,
                        large_arc_flag: false,
                        sweep_flag: clockwise,
                        point: end_point,
                        coordinate_type: Absolute,
                    }));
                    self._sections_height[section_1 as usize] =
                        self._sections_height[section_0 as usize];
                } else if i < sections.len() - 2
                    && self._sections_height[section_0 as usize]
                        > self._sections_height[section_1 as usize]
                    && self._sections_height[section_1 as usize]
                        == self._sections_height[section_2 as usize]
                    && self._sections_height[section_2 as usize]
                        < self._sections_height[section_3 as usize]
                {
                    eprintln!("----> diving in pit");
                    compressed_arc.append(
                        &mut self._new_height_transition(section_0, section_1, clockwise, false),
                    );
                // i += 1;
                } else {
                    eprintln!("----> default case");
                    if self._sections_height[section_0 as usize]
                        < self._sections_height[section_1 as usize]
                    {
                        compressed_arc.pop();
                    }
                    compressed_arc.append(
                        &mut self._new_height_transition(section_0, section_1, clockwise, false),
                    );
                }
                if self._sections_height[section_0 as usize]
                    < self._sections_height[section_1 as usize]
                    && !touchy_nose
                {
                    let height_difference = (self._sections_height[section_1 as usize]
                        - self._sections_height[section_0 as usize])
                        .abs();
                    let prev_section = if section_0 - 1 < 0 { 0 } else { section_0 - 1 };
                    // self._sections_height[prev_section as usize] += 1;
                    height_increment_to_apply_after_arc[section_0 as usize] = height_difference;
                } else if !touchy_nose {
                    // going down
                    let height_difference = (self._sections_height[section_1 as usize]
                        - self._sections_height[section_0 as usize])
                        .abs();
                    let next_section = if section_1 + 1 >= self._sections_height.len() as i8 {
                        self._sections_height.len() - 1
                    } else {
                        (section_1 + 1) as usize
                    };
                    // self._sections_height[next_section] += 1;
                    // height_increment_to_apply_after_arc[next_section as usize] = height_difference;
                    height_increment_to_apply_after_arc[section_1 as usize] = height_difference;
                }
            } else if i < sections.len() - 1 {
                compressed_arc.push(Box::new(Arc {
                    radius: (radius, radius),
                    x_axis_rotation: 0.0,
                    large_arc_flag: false,
                    sweep_flag: clockwise,
                    point: end_point,
                    coordinate_type: Absolute,
                }));
            }
            if !touchy_nose {
                self._sections_height[section_0 as usize] += 1;
            }
            // touchy_nose == false;
            // } else {
            //     touchy_nose = false;
            // }
            i += 1;
        }
        // self._sections_height[sections[sections.len() - 1 as usize] as usize] += 1;
        eprintln!("--> sections heights:\n--> {:?}", self._sections_height);
        eprintln!(
            "--> sections height to add:\n--> {:?}",
            height_increment_to_apply_after_arc
        );
        if !touchy_nose {
            compressed_arc.push(self._new_nose_compressed(section_end, clockwise, touchy_nose));
        } else {
            eprintln!("non touchy")
        }
        self._sections_height = self
            ._sections_height
            .clone()
            .iter()
            .zip(height_increment_to_apply_after_arc.iter())
            .map((|(&a, &b)| a + b))
            .collect::<Vec<i32>>();
        eprintln!(
            "--> sections heights added\n--> {:?}",
            self._sections_height
        );
        self._fill_tight_gaps();

        compressed_arc
    }
    fn _new_height_transition(
        &self,
        section_0: i8,
        section_1: i8,
        clockwise: bool,
        include_nose: bool,
    ) -> Vec<Box<dyn Command>> {
        let is_raising =
            self._sections_height[section_0 as usize] < self._sections_height[section_1 as usize];
        let section_0 = section_0 as f64;
        let section_1 = section_1 as f64;

        let sweep = is_raising ^ clockwise;
        let height_change;
        let angle_change;
        if is_raising && clockwise {
            height_change = 0.5;
            angle_change = 0.;
        } else if !is_raising && !clockwise {
            height_change = -0.5;
            angle_change = 1.;
        } else if is_raising && !clockwise {
            height_change = 0.5;
            angle_change = 0.;
        } else {
            height_change = -0.5;
            angle_change = -1.;
        };
        let turn_1_start_radius = self._inner_circle_radius
            + (self._sections_height[section_0 as usize] as f64) * self._nose_size;
        let angular_len_nose = if clockwise {
            self._compute_nose_angular_size(turn_1_start_radius) * -1.
        } else {
            self._compute_nose_angular_size(turn_1_start_radius)
        };
        let turn_1_start_angle = (section_0 - angle_change) as f64
            * (2. * PI / (self._nb_sections * 2) as f64)
            + angular_len_nose;
        let turn_1_start_point = (
            turn_1_start_radius * (turn_1_start_angle.cos()),
            turn_1_start_radius * (turn_1_start_angle.sin()),
        );
        let turn_1_end_radius = self._inner_circle_radius
            + (self._sections_height[section_0 as usize] as f64 + height_change) * self._nose_size;
        let turn_1_end_angle =
            (section_0 - angle_change) as f64 * (2. * PI / (self._nb_sections * 2) as f64);
        let turn_1_end_point = (
            turn_1_end_radius * (turn_1_end_angle.cos()),
            turn_1_end_radius * (turn_1_end_angle.sin()),
        );
        let turn_2_start_radius = self._inner_circle_radius
            + (self._sections_height[section_1 as usize] as f64 - height_change) * self._nose_size;
        let turn_2_start_angle = turn_1_end_angle;
        let turn_2_start_point = (
            turn_2_start_radius * (turn_2_start_angle.cos()),
            turn_2_start_radius * (turn_2_start_angle.sin()),
        );
        let turn_2_end_radius = self._inner_circle_radius
            + (self._sections_height[section_1 as usize] as f64) * self._nose_size;
        let angular_len_nose = if clockwise {
            self._nose_size / 2. / turn_2_end_radius * -1.
        } else {
            self._nose_size / 2. / turn_2_end_radius
        };

        let turn_2_end_angle = turn_2_start_angle - angular_len_nose;
        let turn_2_end_point = (
            turn_2_end_radius * (turn_2_end_angle.cos()),
            turn_2_end_radius * (turn_2_end_angle.sin()),
        );
        let end_section_angle =
            (section_1 + angle_change * 2.) as f64 * (2. * PI / (self._nb_sections * 2) as f64);
        let end_section_point = (
            turn_2_end_radius * (end_section_angle.cos()),
            turn_2_end_radius * (end_section_angle.sin()),
        );

        let angle_2_radius = (
            (turn_1_end_point.1 - turn_2_start_point.1).abs(),
            (turn_1_end_point.1 - turn_2_start_point.1).abs(),
            // (turn_1_end_point.0 - turn_2_start_point.0).abs(),
        );

        let mut result: Vec<Box<dyn Command>> = vec![
            Box::new(Arc {
                radius: (turn_1_start_radius, turn_1_start_radius),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: clockwise,
                point: turn_1_start_point,
                coordinate_type: Absolute,
            }),
            Box::new(Arc {
                radius: (self._nose_size / 2., self._nose_size / 2.),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: sweep,
                point: turn_1_end_point,
                coordinate_type: Absolute,
            }),
            Box::new(LineTo {
                point: turn_2_start_point,
                option: LineToOption::Default,
                coordinate_type: Absolute,
            }),
            Box::new(Arc {
                radius: (self._nose_size / 2., self._nose_size / 2.),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: !sweep,
                point: turn_2_end_point,
                coordinate_type: Absolute,
            }),
        ];
        if is_raising {
            result.push(Box::new(Arc {
                radius: (turn_2_end_radius, turn_2_end_radius),
                x_axis_rotation: 0.0,
                large_arc_flag: false,
                sweep_flag: clockwise,
                point: end_section_point,
                coordinate_type: Absolute,
            }));
        }
        result
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
    fn _new_nose_compressed(&self, section: i8, clockwise: bool, is_touching: bool) -> Box<Arc> {
        let section = section as i8 * 2;
        let section_height = if is_touching {
            self._sections_height[section as usize] - 1
        } else {
            self._sections_height[section as usize]
        } as f64;
        self._new_nose(
            self._compute_nosed_angle(section, clockwise),
            self._inner_circle_radius + section_height * self._nose_size,
            clockwise,
        )
    }

    // Compute methods
    fn _compute_angle_from_section(section: i8, nb_sections: i32) -> f64 {
        let nb_sections: f64 = nb_sections as f64;
        section as f64 * (2. * PI / nb_sections)
    }
    fn _compute_distance_to_center(&self, section: i8) -> f64 {
        self._inner_circle_radius + self._sections_height[section as usize] as f64 * self._nose_size
    }
    fn _compute_stroke_angular_size(&self, distance_to_center: f64) -> f64 {
        self._compute_stroke_width() / 2. / distance_to_center
    }
    fn _compute_inner_circle_radius(nb_sections: i8, nose_size: f64) -> f64 {
        (nb_sections as f64 * (nose_size / 2. + 20.)) / (2. * PI)
    }
    fn _compute_stroke_width(&self) -> f64 {
        // TODO
        20.
    }
    fn _compute_nose_angular_size(&self, distance_to_center: f64) -> f64 {
        self._nose_size / 2. / distance_to_center
    }
    fn _compute_size_from_angular(angle_delta: f64, distance_to_center: f64) -> f64 {
        angle_delta * distance_to_center
    }
    fn _compute_nosed_angle(&self, section: i8, clockwise: bool) -> f64 {
        let distance_to_center = self._inner_circle_radius
            + self._sections_height[section as usize] as f64 * self._nose_size;
        let angular_len_nose = (self._nose_size / 2.) / distance_to_center;
        let section_angle =
            Self::_compute_angle_from_section(section, self._nb_sections as i32 * 2);

        if clockwise {
            section_angle - angular_len_nose
        } else {
            section_angle + angular_len_nose
        }
    }

    // Other helper methods
    fn _generate_section_list(&self, section_1: i8, section_2: i8, clockwise: bool) -> Vec<i8> {
        let mut section_list: Vec<i8> = Vec::new();
        if clockwise {
            if section_1 > section_2 {
                section_list = [
                    (section_1..self._nb_sections * 2).collect::<Vec<i8>>(),
                    (0..section_2 + 1).collect::<Vec<i8>>(),
                ]
                .concat();
            } else if section_1 < section_2 {
                section_list = (section_1..section_2 + 1).collect::<Vec<i8>>();
            } else {
                section_list = [
                    (section_1..self._nb_sections * 2).collect::<Vec<i8>>(),
                    (0..section_2 + 1).collect::<Vec<i8>>(),
                ]
                .concat();
            }
        } else {
            if section_1 < section_2 {
                section_list = [
                    (0..section_1 + 1).rev().collect::<Vec<i8>>(),
                    (section_2..self._nb_sections * 2)
                        .rev()
                        .collect::<Vec<i8>>(),
                ]
                .concat();
            } else if section_1 > section_2 {
                section_list = (section_2..section_1 + 1).rev().collect::<Vec<i8>>();
            } else {
                section_list = [
                    (0..section_1 + 1).rev().collect::<Vec<i8>>(),
                    (section_2..self._nb_sections * 2)
                        .rev()
                        .collect::<Vec<i8>>(),
                ]
                .concat();
            }
        }
        section_list
    }
    fn _change_section(&self, current_section: i8, delta: i8, clockwise: bool) -> i8 {
        let delta: i8 = if !clockwise { -delta } else { delta };
        if current_section + delta < 0 {
            self._sections_height.len() as i8 - 1
        } else if current_section + delta >= self._sections_height.len() as i8 {
            0
        } else {
            current_section + delta
        }
    }

    // Post processing methods
    fn _fill_tight_gaps(&mut self) {
        let mut i: usize = 0;

        while i < self._sections_height.len() {
            let index_0 = i;
            let index_1 = (i + 1) % self._sections_height.len();
            let index_2 = (i + 2) % self._sections_height.len();

            let height_0 = self._sections_height[index_0];
            let height_1 = self._sections_height[index_1];
            let height_2 = self._sections_height[index_2];

            if height_1 < height_0 && height_1 < height_2 {
                self._sections_height[index_1] = min(height_0, height_2);
            }

            i += 1;
        }
    }

    // Debugging methods
    fn _generate_rays(&self) -> Vec<Box<dyn Element>> {
        let mut result: Vec<Box<dyn Element>> = Vec::new();
        for i in 0..self._nb_sections * 2 {
            let angle = i as f64 * (2. * PI / (self._nb_sections as f64 * 2.));
            // let shifted_angle = (0.5 + i as f64) * (2. * PI / (self._nb_sections as f64 * 2.));
            let point_2 = (
                angle.cos() * (self._max_radius - 25.),
                angle.sin() * (self._max_radius - 25.),
            );
            let height_pos = (
                angle.cos()
                    * (self._sections_height[i as usize] as f64 * self._nose_size
                        + self._inner_circle_radius),
                angle.sin()
                    * (self._sections_height[i as usize] as f64 * self._nose_size
                        + self._inner_circle_radius),
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
            result.push(Box::new(
                Line::new()
                    .set_point_1((0.0, 0.0))
                    .set_point_2(height_pos)
                    .set_stroke_width(Size::from_length(10.))
                    .set_stroke(Paint::from_color(Color::from_name(ColorName::Aqua))),
            ));
        }
        result
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
