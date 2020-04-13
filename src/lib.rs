#![allow(unused_variables, dead_code)]

#[macro_use]
extern crate bitflags;

mod color;
mod common;
mod core;
mod event;
mod gui;
mod image;
mod math;
mod renderer;
mod shape;
mod state;
mod time;
mod transform;
mod typography;

pub use crate::{
    color::{Color, ColorMode},
    common::{PixEngineError, PixEngineResult},
    core::{PixApp, PixEngine},
    state::State,
};
