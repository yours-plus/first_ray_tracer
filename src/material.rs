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
    pub fuzzy: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, point: Vec3D, normal: Vec3D) -> ScatterRecord {
        let reflected = reflect(normalize(&ray.direction), normal);

        let scattered = Ray {
            origin: point,
            direction: reflected + self.fuzzy * random_in_unit_sphere(),
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

fn refract(v: Vec3D, n: Vec3D, ni_over_nt: f64) -> Option<Vec3D> {
    let uv = normalize(&v);
    let dt = dot(&uv, &n);
    let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - dt * n) - discriminant.sqrt() * n)
    } else {
        None
    }
}

fn schlick(cosine: f64, refractive_index: f64) -> f64 {
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powf(2.0);
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.)
}

pub struct Dielectric {
    pub refractive_index: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, point: Vec3D, normal: Vec3D) -> ScatterRecord {
        let reflected = reflect(ray.direction, normal);
        let mut outward_normal = normal;
        let mut ni_over_nt = 1.0 / self.refractive_index;
        let mut cosine = -dot(&ray.direction, &normal) / ray.direction.norm();

        if dot(&ray.direction, &normal) > 0.0 {
            outward_normal = -1.0 * normal;
            ni_over_nt = self.refractive_index;
            cosine = self.refractive_index * dot(&ray.direction, &normal) / ray.direction.norm();
        }

        let mut scattered = Ray {
            origin: point,
            direction: reflected,
        };

        if let Some(refracted) = refract(ray.direction, outward_normal, ni_over_nt) {
            let reflect_prob = schlick(cosine, self.refractive_index);
            // 超えたら屈折
            if rand::random::<f64>() > reflect_prob {
                scattered = Ray {
                    origin: point,
                    direction: refracted,
                };
            }
        }

        ScatterRecord::Scatter {
            scattered: scattered,
            attenuation: new_vec3d(1., 1., 1.),
        }
    }
}
