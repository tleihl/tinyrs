use sdl2::pixels::Color;
use sdl2::render::WindowCanvas;
use sdl2::rect::Point;

use crate::common::Resolution;
use crate::geometry::{Mat4x1f, Mat4x4f, Triangle, Vec3f};
use crate::model::Face;

#[derive(Default)]
pub struct Renderer {
    resolution: Resolution,
}

impl Renderer {
    pub fn new<R: Into<Resolution>> (resolution: R) -> Self {
        let resolution = resolution.into();
        Renderer { resolution }
    }

    pub fn render_line(&self, canvas: &mut WindowCanvas, p0: Point, p1: Point) -> Result<(), String> {
        let (p0, p1, steep) = if (p0.x - p1.x).abs() < (p0.y - p1.y).abs() {
            (Point::new(p0.y, p0.x), Point::new(p1.y, p1.x), true)
        } else {
            (p0, p1, false)
        };

        let (p0, p1) = if p0.x > p1.x {
            (p1, p0)
        } else {
            (p0, p1)
        };

        let dx = p1.x - p0.x;
        let dy = p1.y - p0.y;

        let iy = if p1.y > p0.y { 1 } else { -1 };

        let derr2 = dy.abs() * 2;
        let mut err2 = 0;

        let mut y = p0.y;
        for x in p0.x..=p1.x {
            if steep {
                canvas.draw_point(Point::new(y, x))?;
            } else {
                canvas.draw_point(Point::new(x, y))?;
            }
            err2 += derr2;
            if err2 > dx {
                y += iy;
                err2 -= dx * 2;
            }
        }
        Ok(())
    }

    fn render_triangle_fn(&self,
                          canvas: &mut WindowCanvas,
                          zbuffer: &mut Vec<f64>,
                          triangle: &Triangle,
                          color_fn: impl Fn([f64; 3]) -> Color) -> Result<(), String> {
        let mut min_x = self.resolution.width - 1;
        let mut min_y = self.resolution.height - 1;
        let mut max_x = 0u32;
        let mut max_y = 0u32;
        for vertex in triangle.vertices() {
            min_x = 0u32.max(min_x.min(vertex.x as u32));
            min_y = 0u32.max(min_y.min(vertex.y as u32));
            max_x = (self.resolution.width - 1).min(max_x.max(vertex.x as u32));
            max_y = (self.resolution.height - 1).min(max_y.max(vertex.y as u32));
        }

        for x in min_x..=max_x {
            for y in min_y..=max_y {
                if let Some(bcs)  = triangle.barycentric(Vec3f::new(x as f64, y as f64, 0f64)) {
                    let z = triangle
                        .vertices().iter()
                        .zip(bcs)
                        .map(|(v, g)| v.z * g)
                        .sum::<f64>();
                    let index = (x + self.resolution.width * y) as usize;
                    if zbuffer[index] < z {
                        zbuffer[index] = z;
                        canvas.set_draw_color(color_fn(bcs));
                        canvas.draw_fpoint((x as f32, y as f32))?;
                    }
                }
            }
        }
        Ok(())
    }

    pub fn render_triangle(&self,
                           canvas: &mut WindowCanvas,
                           zbuffer: &mut Vec<f64>,
                           triangle: &Triangle,
                           colors: [Vec3f; 3]) -> Result<(), String> {
        let color_fn = |bcs: [f64; 3]| {
            let color = colors.into_iter()
                .zip(bcs.into_iter())
                .map(|(color, mul)| color * mul)
                .reduce(|v1, v2| v1 + v2)
                .unwrap();

            Color::RGB(f64::clamp(color.x, 0.0, 255.0) as u8,
                       f64::clamp(color.y, 0.0, 255.0) as u8,
                       f64::clamp(color.z, 0.0, 255.0)  as u8)
        };
        self.render_triangle_fn(canvas, zbuffer, triangle, color_fn)
    }

    pub fn render_face(&self,
                       canvas: &mut WindowCanvas,
                       zbuffer: &mut Vec<f64>,
                       light_direction: &Vec3f,
                       face: &Face,
                       view_port: Mat4x4f,
                       projection: Mat4x4f) -> Result<(), String> {
        if face.vertices.len() != 3 {
            return Ok(())
        }

        let [p1, p2, p3] = [
            face.vertices[0],
            face.vertices[1],
            face.vertices[2]
        ].map(|v| (view_port * projection * Mat4x1f::from(v)).into());

        let triangle = Triangle::new(p1, p2, p3);

        if face.normals.len() != 3 {
            let colors = [
                Vec3f::new(255.0, 0.0, 0.0),
                Vec3f::new(0.0, 255.0, 0.0),
                Vec3f::new(0.0, 0.0, 255.0),
            ];

            self.render_triangle(canvas, zbuffer, &triangle, colors)
        } else {
            let intensities = face.normals.iter()
                .map(|normal| light_direction.dot(normal))
                .filter(|&intensity| intensity > 0.0)
                .collect::<Vec<f64>>();

            if intensities.len() == 3 {
                let maybe_colors = [Vec3f::new(255.0, 255.0, 255.0); 3]
                    .into_iter().zip(intensities.into_iter())
                    .map(|(color, intensity)| color * intensity)
                    .collect::<Vec<Vec3f>>();
                let colors = <[Vec3f; 3]>::try_from(maybe_colors.as_slice()).unwrap();
                self.render_triangle(canvas, zbuffer, &triangle, colors)
            } else {
                Ok(())
            }
        }
    }
}
