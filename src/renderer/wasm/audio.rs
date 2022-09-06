use super::Renderer;
use crate::{
    audio::{AudioDeviceDriver, AudioDriver},
    prelude::*,
};
use std::marker::PhantomData;

/// Audio callback or playback device that can be paused and resumed.
#[derive(Debug)]
pub struct AudioDevice<CB: AudioCallback>(PhantomData<CB>);

impl<CB: AudioCallback> AudioDeviceDriver for AudioDevice<CB> {
    fn status(&self) -> AudioStatus {
        todo!()
    }

    fn driver(&self) -> &'static str {
        todo!()
    }

    fn spec(&self) -> AudioSpec {
        todo!()
    }

    fn resume(&self) {
        todo!()
    }

    fn pause(&self) {
        todo!()
    }
}

/// Represents a valid Channel format for generic audio sample types.
pub trait AudioFormatNum {}

impl AudioFormatNum for i8 {}
impl AudioFormatNum for u8 {}
impl AudioFormatNum for i16 {}
impl AudioFormatNum for u16 {}
impl AudioFormatNum for i32 {}
impl AudioFormatNum for f32 {}

impl AudioDriver for Renderer {
    fn enqueue_audio(&mut self, samples: &[f32]) -> crate::prelude::Result<()> {
        todo!()
    }

    fn clear_audio(&mut self) {
        todo!()
    }

    fn audio_status(&self) -> crate::prelude::AudioStatus {
        todo!()
    }

    fn audio_driver(&self) -> &'static str {
        todo!()
    }

    fn audio_sample_rate(&self) -> i32 {
        todo!()
    }

    fn audio_queued_size(&self) -> u32 {
        todo!()
    }

    fn audio_size(&self) -> u32 {
        todo!()
    }

    fn resume_audio(&mut self) {
        todo!()
    }

    fn pause_audio(&mut self) {
        todo!()
    }

    #[allow(single_use_lifetimes)]
    fn open_playback<'a, CB, F, D>(
        &self,
        device: D,
        desired_spec: &crate::prelude::AudioSpecDesired,
        get_callback: F,
    ) -> crate::prelude::Result<crate::prelude::AudioDevice<CB>>
    where
        CB: crate::prelude::AudioCallback,
        F: FnOnce(crate::prelude::AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        todo!()
    }

    #[allow(single_use_lifetimes)]
    fn open_capture<'a, CB, F, D>(
        &self,
        device: D,
        desired_spec: &crate::prelude::AudioSpecDesired,
        get_callback: F,
    ) -> crate::prelude::Result<crate::prelude::AudioDevice<CB>>
    where
        CB: crate::prelude::AudioCallback,
        F: FnOnce(crate::prelude::AudioSpec) -> CB,
        D: Into<Option<&'a str>>,
    {
        todo!()
    }
}
