pub mod steg_00_spiral;
pub mod steg_01_fingerprint;

use std::fmt::Display;
use tiny_skia::{Pixmap, PixmapMut};
use usvg;
use usvg::ShapeRendering;

// render
// render_debug
// get bounding sides
// get stroke size

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(message: &str) -> Self {
        Error {
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> Result<(), ::std::fmt::Error> {
        f.write_str(self.message.as_str())
    }
}

pub trait Steg {
    fn set_text(self, text: &str) -> Self;
    fn set_render_debug(self, should_render_debug: bool) -> Self;

    fn set_stroke_constraint(self, min: f64, max: f64) -> Self
    where
        Self: Sized,
    {
        unimplemented!();
    }
    fn get_svg(&self) -> Result<svg_composer::Document, Error>;
    // fn render_png(&self, width: u32, height: u32) -> Result<Pixmap, Error> {
    //     let mut svg_document = self.get_svg()?;
    //     let aspect_ratio = width / height;
    //     svg_document.view_box = Some([0., 0., 0., 0.]);
    //
    //     let svg_str = svg_document.render();
    //     let rendering_options: usvg::Options = usvg::Options {
    //         resources_dir: None,
    //         dpi: 96.0,
    //         font_family: "Times New Roman".to_string(),
    //         font_size: 12.0,
    //         languages: vec!["en".to_string()],
    //         shape_rendering: ShapeRendering::CrispEdges,
    //         text_rendering: Default::default(),
    //         image_rendering: Default::default(),
    //         keep_named_groups: false,
    //         fontdb: fontdb::new(),
    //     };
    //     let mut result_pixmap: Pixmap =
    //         Pixmap::new(width, height).ok_or(Error::new("Couldn't allocate pixmap..."))?;
    //     let svg_tree = usvg::Tree::from_str(svg_str.as_str(), &rendering_options)?;
    //
    //     match resvg::render(&svg_tree, FitTo {}, result_pixmap.as_mut()) {
    //         None => Err(Error::new("Couldn't render svg")),
    //         Some(_) => Ok(result_pixmap),
    //     }
    // }
}

fn scale_to_fit(
    width: f64,
    height: f64,
    min_stroke: f64,
    max_stroke: f64,
    margin: f64,
    shape_width: f64,
    shape_height: f64,
    shape_stroke_width: f64,
) -> Result<[f64; 4], Error> {
    // account for margin
    let margin_ratio;
    let margin_ratio_width = 1. / ((width - margin * 2.) / width);
    let margin_ratio_height = 1. / ((height - margin * 2.) / height);
    let height = height - margin * 2.;
    let width = width - margin * 2.;

    if height <= 0. || width <= 0. {
        return Err(Error::new("Margin bigger than width and/or height"));
    }

    // compute conversion ratio based on vertical or horizontal overflow
    let mut corrected_shape_width = shape_width;
    let mut corrected_shape_height = shape_height;
    let shape_aspect_ratio = shape_width / shape_height;
    let frame_aspect_ratio = width / height;
    let vertical_ratio = height / shape_height;
    let horizontal_ratio = width / shape_width;
    let shape_scalar;
    match shape_aspect_ratio < frame_aspect_ratio {
        true => {
            shape_scalar = vertical_ratio;
            margin_ratio = margin_ratio_height;
            corrected_shape_width = corrected_shape_height * frame_aspect_ratio;
        }
        false => {
            shape_scalar = horizontal_ratio;
            margin_ratio = margin_ratio_width;
            corrected_shape_height = corrected_shape_width / frame_aspect_ratio;
        }
    };

    // compute adjustment for stroke
    let vb_stroke = shape_stroke_width * shape_scalar;
    eprintln!("vb_stroke: {}, view_box_scalar {}", vb_stroke, shape_scalar);
    match vb_stroke {
        s if s < min_stroke => {
            return Err(Error::new("Can't fit shape while respecting min stroke"));
        }
        s if s > max_stroke => {
            let stroke_correction = vb_stroke / max_stroke;
            corrected_shape_width = shape_width * stroke_correction;
            corrected_shape_height = shape_height * stroke_correction;
        }
        _ => {}
    };

    // correct margin
    corrected_shape_width *= margin_ratio;
    corrected_shape_height *= margin_ratio;
    eprintln!(
        "margin_ratio_width: {}, margin_ratio_height {}",
        margin_ratio_width, margin_ratio_height
    );

    // compute view box
    let vb_min_x = -(corrected_shape_width / 2.);
    let vb_min_y = -(corrected_shape_height / 2.);
    let vb_width = corrected_shape_width;
    let vb_height = corrected_shape_height;

    Ok([vb_min_x, vb_min_y, vb_width, vb_height])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_preserve_aspect_ratio() {
        let width = 20.;
        let height = 10.;
        let view_box = scale_to_fit(width, height, 1., 5., 0., 100., 100., 20.).unwrap();
        assert_eq!(view_box[2] / view_box[3], width / height);

        let width = 10.;
        let height = 20.;
        let view_box = scale_to_fit(width, height, 1., 5., 0., 100., 100., 20.).unwrap();
        assert_eq!(view_box[2] / view_box[3], width / height);
    }

    #[test]
    fn should_scale_to_fit() {
        let view_box = scale_to_fit(10., 10., 1., 5., 0., 100., 100., 40.);
        let expected: Result<[f64; 4], Error> = Ok([-50.0, -50.0, 100.0, 100.0]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_scale_to_fit_and_respect_max_stroke() {
        let view_box = scale_to_fit(10., 10., 1., 2., 0., 100., 100., 40.);
        let expected: Result<[f64; 4], Error> = Ok([-100.0, -100.0, 200.0, 200.0]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_scale_to_fit_accounting_for_margin() {
        let view_box = scale_to_fit(10., 10., 1., 4., 2.5, 10., 10., 3.);
        let expected: Result<[f64; 4], Error> = Ok([-10., -10., 20.0, 20.0]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_scale_to_fit_vertical_1() {
        let view_box = scale_to_fit(10., 100., 0.001, 100., 0., 10., 5., 1.);
        let expected: Result<[f64; 4], Error> = Ok([-5., -50., 10., 100.]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_scale_to_fit_vertical_2() {
        let view_box = scale_to_fit(10., 100., 0.001, 100., 0., 1., 15., 1.);
        let expected: Result<[f64; 4], Error> = Ok([-0.75, -7.5, 1.5, 15.]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_scale_to_fit_horizontal_1() {
        let view_box = scale_to_fit(100., 10., 0.001, 100., 0., 10., 5., 1.);
        let expected: Result<[f64; 4], Error> = Ok([-25., -2.5, 50., 5.]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_scale_to_fit_horizontal_2() {
        let view_box = scale_to_fit(100., 10., 0.001, 100., 0., 5., 10., 1.);
        let expected: Result<[f64; 4], Error> = Ok([-50., -5., 100., 10.]);

        assert_eq!(view_box.unwrap(), expected.unwrap());
    }
    #[test]
    fn should_error_to_respect_min_stroke_and_margin() {
        let view_box = scale_to_fit(10., 10., 2., 4., 2.5, 10., 10., 3.);

        assert!(view_box.is_err());
    }
}
