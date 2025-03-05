use std::fmt::{Display, Formatter};
use std::ops::{Add, Index, IndexMut, Mul, Sub};

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

impl From<[f64; 3]> for Vec3f {
    fn from(v: [f64; 3]) -> Vec3f {
        Vec3f::new(v[0], v[1], v[2])
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

pub trait SqMatrix<T> : Sized {
    fn dim(&self) -> usize;
    fn invert(&self) -> Option<Self>;
    fn det(&self) -> T;
}

#[derive(Copy, Clone, Debug)]
pub struct Mat3x3f {
    data: [f64; 9],
}

impl Mat3x3f {
    pub fn new() -> Mat3x3f {
        Mat3x3f { data: [0.0; 9] }
    }

    pub fn identity() -> Mat3x3f {
        Mat3x3f::from([
            1.0, 0.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 0.0, 1.0,
        ])
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        match (row, col) {
            (0, 0) => self[1][1] * self[2][2] - self[1][2] * self[2][1],
            (1, 0) => self[0][2] * self[2][1] - self[0][1] * self[2][2],
            (2, 0) => self[0][1] * self[1][2] - self[0][2] * self[1][1],
            (0, 1) => self[1][2] * self[2][0] - self[1][0] * self[2][2],
            (1, 1) => self[0][0] * self[2][2] - self[0][2] * self[2][0],
            (2, 1) => self[0][2] * self[1][0] - self[0][0] * self[1][2],
            (0, 2) => self[1][0] * self[2][1] - self[1][1] * self[2][0],
            (1, 2) => self[0][1] * self[2][0] - self[0][0] * self[2][1],
            (2, 2) => self[0][0] * self[1][1] - self[0][1] * self[1][0],
            _ => panic!("Index ({}, {}) out of range", row, col),
        }
    }
}

impl SqMatrix<f64> for Mat3x3f {
    fn dim(&self) -> usize {
        3
    }

    fn invert(&self) -> Option<Self> {
        let c00 = self.cofactor(0, 0);
        let c01 = self.cofactor(0, 1);
        let c02 = self.cofactor(0, 2);

        let det = self[0][0] * c00 + self[0][1] * c01 + self[0][2] * c02;
        if det.abs() < f64::MIN_POSITIVE {
            None
        } else {
            let c10 = self.cofactor(1, 0);
            let c11 = self.cofactor(1, 1);
            let c12 = self.cofactor(1, 2);

            let c20 = self.cofactor(2, 0);
            let c21 = self.cofactor(2, 1);
            let c22 = self.cofactor(2, 2);

            let res = Mat3x3f::from([
                c00, c10, c20,
                c01, c11, c21,
                c02, c12, c22
            ].map(|c| c / det));

            Some(res)
        }
    }

    fn det(&self) -> f64 {
        self[0][0] * self.cofactor(0, 0) +
        self[0][1] * self.cofactor(0, 1) +
        self[0][2] * self.cofactor(0, 2)
    }
}

impl From<[f64; 9]> for Mat3x3f {
    fn from(data: [f64; 9]) -> Self {
        Mat3x3f { data }
    }
}

impl Index<usize> for Mat3x3f {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[self.dim() * row .. self.dim() * (row + 1)]
    }
}

impl IndexMut<usize> for Mat3x3f {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let dim = self.dim();
        &mut self.data[dim * row .. dim * (row + 1)]
    }
}

impl Mul for Mat3x3f {
    type Output = Mat3x3f;
    fn mul(self, rhs: Mat3x3f) -> Mat3x3f {
        let mut res = Mat3x3f::new();
        for row in 0..self.dim() {
            for col in 0..self.dim() {
                res[row][col] += self[row][0] * rhs[0][col];
                res[row][col] += self[row][1] * rhs[1][col];
                res[row][col] += self[row][2] * rhs[2][col];
            }
        }
        res
    }
}

impl Display for Mat3x3f {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim() {
            for col in 0..self.dim() {
                write!(f, "{:8.3} ", self[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Mat4x4f {
    data: [f64; 16],
}

impl Mat4x4f {
    pub fn new() -> Mat4x4f {
        Mat4x4f { data: [0.0; 16] }
    }

    pub fn identity() -> Self {
        Mat4x4f::from([
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn viewport(x: f64, y: f64, width: f64, height: f64) -> Self {
        Mat4x4f::from([
            width / 2.0, 0.0,          0.0,         x + width / 2.0,
            0.0,         height / 2.0, 0.0,         y + height / 2.0,
            0.0,         0.0,          1.0,         0.0,
            0.0,         0.0,          0.0,         1.0,
        ])
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        match (row, col) {
            (0, 0) => Mat3x3f::from([
                self[1][1], self[1][2], self[1][3],
                self[2][1], self[2][2], self[2][3],
                self[3][1], self[3][2], self[3][3]
            ]).det(),
            (1, 0) => -Mat3x3f::from([
                self[0][1], self[0][2], self[0][3],
                self[2][1], self[2][2], self[2][3],
                self[3][1], self[3][2], self[3][3]
            ]).det(),
            (2, 0) => Mat3x3f::from([
                self[0][1], self[0][2], self[0][3],
                self[1][1], self[1][2], self[1][3],
                self[3][1], self[3][2], self[3][3]
            ]).det(),
            (3, 0) => -Mat3x3f::from([
                self[0][1], self[0][2], self[0][3],
                self[1][1], self[1][2], self[1][3],
                self[2][1], self[2][2], self[2][3]
            ]).det(),
            (0, 1) => -Mat3x3f::from([
                self[1][0], self[1][2], self[1][3],
                self[2][0], self[2][2], self[2][3],
                self[3][0], self[3][2], self[3][3],
            ]).det(),
            (1, 1) => Mat3x3f::from([
                self[0][0], self[0][2], self[0][3],
                self[2][0], self[2][2], self[2][3],
                self[3][0], self[3][2], self[3][3]
            ]).det(),
            (2, 1) => -Mat3x3f::from([
                self[0][0], self[0][2], self[0][3],
                self[1][0], self[1][2], self[1][3],
                self[3][0], self[3][2], self[3][3]
            ]).det(),
            (3, 1) => Mat3x3f::from([
                self[0][0], self[0][2], self[0][3],
                self[1][0], self[1][2], self[1][3],
                self[2][0], self[2][2], self[2][3]
            ]).det(),
            (0, 2) => Mat3x3f::from([
                self[1][0], self[1][1], self[1][3],
                self[2][0], self[2][1], self[2][3],
                self[3][0], self[3][1], self[3][3]
            ]).det(),
            (1, 2) => -Mat3x3f::from([
                self[0][0], self[0][1], self[0][3],
                self[2][0], self[2][1], self[2][3],
                self[3][0], self[3][1], self[3][3]
            ]).det(),
            (2, 2) => Mat3x3f::from([
                self[0][0], self[0][1], self[0][3],
                self[1][0], self[1][1], self[1][3],
                self[3][0], self[3][1], self[3][3]
            ]).det(),
            (3, 2) => -Mat3x3f::from([
                self[0][0], self[0][1], self[0][3],
                self[1][0], self[1][1], self[1][3],
                self[2][0], self[2][1], self[2][3]
            ]).det(),
            (0, 3) => -Mat3x3f::from([
                self[1][0], self[1][1], self[1][2],
                self[2][0], self[2][1], self[2][2],
                self[3][0], self[3][1], self[3][2]
            ]).det(),
            (1, 3) => Mat3x3f::from([
                self[0][0], self[0][1], self[0][2],
                self[2][0], self[2][1], self[2][2],
                self[3][0], self[3][1], self[3][2]
            ]).det(),
            (2, 3) => -Mat3x3f::from([
                self[0][0], self[0][1], self[0][2],
                self[1][0], self[1][1], self[1][2],
                self[3][0], self[3][1], self[3][2]
            ]).det(),
            (3, 3) => Mat3x3f::from([
                self[0][0], self[0][1], self[0][2],
                self[1][0], self[1][1], self[1][2],
                self[2][0], self[2][1], self[2][2]
            ]).det(),
            _ => panic!("Index ({}, {}) out of range", row, col),
        }
    }
}

impl SqMatrix<f64> for Mat4x4f {
    fn dim(&self) -> usize {
        4
    }

    fn invert(&self) -> Option<Self> {
        let c00 = self.cofactor(0, 0);
        let c01 = self.cofactor(0, 1);
        let c02 = self.cofactor(0, 2);
        let c03 = self.cofactor(0, 3);

        let det = self[0][0] * c00 + self[0][1] * c01 + self[0][2] * c02 + self[0][3] * c03;
        if det.abs() < f64::MIN_POSITIVE {
            None
        } else {
            let c10 = self.cofactor(1, 0);
            let c11 = self.cofactor(1, 1);
            let c12 = self.cofactor(1, 2);
            let c13 = self.cofactor(1, 3);

            let c20 = self.cofactor(2, 0);
            let c21 = self.cofactor(2, 1);
            let c22 = self.cofactor(2, 2);
            let c23 = self.cofactor(2, 3);

            let c30 = self.cofactor(3, 0);
            let c31 = self.cofactor(3, 1);
            let c32 = self.cofactor(3, 2);
            let c33 = self.cofactor(3, 3);

            let res = Mat4x4f::from([
                c00, c10, c20, c30,
                c01, c11, c21, c31,
                c02, c12, c22, c32,
                c03, c13, c23, c33,
            ].map(|c| c / det));

            Some(res)
        }
    }

    fn det(&self) -> f64 {
        self[0][0] * self.cofactor(0, 0) +
        self[0][1] * self.cofactor(0, 1) +
        self[0][2] * self.cofactor(0, 2) +
        self[0][3] * self.cofactor(0, 3)
    }
}

impl From<[f64; 16]> for Mat4x4f {
    fn from(data: [f64; 16]) -> Mat4x4f {
        Mat4x4f { data }
    }
}

impl Index<usize> for Mat4x4f {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[self.dim() * row .. self.dim() * (row + 1) ]
    }
}

impl IndexMut<usize> for Mat4x4f {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let dim = self.dim();
        &mut self.data[dim * row .. dim * (row + 1)]
    }
}

impl Mul for Mat4x4f {
    type Output = Mat4x4f;
    fn mul(self, rhs: Mat4x4f) -> Mat4x4f {
        let mut res = Mat4x4f::new();
        for row in 0..4 {
            for col in 0..4 {
                res[row][col] += self[row][0] * rhs[0][col];
                res[row][col] += self[row][1] * rhs[1][col];
                res[row][col] += self[row][2] * rhs[2][col];
                res[row][col] += self[row][3] * rhs[3][col];
            }
        }
        res
    }
}

impl Display for Mat4x4f {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim() {
            for col in 0..self.dim() {
                write!(f, "{:8.3} ", self[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Mat4x1f {
    data: [f64; 4]
}

impl Mat4x1f {
    pub fn new() -> Mat4x1f {
        Mat4x1f { data: [0.0; 4] }
    }
}

impl From<[f64; 4]> for Mat4x1f {
    fn from(data: [f64; 4]) -> Mat4x1f {
        Mat4x1f { data }
    }
}

impl From<Vec3f> for Mat4x1f {
    fn from(vec: Vec3f) -> Mat4x1f {
        [vec.x, vec.y, vec.z, 1.0].into()
    }
}

impl From<Mat4x1f> for Vec3f {
    fn from(mat4x1f: Mat4x1f) -> Vec3f {
        [
            mat4x1f[0][0] / mat4x1f[3][0],
            mat4x1f[1][0] / mat4x1f[3][0],
            mat4x1f[2][0] / mat4x1f[3][0]
        ].into()
    }
}

impl Index<usize> for Mat4x1f {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[row .. (row + 1)]
    }
}

impl IndexMut<usize> for Mat4x1f {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[row .. (row + 1)]
    }
}

impl Mul<Mat4x1f> for Mat4x4f {
    type Output = Mat4x1f;
    fn mul(self, rhs: Mat4x1f) -> Mat4x1f {
        let mut res = Mat4x1f::new();
        res[0][0] = self[0][0] * rhs[0][0] + self[0][1] * rhs[1][0] +
                    self[0][2] * rhs[2][0] + self[0][3] * rhs[3][0];

        res[1][0] = self[1][0] * rhs[0][0] + self[1][1] * rhs[1][0] +
                    self[1][2] * rhs[2][0] + self[1][3] * rhs[3][0];

        res[2][0] = self[2][0] * rhs[0][0] + self[2][1] * rhs[1][0] +
                    self[2][2] * rhs[2][0] + self[2][3] * rhs[3][0];

        res[3][0] = self[3][0] * rhs[0][0] + self[3][1] * rhs[1][0] +
                    self[3][2] * rhs[2][0] + self[3][3] * rhs[3][0];
        res
    }
}

impl Display for Mat4x1f {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..4 {
            write!(f, "{:8.3}\n", self[row][0])?;
        }
        Ok(())
    }
}

struct MatNxMf {
    rows: usize,
    cols: usize,
    data: Vec<f64>,
}

impl MatNxMf {
    fn new(rows: usize, cols: usize, data: Vec<f64>) -> MatNxMf {
        assert_eq!(rows * cols, data.len(), "Data should fit into matrix");
        MatNxMf { rows, cols, data }
    }

    fn augmented<T: SqMatrix<f64> + Index<usize, Output = [f64]>>(mat: &T) -> MatNxMf {
        let n = mat.dim();
        let mut data = vec![0.0; 2 * n * n];
        for i in 0..n {
            for j in 0..n {
                data[i * n * 2 + j] = mat[i][j];
            }
            data[i * n * 2 + n + i] = 1.0;
        }
        MatNxMf::new(n, 2 * n, data)
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) {
        for col in 0..self.cols {
            self.data.swap(self.cols * row1 + col, self.cols * row2 + col);
        }
    }

    fn scale_row(&mut self, row: usize, scalar: f64) {
        for col in 0..self.cols {
            self[row][col] *= scalar;
        }
    }

    fn subtract_scaled(&mut self, target_row: usize, source_row: usize, scalar: f64) {
        for col in 0..self.cols {
            self[target_row][col] -= scalar * self[source_row][col];
        }
    }
}

impl Index<usize> for MatNxMf {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[self.cols * row .. self.cols * (row + 1)]
    }
}

impl IndexMut<usize> for MatNxMf {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        let cols = self.cols;
        &mut self.data[cols * row .. cols * (row + 1)]
    }
}

impl Display for MatNxMf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                write!(f, "{:8.3} ", self[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub struct MatNxNf {
    dim: usize,
    data: Vec<f64>,
}

impl MatNxNf {
    pub fn new(dim: usize, data: Vec<f64>) -> MatNxNf {
        assert_eq!(dim * dim, data.len(), "Data should fit into matrix");
        MatNxNf { dim, data }
    }

    pub fn identity(dim: usize) -> Self {
        let mut data = vec![0.0; dim * dim];
        for i in 0..dim {
            data[i + dim * i] = 1.0;
        }
        MatNxNf { dim, data }
    }
}

impl SqMatrix<f64> for MatNxNf {
    fn dim(&self) -> usize {
        self.dim
    }

    fn invert(&self) -> Option<Self> {
        let n = self.dim;

        let mut aug = MatNxMf::augmented(self);
        for i in 0..n {
            if aug[i][i].abs() < f64::MIN_POSITIVE {
                let mut pivot_row = i;
                for j in i + 1..n {
                    if !(aug[j][i].abs() < f64::MIN_POSITIVE) {
                        pivot_row = j;
                        break;
                    }
                }
                if pivot_row == i {
                    return None;
                }
                aug.swap_rows(i, pivot_row);
            }

            let pivot_value = aug[i][i];
            aug.scale_row(i, 1.0 / pivot_value);

            for j in 0..n {
                if j != i {
                    let factor = aug[j][i];
                    aug.subtract_scaled(j, i, factor);
                }
            }
        }

        for i in 0..n {
            aug.data.copy_within(n * (2 * i + 1)..n * (2 * i + 2), n * i);
        }

        aug.data.truncate(n * n);
        Some(MatNxNf::new(n, aug.data))
    }

    fn det(&self) -> f64 {
        let n = self.dim;
        let mut det = 1.0;

        let mut aux = MatNxMf::new(self.dim, self.dim, self.data.clone());
        for i in 0..n - 1 {
            if aux[i][i].abs() < f64::MIN_POSITIVE {
                let mut pivot_row = i;
                for j in i + 1..n {
                    if !(aux[j][i].abs() < f64::MIN_POSITIVE) {
                        pivot_row = j;
                        break;
                    }
                }
                if pivot_row == i {
                    return 0.0
                }
                aux.swap_rows(i, pivot_row);
            }

            let pivot_value = aux[i][i];
            det *= pivot_value;

            for j in i + 1..n {
                let factor = aux[j][i];
                aux.subtract_scaled(j, i, factor / pivot_value);
            }
        }

        det *= aux[n - 1][n - 1];
        det
    }
}

impl Index<usize> for MatNxNf {
    type Output = [f64];
    fn index(&self, row: usize) -> &Self::Output {
        &self.data[self.dim * row .. self.dim * (row + 1)]
    }
}

impl IndexMut<usize> for MatNxNf {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.data[self.dim * row .. self.dim * (row + 1) ]
    }
}

impl Mul for MatNxNf {
    type Output = MatNxNf;
    fn mul(self, rhs: MatNxNf) -> MatNxNf {
        let mut res = MatNxNf::new(self.dim, vec![0.0; self.dim * rhs.dim]);
        let n = self.dim;
        for row in 0..n {
            for col in 0..n {
                for idx in 0..n {
                    res[row][col] += self[row][idx] * rhs[idx][col];
                }
            }
        }
        res
    }
}

impl Display for MatNxNf {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.dim {
            for col in 0..self.dim {
                write!(f, "{:8.3} ", self[row][col])?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
