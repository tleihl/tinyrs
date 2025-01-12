#[cfg(test)]
mod test {
    use sdl2::rect::Point;
    use tinyrs::geometry::{Triangle, Vec2i, Vec3f};

    const EPSILON: f64 = 0.0001;

    #[test]
    fn test_dot_product() {
        let v1 = Vec3f::new(1.0, 0.0, 0.0);
        let v2 = Vec3f::new(0.0, 1.0, 0.0);
        let v3 = Vec3f::new(0.0, 0.0, 1.0);

        assert!(v1.dot(&v2) < EPSILON);
        assert!(v1.dot(&v3) < EPSILON);
        assert!(v2.dot(&v3) < EPSILON);

        let v4 = Vec3f::new(0.57735, 0.57735, 0.57735);
        assert!((v4.dot(&v4) - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_cross_product() {
        let p1 = Point::new(111, 123);
        let p2 = Point::new(224, 275);
        let p3 = Point::new(315, 422);

        let v1 = Vec2i::new(p1, p2);
        let v2 = Vec2i::new(p1, p3);

        assert_eq!(v1.cross(&v2), 2779);
        assert_eq!(v1.cross(&v2), -v2.cross(&v1));
        assert_eq!(v1.cross(&v1), 0);
    }

    #[test]
    fn test_barycentric_coordinates() {
        // an (almost) equilateral triangle
        let p1 = Point::new(-500, -866);    // (-1/2, -√3/2)
        let p2 = Point::new(1000, 0);       // (1, 0)
        let p3 = Point::new(-500, 866);     // (-1/2, √3/2)

        let triangle = Triangle::new(p1, p2, p3);

        if let Some(bcs) = triangle.barycentric(Point::new(0, 0)) {
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

        if let Some(bcs) = triangle.barycentric(Point::new((p1.x + p2.x) / 2, (p1.y + p2.y) / 2)) {
            let [g1, g2, g3] = bcs;
            assert!((g1 - 0.5).abs() < EPSILON);
            assert!((g2 - 0.5).abs() < EPSILON);
            assert!(g3.abs() < EPSILON);
        }

        if let Some(bcs) = triangle.barycentric(Point::new((p2.x + p3.x) / 2, (p2.y + p3.y) / 2)) {
            let [g1, g2, g3] = bcs;
            assert!(g1.abs() < EPSILON);
            assert!((g2 - 0.5).abs() < EPSILON);
            assert!((g3 - 0.5).abs() < EPSILON);
        }

        if let Some(bcs) = triangle.barycentric(Point::new((p1.x + p3.x) / 2, (p1.y + p3.y) / 2)) {
            let [g1, g2, g3] = bcs;
            assert!((g1 - 0.5).abs() < EPSILON);
            assert!(g2.abs() < EPSILON);
            assert!((g3 - 0.5).abs() < EPSILON);
        }

        if let Some(bcs) = triangle.barycentric(Point::new(-501, -867)) {
            let [g1, g2, g3] = bcs;
            assert!(g1 > 1.0);
            assert!(g2 < 0.0);
            assert!(g3 < 0.0);
        }

        if let Some(bcs) = triangle.barycentric(Point::new(1001, 0)) {
            let [g1, g2, g3] = bcs;
            assert!(g1 < 0.0);
            assert!(g2 > 1.0);
            assert!(g3 < 0.0);
        }

        if let Some(bcs) = triangle.barycentric(Point::new(-501, 867)) {
            let [g1, g2, g3] = bcs;
            assert!(g1 < 0.0);
            assert!(g2 < 0.0);
            assert!(g3 > 1.0);
        }
    }
}