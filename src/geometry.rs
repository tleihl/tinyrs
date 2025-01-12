use std::ops::{Add, Mul};
use sdl2::rect::Point;

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

    pub fn normalize(&self) -> Vec3f {
        let norm = f64::sqrt(self.dot(self));
        Vec3f::new(self.x / norm, self.y / norm, self.z / norm)
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

#[derive(Copy, Clone, Debug)]
pub struct Vec2i {
    x: i32,
    y: i32,
}

impl Vec2i {
    pub fn new(p1: Point, p2: Point) -> Vec2i {
        Vec2i {
            x: p2.x - p1.x,
            y: p2.y - p1.y
        }
    }

    pub fn cross(&self, other: &Vec2i) -> i32 {
        self.x * other.y - self.y * other.x
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Triangle {
    p1: Point,
    p2: Point,
    p3: Point,
}

impl Triangle {
    pub fn new(p1: Point, p2: Point, p3: Point) -> Triangle {
        Triangle { p1, p2, p3 }
    }

    pub fn from([p1, p2, p3]: [Point; 3]) -> Triangle {
        Triangle { p1, p2, p3 }
    }

    pub fn barycentric(&self, p: Point) -> Option<[f64; 3]> {
        let v12 = Vec2i::new(self.p1, self.p2);
        let v13 = Vec2i::new(self.p1, self.p3);

        let s = v12.cross(&v13);
        if s == 0 {
            return None;
        };

        let v1p = Vec2i::new(p, self.p1);
        let v2p = Vec2i::new(p, self.p2);
        let v3p = Vec2i::new(p, self.p3);

        let s1 = v2p.cross(&v3p);
        let s2 = v3p.cross(&v1p);
        let s3 = v1p.cross(&v2p);

        let g1 = s1 as f64 / s as f64;
        if g1 < 0.0 || g1 > 1.0 {
            return None;
        }

        let g2 = s2 as f64 / s as f64;
        if g2 < 0.0 || g2 > 1.0 {
            return None;
        }

        let g3 = s3 as f64 / s as f64;
        if g3 < 0.0 || g3 > 1.0 {
            return None;
        }

        Some([g1, g2, g3])
    }

    pub fn vertices(&self) -> [Point; 3] {
        [self.p1, self.p2, self.p3]
    }
}