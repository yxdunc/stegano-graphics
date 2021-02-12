pub mod steg_00_spiral;
pub mod steg_01_fingerprint;

use crate::geometry::transforms::scale_to_fit;
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

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
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
    fn get_stroke_width(&self) -> f64;
    fn get_shape_width(&self) -> f64;
    fn get_shape_height(&self) -> f64;
    fn get_svg(&self) -> Result<svg_composer::Document, Box<dyn std::error::Error>>;
    fn render_png(
        &self,
        width: u32,
        height: u32,
        min_stroke: u32,
        max_stroke: u32,
        margin: u32,
    ) -> Result<Pixmap, Box<dyn std::error::Error>>
    where
        Self: Sized,
    {
        let mut svg_document = self.get_svg()?;
        let view_box = scale_to_fit(
            width as f32,
            height as f32,
            min_stroke as f32,
            max_stroke as f32,
            margin as f32,
            self.get_shape_width() as f32,
            self.get_shape_height() as f32,
            self.get_stroke_width() as f32,
        )?;

        svg_document.view_box = Some(view_box as [f32; 4]);

        let svg_str = svg_document.render();
        let svg_str = svg_str.as_str();
        let rendering_options: usvg::Options = usvg::Options {
            resources_dir: None,
            dpi: 96.0,
            font_family: "Times New Roman".to_string(),
            font_size: 12.0,
            languages: vec!["en".to_string()],
            shape_rendering: ShapeRendering::CrispEdges,
            text_rendering: Default::default(),
            image_rendering: Default::default(),
            keep_named_groups: false,
            fontdb: fontdb::Database::new(),
        };
        let mut result_pixmap: Pixmap =
            Pixmap::new(width, height).ok_or(Error::new("Couldn't allocate pixmap..."))?;
        let svg_tree = usvg::Tree::from_str(svg_str, &rendering_options)?;

        // height will be deduced from the scaled view_box
        match resvg::render(&svg_tree, usvg::FitTo::Width(width), result_pixmap.as_mut()) {
            None => Err(Box::new(Error::new("Couldn't render svg"))),
            Some(_) => Ok(result_pixmap),
        }
    }
}
