pub mod radial;
pub mod transforms;
use std::fmt::Display;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeometryError {
    #[error("[min_stroke_error] Minimum acceptable stroke width is {min_stroke:?} but stroke would be {current_stroke:?}")]
    ScalingConstraintMinStroke {
        min_stroke: f32,
        current_stroke: f32,
    },
    #[error("[margin_error] Margin ({margin:?}) is bigger than width or height (h: {height:?}, w: {width:?})")]
    MarginOutOfBound {
        margin: f32,
        height: f32,
        width: f32,
    },
    #[error("unknown geometry error")]
    Unknown,
}

pub struct Dimensions2D {
    pub width: f64,
    pub height: f64,
}
