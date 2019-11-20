use super::ray::*;
use super::vec3d::*;

pub enum HitRecord {
    Nothing,
    Hit { t: f64, point: Vec3D, normal: Vec3D },
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}
