pub fn compute_angle_from_section(section: i8, nb_sections: i32) -> f64 {
    let nb_sections: f64 = nb_sections as f64;
    section as f64 * (2. * PI / nb_sections)
}

pub fn compute_angular_size(length: f64, distance_to_center: f64) -> f64 {
    length / 2. / distance_to_center
}

pub fn compute_length_from_angular_size(angle_delta: f64, distance_to_center: f64) -> f64 {
    angle_delta * distance_to_center
}
