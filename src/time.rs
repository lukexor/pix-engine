//! Monotonic clock.
//!
//! `std::time::Instant` panics on `wasm32-unknown-unknown`, which has no monotonic clock behind
//! it. `web_time` reads `performance.now()` there and forwards to `std` everywhere else, so the
//! rest of the engine names one type and does not branch on target.

#[cfg(target_arch = "wasm32")]
pub(crate) use web_time::Instant;

#[cfg(not(target_arch = "wasm32"))]
pub(crate) use std::time::Instant;
