//! WASM Renderer

use crate::renderer::{RendererSettings, Rendering};

mod audio;
mod texture;
mod window;

pub use audio::{AudioDevice, AudioFormatNum};

#[derive(Debug)]
pub(crate) struct Renderer {}

impl Rendering for Renderer {
    fn new(settings: RendererSettings) -> crate::prelude::PixResult<Self> {
        todo!()
    }

    fn clear(&mut self) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn set_draw_color(&mut self, color: crate::prelude::Color) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn clip(&mut self, rect: Option<crate::prelude::Rect<i32>>) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn blend_mode(&mut self, mode: crate::prelude::BlendMode) {
        todo!()
    }

    fn present(&mut self) {
        todo!()
    }

    fn scale(&mut self, x: f32, y: f32) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn font_size(&mut self, size: u32) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn font_style(&mut self, style: crate::prelude::FontStyle) {
        todo!()
    }

    fn font_family(&mut self, font: &crate::prelude::theme::Font) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn clipboard_text(&self) -> String {
        todo!()
    }

    fn set_clipboard_text(&self, value: &str) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn open_url(&self, url: &str) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn text(
        &mut self,
        position: crate::prelude::Point<i32>,
        text: &str,
        wrap_width: Option<u32>,
        angle: Option<f64>,
        center: Option<crate::prelude::Point<i32>>,
        flipped: Option<crate::prelude::Flipped>,
        fill: Option<crate::prelude::Color>,
        outline: u16,
    ) -> crate::prelude::PixResult<(u32, u32)> {
        todo!()
    }

    fn size_of(
        &self,
        text: &str,
        wrap_width: Option<u32>,
    ) -> crate::prelude::PixResult<(u32, u32)> {
        todo!()
    }

    fn point(
        &mut self,
        p: crate::prelude::Point<i32>,
        color: crate::prelude::Color,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn line(
        &mut self,
        line: crate::prelude::Line<i32>,
        smooth: bool,
        width: u8,
        color: crate::prelude::Color,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn bezier<I>(
        &mut self,
        ps: I,
        detail: i32,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()>
    where
        I: Iterator<Item = crate::prelude::Point<i32>>,
    {
        todo!()
    }

    fn triangle(
        &mut self,
        tri: crate::prelude::Tri<i32>,
        smooth: bool,
        fill: Option<crate::prelude::Color>,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn rect(
        &mut self,
        rect: crate::prelude::Rect<i32>,
        radius: Option<i32>,
        fill: Option<crate::prelude::Color>,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn quad(
        &mut self,
        quad: crate::prelude::Quad<i32>,
        smooth: bool,
        fill: Option<crate::prelude::Color>,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn polygon<I>(
        &mut self,
        ps: I,
        smooth: bool,
        fill: Option<crate::prelude::Color>,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()>
    where
        I: Iterator<Item = crate::prelude::Point<i32>>,
    {
        todo!()
    }

    fn ellipse(
        &mut self,
        ellipse: crate::prelude::Ellipse<i32>,
        smooth: bool,
        fill: Option<crate::prelude::Color>,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn arc(
        &mut self,
        p: crate::prelude::Point<i32>,
        radius: i32,
        start: i32,
        end: i32,
        mode: crate::prelude::ArcMode,
        fill: Option<crate::prelude::Color>,
        stroke: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn image(
        &mut self,
        img: &crate::prelude::Image,
        src: Option<crate::prelude::Rect<i32>>,
        dst: Option<crate::prelude::Rect<i32>>,
        angle: f64,
        center: Option<crate::prelude::Point<i32>>,
        flipped: Option<crate::prelude::Flipped>,
        tint: Option<crate::prelude::Color>,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn to_bytes(&mut self) -> crate::prelude::PixResult<Vec<u8>> {
        todo!()
    }

    fn open_controller(
        &mut self,
        controller_id: crate::event::ControllerId,
    ) -> crate::prelude::PixResult<()> {
        todo!()
    }

    fn close_controller(&mut self, controller_id: crate::event::ControllerId) {
        todo!()
    }
}
