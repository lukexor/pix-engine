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
