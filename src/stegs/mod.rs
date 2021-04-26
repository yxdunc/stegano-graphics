pub mod color_palette;
pub mod steg_00_spiral;
pub mod steg_01_fingerprint;

// internal
use crate::geometry::transforms::scale_to_fit;
use crate::geometry::{Dimensions2D, GeometryError};
use crate::stegs::color_palette::Palette;

// svg composer
use svg_composer::element::attributes::{Color, ColorName, Paint, Size};
use svg_composer::element::rect::Rectangle;
use svg_composer::element::Element;
use svg_composer::Document;

// std
use std::borrow::BorrowMut;
use std::fmt::Display;
use std::ops::Deref;

// error
use thiserror::Error;

// svg rendering
use tiny_skia::{Pixmap, PixmapMut};
use usvg;
use usvg::ShapeRendering;

#[derive(Error, Debug)]
pub enum StegError {
    #[error("Couldn't allocate pixmap")]
    PixmapAllocation,
    #[error("Error occurred during rendering of SVG to pixmap")]
    PixmapRendering,
    #[error("{source}")]
    GeometryError {
        #[from]
        source: GeometryError,
    },
    #[error("{source}")]
    USVGError {
        #[from]
        source: usvg::Error,
    },
    #[error("unknown steg error")]
    Unknown,
}

pub trait Steg {
    fn set_text(self, text: &str) -> Self
    where
        Self: Sized;
    fn set_color_palette(self, color_palette: Palette) -> Self
    where
        Self: Sized;
    fn set_render_debug(self, should_render_debug: bool) -> Self
    where
        Self: Sized;
    fn get_render_debug(&self) -> bool;
    fn get_stroke_width(&self) -> f64;
    fn get_shape_dimensions(&self) -> Dimensions2D;
    fn render(&mut self);
    fn get_svg(&self) -> &svg_composer::Document;
    fn get_pixmap(
        &self,
        width: u32,
        height: u32,
        min_stroke: u32,
        max_stroke: u32,
        margin: u32,
        antialiasing: bool,
    ) -> Result<Pixmap, StegError> {
        let mut svg_document: &Document = self.get_svg();
        let mut svg_document: Document = svg_document.clone();
        let shape_dimensions = self.get_shape_dimensions();
        let view_box = scale_to_fit(
            width as f32,
            height as f32,
            min_stroke as f32,
            max_stroke as f32,
            margin as f32,
            shape_dimensions.width as f32,
            shape_dimensions.height as f32,
            self.get_stroke_width() as f32,
        )
        .map_err(|err| StegError::GeometryError { source: err })?;

        if self.get_render_debug() {
            eprintln!("view box: {:?}", view_box);
            svg_document.add_element(Box::new(
                Rectangle::new()
                    .set_pos((view_box[0] as f64, view_box[1] as f64))
                    .set_size(
                        Size::from_length(view_box[2] as f64),
                        Size::from_length(view_box[3] as f64),
                    )
                    .set_stroke(Paint::from_color(Color::from_name(ColorName::Black)))
                    .set_fill(Paint::new_empty())
                    .set_stroke_width(Size::from_length(10.)),
            ));
        }

        svg_document.view_box = Some(view_box as [f32; 4]);

        let svg_str = svg_document.render();
        let svg_str = svg_str.as_str();
        let rendering_options: usvg::Options = usvg::Options {
            resources_dir: None,
            dpi: 150.0,
            font_family: "Times New Roman".to_string(),
            font_size: 12.0,
            languages: vec!["en".to_string()],
            shape_rendering: if antialiasing {
                ShapeRendering::GeometricPrecision
            } else {
                ShapeRendering::CrispEdges
            },
            text_rendering: Default::default(),
            image_rendering: Default::default(),
            keep_named_groups: false,
            fontdb: fontdb::Database::new(),
        };
        let mut result_pixmap: Pixmap =
            Pixmap::new(width, height).ok_or(StegError::PixmapAllocation)?;
        let svg_tree = usvg::Tree::from_str(svg_str, &rendering_options)
            .map_err(|err| StegError::USVGError { source: err })?;

        // height will be deduced from the scaled view_box
        match resvg::render(&svg_tree, usvg::FitTo::Width(width), result_pixmap.as_mut()) {
            None => Err(StegError::PixmapRendering),
            Some(_) => Ok(result_pixmap),
        }
    }
}
