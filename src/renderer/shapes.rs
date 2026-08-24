//! Translation from the engine's drawing primitives into `epaint` shapes.
//!
//! Every function here is pure: no GPU, no renderer state, no `self`, so the mapping can be
//! unit-tested against values instead of pixels. Anti-aliasing, blending and clipping are applied
//! later, when the recorded shapes are tessellated.

use crate::prelude::{Color, Ellipse, Flipped, Line, Point, Quad, Rect as PixRect, Tri};
use egui::epaint::{
    Color32, CornerRadius, Mesh, Pos2, Rect, Shape, Stroke, StrokeKind, TextureId, Vec2, Vertex,
};

/// Steps used to sample an arc, per radian of sweep.
///
/// A longer arc needs more segments to stay smooth. Sampling by sweep rather than by a fixed count
/// keeps a small arc cheap and a large one round.
const ARC_STEPS_PER_RADIAN: f32 = 8.0;

/// Smallest number of segments an arc or curve is sampled at.
const MIN_CURVE_STEPS: usize = 8;

/// Largest number of segments an arc or curve is sampled at.
const MAX_CURVE_STEPS: usize = 256;

/// Converts an engine color into the premultiplied form `epaint` blends with.
#[inline]
pub(crate) fn color32(color: Color) -> Color32 {
    let [r, g, b, a] = color.channels();
    Color32::from_rgba_unmultiplied(r, g, b, a)
}

/// Converts an engine point into an `epaint` position.
#[inline]
pub(crate) fn pos2(point: Point<i32>) -> Pos2 {
    #[allow(clippy::cast_precision_loss)]
    Pos2::new(point.x() as f32, point.y() as f32)
}

/// Converts an engine rect into an `epaint` rect.
#[inline]
pub(crate) fn rect(rect: PixRect<i32>) -> Rect {
    #[allow(clippy::cast_precision_loss)]
    Rect::from_min_size(
        Pos2::new(rect.x() as f32, rect.y() as f32),
        Vec2::new(rect.width() as f32, rect.height() as f32),
    )
}

/// Builds a stroke, returning `None` when nothing should be outlined.
#[inline]
#[must_use]
fn stroke(color: Option<Color>, weight: u16) -> Option<Stroke> {
    let color = color?;
    if weight == 0 {
        return None;
    }
    Some(Stroke::new(f32::from(weight), color32(color)))
}

/// Builds a fill color, defaulting to transparent when nothing should be filled.
#[inline]
#[must_use]
fn fill(color: Option<Color>) -> Color32 {
    color.map_or(Color32::TRANSPARENT, color32)
}

/// Builds a mesh covering many single-pixel points in one shape.
///
/// Each point is a one-pixel quad rather than a circle, because a circle is feathered and a point
/// is meant to land on exactly one pixel. An application drawing per pixel emits thousands a
/// frame, and a shape apiece makes the tessellator walk a long list for the same geometry.
pub(crate) fn points(points: &[(Point<i32>, Color)]) -> Shape {
    let mut mesh = Mesh::default();
    mesh.vertices.reserve(points.len() * 4);
    mesh.indices.reserve(points.len() * 6);
    for (p, color) in points {
        let pos = pos2(*p);
        let color = color32(*color);
        #[allow(clippy::cast_possible_truncation)]
        let base = mesh.vertices.len() as u32;
        // A white texel, so the mesh samples nothing and shows the vertex color.
        let uv = Pos2::ZERO;
        for offset in [
            Vec2::ZERO,
            Vec2::new(1.0, 0.0),
            Vec2::new(1.0, 1.0),
            Vec2::new(0.0, 1.0),
        ] {
            mesh.vertices.push(Vertex {
                pos: pos + offset,
                uv,
                color,
            });
        }
        mesh.indices
            .extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
    }
    Shape::mesh(mesh)
}

/// Builds a straight line.
pub(crate) fn line(l: Line<i32>, color: Color, weight: u16) -> Shape {
    let Some(stroke) = stroke(Some(color), weight.max(1)) else {
        return Shape::Noop;
    };
    Shape::line_segment([pos2(l.start()), pos2(l.end())], stroke)
}

/// Builds a Bezier curve of any order.
///
/// `epaint` offers quadratic and cubic curves only, while the drawing API takes any number of
/// control points, so the curve is sampled with de Casteljau's algorithm and emitted as a
/// polyline. `detail` is the caller's requested segment count.
pub(crate) fn bezier(control: &[Point<i32>], detail: i32, color: Color, weight: u16) -> Shape {
    let Some(stroke) = stroke(Some(color), weight.max(1)) else {
        return Shape::Noop;
    };
    if control.len() < 2 {
        return Shape::Noop;
    }
    let steps = usize::try_from(detail)
        .unwrap_or(MIN_CURVE_STEPS)
        .clamp(MIN_CURVE_STEPS, MAX_CURVE_STEPS);
    let control: Vec<Pos2> = control.iter().copied().map(pos2).collect();
    let points = simplify(
        (0..=steps)
            .map(|step| {
                #[allow(clippy::cast_precision_loss)]
                let t = step as f32 / steps as f32;
                de_casteljau(&control, t)
            })
            .collect(),
    );
    outline(&points, stroke, false)
}

/// Evaluates a Bezier curve of any order at `t`.
fn de_casteljau(control: &[Pos2], t: f32) -> Pos2 {
    let mut points = control.to_vec();
    while points.len() > 1 {
        for index in 0..points.len() - 1 {
            points[index] = points[index].lerp(points[index + 1], t);
        }
        points.pop();
    }
    points[0]
}

/// Builds a triangle.
///
/// Already a single triangle, so the fill needs no triangulating.
pub(crate) fn triangle(
    t: Tri<i32>,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    weight: u16,
) -> Shape {
    let points = simplify(vec![pos2(t.p1()), pos2(t.p2()), pos2(t.p3())]);
    let mut shapes = Vec::with_capacity(2);
    if let Some(color) = fill_color {
        if encloses_area(&points) {
            shapes.push(Shape::mesh(fan(&points, color32(color))));
        }
    }
    if let Some(stroke) = stroke(stroke_color, weight) {
        shapes.push(outline(&points, stroke, true));
    }
    collapse(shapes)
}

/// Builds a rectangle, optionally with rounded corners.
///
/// The stroke sits inside the rect so that the drawn edge matches what `Rect::contains` reports.
pub(crate) fn rectangle(
    r: PixRect<i32>,
    radius: i32,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    weight: u16,
) -> Shape {
    // `CornerRadius` stores each corner in a `u8`, so a larger radius saturates.
    let radius = u8::try_from(radius.max(0)).unwrap_or(u8::MAX);
    let mut shape =
        egui::epaint::RectShape::filled(rect(r), CornerRadius::same(radius), fill(fill_color));
    if let Some(stroke) = stroke(stroke_color, weight) {
        shape.stroke = stroke;
        shape.stroke_kind = StrokeKind::Inside;
    }
    Shape::Rect(shape)
}

/// Builds a quadrilateral.
///
/// The four corners are arbitrary and may describe a concave shape, so the fill is emitted as two
/// triangles.
pub(crate) fn quad(
    q: Quad<i32>,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    weight: u16,
) -> Shape {
    let corners = [pos2(q.p1()), pos2(q.p2()), pos2(q.p3()), pos2(q.p4())];
    let mut shapes = Vec::with_capacity(2);
    if let Some(color) = fill_color {
        let mut mesh = Mesh::default();
        let color = color32(color);
        for corner in corners {
            mesh.vertices.push(Vertex {
                pos: corner,
                uv: Pos2::ZERO,
                color,
            });
        }
        mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
        shapes.push(Shape::mesh(mesh));
    }
    if let Some(stroke) = stroke(stroke_color, weight) {
        shapes.push(outline(&simplify(corners.to_vec()), stroke, true));
    }
    collapse(shapes)
}

/// Builds a polygon of any shape.
///
/// A convex outline fills as a fan and a concave one is triangulated by ear clipping. The stroke
/// follows the original outline either way.
pub(crate) fn polygon(
    vertices: &[Point<i32>],
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    weight: u16,
) -> Shape {
    let points = simplify(vertices.iter().copied().map(pos2).collect());
    if points.len() < 3 {
        return stroke(stroke_color, weight)
            .map_or(Shape::Noop, |stroke| outline(&points, stroke, false));
    }

    let mut shapes = Vec::with_capacity(2);
    if let Some(color) = fill_color {
        if !encloses_area(&points) {
            // Nothing to fill.
        } else if is_convex(&points) {
            shapes.push(Shape::mesh(fan(&points, color32(color))));
        } else if let Some(mesh) = ear_clip(&points, color32(color)) {
            shapes.push(Shape::mesh(mesh));
        }
    }
    if let Some(stroke) = stroke(stroke_color, weight) {
        shapes.push(outline(&points, stroke, true));
    }
    collapse(shapes)
}

/// Builds an ellipse.
pub(crate) fn ellipse(
    e: Ellipse<i32>,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    weight: u16,
) -> Shape {
    #[allow(clippy::cast_precision_loss)]
    let radius = Vec2::new(e.width() as f32 / 2.0, e.height() as f32 / 2.0);
    let center = pos2(e.center());
    let mut shapes = Vec::with_capacity(2);
    if let Some(color) = fill_color {
        shapes.push(Shape::ellipse_filled(center, radius, color32(color)));
    }
    if let Some(stroke) = stroke(stroke_color, weight) {
        shapes.push(Shape::ellipse_stroke(center, radius, stroke));
    }
    collapse(shapes)
}

/// Builds an arc.
///
/// `epaint` has no arc, so the sweep is sampled into points. An open arc is a stroked polyline. A
/// pie adds the center, which closes the outline and lets it fill.
#[allow(clippy::too_many_arguments)]
pub(crate) fn arc(
    center: Point<i32>,
    radius: i32,
    start: f32,
    end: f32,
    pie: bool,
    fill_color: Option<Color>,
    stroke_color: Option<Color>,
    weight: u16,
) -> Shape {
    let sweep = (end - start).abs();
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    let steps = ((sweep * ARC_STEPS_PER_RADIAN) as usize).clamp(MIN_CURVE_STEPS, MAX_CURVE_STEPS);
    let origin = pos2(center);
    #[allow(clippy::cast_precision_loss)]
    let radius = radius as f32;

    let mut points = simplify(
        (0..=steps)
            .map(|step| {
                #[allow(clippy::cast_precision_loss)]
                let t = step as f32 / steps as f32;
                let angle = start + (end - start) * t;
                Pos2::new(
                    origin.x + radius * angle.cos(),
                    origin.y + radius * angle.sin(),
                )
            })
            .collect(),
    );

    if !pie {
        return stroke(stroke_color, weight)
            .map_or(Shape::Noop, |stroke| outline(&points, stroke, false));
    }

    points.push(origin);
    let points = simplify(points);
    let mut shapes = Vec::with_capacity(2);
    if let Some(color) = fill_color {
        if encloses_area(&points) {
            if let Some(mesh) = ear_clip(&points, color32(color)) {
                shapes.push(Shape::mesh(mesh));
            }
        }
    }
    if let Some(stroke) = stroke(stroke_color, weight) {
        shapes.push(outline(&points, stroke, true));
    }
    collapse(shapes)
}

/// Builds a textured quad.
///
/// `src` selects a region of an image `size` pixels across and `dst` places it on the target.
/// `angle` is in degrees, clockwise about `center`, defaulting to the center of `dst`. A tint
/// multiplies the sampled color, so `None` draws the image as it is.
#[allow(clippy::too_many_arguments)]
pub(crate) fn textured(
    texture: TextureId,
    size: (u32, u32),
    src: PixRect<i32>,
    dst: PixRect<i32>,
    angle: f64,
    center: Option<Point<i32>>,
    flipped: Option<Flipped>,
    tint: Option<Color>,
) -> Shape {
    #[allow(clippy::cast_precision_loss)]
    let (width, height) = (size.0.max(1) as f32, size.1.max(1) as f32);
    let uv = rect(src);
    let mut left = uv.min.x / width;
    let mut right = uv.max.x / width;
    let mut top = uv.min.y / height;
    let mut bottom = uv.max.y / height;
    if matches!(flipped, Some(Flipped::Horizontal | Flipped::Both)) {
        std::mem::swap(&mut left, &mut right);
    }
    if matches!(flipped, Some(Flipped::Vertical | Flipped::Both)) {
        std::mem::swap(&mut top, &mut bottom);
    }

    let quad = rect(dst);
    let pivot = center.map_or_else(|| quad.center(), pos2);
    #[allow(clippy::cast_possible_truncation)]
    let rotation = egui::emath::Rot2::from_angle(angle.to_radians() as f32);
    let color = tint.map_or(Color32::WHITE, color32);

    let mut mesh = Mesh::with_texture(texture);
    for (corner, uv) in [
        (quad.left_top(), Pos2::new(left, top)),
        (quad.right_top(), Pos2::new(right, top)),
        (quad.right_bottom(), Pos2::new(right, bottom)),
        (quad.left_bottom(), Pos2::new(left, bottom)),
    ] {
        mesh.vertices.push(Vertex {
            pos: pivot + rotation * (corner - pivot),
            uv,
            color,
        });
    }
    mesh.indices.extend_from_slice(&[0, 1, 2, 0, 2, 3]);
    Shape::mesh(mesh)
}

/// Builds an outline as one stroked segment per edge.
///
/// `epaint` joins the segments of a path with a miter, whose length grows without bound as the
/// angle at a corner closes, and the drawing API takes outlines of any shape. Outlines are a pixel
/// wide, where a join covers nothing.
fn outline(points: &[Pos2], stroke: Stroke, closed: bool) -> Shape {
    let count = points.len();
    if count < 2 {
        return Shape::Noop;
    }
    let edges = if closed { count } else { count - 1 };
    collapse(
        (0..edges)
            .map(|index| Shape::line_segment([points[index], points[(index + 1) % count]], stroke))
            .collect(),
    )
}

/// Triangulates a convex outline as a fan.
///
/// A mesh rather than a path, because the tessellator feathers a path's edges and two shapes
/// sharing an edge then blend against each other instead of meeting, leaving a seam.
fn fan(points: &[Pos2], color: Color32) -> Mesh {
    let mut mesh = Mesh::default();
    for point in points {
        mesh.vertices.push(Vertex {
            pos: *point,
            uv: Pos2::ZERO,
            color,
        });
    }
    #[allow(clippy::cast_possible_truncation)]
    for index in 1..points.len().saturating_sub(1) as u32 {
        mesh.indices.extend_from_slice(&[0, index, index + 1]);
    }
    mesh
}

/// Reports whether an outline encloses any area.
///
/// A collapsed outline has no interior, so filling it emits triangles of zero area.
fn encloses_area(points: &[Pos2]) -> bool {
    signed_area(points).abs() > f32::EPSILON
}

/// Drops points that repeat the one before them, and a last point repeating the first.
///
/// A caller can hand over an outline whose points collapse onto each other once rounded to whole
/// pixels. A zero-length edge has no direction, so the tessellator cannot find a normal for it and
/// stroking the outline throws a spike across the target.
fn simplify(mut points: Vec<Pos2>) -> Vec<Pos2> {
    points.dedup();
    if points.len() > 1 && points[0] == points[points.len() - 1] {
        points.pop();
    }
    points
}

/// Reduces a shape list to the cheapest form that draws it.
fn collapse(mut shapes: Vec<Shape>) -> Shape {
    match shapes.len() {
        0 => Shape::Noop,
        1 => shapes.remove(0),
        _ => Shape::Vec(shapes),
    }
}

/// Reports whether an outline turns the same way at every corner.
///
/// A convex outline can go straight to the tessellator. Anything else needs triangulating.
fn is_convex(points: &[Pos2]) -> bool {
    let mut sign = 0.0_f32;
    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        let c = points[(index + 2) % points.len()];
        let cross = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);
        if cross != 0.0 {
            if sign != 0.0 && cross.signum() != sign {
                return false;
            }
            sign = cross.signum();
        }
    }
    true
}

/// Triangulates a simple polygon by clipping ears.
///
/// Returns `None` for an outline with fewer than three points, or one the algorithm cannot reduce,
/// which is the case for a self-intersecting outline.
fn ear_clip(points: &[Pos2], color: Color32) -> Option<Mesh> {
    if points.len() < 3 {
        return None;
    }
    let mut remaining: Vec<usize> = (0..points.len()).collect();
    // Clip in the direction the outline winds, so the ear test agrees with the polygon's interior.
    let winding = signed_area(points).signum();
    let mut mesh = Mesh::default();
    for point in points {
        mesh.vertices.push(Vertex {
            pos: *point,
            uv: Pos2::ZERO,
            color,
        });
    }

    let mut guard = remaining.len() * remaining.len();
    while remaining.len() > 3 {
        guard -= 1;
        if guard == 0 {
            return None;
        }
        let mut clipped = false;
        for slot in 0..remaining.len() {
            let prev = remaining[(slot + remaining.len() - 1) % remaining.len()];
            let current = remaining[slot];
            let next = remaining[(slot + 1) % remaining.len()];
            if !is_ear(points, &remaining, winding, prev, current, next) {
                continue;
            }
            #[allow(clippy::cast_possible_truncation)]
            mesh.indices
                .extend_from_slice(&[prev as u32, current as u32, next as u32]);
            remaining.remove(slot);
            clipped = true;
            break;
        }
        if !clipped {
            return None;
        }
    }
    #[allow(clippy::cast_possible_truncation)]
    mesh.indices.extend_from_slice(&[
        remaining[0] as u32,
        remaining[1] as u32,
        remaining[2] as u32,
    ]);
    Some(mesh)
}

/// Reports whether a corner can be clipped off without cutting through the polygon.
fn is_ear(
    points: &[Pos2],
    remaining: &[usize],
    winding: f32,
    prev: usize,
    current: usize,
    next: usize,
) -> bool {
    let (a, b, c) = (points[prev], points[current], points[next]);
    let cross = (b.x - a.x) * (c.y - b.y) - (b.y - a.y) * (c.x - b.x);
    if cross == 0.0 || cross.signum() != winding {
        return false;
    }
    // A corner with another vertex inside it would cut across the polygon if clipped.
    !remaining
        .iter()
        .filter(|index| **index != prev && **index != current && **index != next)
        .any(|index| in_triangle(points[*index], a, b, c))
}

/// Returns twice the signed area of an outline, whose sign gives the winding direction.
fn signed_area(points: &[Pos2]) -> f32 {
    let mut area = 0.0;
    for index in 0..points.len() {
        let a = points[index];
        let b = points[(index + 1) % points.len()];
        area += a.x * b.y - b.x * a.y;
    }
    area
}

/// Reports whether a point falls inside a triangle.
fn in_triangle(p: Pos2, a: Pos2, b: Pos2, c: Pos2) -> bool {
    let side = |p: Pos2, q: Pos2, r: Pos2| (q.x - p.x) * (r.y - p.y) - (q.y - p.y) * (r.x - p.x);
    let d1 = side(a, b, p);
    let d2 = side(b, c, p);
    let d3 = side(c, a, p);
    let negative = d1 < 0.0 || d2 < 0.0 || d3 < 0.0;
    let positive = d1 > 0.0 || d2 > 0.0 || d3 > 0.0;
    !(negative && positive)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn square() -> Vec<Pos2> {
        vec![
            Pos2::new(0.0, 0.0),
            Pos2::new(10.0, 0.0),
            Pos2::new(10.0, 10.0),
            Pos2::new(0.0, 10.0),
        ]
    }

    /// An L shape: the notch makes one corner reflex.
    fn concave() -> Vec<Pos2> {
        vec![
            Pos2::new(0.0, 0.0),
            Pos2::new(10.0, 0.0),
            Pos2::new(10.0, 4.0),
            Pos2::new(4.0, 4.0),
            Pos2::new(4.0, 10.0),
            Pos2::new(0.0, 10.0),
        ]
    }

    #[test]
    fn convexity_separates_the_two_fill_paths() {
        assert!(is_convex(&square()));
        assert!(!is_convex(&concave()));
    }

    #[test]
    fn ear_clipping_covers_a_concave_outline() {
        let Some(mesh) = ear_clip(&concave(), Color32::WHITE) else {
            panic!("an L shape reduces to triangles");
        };
        // Any simple polygon reduces to two fewer triangles than it has corners.
        assert_eq!(mesh.indices.len(), (concave().len() - 2) * 3);
        assert_eq!(mesh.vertices.len(), concave().len());
    }

    #[test]
    fn ear_clipping_refuses_an_outline_it_cannot_reduce() {
        let too_few = vec![Pos2::new(0.0, 0.0), Pos2::new(1.0, 1.0)];
        assert!(ear_clip(&too_few, Color32::WHITE).is_none());
    }

    #[test]
    fn a_bezier_starts_and_ends_on_its_outer_control_points() {
        let control = [
            Pos2::new(0.0, 0.0),
            Pos2::new(10.0, 20.0),
            Pos2::new(20.0, 0.0),
        ];
        assert_eq!(de_casteljau(&control, 0.0), control[0]);
        assert_eq!(de_casteljau(&control, 1.0), control[2]);
    }

    #[test]
    fn an_arc_samples_more_segments_the_further_it_sweeps() {
        let quarter = arc(
            Point::new([0, 0]),
            50,
            0.0,
            std::f32::consts::FRAC_PI_2,
            false,
            None,
            Some(Color::WHITE),
            1,
        );
        let full = arc(
            Point::new([0, 0]),
            50,
            0.0,
            std::f32::consts::TAU,
            false,
            None,
            Some(Color::WHITE),
            1,
        );
        let count = |shape: &Shape| match shape {
            Shape::Vec(segments) => segments.len(),
            _ => 0,
        };
        assert!(count(&full) > count(&quarter));
    }
}
