# Color

## Creating

* `rgb!(values)`: Create a new RGB color
* `hsv!(values)`: Create a new HSV color
* `lerp(color2, amount)`: Create a new Rgb/Hsv color by linear interpolating between two colors
* `random()`: Create a new random Rgb/Hsv color with full alpha
* `random_alpha()`: Create a new random Rgb/Hsv color with random alpha
* `NAMED_COLOR`: A SVG 1.0 Color Constant

Where `values` can be:
  * `gray`: Gray value ranging from 0-255 (RGB) or 0.0-1.0 (HSV)
  * `[gray]`: 1, 2, or 3 gray values ranging from 0-255 (RGB) or 0.0-1.0 (HSV)
  * `[gray, alpha]`: 1, 2, or 3 gray values, followed by alpha ranging from 0-255 (RGB) or 0.0-1.0 (HSV)
  * `[v1, v2, v3]`: Red, Green, and Blue values for RGB or Hue, Saturating, and Value channels for HSV
  * `[v1, v2, v3, alpha]`: Red, Green, Blue, and Alpha values for RGB or Hue, Saturating, Value, and Alpha values for HSV
  * `hexidecimal`: A hexadecimal string value (in 3, 4, 6, or 8 digit formats, e.g. '#FF0000')
  * `array_slice`: An array slice containing Red, Green, Blue and optionally Alpha or Hue, Saturation, Value and optionally Alpha.

## Reading & Modifying

* `Color`
  * `Rgb(Rgb)`
  * `Hsv(Hsv)`
* `Rgb`
  + `rgb(r, g, b)`: Create a new Rgb color with alpha 255
  + `rgba(r, g, b, a)`: Create a new Rgb color with alpha
  * `from_slice(slice)`: Create a new Rgb color from a u8 slice
  * `from_hex(hex)`: Create a new Rgb color from a u32 hex value
  * `alpha()`: Returns the alpha channel between 0-255
  * `blue()`: Returns the blue channel between 0-255
  * `green()`: Returns the green channel between 0-255
  * `red()`: Returns the red channel between 0-255
  * `channels()`: Returns all channels as a 4-element tuple of u8
  * `set_blue(v)`: Set the blue channel between 0-255
  * `set_green(v)`: Set the green channel between 0-255
  * `set_red(v)`: Set the red channel between 0-255
  * `to_hsv()`: Convert Rgb color to Hsv color
* `Hsv`
  + `hsv(h, s, v)`: Create a new Hsv color with alpha 1.0
  + `hsva(h, s, v, a)`: Create a new Hsv color with alpha
  * `alpha()`: Returns the alpha channel between 0.0-1.0
  * `value()`: Returns the value channel between 0.0-1.0
  * `hue()`: Returns the ue channel between 0.0-360.0
  * `channels()`: Returns all channels as a 4-element tuple of f32
  * `saturation()`: Returns the saturation channel between 0.0-1.0
  * `set_alpha(v)`: Set the alpha channel between 0.0-1.0
  * `set_value(v)`: Set the value channel between 0.0-1.0
  * `set_hue(v)`: Set the ue channel between 0.0-360.0
  * `set_saturation(v)`: Set the saturation channel between 0.0-1.0
  * `to_rgb()`: Convert Hsv color to Rgb color

## Setting

- `State`
  * `background(color)`: Set the background color for clearing the screen each frame.
  - `background_image(image, alpha)`: Set a background image to draw each frame.
  * `clear()`: Clear the screen to the background color.
  * `fill(color)`: Set the fill color for drawing shapes.
  * `no_fill()`: Disable filling in shapes while drawing.
  * `no_stroke()`: Disable outlining shapes while drawing.
  * `stroke(color)`: Set the stroke outline color for drawing shapes.

# Constants

* `INFINITY`: Convenience constant for `f64::INFINITY`.
* `SQRT_2`: Convenience constant for `f64::consts::SQRT_2`
* `HALF_PI`: Convenience constant for 1/2 PI as f64.
* `PI`: Convenience constant for PI as f64.
* `QUARTER_PI`: Convenience constant for 1/4 PI as f64.
* `TAU`: Convenience constant for 2 PI as f64. Same as `TWO_PI`.
* `TWO_PI`: Convenience constant for 2 PI as f64. Same as `TAU`.

# Environment

- `State`
  * `width()`: Get the width of the current canvas.
  * `height()`: Get the height of the current canvas.
  - `cursor(type, hot_x, hot_y)`: Change the system mouse cursor.
  * `delta_time()`: Get the elapsed time in milliseconds since last frame.
  - `display_density()`: Get the pixel density of the current display.
  - `display_height()`: Get the height of the current display.
  - `display_width()`: Get the width of the current display.
  * `focused()`: Get whether the application has focus.
  * `frame_count()`: Get the total number of frames since start.
  * `frame_rate()`: Get the current frame rate per second.
  * `fullscreen()`: Get whether the application is fullscreen or not.
  - `no_cursor()`: Hide the system mouse cursor.
  - `pixel_density()`: Get the current pixel density setting.
  - `resize_canvas(width, height)`: Resize the current canvas.
  - `resize_window(width, height)`: Resize the current window.
  - `set_frame_rate(fps)`: Set the target frame rate per second.
  * `set_fullscreen(val)`: Set whether the application is fullscreen or not.
  - `set_pixel_density(val)`: Set the current pixel density setting.
- `Stateful`
  - `window_resized(id, width, height)`: This function is called whenever a window is resized.

# Events

## Acceleration

- `touches()`: Get a list of all touches relative to (0, 0) of the active canvas.
- `touchStarted()`
- `touchMoved()`
- `touchEnded()`

## Keyboard

- `State`
  * `key_pressed(keycode)`: Get whether a given key is held down or not.
  * `keys()`: Get a set of keys currently pressed.
- `Stateful`
  * `on_key_pressed(state, key)`: This function is called whenever a key is pressed.
  * `on_key_released(state, key)`: This function is called whenever a key is released.
  - `on_key_typed(state, key)`: This function is called whenever a letter or punctuation key is typed.

## Mouse

- `State`
  * `mouse_pos()`: Get the current mouse position as a tuple of (x, y) relative to (0, 0) of the active canvas.
  * `pmouse_pos()`: Get the previous mouse position last frame as a tuple of (x, y) relative to (0, 0) of the active canvas.
  - `mouse_moved_pos()`: Gets the change in mouse position since last frame as a tuple of (x, y).
  - `win_mouse_pos()`: Get the current mouse position as a tuple of (x, y) relative to (0, 0) of the active window.
  - `prev_win_mouse_pos()`: Get the previous position last frame as a tuple of
    (x, y) relative to (0, 0) of the active window.
  * `mouse_pressed(btn)`: Get whether any mouse button is pressed or not.
  * `mouse_buttons()`: Get a set of mouse buttons currently pressed.
- `Stateful`
  - `on_mouse_moved(x, y)`: This function is called whenever the mouse pointer is moved.
  * `on_mouse_dragged(buttons)`: This function is called whenever the mouse pointer is moved while a mouse button is held down.
  * `on_mouse_pressed(button)`: This function is called whenever a mouse button is pressed.
  * `on_mouse_released(button)`: This function is called whenever a mouse button is released.
  - `on_mouse_clicked(button)`: This function is called whenever a mouse button is pressed and then released.
  - `on_mouse_dbl_clicked()`: This function is called whenever a mouse button is double clicked.
  - `on_mouse_wheel(delta)`: This function is called whenever the mouse wheel is scrolled.

# IO

## Input

TODO

## Output

- `State`
  - `save(filename)`

## Logging??

- `LoggerConfig`
- `Logger`
  - `from_config(config)`
  - `start`
- `PixEngine`
  - `start_logger()`

## Time & Date

- `State`
  - `day()`: Get the current day of the month between 1 and 31.
  - `hour()`: Get the current hour between 0 and 23.
  - `millis()`: Get the number of milliseconds elapsed since the application start.
  - `minute()`: Get the current minute between 0 and 59.
  - `month()`: Get the current month of the year between 1 and 12.
  - `second()`: Get the current second between 0 and 59.
  - `year()`: Get the current year (e.g. 2020, 2021).

# Image

## Loading & Displaying

- `image!(state, values)`: Draws an image to the canvas.
  - `(img, x, y)`
  - `(img, vector)`
- `image_resized!(state, values)`
  - `(img, x, y, w, h)`
  - `(img, rect)`
- `image_partial!(state, values)`
  - `(img, (sx, sy, sw, sh), (dx, dy, dw, dh))`
  - `(img, srect, drect)`
- `State`
  - `create_image(width, height)`: Creates an empty image with width and height.
  - `image_mode(mode)`: Change the current image mode which determines how
    images are drawn to the canvas.
  - `load_image(filename)`: Loads an image from a filename
  - `no_tint()`: Disable tinting while drawing images
  - `tint(color)`: Tint image to a given color when drawing
  - `draw_pixels(pixels)`: Draws an array of u8 pixels to thee canvas.
- `Image`
  - `load(filename)`: Loads an image from a filename
  - `save(filename)`: Save an image to a filename

## Pixels

- `FilterParams`
- `Image`
  - `blend()`
    - `blend([sx, sy, sw, sh], [dx, dy, dw, dh], blend_mode)`
    - `blend(srect, drect, blend_mode)`
  - `copy()`
    - `copy([sx, sy, sw, sh], [dx, dy, dw, dh])`
    - `copy(srect, drect)`
  - `filter(type, params)`
  - `get()`
  - `load_pixels()`
  - `mask(src_image)`
  - `pixels()`
  - `resize(width, height)`
  - `set()`
  - `update_pixels()`

# Lights, Camera

TODO

# Math

## Calculation

* `Math`
  * `constrain(value, min, max)`: Constraints an integer value between a minimum and maximum value.
  * `constrainf(value, min, max)`: Constraints a floating point value between a minimum and maximum value.
  * `lerp(start, end, amount)`: Linear interpolates between two numbers by a given amount
  * `lerp_map(start1, end1, start2, end2)`: Linear interpolates values for a range of independent values based on depdendent values
  * `map(value, start1, stop1, start2, stop2)`: Maps a number value from one range to another
  * `inside_circle(x, y, cx, cy, r)`: Calculates if a point (x, y) is inside a circle at (cx, cy) with radius r.

## Vector

* `vector!(values)`
  * `(x)`
  * `(x, y)`
  * `(x, y, z)`
* `Vector`
  * `copy()`
  * `get()`
  * `set(v2)`
  * `mag()`
  * `mag_sq()`
  * `dot(v2)`
  * `cross(v2)`
  * `dist(v2)`
  * `normalize()`
  * `limit(max)`
  * `set_mag(mag)`
  * `heading()`
  * `rotate(angle)`
  * `angle_between(v2)`
  * `lerp(v2)`
  * `reflect()`
  * `values()`
  * `to_vec()`
  * `from_angle(angle)`
  * `random_2d()`
  * `random_3d()`


## Noise

- `noise!(values, [seed], [lod], [falloff])`
  - `(x)`
  - `(x, y)`
  - `(x, y, z)`
  - `(vector)`

## Random

- `random!(value(s), [seed=])`
  * `(max)`
  * `(min, max)`
  - `(max, seed=value)`
  - `(min, max, seed=value)`
- `randomf!(value(s), [seed=])`
  * `(max)`
  * `(min, max)`
  - `(max, seed=value)`
  - `(min, max, seed=value)`
- `choose_random!(values, [seed])`
  - `(choices, max)`
  - `(choices, min, max)`
  - `(choices, max, seed=value)`
  - `(choices, min, max, seed=value)`

## Trigonometry

- `AngleMode` Radians | Degrees
- `State`
  - `angle_mode(mode)`

# Rendering

- `BlendMode` Blend | Darkest | Lightest | Difference | Multiply | Exclusion | Screen | Replace | Overlay | HardLight | SoftLight | Dodge | Burn | Add | Remove | Subtract
- `State`
  - `create_graphics(width, height)`
  - `blend_mode(mode)`

# Shape

## 2D Primitives

- `State`
  - `arc()`
    - `arc((x, y), (w, h), start, stop)`
    - `arc(v1, v2, start, stop)`
  - `ellipse()`
    * `ellipse(x, y, w, h)`
    - `ellipse((x, y), (w, h))`
    - `ellipse(v1, v2)`
  - `circle()`
    * `circle(x, y, r)`
    - `circle((x, y), r)`
    - `circlev(v, r)`
  - `line()`
    * `line(x1, y1, x2, y2)`
    - `line((x1, y1), (x2, y2))`
    - `linev(v1, v2)`
  - `point()`
    - `point((x, y))`
    - `pointv(vector)`
  - `quad()`
    - `quad((x1, y1), (x2, y2), (x3, y3), (x4, y4))`
    - `quad(v1, v2, v3, v4)`
  - `rect()`
    * `rect(x, y, w, h)`
    - `rect((x, y), (w, h))`
    - `rect(v1, v2)`
  - `rect_rounded()`
    - `rect_rounded((x, y), (w, h), (tl, tr), (br, bl))`
    - `rect_rounded(v1, v2, r1, r2)`
  - `square()`
    * `square(x, y, s)`
    - `square((x, y), s)`
    - `square(v, s)`
  - `square_rounded()`
    - `square_rounded((x, y), s, (tl, tr), (br, bl))`
    - `square_rounded(v1, s, r1, r2)`
  - `triangle()`
    * `triangle(x1, y1, x2, y2, x3, y3)`
    - `triangle((x1, y1), (x2, y2), (x3, y3))`
    - `triangle(v1, v2, v3)`

## 3D Primitives

- `State`
    - `point((x, y, z))`
    - `point(vector)`
  - `line()`
    - `line((x1, y1, z1), (x2, y2, z3))`
    - `line(v1, v2)`
  - `quad()`
    - `quad((x1, y1, z1), (x2, y2, z2), (x3, y3, z3), (x4, y4, z4))`
    - `quad(v1, v2, v3, v4)`
  - `plane()`
    - `plane((x1, y1, z1), (x2, y2, z2))`
    - `plane(v1, v2)`
  - `box_()`
    - `box_((x, y, z), (w, h, d))`
    - `box_(v1, v2)`
  - `sphere()`
    - `sphere((x, y, z), r)`
    - `sphere(c, r)`
  - `cylinder()`
    - `cylinder((x, y, z), r, h)`
    - `cylinder(c, r, h)`
  - `cone()`
    - `cone((x, y, z), r, h)`
    - `cone(c, r, h)`
  - `ellipsoid()`
    - `ellipsoid((x, y, z), (rx, ry, rz))`
    - `ellipsoid(v1, v2)`
  - `torus()`
    - `torus((x, y, z), r, tr)`
    - `torus(v1, r, tr)`
  - `point()`

## 3D Models

- `Model`
  - `normalize()`
- `State`
  - `load_model(filename)`
  - `model(model)`

## Attributes

- `ArcMode` Pie, Cord, Open
* `DrawMode` Corner, Center, Radius
- `StrokeCap` Round, Square, Project
- `StrokeJoin` Miter, Bevel, Round
- `State`
  - `arc_mode(mode)`
  * `ellipse_mode(mode)`
  * `rect_mode(mode)`
  - `stroke_cap(cap)`
  - `stroke_join(join)`
  - `stroke_weight(weight)`

## Curves

TODO

## Vertex

TODO

# Structure

- `Stateful`
  * `on_start()`
  * `on_update()`
  * `on_stop()`
  - `on_pause()`
  - `on_resume()`
- `State`
  - `loop()`
  - `no_loop()`
  - `push()`
  - `pop()`
  - `update()`

# Transform

- `Matrix`
- `State`
  - `apply_matrix()`
    - `apply_matrix([a, b, c, d, e, f])`
    - `apply_matrix(matrix)`
  - `reset_matrix()`
  - `rotate()`
    - `rotate(angle)`
    - `rotate_axis(angle, vector)`
    - `rotate_x(angle)`
    - `rotate_y(angle)`
    - `rotate_z(angle)`
  - `scale()`
    - `scale([x, y])`
    - `scale([x, y, z])`
    - `scale(vector)`
  - `shear()`
    - `shear_x(angle)`
    - `shear_y(angle)`
  - `translate()`
    - `translate([x, y])`
    - `translate([x, y, z])`
    - `translate(vector)`

# Typography

## Attributes

- `FontAlign`
- `FontStyle`
- `Font`
  - `bounds()`
    - `bounds(str, [x, y], size)`
    - `bounds(str, v, size)`
- `State`
  - `text_align(align)`
  - `text_leading(leading)`
  * `text_size(size)`
  - `text_font(font)`
  - `text_style(style)`
  - `text_width(text)`
  - `text_ascent()`
  - `text_descent()`

## Loading & Displaying

- `State`
  - `load_font(filename)`
  * `text()`
    * `text(str, x, y)`
    - `text(str, [x, y])`
    - `text(str, v)`
  - `text_rect(str, rect)`

# UI

- `Element`
  - `position()`
    - `position([x, y])`
    - `position(vector)`
  - `size()`
    - `size(w, h)`
    - `size(vector)`
  - `bg_color(color)`
  - `text_color(color)`
  - `font(font)`
  - `value()`
- `Slider`
  - `changed(callback)`
- `Button`
  - `mouse_pressed(callback)`
- `Checkbox`
  - `checked()`
  - `changed(callback)`
- `SelectBox`
  - `option(str)`
  - `selected(str)`
  - `enable(str)`
  - `disable(str)`
  - `changed(callback)`
- `Radio`
  - `option(str)`
  - `enable(str)`
  - `disable(str)`
  - `changed(callback)`
- `ColorPicker`
  - `input(callback)`
- `Input`
  - `InputType` Text | Password
  - `input(callback)`
- `FileInput`
  - `input(callback)`
- `State`
  - `create_slider(label, min, max, value, step)`
  - `create_button(label, value)`
  - `create_checkbox(label, value)`
  - `create_selectbox(multiple)`
  - `create_radio(multiple)`
  - `create_colorpicker(default_color)`
  - `create_input(placeholder, type)`
  - `create_fileinput(multiple)`

