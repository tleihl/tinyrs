#[cfg(test)]
mod test {
    use tinyrs::geometry::{Triangle, Vec3f};

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
}