#[cfg(test)]
mod test {
    use tinyrs::geometry::{Mat3x3f, Mat4x1f, Mat4x4f, MatNxNf, SqMatrix, Triangle, Vec3f};

    const EPSILON: f64 = 1e-4_f64;

    #[test]
    fn test_dot_product() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let v3 = Vec3f::new(0.0, 0.0, 1.0);

        assert!(v1.dot(&v2) < EPSILON);
        assert!(v1.dot(&v3) < EPSILON);
        assert!(v2.dot(&v3) < EPSILON);

        let inv_sqrt3 = 1.0 / f64::sqrt(3.0);
        let v4 = Vec3f::new(inv_sqrt3, inv_sqrt3, inv_sqrt3);
        assert!((v4.dot(&v4) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_cross_product() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let v3 = Vec3f::new(0.0, 0.0, 1.0);

        assert!((v1.cross(&v2) - v3).norm() < EPSILON);
        assert!((v2.cross(&v3) - v1).norm() < EPSILON);
        assert!((v3.cross(&v1) - v2).norm() < EPSILON);
    }

    #[test]
    fn test_barycentric_coordinates() {
        // an equilateral triangle
        let p1 = Vec3f::new(-0.5, -f64::sqrt(3.0) * 0.5, 0.0);
        let p2 = Vec3f::new(1.0, 0.0, 0.0);
        let p3 = Vec3f::new(-0.5, f64::sqrt(3.0) * 0.5, 0.0);

        let triangle = Triangle::new(p1, p2, p3);

        if let Some(bcs) = triangle.barycentric(Vec3f::new(0.0, 0.0, 0.0)) {
            let [g1, g2, g3] = bcs;
            assert!((g1 - 0.3333).abs() < EPSILON);
            assert!((g2 - 0.3333).abs() < EPSILON);
            assert!((g3 - 0.3333).abs() < EPSILON)
        };

        if let Some(bcs) = triangle.barycentric(p1) {
            let [g1, g2, g3] = bcs;
            assert!((g1 - 1.0).abs() < EPSILON);
            assert!(g2.abs() < EPSILON);
            assert!(g3.abs() < EPSILON)
        }

        if let Some(bcs) = triangle.barycentric(p2) {
            let [g1, g2, g3] = bcs;
            assert!(g1.abs() < EPSILON);
            assert!((g2 - 1.0).abs() < EPSILON);
            assert!(g3.abs() < EPSILON)
        }

        if let Some(bcs) = triangle.barycentric(p3) {
            let [g1, g2, g3] = bcs;
            assert!(g1.abs() < EPSILON);
            assert!(g2.abs() < EPSILON);
            assert!((g3 - 1.0).abs() < EPSILON)
        }

        let mid12 = Vec3f::new((p1.x + p2.x) * 0.5, (p1.y + p2.y) * 0.5, 0.0);

        if let Some(bcs) = triangle.barycentric(mid12) {
            let [g1, g2, g3] = bcs;
            assert!((g1 - 0.5).abs() < EPSILON);
            assert!((g2 - 0.5).abs() < EPSILON);
            assert!(g3.abs() < EPSILON);
        }

        let mid23 = Vec3f::new((p2.x + p3.x) * 0.5, (p2.y + p3.y) * 0.5, 0.0);

        if let Some(bcs) = triangle.barycentric(mid23) {
            let [g1, g2, g3] = bcs;
            assert!(g1.abs() < EPSILON);
            assert!((g2 - 0.5).abs() < EPSILON);
            assert!((g3 - 0.5).abs() < EPSILON);
        }

        let mid13 = Vec3f::new((p1.x + p3.x) * 0.5, (p1.y + p3.y) * 0.5, 0.0);

        if let Some(bcs) = triangle.barycentric(mid13) {
            let [g1, g2, g3] = bcs;
            assert!((g1 - 0.5).abs() < EPSILON);
            assert!(g2.abs() < EPSILON);
            assert!((g3 - 0.5).abs() < EPSILON);
        }

        if let Some(bcs) = triangle.barycentric(Vec3f::new(-0.51, -0.87, 0.0)) {
            let [g1, g2, g3] = bcs;
            assert!(g1 > 1.0);
            assert!(g2 < 0.0);
            assert!(g3 < 0.0);
        }

        if let Some(bcs) = triangle.barycentric(Vec3f::new(1.1, 0.0, 0.0)) {
            let [g1, g2, g3] = bcs;
            assert!(g1 < 0.0);
            assert!(g2 > 1.0);
            assert!(g3 < 0.0);
        }

        if let Some(bcs) = triangle.barycentric(Vec3f::new(-0.51, 0.87, 0.0)) {
            let [g1, g2, g3] = bcs;
            assert!(g1 < 0.0);
            assert!(g2 < 0.0);
            assert!(g3 > 1.0);
        }
    }

    #[test]
    fn test_mul_3x3f() {
        let mat_a = Mat3x3f::from([
            8.0, 4.0, 3.0,
            5.0, 1.0, 0.0,
            6.0, 7.0, 2.0,
        ]);

        let mat_b = Mat3x3f::from([
            3.0, 9.0, 6.0,
            4.0, 5.0, 0.0,
            8.0, 2.0, 7.0,
        ]);

        let mat_p = Mat3x3f::from([
            64.0, 98.0, 69.0,
            19.0, 50.0, 30.0,
            62.0, 93.0, 50.0,
        ]);

        let product = mat_a * mat_b;

        for row in 0..product.dim() {
            for col in 0..product.dim() {
                let diff = (product[row][col] - mat_p[row][col]).abs();
                assert!(diff < EPSILON);
            }
        }
    }

    #[test]
    fn test_invert_3x3f() {
        let mat = Mat3x3f::from([
            2.0, 1.0, 5.0,
            7.0, 4.0, 9.0,
            6.0, 5.0, 8.0,
        ]);

        if let Some(inverse) = mat.invert() {
            let product = mat * inverse;
            let identity = Mat3x3f::identity();

            for row in 0..product.dim() {
                for col in 0..product.dim() {
                    let diff = (product[row][col] - identity[row][col]).abs();
                    assert!(diff < EPSILON);
                }
            }
        } else {
            assert!(false, "Matrix is not invertible");
        }
    }

    #[test]
    fn test_singular_3x3f() {
        let mat = Mat3x3f::from([
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]);

        let maybe_inverted = mat.invert();
        assert!(maybe_inverted.is_none());
    }

    #[test]
    fn test_det_3x3f() {
        let mat = Mat3x3f::from([
            2.0, 1.0, 5.0,
            7.0, 4.0, 9.0,
            6.0, 5.0, 8.0,
        ]);

        let diff = (mat.det() - 27.0).abs();
        assert!(diff < EPSILON);
    }

    #[test]
    fn test_det_zero_3x3f() {
        let mat = Mat3x3f::from([
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]);

        assert!(mat.det().abs() < EPSILON);
    }

    #[test]
    fn test_mul_4x4f() {
        let mat_a = Mat4x4f::from([
            3.0, 5.0, 3.0, 9.0,
            7.0, 1.0, 8.0, 5.0,
            0.0, 2.0, 4.0, 4.0,
            6.0, 1.0, 3.0, 0.0,
        ]);

        let mat_b = Mat4x4f::from([
            3.0, 3.0, 2.0, 5.0,
            8.0, 0.0, 4.0, 5.0,
            9.0, 6.0, 4.0, 2.0,
            1.0, 7.0, 1.0, 0.0,
        ]);

        let mat_p = Mat4x4f::from([
             85.0,  90.0, 47.0, 46.0,
            106.0, 104.0, 55.0, 56.0,
             56.0,  52.0, 28.0, 18.0,
             53.0,  36.0, 28.0, 41.0,
        ]);

        let product = mat_a * mat_b;
        for row in 0..product.dim() {
            for col in 0..product.dim() {
                let diff = (product[row][col] - mat_p[row][col]).abs();
                assert!(diff < EPSILON);
            }
        }
    }

    #[test]
    fn test_mul_4x1f() {
        let mat_a = Mat4x4f::from([
            2.0, 3.0, 1.0, 5.0,
            7.0, 4.0, 9.0, 8.0,
            6.0, 5.0, 8.0, 7.0,
            9.0, 2.0, 6.0, 5.0,
        ]);

        let mat_b = Mat4x1f::from([
            1.0,
            2.0,
            3.0,
            4.0,
        ]);

        let mat_p = Mat4x1f::from([
            31.0,
            74.0,
            68.0,
            51.0,
        ]);

        let product = mat_a * mat_b;

        for row in 0..4 {
            let diff = (product[row][0] - mat_p[row][0]).abs();
            assert!(diff < EPSILON);
        }
    }

    #[test]
    fn test_mul_vec_4x1() {
        let vec = Vec3f::new(3.0, 5.0, 7.0);

        let mat = Mat4x4f::from([
            2.0, 3.0, 1.0, 5.0,
            7.0, 4.0, 9.0, 8.0,
            6.0, 5.0, 8.0, 7.0,
            9.0, 2.0, 6.0, 5.0,
        ]);

        let prod = mat * Mat4x1f::from(vec);

        let vec = Vec3f::from(prod);

        let diff = [
            vec.x - 33.0 / 84.0,
            vec.y - 112.0 / 84.0,
            vec.z - 106.0 / 84.0,
        ].map(|v| v.abs() < EPSILON);

        assert!(diff[0]);
        assert!(diff[1]);
        assert!(diff[2]);
    }

    #[test]
    fn test_invert_4x4f() {
        let mat = Mat4x4f::from([
            2.0, 3.0, 1.0, 5.0,
            7.0, 4.0, 9.0, 8.0,
            6.0, 5.0, 8.0, 7.0,
            9.0, 2.0, 6.0, 5.0,
        ]);

        if let Some(inverse) = mat.invert() {
            let product = mat * inverse;
            let identity = Mat4x4f::identity();

            for row in 0..product.dim() {
                for col in 0..product.dim() {
                    let diff = (product[row][col] - identity[row][col]).abs();
                    assert!(diff < EPSILON);
                }
            }
        } else {
            assert!(false, "Matrix is not invertible");
        }
    }

    #[test]
    fn test_singular_4x4f() {
        let mat = Mat4x4f::from([
             1.0,  2.0,  3.0,  4.0,
             5.0,  6.0,  7.0,  8.0,
             9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0
        ]);

        let maybe_inverted = mat.invert();
        assert!(maybe_inverted.is_none());
    }

    #[test]
    fn test_det_4x4f() {
        let mat = Mat4x4f::from([
            2.0, 3.0, 1.0, 5.0,
            7.0, 4.0, 9.0, 8.0,
            6.0, 5.0, 8.0, 7.0,
            9.0, 2.0, 6.0, 5.0,
        ]);

        let diff = (mat.det() - 245.0).abs();
        assert!(diff < EPSILON);
    }

    #[test]
    fn test_det_zero_4x4f() {
        let mat = Mat4x4f::from([
            1.0,  2.0,  3.0,  4.0,
            5.0,  6.0,  7.0,  8.0,
            9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0
        ]);

        assert!(mat.det().abs() < EPSILON);
    }

    #[test]
    fn test_invert_3x3() {
        let mat = MatNxNf::new(3, vec![
            2.0, 1.0, 5.0,
            7.0, 4.0, 9.0,
            6.0, 5.0, 8.0,
        ]);

        if let Some(inverse) = mat.invert() {
            let product = mat * inverse;
            let identity = Mat3x3f::identity();

            for row in 0..product.dim() {
                for col in 0..product.dim() {
                    let diff = (product[row][col] - identity[row][col]).abs();
                    assert!(diff < EPSILON);
                }
            }
        } else {
            assert!(false, "Matrix is not invertible");
        }
    }

    #[test]
    fn test_singular_3x3() {
        let mat = MatNxNf::new(3, vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]);

        let maybe_inverted = mat.invert();
        assert!(maybe_inverted.is_none());
    }

    #[test]
    fn test_invert_4x4() {
        let mat = MatNxNf::new(4, vec![
            2.0, 3.0, 1.0, 5.0,
            7.0, 4.0, 9.0, 8.0,
            6.0, 5.0, 8.0, 7.0,
            9.0, 2.0, 6.0, 5.0,
        ]);

        if let Some(inverse) = mat.invert() {
            let product = mat * inverse;
            let identity = MatNxNf::identity(4);

            for row in 0..product.dim() {
                for col in 0..product.dim() {
                    let diff = (product[row][col] - identity[row][col]).abs();
                    assert!(diff < EPSILON);
                }
            }
        }
    }

    #[test]
    fn test_singular_4x4() {
        let mat = MatNxNf::new( 4, vec![
             1.0,  2.0,  3.0,  4.0,
             5.0,  6.0,  7.0,  8.0,
             9.0, 10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ]);

        let maybe_inverted = mat.invert();
        assert!(maybe_inverted.is_none());
    }

    #[test]
    fn test_det_3x3() {
        let mat = MatNxNf::new(3,vec![
            2.0, 1.0, 5.0,
            7.0, 4.0, 9.0,
            6.0, 5.0, 8.0,
        ]);

        let det = mat.det();
        let diff = (det - 27.0).abs();
        assert!(diff < EPSILON);
    }

    #[test]
    fn test_det_zero_3x3() {
        let mat = MatNxNf::new(3,vec![
            1.0, 2.0, 3.0,
            4.0, 5.0, 6.0,
            7.0, 8.0, 9.0,
        ]);

        assert!(mat.det().abs() < EPSILON);
    }

    #[test]
    fn test_det_4x4() {
        let mat = MatNxNf::new(4, vec![
            2.0, 3.0, 1.0, 5.0,
            7.0, 4.0, 9.0, 8.0,
            6.0, 5.0, 8.0, 7.0,
            9.0, 2.0, 6.0, 5.0,
        ]);

        let det = mat.det();
        let diff = (det - 245.0).abs();
        assert!(diff < EPSILON);
    }

    #[test]
    fn test_det_zero_4x4() {
        let mat = MatNxNf::new( 4, vec![
            1.0,  2.0,  3.0,  4.0,
            5.0,  6.0,  7.0,  8.0,
            9.0,  10.0, 11.0, 12.0,
            13.0, 14.0, 15.0, 16.0,
        ]);

        assert!(mat.det().abs() < EPSILON);
    }
}