use super::StateData;
use crate::PixEngineResult;

pub mod element;
pub mod screen;
pub mod selection;

pub trait Drawable {
    fn update(&mut self, _data: &mut StateData) {}
    fn draw(&mut self, _data: &mut StateData) -> PixEngineResult<()> {
        Ok(())
    }
}
