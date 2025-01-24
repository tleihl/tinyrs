use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct VecUV2f {
    pub u: f64,
    pub v: f64,
}

impl VecUV2f {
    pub fn new(u: f64, v: f64) -> VecUV2f {
        VecUV2f { u, v }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3f {
        Vec3f { x, y, z }
    }

    pub fn dot(&self, other: &Vec3f) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn norm(&self) -> f64 {
        self.dot(self).sqrt()
    }

    pub fn normalize(&self) -> Vec3f {
        let inv_norm = 1.0 / self.norm();
        Vec3f::new(self.x * inv_norm, self.y * inv_norm, self.z * inv_norm)
    }

    pub fn cross(&self, other: &Vec3f) -> Vec3f {
        Vec3f::new(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x,
        )
    }
}

impl Mul<f64> for Vec3f {
    type Output = Vec3f;
    fn mul(self, mul: f64) -> Vec3f {
        Vec3f::new(self.x * mul, self.y * mul, self.z * mul)
    }
}

impl Add<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn add(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub<Vec3f> for Vec3f {
    type Output = Vec3f;
    fn sub(self, other: Vec3f) -> Vec3f {
        Vec3f::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    p1: Vec3f,
    p2: Vec3f,
    p3: Vec3f,

    v0: Vec3f,
    v1: Vec3f,

    d00: f64,
    d01: f64,
    d11: f64,

    det: f64,
}

impl Triangle {
    pub fn new(p1: Vec3f, p2: Vec3f, p3: Vec3f) -> Triangle {
        let v0 = p2 - p1;
        let v1 = p3 - p1;

        let d00 = v0.dot(&v0);
        let d01 = v0.dot(&v1);
        let d11 = v1.dot(&v1);

        let det = d00 * d11 - d01 * d01;

        Triangle { p1, p2, p3, v0, v1, d00, d01, d11, det }
    }

    pub fn barycentric(&self, p: Vec3f) -> Option<[f64; 3]> {
        let v2 = p - self.p1;

        let d02 = self.v0.dot(&v2);
        let d12 = self.v1.dot(&v2);

        let v = (d02 * self.d11 - self.d01 * d12) / self.det;

        if v.is_nan() || v < 0.0 || v > 1.0 {
            return None;
        }

        let w = (self.d00 * d12 - d02 * self.d01) / self.det;

        if w.is_nan() || w < 0.0 || w > 1.0 {
            return None;
        }

        Some([1.0 - v - w, v, w])
    }

    pub fn vertices(&self) -> [Vec3f; 3] {
        [self.p1, self.p2, self.p3]
    }
}