pub mod camera;
pub mod hitable;
pub mod ray;
pub mod sphere;
pub mod vec3d;

use camera::*;
use hitable::*;
use ray::*;
use sphere::*;
use vec3d::*;

type Color = Vec3D;

fn init_ppm_format(width: i32, height: i32) {
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
}

fn compute_color(ray: &Ray) -> Color {
    let world = &[
        Sphere {
            center: new_vec3d(0., 0., -1.),
            radius: 0.5,
        },
        Sphere {
            center: new_vec3d(0., -100.5, -1.),
            radius: 100.,
        },
    ];

    match world.hit(&ray, 0.0, 1e50) {
        HitRecord::Nothing => {
            let unit_direction = normalize(&ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.);

            (1. - t) * new_vec3d(1., 1., 1.) + t * new_vec3d(0.5, 0.7, 1.)
        }
        HitRecord::Hit {
            t: _,
            point: _,
            normal: n,
        } => {
            return 0.5 * new_vec3d(n.x() + 1., n.y() + 1., n.z() + 1.);
        }
    }
}

// Output sample image in PPM format.
fn main() {
    const WIDTH: i32 = 200;
    const HEIGHT: i32 = 100;

    init_ppm_format(WIDTH, HEIGHT);

    let camera = Camera::initial_camera();

    for y in (0..HEIGHT).rev() {
        for x in 0..WIDTH {
            let u = x as f64 / WIDTH as f64;
            let v = y as f64 / HEIGHT as f64;
            let ray = camera.get_ray(u, v);
            let color = compute_color(&ray);

            let ir = (255.99 * color.elements[0]) as i32;
            let ig = (255.99 * color.elements[1]) as i32;
            let ib = (255.99 * color.elements[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
