use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vec3D {
    pub elements: [f64; 3],
}

impl Vec3D {
    pub fn norm(&self) -> f64 {
        self.elements.iter().fold(0., |sum, e| sum + e * e).sqrt()
    }

    pub fn x(&self) -> f64 {
        self.elements[0]
    }

    pub fn y(&self) -> f64 {
        self.elements[1]
    }

    pub fn z(&self) -> f64 {
        self.elements[2]
    }
}

impl Add for Vec3D {
    type Output = Vec3D;

    fn add(self, other: Vec3D) -> Vec3D {
        let mut elements = [0.0; 3];
        for i in 0..3 {
            elements[i] = self.elements[i] + other.elements[i];
        }
        Vec3D { elements }
    }
}

impl Sub for Vec3D {
    type Output = Vec3D;

    fn sub(self, other: Vec3D) -> Vec3D {
        let mut elements = [0.0; 3];
        for i in 0..3 {
            elements[i] = self.elements[i] - other.elements[i];
        }
        Vec3D { elements }
    }
}

impl Mul<Vec3D> for f64 {
    type Output = Vec3D;

    fn mul(self, other: Vec3D) -> Vec3D {
        let mut elements = [0.0; 3];
        for i in 0..3 {
            elements[i] = other.elements[i] * self;
        }
        Vec3D { elements }
    }
}

impl Div<f64> for Vec3D {
    type Output = Vec3D;

    fn div(self, other: f64) -> Vec3D {
        (1. / other) * self
    }
}

pub fn new_vec3d(x: f64, y: f64, z: f64) -> Vec3D {
    Vec3D {
        elements: [x, y, z],
    }
}

pub fn normalize(v: &Vec3D) -> Vec3D {
    (1. / v.norm()) * *v
}

pub fn dot(v1: &Vec3D, v2: &Vec3D) -> f64 {
    let mut res = 0.0;
    for i in 0..3 {
        res += v1.elements[i] * v2.elements[i];
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    fn nearly_equals(a: f64, b: f64) -> bool {
        const EPS: f64 = 1e-7;
        (a - b).abs() < EPS
    }

    fn test_add() {}

    #[test]
    fn test_mul() {
        assert_eq!(
            2.,
            (2.0 * Vec3D {
                elements: [1., 0., 0.]
            })
            .elements[0]
        );
    }

    fn test_new_vec3d() {}

    #[test]
    fn test_norm() {
        assert_eq!(1., new_vec3d(0., 1., 0.).norm());
        assert_eq!(2., new_vec3d(0., 2., 0.).norm());
    }

    #[test]
    fn test_vec() {
        assert_eq!(
            4.,
            Vec3D {
                elements: [0., 4., 1.]
            }
            .elements[1]
        );
    }
}
