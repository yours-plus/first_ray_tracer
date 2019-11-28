use super::hitable::*;
use super::material::*;
use super::ray::*;
use super::vec3d::*;

pub struct Sphere<'a> {
    pub center: Vec3D,
    pub radius: f64,
    pub material: &'a dyn Material,
}

impl<'a> Hitable for Sphere<'a> {
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
                    normal: (p - self.center) / self.radius,
                    material: self.material,
                };
            }
        }
        HitRecord::Nothing
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nearly_equals(a: f64, b: f64) -> bool {
        const EPS: f64 = 1e-7;
        (a - b).abs() < EPS
    }

    #[test]
    fn test1() {
        /*
        let sphere = Sphere {
            center: new_vec3d(0., 0., 10.),
            radius: 1.0,
        };
        let ray = Ray {
            origin: new_vec3d(0., 0., 0.),
            direction: new_vec3d(0., 0., 1.),
        };
        let record = sphere.hit(&ray, 0., 100.0);
        println!("{:?}", record);
        */
    }
}
