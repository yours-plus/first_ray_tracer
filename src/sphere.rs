use super::hitable::*;
use super::ray::*;
use super::vec3d::*;

pub struct Sphere {
    pub center: Vec3D,
    pub radius: f64,
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let oc = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = dot(&oc, &ray.direction);
        let c = dot(&oc, &oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        if discriminant > 0.0 {
            let mut t = (-b - discriminant.sqrt()) / a;
            if t < t_min || t > t_max {
                t = (-b + discriminant.sqrt()) / a;
            }

            if t_min <= t && t <= t_max {
                let p = ray.at(t);
                return HitRecord::Hit {
                    t: t,
                    point: p,
                    normal: (1.0 / self.radius) * (p - self.center),
                };
            }
        }
        HitRecord::Nothing
    }
}
