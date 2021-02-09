pub mod steg_00_spiral;
pub mod steg_01_fingerprint;

use tiny_skia::{Pixmap, PixmapMut};
use usvg;
use usvg::ShapeRendering;

// render
// render_debug
// get bounding sides
// get stroke size

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

pub trait Steg {
    fn set_text(mut self, text: &str) -> Self;
    fn set_render_debug(mut self, should_render_debug: bool) -> Self;
    fn set_width_constraint(mut self, min: f64, max: f64) -> Self
    where
        Self: Sized,
    {
        unimplemented!();
    }
    fn set_height_constraint(mut self, min: f64, max: f64) -> Self
    where
        Self: Sized,
    {
        unimplemented!();
    }
    fn set_stroke_constraint(mut self, min: f64, max: f64) -> Self
    where
        Self: Sized,
    {
        unimplemented!();
    }
    fn render_svg(&self) -> Result<String, Error>;
    fn render_png(&self, width: u32, height: u32) -> Result<Pixmap, Error> {
        let svg_str = self.render_svg()?;
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
            fontdb: fontdb::new(),
        };
        let mut result_pixmap: Pixmap =
            Pixmap::new(width, height).ok_or(Error::new("Couldn't allocate pixmap..."))?;
        let svg_tree = usvg::Tree::from_str(svg_str.as_str(), &rendering_options)?;

        match resvg::render(&svg_tree, FitTo {}, result_pixmap.as_mut()) {
            None => Err(Error::new("Couldn't render svg")),
            Some(_) => Ok(result_pixmap),
        }
    }
}
