pub mod camera;
pub mod hitable;
pub mod material;
pub mod ray;
pub mod sphere;
pub mod util;
pub mod vec3d;

use camera::*;
use hitable::*;
use material::*;
use ray::*;
use sphere::*;
use vec3d::*;

type Color = Vec3D;

fn init_ppm_format(width: i32, height: i32) {
    println!("P3");
    println!("{} {}", width, height);
    println!("255");
}

fn compute_color<T: Hitable + ?Sized>(ray: &Ray, world: &T, depth: i32) -> Color {
    match world.hit(&ray, 0.001, 1e50) {
        HitRecord::Nothing => {
            let unit_direction = normalize(&ray.direction);
            let t = 0.5 * (unit_direction.y() + 1.);

            (1. - t) * new_vec3d(1., 1., 1.) + t * new_vec3d(0.5, 0.7, 1.)
        }
        HitRecord::Hit {
            t: _,
            point: p,
            normal: n,
            material: m,
        } => {
            if depth >= 50 {
                return new_vec3d(0., 0., 0.);
            }
            // scatterdで分岐
            match m.scatter(&ray, p, n) {
                ScatterRecord::Absorption => new_vec3d(0., 0., 0.),
                ScatterRecord::Scatter {
                    scattered: s,
                    attenuation: a,
                } => a * compute_color(&s, world, depth + 1),
            }
        }
    }
}

// 1/2 乗する
fn linear_to_gamma(v: &Vec3D) -> Vec3D {
    new_vec3d(v.x().sqrt(), v.y().sqrt(), v.z().sqrt())
}

// Output sample image in PPM format.
fn main() {
    const WIDTH: i32 = 200;
    const HEIGHT: i32 = 100;

    init_ppm_format(WIDTH, HEIGHT);

    let camera = Camera::initial_camera();

    let world = [
        Sphere {
            center: new_vec3d(0., 0., -1.),
            radius: 0.5,
            material: &Lambertian {
                albedo: new_vec3d(0.8, 0.3, 0.3),
            },
        },
        Sphere {
            center: new_vec3d(0., -100.5, -1.),
            radius: 100.,
            material: &Lambertian {
                albedo: new_vec3d(0.8, 0.8, 0.0),
            },
        },
        Sphere {
            center: new_vec3d(1., 0., -1.),
            radius: 0.5,
            material: &Metal {
                albedo: new_vec3d(0.8, 0.6, 0.2),
                fuzzy: 1.0,
            },
        },
        Sphere {
            center: new_vec3d(-1., 0., -1.),
            radius: 0.5,
            material: &Metal {
                albedo: new_vec3d(0.8, 0.8, 0.8),
                fuzzy: 0.3,
            },
        },
    ];

    const NUM_SAMPLES: i32 = 100;

    for y in (0..HEIGHT).rev() {
        eprintln!("y: {}", y);

        for x in 0..WIDTH {
            let mut color = new_vec3d(0., 0., 0.);

            for _ in 0..NUM_SAMPLES {
                let u = (x as f64 + rand::random::<f64>()) / WIDTH as f64;
                let v = (y as f64 + rand::random::<f64>()) / HEIGHT as f64;
                let ray = camera.get_ray(u, v);

                color = color + compute_color(&ray, &world[..], 0);
            }

            color = color / (NUM_SAMPLES as f64);
            color = linear_to_gamma(&color);

            let ir = (255.99 * color.elements[0]) as i32;
            let ig = (255.99 * color.elements[1]) as i32;
            let ib = (255.99 * color.elements[2]) as i32;

            println!("{} {} {}", ir, ig, ib);
        }
    }
}
