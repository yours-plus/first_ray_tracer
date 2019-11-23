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
        Camera {
            lower_left_corner: new_vec3d(-2., -1., -1.),
            horizontal: new_vec3d(4., 0., 0.),
            vertical: new_vec3d(0., 2., 0.),
            origin: new_vec3d(0., 0., 0.),
        }
    }
}
