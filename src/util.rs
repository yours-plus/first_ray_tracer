use super::vec3d::*;

pub fn random_in_unit_sphere() -> Vec3D {
    loop {
        let p =
            2.0 * new_vec3d(rand::random(), rand::random(), rand::random()) - new_vec3d(1., 1., 1.);
        if p.norm() <= 1.0 {
            return p;
        }
    }
}
