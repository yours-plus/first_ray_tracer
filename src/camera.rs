use super::ray::*;
use super::vec3d::*;

pub struct Camera {
    lower_left_corner: Vec3D,
    horizontal: Vec3D,
    vertical: Vec3D,
    origin: Vec3D,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical
                - self.origin,
        }
    }

    pub fn initial_camera() -> Camera {
        /*
        Camera {
            lower_left_corner: new_vec3d(-2., -1., -1.),
            horizontal: new_vec3d(4., 0., 0.),
            vertical: new_vec3d(0., 2., 0.),
            origin: new_vec3d(0., 0., 0.),
        }
        */
        Camera::new(
            new_vec3d(-0.5, 0.5, 1.),
            new_vec3d(0., 0., -1.),
            new_vec3d(0., 1., 0.),
            90.,
            2.,
        )
    }

    pub fn new(lookfrom: Vec3D, lookat: Vec3D, vup: Vec3D, vfov: f64, aspect: f64) -> Camera {
        let theta = vfov * std::f64::consts::PI / 180.0;
        let half_height = (theta / 2.0).tan();
        let half_width = aspect * half_height;
        let w = normalize(&(lookfrom - lookat));
        let u = normalize(&cross(&vup, &w));
        let v = cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width * u - half_height * v - w,
            horizontal: 2.0 * half_width * u,
            vertical: 2.0 * half_height * v,
        }
    }
}
