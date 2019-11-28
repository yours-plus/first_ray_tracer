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

fn reflect(v: Vec3D, n: Vec3D) -> Vec3D {
    v - 2. * dot(&v, &n) * n
}

pub struct Metal {
    pub albedo: Vec3D, // 入射光に対する反射光の比
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, point: Vec3D, normal: Vec3D) -> ScatterRecord {
        let reflected = reflect(normalize(&ray.direction), normal);

        let scattered = Ray {
            origin: point,
            direction: reflected,
        };
        let attenuation = self.albedo;

        if dot(&scattered.direction, &normal) > 0.0 {
            ScatterRecord::Scatter {
                scattered,
                attenuation,
            }
        } else {
            ScatterRecord::Absorption
        }
    }
}
