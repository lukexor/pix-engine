#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
use std::time::Instant;

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(crate) fn now() -> Instant {
    Instant::now()
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(crate) fn now() -> f64 {
    // TODO: Get wasm time
    0.0
}

#[cfg(all(feature = "sdl2-renderer", not(feature = "wasm-renderer")))]
pub(crate) fn sub(time1: Instant, time2: Instant) -> f64 {
    (time1 - time2).as_secs_f64()
}
#[cfg(all(feature = "wasm-renderer", not(feature = "sdl2-renderer")))]
pub(crate) fn sub(time1: f64, time2: f64) -> f64 {
    time1 - time2
}
