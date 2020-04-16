#![allow(unused_variables, dead_code)]

pub mod color;
pub mod event;
pub mod gui;
pub mod image;
pub mod math;
pub mod shape;
pub mod transform;
pub mod typography;

mod common;
mod core;
mod renderer;
mod state;
mod time;

pub use crate::{
    color::{Color, ColorMode},
    common::{Error, Result},
    core::{PixApp, PixEngine},
    event::{PixEvent, WindowEvent},
    state::State,
};

pub mod constants {
    pub use std::f32::consts::{FRAC_PI_2, FRAC_PI_4, PI};
    pub const QUARTER_PI: f32 = FRAC_PI_2;
    pub const HALF_PI: f32 = FRAC_PI_2;
    pub const TWO_PI: f32 = 2.0 * PI;
    pub const TAU: f32 = TWO_PI;
}
