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
}
