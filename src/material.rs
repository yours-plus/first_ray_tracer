use super::ray::*;
use super::util::*;
use super::vec3d::*;

#[derive(Debug)]
pub enum ScatterRecord {
    Absorption,
    Scatter { scattered: Ray, attenuation: Vec3D },
}

pub trait Material {
    fn scatter(&self, ray: &Ray, point: Vec3D, normal: Vec3D) -> ScatterRecord;
}

pub struct Lambertian {
    pub albedo: Vec3D, // 入射光に対する反射光の比
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, point: Vec3D, normal: Vec3D) -> ScatterRecord {
        let target = point + normal + random_in_unit_sphere();
        let scattered = Ray {
            origin: point,
            direction: target - point,
        };
        let attenuation = self.albedo;

        ScatterRecord::Scatter {
            scattered,
            attenuation,
        }
    }
}
