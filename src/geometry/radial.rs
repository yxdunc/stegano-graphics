use std::cmp::Ordering;
use std::f64::consts::PI;

static TWO_PI: f64 = PI * 2.;

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

pub fn add_circular_angles(angle_1: f64, angle_2: f64, is_clockwise: bool) -> f64 {
    // normalize circular direction
    let diff = match is_clockwise {
        true => (angle_1 + angle_2) % TWO_PI,
        false => (angle_1 - angle_2) % TWO_PI,
    };

    // map negative results onto circular number line (-1 => (2PI - 1))
    (TWO_PI + diff) % TWO_PI
}

pub fn subtract_circular_angles(angle_1: f64, angle_2: f64, is_clockwise: bool) -> f64 {
    add_circular_angles(angle_1, angle_2 * -1.0, is_clockwise)
}

pub fn is_between_circular_angles(
    angle_1: f64,
    angle_x: f64,
    angle_2: f64,
    is_clockwise: bool,
) -> bool {
    let delta_1_2 = subtract_circular_angles(angle_2, angle_1, is_clockwise);
    let delta_x_2 = subtract_circular_angles(angle_2, angle_x, is_clockwise);

    delta_1_2 > delta_x_2
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_add_circular_angles_clockwise() {
        let is_clockwise = true;

        assert_eq!(PI, add_circular_angles(0., PI, is_clockwise));
        assert_eq!(0., add_circular_angles(PI, PI, is_clockwise));
        assert_eq!(PI, add_circular_angles(0., -PI, is_clockwise));
        assert_eq!(1., add_circular_angles(TWO_PI, 1., is_clockwise));
    }
    #[test]
    fn should_add_circular_angles_anti_clockwise() {
        let is_clockwise = false;

        assert_eq!(PI, add_circular_angles(0., PI, is_clockwise));
        assert_eq!(0., add_circular_angles(PI, PI, is_clockwise));
        assert_eq!(PI, add_circular_angles(0., -PI, is_clockwise));
        assert_eq!(TWO_PI - 1., add_circular_angles(TWO_PI, 1., is_clockwise));
    }
    #[test]
    fn should_subtract_circular_angles_clockwise() {
        let is_clockwise = true;

        assert_eq!(PI, subtract_circular_angles(0., PI, is_clockwise));
        assert_eq!(0., subtract_circular_angles(PI, PI, is_clockwise));
        assert_eq!(PI, subtract_circular_angles(0., -PI, is_clockwise));
        assert_eq!(
            TWO_PI - 1.,
            subtract_circular_angles(TWO_PI, 1., is_clockwise)
        );
    }
    #[test]
    fn should_subtract_circular_angles_anti_clockwise() {
        let is_clockwise = false;

        assert_eq!(PI, subtract_circular_angles(0., PI, is_clockwise));
        assert_eq!(0., subtract_circular_angles(PI, PI, is_clockwise));
        assert_eq!(PI, subtract_circular_angles(0., -PI, is_clockwise));
        assert_eq!(1., subtract_circular_angles(TWO_PI, 1., is_clockwise));
    }
    #[test]
    fn should_find_in_between_angle() {
        let is_clockwise = true;

        assert_eq!(
            true,
            is_between_circular_angles(0., PI / 2., PI, is_clockwise)
        );
        assert_eq!(
            false,
            is_between_circular_angles(0., PI, PI / 2., is_clockwise)
        );
        assert_eq!(
            true,
            is_between_circular_angles(-PI / 2., 0., PI / 2., is_clockwise)
        );
    }
}
