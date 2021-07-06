use pix_engine::prelude::*;

const TITLE: &str = "Example App";
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct SphereObj {
    sphere: Sphere,
    color: Color,
    specular: Option<i32>,
    reflective: Scalar,
}

struct App {
    view_width: Scalar,
    view_height: Scalar,
    proj_plane_dist: Scalar,
    spheres: [SphereObj; 4],
    lights: [Light; 3],
}

fn intersect_ray_sphere(origin: Point, direction: Vector, obj: &SphereObj) -> (Scalar, Scalar) {
    let r = obj.sphere.radius;
    let center_origin = origin - obj.sphere.center;

    let a = direction.mag_sq();
    let b = 2.0 * center_origin.dot(direction);
    let c = center_origin.mag_sq() - (r * r);

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (Scalar::INFINITY, Scalar::INFINITY);
    }

    let sqrt = discriminant.sqrt();
    let two_a = 2.0 * a;
    let t1 = (-b + sqrt) / two_a;
    let t2 = (-b - sqrt) / two_a;
    (t1, t2)
}

impl App {
    fn new() -> Self {
        let spheres = [
            SphereObj {
                sphere: sphere!([0.0, -1.0, 3.0], 1.0),
                color: RED,
                specular: Some(500),
                reflective: 0.2,
            },
            SphereObj {
                sphere: sphere!([2.0, 0.0, 4.0], 1.0),
                color: BLUE,
                specular: Some(500),
                reflective: 0.3,
            },
            SphereObj {
                sphere: sphere!([-2.0, 0.0, 4.0], 1.0),
                color: GREEN,
                specular: Some(10),
                reflective: 0.4,
            },
            SphereObj {
                sphere: sphere!([0.0, -5001.0, 0.0], 5000.0),
                color: YELLOW,
                specular: Some(1000),
                reflective: 0.5,
            },
        ];
        let lights = [
            Light::ambient(0.2),
            Light::point(0.6, [2.0, 1.0, 0.0]),
            Light::direction(0.2, [1.0, 4.0, 4.0]),
        ];
        Self {
            view_width: 1.0,
            view_height: 1.0,
            proj_plane_dist: 1.0,
            spheres,
            lights,
        }
    }

    fn canvas_to_viewport(&self, x: i32, y: i32, s: &PixState) -> Point {
        let width = s.width() as Scalar;
        let height = s.height() as Scalar;
        point!(
            x as Scalar * self.view_width / width,
            y as Scalar * self.view_height / height,
            self.proj_plane_dist
        )
    }

    fn canvas_to_screen(&self, x: i32, y: i32, s: &PixState) -> Point {
        let width = s.width() as Scalar;
        let height = s.height() as Scalar;
        point!(
            (width / 2.0 + x as Scalar).round(),
            (height / 2.0 - y as Scalar).round(),
            self.proj_plane_dist.round()
        )
    }

    fn compute_lighting(
        &self,
        position: Point,
        normal: Vector,
        camera: Vector,
        specular: Option<i32>,
    ) -> Scalar {
        let mut intensity = 0.0;
        for light in &self.lights {
            match light.source {
                LightSource::Ambient => intensity += light.intensity,
                _ => {
                    let (light_dir, t_max) = match light.source {
                        LightSource::Point(p) => (p - position, 1.0),
                        LightSource::Direction(d) => (d, Scalar::INFINITY),
                        _ => unreachable!("unreachable arm"),
                    };

                    // Shadows
                    let (_, shadow_sphere) =
                        self.closest_intersection(position, light_dir, 0.001, t_max);
                    if shadow_sphere.is_some() {
                        continue;
                    }

                    // Diffuse
                    let normal_dot_dir = normal.dot(light_dir);
                    if normal_dot_dir > 0.0 {
                        intensity +=
                            light.intensity * normal_dot_dir / (normal.mag() * light_dir.mag());
                    }

                    // Specular
                    if let Some(s) = specular {
                        let r = Vector::reflection(light_dir, normal);
                        let r_dot_camera = r.dot(camera);
                        if r_dot_camera > 0.0 {
                            intensity +=
                                light.intensity * (r_dot_camera / (r.mag() * camera.mag())).powi(s);
                        }
                    }
                }
            }
        }
        intensity
    }

    fn trace_ray(
        &self,
        origin: Point,
        direction: Vector,
        t_min: Scalar,
        t_max: Scalar,
        recurse_depth: isize,
    ) -> Color {
        let (closest_t, closest_sphere) =
            self.closest_intersection(origin, direction, t_min, t_max);

        if let Some(obj) = closest_sphere {
            // Local color
            let intersection = origin + closest_t * direction;
            let normal = Vector::normalized(intersection - obj.sphere.center);
            let local_color =
                obj.color * self.compute_lighting(intersection, normal, -direction, obj.specular);

            let r = obj.reflective;
            if recurse_depth <= 0 || r <= 0.0 {
                return local_color;
            }

            // Compute reflection
            let r_dir = Vector::reflection(-direction, normal);
            let reflected_color = self.trace_ray(
                intersection,
                r_dir,
                0.001,
                Scalar::INFINITY,
                recurse_depth - 1,
            );

            local_color * (1.0 - r) + reflected_color * r
        } else {
            BLACK
        }
    }

    fn closest_intersection(
        &self,
        origin: Point,
        direction: Vector,
        t_min: Scalar,
        t_max: Scalar,
    ) -> (Scalar, Option<&SphereObj>) {
        let mut closest_t = Scalar::INFINITY;
        let mut closest_sphere = None;
        for sphere in &self.spheres {
            let (t1, t2) = intersect_ray_sphere(origin, direction, sphere);
            if (t_min..t_max).contains(&t1) && t1 < closest_t {
                closest_t = t1;
                closest_sphere = Some(sphere);
            }
            if (t_min..t_max).contains(&t2) && t2 < closest_t {
                closest_t = t2;
                closest_sphere = Some(sphere);
            }
        }
        (closest_t, closest_sphere)
    }
}

impl AppState for App {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        s.fill(BLACK);
        let origin = point!(0.0, 0.0, -2.0);
        let half_w = s.width() as i32 / 2;
        let half_h = s.height() as i32 / 2;
        for x in -half_w..=half_w {
            for y in -half_h..=half_h {
                let direction = self.canvas_to_viewport(x, y, s).into();
                let color = self.trace_ray(origin, direction, 1.0, Scalar::INFINITY, 3);
                s.stroke(color);
                s.point(self.canvas_to_screen(x, y, s))?;
            }
        }
        s.pause(true);
        Ok(())
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = PixEngine::builder()
        .with_dimensions(WIDTH, HEIGHT)
        .with_title(TITLE)
        .with_frame_rate()
        .position_centered()
        .build();
    let mut app = App::new();
    engine.run(&mut app)
}
