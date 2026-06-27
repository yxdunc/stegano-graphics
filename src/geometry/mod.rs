pub mod radial;
pub mod transforms;

use std::fmt;

#[derive(Debug, PartialEq)]
pub enum GeometryError {
    ScalingConstraintMinStroke {
        min_stroke: f32,
        current_stroke: f32,
    },
    MarginOutOfBound {
        margin: f32,
        height: f32,
        width: f32,
    },
    Unknown,
}

impl fmt::Display for GeometryError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GeometryError::ScalingConstraintMinStroke {
                min_stroke,
                current_stroke,
            } => write!(
                f,
                "[min_stroke_error] Minimum acceptable stroke width is {:?} but stroke would be {:?}",
                min_stroke, current_stroke
            ),
            GeometryError::MarginOutOfBound {
                margin,
                height,
                width,
            } => write!(
                f,
                "[margin_error] Margin ({:?}) is bigger than width or height (h: {:?}, w: {:?})",
                margin, height, width
            ),
            GeometryError::Unknown => write!(f, "unknown geometry error"),
        }
    }
}

impl std::error::Error for GeometryError {}

pub struct Dimensions2D {
    pub width: f64,
    pub height: f64,
}
