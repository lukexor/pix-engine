# Renderer performance baseline

Frame timings for the SDL2 renderer, taken before the wgpu, egui and winit migration, and for
the wgpu renderer that replaced it. These measure how long a frame takes to render, so a rerun
after each renderer phase says whether batching arrived and whether anything regressed.

Numbers only mean something against the same machine and the same protocol. Both are recorded
below. Rerun on this machine, not a different one, and compare like for like.

## Protocol

Set `PIX_BENCH_FRAMES` to the sample size and run the example in release:

```sh
PIX_BENCH_FRAMES=600 cargo run --release --features serde --example matrix
```

- **Sampling.** The harness discards at least 60 frames and at least the first two seconds,
  whichever is longer, then records the next `PIX_BENCH_FRAMES` frames and exits. The time floor
  matters because several examples hold off real work at startup. `matrix` draws nothing for its
  first 1.5 seconds, and a frame-only warmup lands entirely inside that window.
- **What a sample covers.** Each sample is taken after the frame presents and before the pacing
  sleep, so it is the work the frame did, not the interval it was scheduled at.
- **Pacing is left alone.** Examples keep their own `target_frame_rate`. Removing it shrinks
  [`PixState::delta_time`] to near zero, and `matrix`, `asteroids` and `flocking` all drive
  motion from it, so an unpaced run renders a still scene and measures the wrong thing. `matrix`
  then times ten times faster, which reads as a quick renderer and is an idle one.
- **Three runs per example**, reporting the median run by mean frame time. The spread column
  gives all three means so a noisy run is visible rather than hidden.
- VSync is off for every example here. The harness prints a warning when it is on, because the
  display wait then lands inside the sample.

## Machine

| | |
|---|---|
| CPU | Intel Core i9-9900K, 8 cores / 16 threads, 3.60 GHz |
| GPU | NVIDIA GeForce RTX 3060 Ti (GA104) |
| Display | 2560x1440 at 59.91 Hz |
| OS | Arch Linux, kernel 7.1.4-arch1-1, x86_64 |
| Rust | rustc 1.93.1 (01f6ddf75 2026-02-11), release profile |
| SDL | sdl2-compat 2.32.70, sdl2_gfx 1.0.4, sdl2_ttf 2.24.0, sdl2_image 2.8.12 |
| Crate | pix-engine 0.8.0, `--features serde` |

The SDL line is worth reading twice. This is sdl2-compat, which implements the SDL2 API on top
of SDL3, not the original SDL2. Anything measured here includes that translation layer.

## SDL2 results

Frame times in milliseconds. Lower is better.

| Example | Frames | Paced | Mean | p50 | p99 | Max | fps | First draw | Peak RSS |
|---|---|---|---|---|---|---|---|---|---|
| `matrix` | 600 | 30 | 2.486 | 2.885 | 4.081 | 5.610 | 402 | 3.4 | 247 MB |
| `fluid_simulation` | 600 | 30 | 10.223 | 11.319 | 17.636 | 21.978 | 98 | 7.5 | 636 MB |
| `3d_raytracing` | 120 | none | 237.208 | 237.310 | 251.497 | 253.997 | 4.2 | 289.6 | 324 MB |
| `2d_raycasting` | 600 | none | 0.860 | 0.834 | 1.354 | 2.040 | 1162 | 7.3 | 246 MB |
| `asteroids` | 600 | none | 0.885 | 0.878 | 1.044 | 1.932 | 1130 | 4.5 | 244 MB |
| `flocking` | 600 | 60 | 14.999 | 14.359 | 22.187 | 27.414 | 67 | 26.7 | 257 MB |
| `shapes` | 600 | 60 | 1.650 | 1.484 | 5.273 | 6.855 | 606 | 3.9 | 243 MB |
| `gui` | 600 | 60 | 0.537 | 0.501 | 1.113 | 1.511 | 1864 | 5.7 | 242 MB |

Run-to-run spread of the mean, smallest to largest:

| Example | Run means |
|---|---|
| `matrix` | 2.485 / 2.486 / 2.557 |
| `fluid_simulation` | 10.163 / 10.223 / 10.482 |
| `3d_raytracing` | 236.635 / 237.208 / 238.209 |
| `2d_raycasting` | 0.828 / 0.860 / 0.998 |
| `asteroids` | 0.882 / 0.885 / 0.957 |
| `flocking` | 14.775 / 14.999 / 17.960 |
| `shapes` | 1.473 / 1.650 / 1.731 |
| `gui` | 0.494 / 0.537 / 0.640 |

Every example repeats within a few percent apart from `flocking`, whose slowest run sits 21%
above its fastest. Treat a `flocking` change under about 25% as noise.

## What each example exercises

| Example | Window | Load |
|---|---|---|
| `matrix` | 2560x1440 | Thousands of text draws per frame, one cached texture per unique string |
| `fluid_simulation` | 700x300 | Per-pixel `Image` mutation and a full image re-upload every frame |
| `3d_raytracing` | 800x800 | One `point()` call per pixel, the worst case for unbatched drawing |
| `2d_raycasting` | 1000x800 | `clip`, `BlendMode::Mod`, visibility polygons, `light.png` re-upload |
| `asteroids` | 800x600 | `wireframe` into `polygon`, two `Vec<i16>` allocations per call |
| `flocking` | 1000x800 | Many small polygons, boid simulation |
| `shapes` | 800x600 | Every drawing primitive once |
| `gui` | 1024x768 | Text measurement, `size_of` per line per widget per frame |

## Reading the SDL2 numbers

`3d_raytracing` at 237 ms a frame is the headline. It draws one `point()` per pixel, and every
one is a separate SDL2_gfx call. It is the clearest test of whether batching landed.

`matrix` has a mean below its median, which says it had not reached steady state within the
sample. Its stream count grows over the run, so early frames are quick. The protocol is applied
identically before and after, so the comparison holds, but its steady-state frame time runs
higher than the number here.

`shapes` has a p99 more than three times its median. Each call draws little enough that cache
misses and text uploads dominate the tail.

Peak RSS sits near 240 MB for almost everything, which is the floor SDL and the bundled fonts
impose rather than anything the example does. `fluid_simulation` at 636 MB is the exception and
comes from its own buffers.

## wgpu results

Taken after Phase 3, on the same machine and the same protocol. The graphics stack replaces the
SDL line above, and the kernel moved one point release between the two runs.

| | |
|---|---|
| Kernel | Arch Linux, kernel 7.1.9-arch1-2, x86_64 |
| Driver | NVIDIA 610.57.04, vulkan-icd-loader 1.4.357.0 |
| Stack | wgpu 29.0.4, egui 0.35.0, epaint 0.35.0, winit 0.30.13 |
| Rust | rustc 1.93.1 (01f6ddf75 2026-02-11), release profile |
| Crate | pix-engine 0.8.0, `--features serde` |

wgpu selects a backend at runtime, so which one served these runs is not recorded.

| Example | Frames | Paced | Mean | p50 | p99 | Max | fps | First draw | Peak RSS |
|---|---|---|---|---|---|---|---|---|---|
| `matrix` | 600 | 30 | 1.832 | 1.942 | 2.748 | 3.104 | 546 | 19.4 | 290 MB |
| `fluid_simulation` | 600 | 30 | 8.674 | 9.532 | 12.638 | 14.261 | 115 | 20.4 | 305 MB |
| `3d_raytracing` | 120 | none | 157.537 | 154.223 | 174.746 | 177.370 | 6.3 | 174.2 | 413 MB |
| `2d_raycasting` | 600 | none | 0.638 | 0.492 | 10.613 | 10.707 | 1567 | 27.9 | 327 MB |
| `asteroids` | 600 | none | 0.244 | 0.114 | 10.320 | 10.638 | 4098 | 15.5 | 290 MB |
| `flocking` | 600 | 60 | 4.434 | 4.364 | 5.943 | 8.560 | 226 | 20.1 | 289 MB |
| `shapes` | 600 | 60 | 0.315 | 0.301 | 0.589 | 0.668 | 3178 | 15.0 | 288 MB |
| `gui` | 600 | 60 | 0.350 | 0.342 | 0.669 | 0.863 | 2858 | 15.7 | 290 MB |

Run-to-run spread of the mean, smallest to largest:

| Example | Run means |
|---|---|
| `matrix` | 1.799 / 1.832 / 2.026 |
| `fluid_simulation` | 8.613 / 8.674 / 8.694 |
| `3d_raytracing` | 156.381 / 157.537 / 158.976 |
| `2d_raycasting` | 0.591 / 0.638 / 0.770 |
| `asteroids` | 0.239 / 0.244 / 0.270 |
| `flocking` | 3.493 / 4.434 / 4.578 |
| `shapes` | 0.311 / 0.315 / 0.497 |
| `gui` | 0.340 / 0.350 / 0.352 |

Against SDL2, mean and p99 side by side:

| Example | SDL mean | wgpu mean | Change | SDL p99 | wgpu p99 | Change |
|---|---|---|---|---|---|---|
| `matrix` | 2.486 | 1.832 | -26% | 4.081 | 2.748 | -33% |
| `fluid_simulation` | 10.223 | 8.674 | -15% | 17.636 | 12.638 | -28% |
| `3d_raytracing` | 237.208 | 157.537 | -34% | 251.497 | 174.746 | -31% |
| `2d_raycasting` | 0.860 | 0.638 | -26% | 1.354 | 10.613 | +684% |
| `asteroids` | 0.885 | 0.244 | -72% | 1.044 | 10.320 | +888% |
| `flocking` | 14.999 | 4.434 | -70% | 22.187 | 5.943 | -73% |
| `shapes` | 1.650 | 0.315 | -81% | 5.273 | 0.589 | -89% |
| `gui` | 0.537 | 0.350 | -35% | 1.113 | 0.669 | -40% |

## Reading the wgpu numbers

Mean frame time drops on all eight. `shapes`, `asteroids` and `flocking` move most, and all
three are dominated by primitives that SDL2_gfx issued one call at a time. That is the batching
the migration was for.

`3d_raytracing` improves 34% and still takes 157 ms a frame. It draws one `point()` per pixel,
and consecutive points already coalesce into one mesh. A profile puts the whole draw path at
about 13% of the frame against 45% for the example's own raytracing, so what is left is not the
renderer's to give back.

`2d_raycasting` and `asteroids` regress on p99, from about 1 ms to about 10.5 ms. Both are the
examples that set no `target_frame_rate`, and both now run above 1500 frames a second. The
likely mechanism is swapchain back-pressure: `desired_maximum_frame_latency` is 2, so a frame
that outruns the presentation engine blocks in `Surface::get_current_texture`, and the protocol
samples after present. That is a wait rather than work, but it has not been confirmed. Nothing
else regressed.

Peak RSS rises about 45 MB across the board, which is the wgpu device and its driver. Within a
run the first invocation reports 35 to 50 MB more than the two that follow it, on every example,
so compare like against like. `fluid_simulation` is the one large change, down from 636 MB to
305 MB. It is the only example driving a full image upload every frame, so the upload path is
where to look, but the cause was not traced.

First draw rises on the cheap examples, `matrix` from 3.4 ms to 19.4 ms, and this is pipeline
creation on the first frame. `3d_raytracing` improves, from 290 ms to 174 ms.

## Color scaling and the renderer hasher

Two changes taken after the table above: scaling an [`Rgb`] color skips the level round trip, the
byte conversion adds a half instead of calling `f64::round`, and the renderer maps hash with
`ahash` rather than SipHash.

Frame times drift between sessions on this machine. The same HEAD binary measured `matrix` at
1.80 to 2.03 ms in the session that produced the table and 2.02 to 2.09 ms in the one that
produced this section, so absolute numbers here do not compare against it. Both builds were run
interleaved in one session instead, alternating which went first, because whichever runs second
measures slower.

| Example | HEAD | With both changes | Change |
|---|---|---|---|
| `3d_raytracing` | 164.7 | 145.2 | -12% |
| `fluid_simulation` | 9.68 | 9.95 | within noise |
| `matrix` | 2.07 | 2.15 | within noise |

`3d_raytracing` is the one that moves, and it is the one scaling a color per pixel. Its -12% was
measured with the new build in the slower second slot, so the real figure is a little better.
`matrix` and `fluid_simulation` sit inside the ordering effect: whichever build ran first won,
whichever way round they were run.

Scaling one color, timed on its own away from any example, runs at 18.4 ns before the change,
16.0 ns with the round trip skipped, and 8.2 ns once `f64::round` goes. `round` is a libm call on
a target without SSE4.1, which is why dropping it is worth more than the restructure around it.

Hashing no longer shows up. `BuildHasher::hash_one` was 2.0% of a `3d_raytracing` frame, one
lookup per `point()` call, and it is absent from the profile after the swap.
