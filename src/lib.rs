#![allow(unused_variables, dead_code)]

pub mod color;
pub mod event;
pub mod gui;
pub mod image;

mod common;
mod core;
mod math;
mod renderer;
mod shape;
mod state;
mod time;
mod transform;
mod typography;

pub use crate::{
    color::{Color, ColorMode},
    common::{Error, Result},
    core::{PixApp, PixEngine},
    event::{PixEvent, WindowEvent},
    state::State,
};
