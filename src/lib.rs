pub mod encoder;
pub mod geometry;

use std::f64::consts::{PI, TAU};
use wasm_bindgen::prelude::*;

use encoder::simple_latin_symbols;

const SUPPORTED_INPUT_CHARS: &str = "abcdefghijklmnopqrstuvwxyz ";

const DEFAULT_SECTIONS: usize = 28;
const DEFAULT_STEP: f64 = 34.0;
const DEFAULT_INNER_RADIUS: f64 = 92.0;
const DEFAULT_STROKE_WIDTH: f64 = 13.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StegKind {
    Spiral,
    Fingerprint,
}

impl StegKind {
    fn from_name(name: &str) -> Self {
        match name.trim().to_lowercase().as_str() {
            "fingerprint" | "maze" | "01" => Self::Fingerprint,
            _ => Self::Spiral,
        }
    }

    fn as_label(self) -> &'static str {
        match self {
            Self::Spiral => "spiral",
            Self::Fingerprint => "fingerprint",
        }
    }
}

#[derive(Debug, Clone)]
pub struct StegOptions {
    pub foreground: String,
    pub background: String,
    pub kind: StegKind,
    pub width: u32,
    pub height: u32,
    pub margin: f64,
}

impl Default for StegOptions {
    fn default() -> Self {
        Self {
            foreground: "#f5c266".to_string(),
            background: "transparent".to_string(),
            kind: StegKind::Fingerprint,
            width: 1600,
            height: 1600,
            margin: 120.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct StegSvg {
    pub svg: String,
    pub encoded: Vec<i8>,
}

#[wasm_bindgen]
pub fn generate_steg_svg(message: &str, foreground: &str, background: &str, kind: &str) -> String {
    let mut options = StegOptions::default();
    options.foreground = foreground.to_string();
    options.background = background.to_string();
    options.kind = StegKind::from_name(kind);
    generate_svg(message, &options).svg
}

pub fn generate_svg(message: &str, options: &StegOptions) -> StegSvg {
    match options.kind {
        StegKind::Spiral => generate_spiral_svg(message, options),
        StegKind::Fingerprint => generate_fingerprint_svg(message, options),
    }
}

pub fn generate_spiral_svg(message: &str, options: &StegOptions) -> StegSvg {
    let normalized = normalize_message(message);
    let encoded = simple_latin_symbols::encode(&normalized);
    let path = spiral_path(&encoded);
    let radius = DEFAULT_INNER_RADIUS + DEFAULT_STEP * encoded.len().max(1) as f64;
    build_svg(&normalized, &encoded, &path, radius, options)
}

pub fn generate_fingerprint_svg(message: &str, options: &StegOptions) -> StegSvg {
    let normalized = normalize_message(message);
    let encoded = simple_latin_symbols::encode(&normalized);
    let mut renderer = FingerprintRenderer::new();
    let path = renderer.render(&encoded);
    build_svg(
        &normalized,
        &encoded,
        &path,
        renderer.max_radius().max(DEFAULT_INNER_RADIUS),
        options,
    )
}

fn build_svg(
    normalized: &str,
    encoded: &[i8],
    path: &str,
    radius: f64,
    options: &StegOptions,
) -> StegSvg {
    let half = radius + options.margin + DEFAULT_STROKE_WIDTH;
    let view_box = format!(
        "{:.3} {:.3} {:.3} {:.3}",
        -half,
        -half,
        half * 2.0,
        half * 2.0
    );

    let background = match options.background.trim() {
        "" | "none" | "transparent" => String::new(),
        background => format!(
            r#"<rect x="{:.3}" y="{:.3}" width="{:.3}" height="{:.3}" rx="{:.3}" ry="{:.3}" fill="{}"/>"#,
            -half,
            -half,
            half * 2.0,
            half * 2.0,
            (half * 0.055).min(72.0),
            (half * 0.055).min(72.0),
            escape_attr(background),
        ),
    };

    let svg = format!(
        r#"<svg xmlns="http://www.w3.org/2000/svg" viewBox="{}" width="{}" height="{}" role="img" aria-label="generated {} steg"><title>{}</title>{}<path d="{}" fill="none" stroke="{}" stroke-width="{:.3}" stroke-linecap="round" stroke-linejoin="round"/></svg>"#,
        view_box,
        options.width,
        options.height,
        options.kind.as_label(),
        escape_text(&normalized),
        background,
        path,
        escape_attr(&options.foreground),
        DEFAULT_STROKE_WIDTH
    );

    StegSvg {
        svg,
        encoded: encoded.to_vec(),
    }
}

fn normalize_message(message: &str) -> String {
    let lowered = message.trim().to_lowercase();
    let filtered: String = lowered
        .chars()
        .filter(|character| SUPPORTED_INPUT_CHARS.contains(*character))
        .collect();

    if filtered.is_empty() {
        "the hidden line".to_string()
    } else {
        filtered
    }
}

fn spiral_path(encoded: &[i8]) -> String {
    let section_angle = TAU / DEFAULT_SECTIONS as f64;
    let mut commands = Vec::with_capacity(encoded.len() * 2 + 1);
    let mut angle: f64 = 0.0;
    let mut radius = DEFAULT_INNER_RADIUS;
    let mut clockwise = true;

    commands.push(format!(
        "M {:.3} {:.3}",
        angle.cos() * radius,
        angle.sin() * radius
    ));

    for value in encoded {
        let previous_angle = angle;
        angle = (*value as f64 % DEFAULT_SECTIONS as f64) * section_angle;

        if circular_delta(previous_angle, angle, clockwise) > section_angle / 2.0 {
            commands.push(arc_command(
                radius,
                angle,
                radius,
                large_arc(previous_angle, angle, clockwise),
                clockwise,
            ));
        }

        radius += DEFAULT_STEP;
        commands.push(arc_command(
            DEFAULT_STEP / 2.0,
            angle,
            radius,
            false,
            !clockwise,
        ));
        clockwise = !clockwise;
    }

    commands.join(" ")
}

#[derive(Clone, Debug)]
enum PathCommand {
    Arc {
        radius: f64,
        large_arc: bool,
        sweep: bool,
        point: (f64, f64),
    },
    LineTo {
        point: (f64, f64),
    },
}

impl PathCommand {
    fn render(&self) -> String {
        match self {
            PathCommand::Arc {
                radius,
                large_arc,
                sweep,
                point,
            } => format!(
                "A {:.3} {:.3} 0 {} {} {:.3} {:.3}",
                radius,
                radius,
                if *large_arc { 1 } else { 0 },
                if *sweep { 1 } else { 0 },
                point.0,
                point.1
            ),
            PathCommand::LineTo { point } => format!("L {:.3} {:.3}", point.0, point.1),
        }
    }
}

struct FingerprintRenderer {
    nb_sections: i8,
    sections_height: Vec<i32>,
    inner_circle_radius: f64,
    nose_size: f64,
    stroke_width: f64,
}

impl FingerprintRenderer {
    fn new() -> Self {
        let nb_sections = 28;
        let nose_size = 50.0;

        Self {
            nb_sections,
            sections_height: vec![0; nb_sections as usize * 2],
            inner_circle_radius: Self::compute_inner_circle_radius(nb_sections, nose_size),
            nose_size,
            stroke_width: 20.0,
        }
    }

    fn render(&mut self, encoded: &[i8]) -> String {
        let mut current_section: i8 = 0;
        let mut clockwise = true;
        let mut commands = Vec::new();

        if !encoded.is_empty() {
            let starting_section = 0.5;
            let angle = starting_section * (2.0 * PI / self.nb_sections as f64);
            commands.push(format!(
                "M {:.3} {:.3}",
                angle.cos() * self.inner_circle_radius,
                angle.sin() * self.inner_circle_radius
            ));
        }

        for current_char in encoded {
            let previous_section = current_section;
            current_section = *current_char;
            commands.extend(
                self.new_compressed_arc(previous_section, current_section, clockwise)
                    .iter()
                    .map(PathCommand::render),
            );
            clockwise = !clockwise;
        }

        commands.join(" ")
    }

    fn max_radius(&self) -> f64 {
        let max_layer = self
            .sections_height
            .iter()
            .copied()
            .max()
            .unwrap_or(0)
            .max(0) as f64;
        self.inner_circle_radius + max_layer * self.nose_size + self.stroke_width
    }

    fn new_arc(
        &self,
        point_angle: f64,
        point_radius: f64,
        radius: f64,
        sweep: bool,
        large_arc: bool,
    ) -> PathCommand {
        PathCommand::Arc {
            radius,
            large_arc,
            sweep,
            point: (
                point_radius * point_angle.cos(),
                point_radius * point_angle.sin(),
            ),
        }
    }

    fn new_arc_safe(
        &self,
        angle: f64,
        radius: f64,
        arc_radius: f64,
        sweep: bool,
        large_arc: bool,
    ) -> Vec<PathCommand> {
        vec![self.new_arc(angle, radius, arc_radius, sweep, large_arc)]
    }

    fn new_downward_nose(&self, section_0: i8, section_1: i8, clockwise: bool) -> Vec<PathCommand> {
        let split_arc = self.sections_height[section_0 as usize] > 4;
        let mut downward_nose = Vec::new();
        let angle_section_1 =
            Self::compute_angle_from_section(section_1, self.nb_sections as i32 * 2);
        let section_angle_delta = Self::compute_angle_from_section(1, self.nb_sections as i32 * 2);
        let mut nose_radius = Self::compute_size_from_angular(
            section_angle_delta / 2.0,
            self.compute_section_radius(section_0),
        );
        let distance_to_center = self.compute_section_radius(section_0);
        let arc_2_end_point = (
            (distance_to_center + self.nose_size / 2.0) * angle_section_1.cos(),
            (distance_to_center + self.nose_size / 2.0) * angle_section_1.sin(),
        );

        if split_arc {
            nose_radius = self.nose_size / 2.0;
            let nose_angular_size = self.compute_nose_angular_size(distance_to_center)
                * (-2.0 * (!clockwise) as i8 as f64 + 1.0);
            let angle_section_0 =
                Self::compute_angle_from_section(section_0, self.nb_sections as i32 * 2)
                    + nose_angular_size;
            let arc_1_end_point = (
                distance_to_center * angle_section_0.cos(),
                distance_to_center * angle_section_0.sin(),
            );
            let angle_arc_2_start = angle_section_1 - nose_angular_size;
            let arc_2_start_point = (
                distance_to_center * angle_arc_2_start.cos(),
                distance_to_center * angle_arc_2_start.sin(),
            );
            downward_nose.push(PathCommand::Arc {
                radius: nose_radius,
                large_arc: false,
                sweep: !clockwise,
                point: arc_1_end_point,
            });
            downward_nose.push(PathCommand::LineTo {
                point: arc_2_start_point,
            });
        }

        downward_nose.push(PathCommand::Arc {
            radius: nose_radius,
            large_arc: false,
            sweep: !clockwise,
            point: arc_2_end_point,
        });

        downward_nose
    }

    fn new_compressed_arc(
        &mut self,
        section_start: i8,
        section_end: i8,
        clockwise: bool,
    ) -> Vec<PathCommand> {
        let sections = self.generate_section_list(section_start * 2, section_end * 2, clockwise);
        let mut compressed_arc = Vec::new();
        let mut height_increment_to_apply_after_arc = vec![0; self.nb_sections as usize * 2];
        let mut touchy_nose = false;
        let mut i = 0;

        while i < sections.len() {
            let section_0 = sections[i];
            let section_1 = self.change_section(section_0, 1, clockwise);
            let section_2 = self.change_section(section_1, 1, clockwise);
            let section_3 = self.change_section(section_2, 1, clockwise);
            let section_minus_1 = self.change_section(section_0, -1, clockwise);
            let section_minus_2 = self.change_section(section_minus_1, -1, clockwise);
            let section_minus_3 = self.change_section(section_minus_2, -1, clockwise);
            let radius = self.compute_section_radius(section_0);
            let mut angle_2 =
                Self::compute_angle_from_section(section_1, self.nb_sections as i32 * 2);

            if i == sections.len() - 2 {
                angle_2 = self.compute_nosed_angle(section_1, clockwise);
            }

            let end_point = (radius * angle_2.cos(), radius * angle_2.sin());

            if self.sections_height[section_0 as usize] != self.sections_height[section_1 as usize]
                && i < sections.len() - 1
            {
                if self.sections_height[section_minus_2 as usize]
                    > self.sections_height[section_minus_1 as usize]
                    && self.sections_height[section_minus_1 as usize] - 1
                        == self.sections_height[section_0 as usize]
                    && self.sections_height[section_0 as usize]
                        < self.sections_height[section_1 as usize]
                {
                    compressed_arc.pop();
                    compressed_arc.pop();
                    self.sections_height[section_minus_1 as usize] -= 1;
                    compressed_arc.append(&mut self.new_downward_nose(
                        section_minus_1,
                        section_0,
                        clockwise,
                    ));
                    let mut height_transition =
                        self.new_height_transition(section_0, section_1, clockwise);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    compressed_arc.append(&mut height_transition);
                    compressed_arc.pop();
                    let tmp_angle_2 = self.compute_nosed_angle(section_1, clockwise);
                    let tmp_radius = self.compute_section_radius(section_1);
                    compressed_arc.push(PathCommand::Arc {
                        radius: self.compute_section_radius(section_1),
                        large_arc: false,
                        sweep: clockwise,
                        point: (
                            tmp_radius * tmp_angle_2.cos(),
                            tmp_radius * tmp_angle_2.sin(),
                        ),
                    });
                    self.sections_height[section_minus_1 as usize] += 1;
                } else if i == sections.len() - 3
                    && self.sections_height[section_0 as usize]
                        > self.sections_height[section_1 as usize]
                {
                    i += 1;
                    touchy_nose = true;
                    compressed_arc
                        .append(&mut self.new_height_transition(section_0, section_1, clockwise));
                    compressed_arc.pop();
                    compressed_arc
                        .append(&mut self.new_downward_nose(section_1, section_2, clockwise));
                    self.sections_height[section_0 as usize] += 1;
                    self.sections_height[section_1 as usize] =
                        self.sections_height[section_0 as usize];
                    let mut height_transition =
                        self.new_height_transition(section_2, section_1, !clockwise);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    height_transition.pop();
                    compressed_arc.append(&mut height_transition);
                    self.sections_height[section_2 as usize] =
                        self.sections_height[section_0 as usize];
                } else if i < sections.len() - 2
                    && self.sections_height[section_0 as usize]
                        > self.sections_height[section_1 as usize]
                    && self.sections_height[section_0 as usize]
                        <= self.sections_height[section_2 as usize]
                {
                    compressed_arc.push(PathCommand::Arc {
                        radius,
                        large_arc: false,
                        sweep: clockwise,
                        point: end_point,
                    });
                    self.sections_height[section_1 as usize] =
                        self.sections_height[section_0 as usize];
                } else if i == sections.len() - 2
                    && self.sections_height[section_0 as usize]
                        > self.sections_height[section_1 as usize]
                {
                    i += 1;
                    touchy_nose = true;
                    compressed_arc
                        .append(&mut self.new_height_transition(section_0, section_1, clockwise));
                    compressed_arc.pop();
                    compressed_arc
                        .append(&mut self.new_downward_nose(section_1, section_2, clockwise));
                    self.sections_height[section_0 as usize] += 1;
                    self.sections_height[section_1 as usize] =
                        self.sections_height[section_0 as usize];
                    let mut height_transition =
                        self.new_height_transition(section_2, section_1, !clockwise);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    height_transition.pop();
                    compressed_arc.append(&mut height_transition);
                    self.sections_height[section_2 as usize] =
                        self.sections_height[section_0 as usize] + 1;
                } else if i == sections.len() - 4
                    && self.sections_height[section_0 as usize]
                        > self.sections_height[section_1 as usize]
                    && self.sections_height[section_1 as usize] < 4
                {
                    i += 2;
                    touchy_nose = true;
                    let section_angle_delta =
                        Self::compute_angle_from_section(1, self.nb_sections as i32 * 2);
                    let radius_0 = self.compute_section_radius(section_1);
                    let tmp_radius = Self::compute_size_from_angular(
                        section_angle_delta / 2.0,
                        self.compute_section_radius(section_1),
                    );
                    self.sections_height[section_1 as usize] =
                        self.sections_height[section_0 as usize];
                    compressed_arc
                        .append(&mut self.new_height_transition(section_1, section_2, clockwise));
                    let angle_3 =
                        Self::compute_angle_from_section(section_3, self.nb_sections as i32 * 2);
                    compressed_arc.pop();
                    compressed_arc.push(PathCommand::Arc {
                        radius: tmp_radius,
                        large_arc: false,
                        sweep: !clockwise,
                        point: (
                            (radius_0 + self.nose_size / 2.0) * angle_3.cos(),
                            (radius_0 + self.nose_size / 2.0) * angle_3.sin(),
                        ),
                    });
                    self.sections_height[section_0 as usize] += 1;
                    self.sections_height[section_2 as usize] =
                        self.sections_height[section_0 as usize];
                    let mut height_transition =
                        self.new_height_transition(section_3, section_2, !clockwise);
                    height_transition.remove(0);
                    height_transition.remove(0);
                    height_transition.pop();
                    compressed_arc.append(&mut height_transition);
                    self.sections_height[section_3 as usize] =
                        self.sections_height[section_0 as usize];
                    self.sections_height[section_2 as usize] =
                        self.sections_height[section_0 as usize];
                    self.sections_height[section_1 as usize] =
                        self.sections_height[section_0 as usize];
                } else if i == sections.len() - 2
                    && self.sections_height[section_0 as usize]
                        < self.sections_height[section_1 as usize]
                {
                    touchy_nose = true;
                    compressed_arc.pop();
                    compressed_arc.pop();
                    let orig_section_height_minus_2 =
                        self.sections_height[section_minus_2 as usize];
                    let orig_section_height_minus_3 =
                        self.sections_height[section_minus_3 as usize];
                    self.sections_height[section_minus_1 as usize] -= 1;
                    self.sections_height[section_0 as usize] =
                        self.sections_height[section_1 as usize];
                    let mut height_transition =
                        self.new_height_transition(section_minus_1, section_0, clockwise);
                    if orig_section_height_minus_3 - 1 > orig_section_height_minus_2 {
                        compressed_arc.pop();
                        height_transition.remove(0);
                        height_transition.remove(0);
                        let section_angle_delta =
                            Self::compute_angle_from_section(1, self.nb_sections as i32 * 2);
                        let tmp_radius = Self::compute_size_from_angular(
                            section_angle_delta / 2.0,
                            self.compute_section_radius(section_minus_1),
                        );
                        let tmp_angle = Self::compute_angle_from_section(
                            section_minus_1,
                            self.nb_sections as i32 * 2,
                        );
                        compressed_arc.push(PathCommand::Arc {
                            radius: tmp_radius,
                            large_arc: false,
                            sweep: !clockwise,
                            point: (
                                (radius + self.nose_size / 2.0) * tmp_angle.cos(),
                                (radius + self.nose_size / 2.0) * tmp_angle.sin(),
                            ),
                        });
                    }
                    compressed_arc.append(&mut height_transition);
                    self.sections_height[section_minus_1 as usize] =
                        self.sections_height[section_1 as usize] + 1;
                    self.sections_height[section_0 as usize] += 1;
                    let tmp_angle_2 = self.compute_nosed_angle(section_1, clockwise);
                    let tmp_radius = self.compute_section_radius(section_0);
                    compressed_arc.push(PathCommand::Arc {
                        radius: self.compute_section_radius(section_0) - self.nose_size,
                        large_arc: false,
                        sweep: clockwise,
                        point: (
                            (tmp_radius - self.nose_size) * tmp_angle_2.cos(),
                            (tmp_radius - self.nose_size) * tmp_angle_2.sin(),
                        ),
                    });
                    self.sections_height[section_1 as usize] += 1;
                    compressed_arc.push(self.new_nose_compressed(section_end, clockwise, false));
                } else if i == sections.len() - 3
                    && self.sections_height[section_0 as usize]
                        < self.sections_height[section_1 as usize]
                {
                    compressed_arc.push(PathCommand::Arc {
                        radius,
                        large_arc: false,
                        sweep: clockwise,
                        point: end_point,
                    });
                    self.sections_height[section_1 as usize] =
                        self.sections_height[section_0 as usize];
                } else if i < sections.len() - 2
                    && self.sections_height[section_0 as usize]
                        > self.sections_height[section_1 as usize]
                    && self.sections_height[section_1 as usize]
                        == self.sections_height[section_2 as usize]
                    && self.sections_height[section_2 as usize]
                        < self.sections_height[section_3 as usize]
                {
                    compressed_arc
                        .append(&mut self.new_height_transition(section_0, section_1, clockwise));
                } else {
                    if self.sections_height[section_0 as usize]
                        < self.sections_height[section_1 as usize]
                    {
                        compressed_arc.pop();
                    }
                    compressed_arc
                        .append(&mut self.new_height_transition(section_0, section_1, clockwise));
                }

                if self.sections_height[section_0 as usize]
                    < self.sections_height[section_1 as usize]
                    && !touchy_nose
                {
                    let height_difference = (self.sections_height[section_1 as usize]
                        - self.sections_height[section_0 as usize])
                        .abs();
                    height_increment_to_apply_after_arc[section_0 as usize] = height_difference;
                } else if !touchy_nose {
                    let height_difference = (self.sections_height[section_1 as usize]
                        - self.sections_height[section_0 as usize])
                        .abs();
                    height_increment_to_apply_after_arc[section_1 as usize] = height_difference;
                }
            } else if i < sections.len() - 1 {
                compressed_arc
                    .append(&mut self.new_arc_safe(angle_2, radius, radius, clockwise, false));
            }

            if !touchy_nose {
                self.sections_height[section_0 as usize] += 1;
            }
            i += 1;
        }

        if !touchy_nose {
            compressed_arc.push(self.new_nose_compressed(section_end, clockwise, touchy_nose));
        }

        self.sections_height = self
            .sections_height
            .iter()
            .zip(height_increment_to_apply_after_arc.iter())
            .map(|(&a, &b)| a + b)
            .collect();
        self.fill_tight_gaps();

        compressed_arc
    }

    fn new_height_transition(
        &self,
        section_0: i8,
        section_1: i8,
        clockwise: bool,
    ) -> Vec<PathCommand> {
        let is_raising =
            self.sections_height[section_0 as usize] < self.sections_height[section_1 as usize];
        let sweep = is_raising ^ clockwise;
        let (height_change, angle_change) = if is_raising && clockwise {
            (0.5, 0.0)
        } else if !is_raising && !clockwise {
            (-0.5, 1.0)
        } else if is_raising && !clockwise {
            (0.5, 0.0)
        } else {
            (-0.5, -1.0)
        };

        let turn_1_start_radius = self.compute_section_radius(section_0);
        let section_0_f = section_0 as f64;
        let section_1_f = section_1 as f64;
        let angular_len_nose = if clockwise {
            self.compute_nose_angular_size(turn_1_start_radius) * -1.0
        } else {
            self.compute_nose_angular_size(turn_1_start_radius)
        };
        let turn_1_start_angle = (section_0_f - angle_change)
            * (2.0 * PI / (self.nb_sections * 2) as f64)
            + angular_len_nose;
        let turn_1_end_radius = self.inner_circle_radius
            + (self.sections_height[section_0 as usize] as f64 + height_change) * self.nose_size;
        let turn_1_end_angle =
            (section_0_f - angle_change) * (2.0 * PI / (self.nb_sections * 2) as f64);
        let turn_2_start_radius = self.inner_circle_radius
            + (self.sections_height[section_1 as usize] as f64 - height_change) * self.nose_size;
        let turn_2_start_point = (
            turn_2_start_radius * turn_1_end_angle.cos(),
            turn_2_start_radius * turn_1_end_angle.sin(),
        );
        let turn_2_end_radius = self.inner_circle_radius
            + self.sections_height[section_1 as usize] as f64 * self.nose_size;
        let angular_len_nose = if clockwise {
            self.nose_size / 2.0 / turn_2_end_radius * -1.0
        } else {
            self.nose_size / 2.0 / turn_2_end_radius
        };
        let turn_2_end_angle = turn_1_end_angle - angular_len_nose;
        let end_section_angle =
            (section_1_f + angle_change * 2.0) * (2.0 * PI / (self.nb_sections * 2) as f64);
        let mut result = Vec::new();
        result.append(&mut self.new_arc_safe(
            turn_1_start_angle,
            turn_1_start_radius,
            turn_1_start_radius,
            clockwise,
            false,
        ));
        result.append(&mut self.new_arc_safe(
            turn_1_end_angle,
            turn_1_end_radius,
            self.nose_size / 2.0,
            sweep,
            false,
        ));
        result.push(PathCommand::LineTo {
            point: turn_2_start_point,
        });
        result.append(&mut self.new_arc_safe(
            turn_2_end_angle,
            turn_2_end_radius,
            self.nose_size / 2.0,
            !sweep,
            false,
        ));
        if is_raising {
            result.append(&mut self.new_arc_safe(
                end_section_angle,
                turn_2_end_radius,
                turn_2_end_radius,
                clockwise,
                false,
            ));
        }
        result
    }

    fn new_nose(&self, angle: f64, radius: f64, clockwise: bool) -> PathCommand {
        PathCommand::Arc {
            radius: self.nose_size / 2.0,
            large_arc: false,
            sweep: !clockwise,
            point: (radius * angle.cos(), radius * angle.sin()),
        }
    }

    fn new_nose_compressed(&self, section: i8, clockwise: bool, is_touching: bool) -> PathCommand {
        let section = section * 2;
        let section_height = if is_touching {
            self.sections_height[section as usize] - 1
        } else {
            self.sections_height[section as usize]
        } as f64;
        self.new_nose(
            self.compute_nosed_angle(section, clockwise),
            self.inner_circle_radius + section_height * self.nose_size,
            clockwise,
        )
    }

    fn compute_angle_from_section(section: i8, nb_sections: i32) -> f64 {
        section as f64 * (2.0 * PI / nb_sections as f64)
    }

    fn compute_layer_radius(&self, layer: f64) -> f64 {
        self.inner_circle_radius + layer * self.nose_size
    }

    fn compute_section_radius(&self, section: i8) -> f64 {
        self.compute_layer_radius(self.sections_height[section as usize] as f64)
    }

    fn compute_inner_circle_radius(nb_sections: i8, nose_size: f64) -> f64 {
        (nb_sections as f64 * nose_size * 1.5) / (2.0 * PI)
    }

    fn compute_nose_angular_size(&self, distance_to_center: f64) -> f64 {
        self.nose_size / 2.0 / distance_to_center
    }

    fn compute_size_from_angular(angle_delta: f64, distance_to_center: f64) -> f64 {
        angle_delta * distance_to_center
    }

    fn compute_nosed_angle(&self, section: i8, clockwise: bool) -> f64 {
        let distance_to_center = self.inner_circle_radius
            + self.sections_height[section as usize] as f64 * self.nose_size;
        let angular_len_nose = (self.nose_size / 2.0) / distance_to_center;
        let section_angle = Self::compute_angle_from_section(section, self.nb_sections as i32 * 2);

        if clockwise {
            section_angle - angular_len_nose
        } else {
            section_angle + angular_len_nose
        }
    }

    fn generate_section_list(&self, section_1: i8, section_2: i8, clockwise: bool) -> Vec<i8> {
        if clockwise {
            if section_1 > section_2 || section_1 == section_2 {
                [
                    (section_1..self.nb_sections * 2).collect::<Vec<i8>>(),
                    (0..section_2 + 1).collect::<Vec<i8>>(),
                ]
                .concat()
            } else {
                (section_1..section_2 + 1).collect::<Vec<i8>>()
            }
        } else if section_1 < section_2 || section_1 == section_2 {
            [
                (0..section_1 + 1).rev().collect::<Vec<i8>>(),
                (section_2..self.nb_sections * 2).rev().collect::<Vec<i8>>(),
            ]
            .concat()
        } else {
            (section_2..section_1 + 1).rev().collect::<Vec<i8>>()
        }
    }

    fn change_section(&self, current_section: i8, delta: i8, clockwise: bool) -> i8 {
        let delta = if !clockwise { -delta } else { delta };
        if current_section + delta < 0 {
            self.sections_height.len() as i8 - 1
        } else if current_section + delta >= self.sections_height.len() as i8 {
            0
        } else {
            current_section + delta
        }
    }

    fn fill_tight_gaps(&mut self) {
        for i in 0..self.sections_height.len() {
            let index_0 = i;
            let index_1 = (i + 1) % self.sections_height.len();
            let index_2 = (i + 2) % self.sections_height.len();
            let height_0 = self.sections_height[index_0];
            let height_1 = self.sections_height[index_1];
            let height_2 = self.sections_height[index_2];

            if height_1 < height_0 && height_1 < height_2 {
                self.sections_height[index_1] = height_0.min(height_2);
            }
        }
    }
}

fn arc_command(
    arc_radius: f64,
    end_angle: f64,
    end_radius: f64,
    large_arc: bool,
    sweep: bool,
) -> String {
    let end_x = end_angle.cos() * end_radius;
    let end_y = end_angle.sin() * end_radius;
    format!(
        "A {:.3} {:.3} 0 {} {} {:.3} {:.3}",
        arc_radius,
        arc_radius,
        if large_arc { 1 } else { 0 },
        if sweep { 1 } else { 0 },
        end_x,
        end_y
    )
}

fn circular_delta(from: f64, to: f64, clockwise: bool) -> f64 {
    if clockwise {
        if from > to {
            to + (TAU - from)
        } else {
            to - from
        }
    } else if from < to {
        from + (TAU - to)
    } else {
        from - to
    }
}

fn large_arc(from: f64, to: f64, clockwise: bool) -> bool {
    circular_delta(from, to, clockwise) > PI
}

fn escape_attr(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('"', "&quot;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn escape_text(value: &str) -> String {
    value
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_svg_path_for_message() {
        let generated = generate_spiral_svg("hello world", &StegOptions::default());

        assert!(generated.svg.starts_with("<svg "));
        assert!(generated.svg.contains("<path d=\"M "));
        assert_eq!(generated.encoded.len(), 11);
    }

    #[test]
    fn normalizes_empty_messages() {
        let generated = generate_spiral_svg("", &StegOptions::default());

        assert!(generated.svg.contains("the hidden line"));
    }

    #[test]
    fn ignores_unsupported_characters() {
        let generated = generate_spiral_svg("héllo 123, 💀 world!", &StegOptions::default());

        assert!(generated.svg.contains("hllo"));
        assert!(generated.svg.contains("world"));
        assert!(!generated.svg.contains("123"));
        assert!(!generated.svg.contains("é"));
        assert!(!generated.svg.contains(","));
        assert!(!generated.svg.contains("💀"));
    }

    #[test]
    fn supports_fingerprint_kind() {
        let mut options = StegOptions::default();
        options.kind = StegKind::Fingerprint;
        let generated = generate_svg("hello world", &options);

        assert!(generated.svg.contains("generated fingerprint steg"));
        assert!(generated.svg.contains("<path d=\"M "));
    }

    #[test]
    fn rounds_optional_background() {
        let mut options = StegOptions::default();
        options.background = "#050708".to_string();
        let generated = generate_svg("hello world", &options);

        assert!(generated.svg.contains("<rect "));
        assert!(generated.svg.contains(" rx=\""));
        assert!(generated.svg.contains(" ry=\""));
    }
}
