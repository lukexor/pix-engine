use pix_engine::prelude::*;

const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;

struct SphereObj {
    sphere: Sphere<f64>,
    color: Color,
    specular: Option<i32>,
    reflective: f64,
}

struct App {
    origin: Point<f64, 3>,
    looking: Point<f64, 3>,
    width: f64,
    height: f64,
    view_width: f64,
    view_height: f64,
    proj_plane_dist: f64,
    spheres: [SphereObj; 4],
    lights: [Light<f64>; 3],
}

fn intersect_ray_sphere(
    origin: Point<f64, 3>,
    direction: Vector<f64, 3>,
    obj: &SphereObj,
) -> (f64, f64) {
    let r = obj.sphere.radius();
    let center_origin = origin - obj.sphere.center();

    let a = direction.mag_sq();
    let b = 2.0 * center_origin.dot(direction);
    let c = center_origin.mag_sq() - (r * r);

    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return (f64::INFINITY, f64::INFINITY);
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
                color: Color::RED,
                specular: Some(500),
                reflective: 0.2,
            },
            SphereObj {
                sphere: sphere!([2.0, 0.0, 4.0], 1.0),
                color: Color::BLUE,
                specular: Some(500),
                reflective: 0.3,
            },
            SphereObj {
                sphere: sphere!([-2.0, 0.0, 4.0], 1.0),
                color: Color::GREEN,
                specular: Some(10),
                reflective: 0.4,
            },
            SphereObj {
                sphere: sphere!([0.0, -1001.0, 0.0], 1000.0),
                color: Color::YELLOW,
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
            origin: point!(0.0, 0.0, -6.0),
            looking: point!(0.0, 0.0, 0.0),
            width: WIDTH as f64,
            height: HEIGHT as f64,
            view_width: 1.0,
            view_height: 1.0,
            proj_plane_dist: 1.0,
            spheres,
            lights,
        }
    }

    fn canvas_to_viewport(&self, x: i32, y: i32) -> Point<f64, 3> {
        point!(
            x as f64 * self.view_width / self.width,
            y as f64 * self.view_height / self.height,
            self.proj_plane_dist
        )
    }

    fn canvas_to_screen(&self, x: i32, y: i32) -> Point<i32> {
        let x = self.width / 2.0 + x as f64;
        let y = self.height / 2.0 - y as f64;
        point!(x, y).round().as_::<i32>()
    }

    fn compute_lighting(
        &self,
        position: Point<f64, 3>,
        normal: Vector<f64, 3>,
        camera: Vector<f64, 3>,
        specular: Option<i32>,
    ) -> f64 {
        let mut intensity = 0.0;
        for light in &self.lights {
            match light.source {
                LightSource::Ambient => intensity += light.intensity,
                _ => {
                    let (light_dir, t_max) = match light.source {
                        LightSource::Point(p) => (p - position, 1.0),
                        LightSource::Direction(d) => (d, f64::INFINITY),
                        _ => unreachable!("invalid light source"),
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
        origin: Point<f64, 3>,
        direction: Vector<f64, 3>,
        t_min: f64,
        t_max: f64,
        recurse_depth: isize,
    ) -> Color {
        let (closest_t, closest_sphere) =
            self.closest_intersection(origin, direction, t_min, t_max);

        if let Some(obj) = closest_sphere {
            // Local color
            let intersection = origin + closest_t * direction;
            let normal = Vector::normalized(intersection - obj.sphere.center());
            let local_color =
                obj.color * self.compute_lighting(intersection, normal, -direction, obj.specular);

            let r = obj.reflective;
            if recurse_depth <= 0 || r <= 0.0 {
                return local_color;
            }

            // Compute reflection
            let r_dir = Vector::reflection(-direction, normal);
            let reflected_color =
                self.trace_ray(intersection, r_dir, 0.001, f64::INFINITY, recurse_depth - 1);

            local_color * (1.0 - r) + reflected_color * r
        } else {
            Color::BLACK
        }
    }

    fn closest_intersection(
        &self,
        origin: Point<f64, 3>,
        direction: Vector<f64, 3>,
        t_min: f64,
        t_max: f64,
    ) -> (f64, Option<&SphereObj>) {
        let mut closest_t = f64::INFINITY;
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

impl PixEngine for App {
    fn on_update(&mut self, s: &mut PixState) -> PixResult<()> {
        let half_w = s.width()? as i32 / 2;
        let half_h = s.height()? as i32 / 2;
        for y in -half_h..=half_h {
            for x in -half_w..=half_w {
                let direction: Point<f64, 3> = self.canvas_to_viewport(x, y);
                let color =
                    self.trace_ray(self.origin, direction - self.looking, 1.0, f64::INFINITY, 3);
                s.stroke(color);
                s.point(self.canvas_to_screen(x, y))?;
            }
        }
        Ok(())
    }

    fn on_key_pressed(&mut self, _s: &mut PixState, event: KeyEvent) -> PixResult<bool> {
        match (event.key, event.keymod) {
            // Move left/right
            (Key::Left, KeyMod::NONE) => self.origin -= point!(1.0, 0.0, 0.0) - self.looking,
            (Key::Right, KeyMod::NONE) => self.origin += point!(1.0, 0.0, 0.0) - self.looking,
            // Move forward/back
            (Key::Up, KeyMod::NONE) => self.origin += point!(0.0, 0.0, 1.0) - self.looking,
            (Key::Down, KeyMod::NONE) => self.origin -= point!(0.0, 0.0, 1.0) - self.looking,
            // Move up/down
            (Key::Up, KeyMod::SHIFT) => self.origin += point!(0.0, 1.0, 0.0) - self.looking,
            (Key::Down, KeyMod::SHIFT) => self.origin -= point!(0.0, 1.0, 0.0) - self.looking,
            // Look left/right
            (Key::Left, KeyMod::GUI) => self.looking.offset(point!(0.05, 0.0, 0.0)),
            (Key::Right, KeyMod::GUI) => self.looking.offset(-point!(0.05, 0.0, 0.0)),
            // Look up/down
            (Key::Up, KeyMod::GUI) => self.looking.offset(-point!(0.0, 0.05, 0.0)),
            (Key::Down, KeyMod::GUI) => self.looking.offset(point!(0.0, 0.05, 0.0)),
            _ => (),
        }
        Ok(false)
    }
}

pub fn main() -> PixResult<()> {
    let mut engine = Engine::builder()
        .dimensions(WIDTH, HEIGHT)
        .title("3D Raytracing")
        .show_frame_rate()
        .build()?;
    let mut app = App::new();
    engine.run(&mut app)
}
