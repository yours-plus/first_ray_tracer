use super::ray::*;
use super::vec3d::*;

#[derive(Debug)]
pub enum HitRecord {
    Nothing,
    Hit { t: f64, point: Vec3D, normal: Vec3D },
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord;
}

impl<T: Hitable> Hitable for [T] {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> HitRecord {
        let mut closest_so_far = t_max;
        let mut result = HitRecord::Nothing;

        for i in 0..self.len() {
            let record = self[i].hit(ray, t_min, closest_so_far);
            if let HitRecord::Hit { t, .. } = record {
                if t < closest_so_far {
                    result = record;
                    closest_so_far = t;
                }
            }
        }
        result
    }
}
