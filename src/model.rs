use std::fmt::Display;
use std::io::BufRead;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use crate::errors::RenderError;
use crate::geometry::{VecUV2f, Vec3f};

enum Coordinate {
    X, Y, Z, U, V,
}

impl Display for Coordinate {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Coordinate::X => write!(f, "x"),
            Coordinate::Y => write!(f, "y"),
            Coordinate::Z => write!(f, "z"),
            Coordinate::U => write!(f, "u"),
            Coordinate::V => write!(f, "v"),
        }
    }
}

impl Coordinate {
    fn parse<'a, I>(&self, iter: &mut I) -> Result<f64, String>
    where I: Iterator<Item = &'a str> {
        let res = iter
            .next()
            .ok_or(format!("missing coordinate {self}"))?
            .parse::<f64>()
            .map_err(|err| format!("invalid coordinate {self} format: {}", err))?;

        Ok(res)
    }
}

fn parse_vec3f(line: &str) -> Result<Vec3f, String> {
    let mut parts = line
        .split_whitespace()
        .filter(|s| !s.is_empty());

    Ok(Vec3f::new(Coordinate::X.parse(&mut parts)?,
                  Coordinate::Y.parse(&mut parts)?,
                  Coordinate::Z.parse(&mut parts)?))
}

fn parse_vec_uv_2f(line: &str) -> Result<VecUV2f, String> {
    let mut parts = line
        .split_whitespace()
        .filter(|s| !s.is_empty());

    Ok(VecUV2f::new(Coordinate::U.parse(&mut parts)?,
                    Coordinate::V.parse(&mut parts)?))
}

enum FaceIndex {
    Vertex,
    Texture,
    Normal,
}

impl FaceIndex {
    fn parse<'a, I>(&self, iter: &mut I) -> Result<usize, String>
    where I: Iterator<Item = &'a str>{
        match self {
            FaceIndex::Vertex => iter.next()
                .map(|elem| {
                    elem.parse::<usize>()
                        .map_err(|err| format!("invalid vertex index format: {}", err))
                })
                .ok_or("missing vertex index")?,
            FaceIndex::Texture => iter.next()
                    .map(|elem| {
                        if elem.is_empty() {
                            Ok(0)
                        } else {
                            elem.parse::<usize>()
                                .map_err(|err| {
                                    format!("invalid texture index format: {}", err)
                                })
                        }
                    })
                    .ok_or("missing texture index")?,
            FaceIndex::Normal => iter.next()
                .map(|elem| {
                    if elem.is_empty() {
                        Ok(0)
                    } else {
                        elem.parse::<usize>()
                            .map_err(|err| {
                                format!("invalid normal index format: {}", err)
                            })
                    }
                })
                .ok_or("missing normal index")?,
        }
    }
}

pub struct Face {
    pub vertices: Vec<Vec3f>,
    pub textures: Vec<VecUV2f>,
    pub normals:  Vec<Vec3f>,
}

impl Face {
    fn from(line: &str,
            vertices: &Vec<Vec3f>,
            textures: &Vec<VecUV2f>,
            normals: &Vec<Vec3f>) -> Result<Face, String> {
        let parts = line
            .split_whitespace()
            .filter(|s| !s.is_empty());

        let mut face_vertices= Vec::new();
        let mut face_textures = Vec::new();
        let mut face_normals = Vec::new();

        for part in parts.into_iter() {
            let mut indices = part.split('/');

            let vertex_index = FaceIndex::Vertex.parse(&mut indices)?;
            if vertex_index == 0 || vertex_index > vertices.len() {
                return Err(format!("face index out of bounds: {}", vertex_index));
            }
            face_vertices.push(vertices[vertex_index - 1]);

            let texture_index = FaceIndex::Texture.parse(&mut indices)?;
            if texture_index > textures.len() {
                return Err(format!("texture index out of bounds: {}", texture_index));
            }
            if texture_index > 0 {
                face_textures.push(textures[texture_index - 1]);
            }

            let normal_index = FaceIndex::Normal.parse(&mut indices)?;
            if normal_index > normals.len() {
                return Err(format!("normal index out of bounds: {}", normal_index));
            }
            if normal_index > 0 {
                face_normals.push(normals[normal_index - 1]);
            }
        }

        if face_textures.len() != face_vertices.len() {
            std::mem::swap(&mut face_textures, & mut Vec::new());
        }

        if face_normals.len() != face_vertices.len() {
            std::mem::swap(&mut face_normals, & mut Vec::new());
        }

        Ok(Face {
            vertices: face_vertices,
            textures: face_textures,
            normals: face_normals,
        })

    }
}

pub struct Model {
    faces: Vec<Face>
}

pub struct ModelIterator<'a> {
    model: &'a Model,
    index: usize,
}


impl<'a> Iterator for ModelIterator<'a> {
    type Item = &'a Face;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.model.faces.len() {
            let result = Some(&self.model.faces[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl Model {
    pub fn from_file<P>(filename: P) -> Result<Model, RenderError>
        where P: AsRef<Path>, {

        let file = File::open(&filename)?;
        let file = BufReader::new(file);

        let mut vertices = Vec::new();
        let mut normals = Vec::new();
        let mut textures = Vec::new();
        let mut faces = Vec::new();

        for (line, maybe_line) in file.lines().enumerate() {
            if let Some((first, rest)) = maybe_line?.split_once(' ') {
                match first {
                    "v"  => parse_vec3f(rest)
                        .map(|vertex| vertices.push(vertex))
                        .map_err(|msg| RenderError::VertexParsingError(
                            format!("at line {}: {}", line + 1, msg))),
                    "vn" => parse_vec3f(rest)
                        .map(|normal| normals.push(normal.normalize()))
                        .map_err(|msg| RenderError::NormalParsingError(
                            format!("at line {}: {}", line + 1, msg))),
                    "vt" => parse_vec_uv_2f(rest)
                        .map(|texture| textures.push(texture))
                        .map_err(|msg| RenderError::TextureParsingError(
                            format!("at line {}: {}", line + 1, msg))),
                    "f"  => Face::from(rest, &vertices, &textures, &normals)
                        .map(|face| faces.push(face))
                        .map_err(|msg| RenderError::FaceParsingError(
                            format!("at line {}: {}", line + 1, msg))),
                    _ => Ok(())
                }?
            }
        }

        Ok(Model{faces})
    }

    pub fn iter(&self) -> ModelIterator {
        ModelIterator {
            model: self,
            index: 0,
        }
    }
}