use super::sphere::*;
use super::vec3d::*;

pub struct Ray {
    pub origin: Vec3D,
    pub direction: Vec3D,
}

impl Ray {
    // origin + t * direction
    pub fn at(&self, t: f64) -> Vec3D {
        self.origin + t * self.direction
    }

    pub fn hit_sphere(&self, sphere: &Sphere) -> f64 {
        // a t^2 + b t + c = 0
        let oc = self.origin - sphere.center;
        let a = dot(&self.direction, &self.direction);
        let b = 2.0 * dot(&oc, &self.direction);
        let c = dot(&oc, &oc) - sphere.radius * sphere.radius;
        let discrimination = b * b - 4.0 * a * c;

        if discrimination < 0.0 {
            0.0
        } else {
            (-b - discrimination.sqrt()) / (2.0 * a)
        }
    }
}
