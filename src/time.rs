#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
use std::time::Instant;

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(crate) fn now() -> Instant {
    Instant::now()
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(crate) fn now() -> f32 {
    // TODO
    0.0
}

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(crate) fn sub(time1: Instant, time2: Instant) -> f32 {
    (time1 - time2).as_secs_f32()
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(crate) fn sub(time1: f32, time2: f32) -> &str {
    time1 - time2
}
