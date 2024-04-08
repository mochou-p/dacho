// dacho/src/renderer/primitive.rs

use super::{
    color::Color,
    vertex::Vertex
};

pub type VertexData       = u16;
    type TriangleData     = (VertexData, VertexData, VertexData);
    type QuadData         = (TriangleData, TriangleData);
pub type CubeIndicesData  = (QuadData, QuadData, QuadData, QuadData, QuadData, QuadData);
pub type CubeVerticesData = (
    Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex,
    Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex,
    Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex
);

// per cube
pub const INDEX_COUNT:  usize = 36;
    const VERTEX_COUNT: u16   = 24;

struct Triangle;

impl Triangle {
    const fn new(
        a: VertexData,
        b: VertexData,
        c: VertexData
    ) -> TriangleData {
        (a, b, c)
    }
}

struct Quad;

impl Quad {
    const fn new(
        a: VertexData,
        b: VertexData,
        c: VertexData,
        d: VertexData
    ) -> QuadData {
        (
            Triangle::new(a, b, c),
            Triangle::new(c, d, a)
        )
    }
}

pub struct CubeVertices;

impl CubeVertices {
    pub fn new(
        x: f32,
        y: f32,
        z: f32
    ) -> CubeVerticesData {
        let color = [Color::GREEN, Color::CYAN][(y.abs() / 2.0 % 2.0) as usize];

        (
            // top
            Vertex::new(&(x - 1.0, y + 1.0, z - 1.0), &color, 0),
            Vertex::new(&(x + 1.0, y + 1.0, z - 1.0), &color, 0),
            Vertex::new(&(x + 1.0, y + 1.0, z + 1.0), &color, 0),
            Vertex::new(&(x - 1.0, y + 1.0, z + 1.0), &color, 0),

            // bottom
            Vertex::new(&(x - 1.0, y - 1.0, z - 1.0), &color, 1),
            Vertex::new(&(x + 1.0, y - 1.0, z - 1.0), &color, 1),
            Vertex::new(&(x + 1.0, y - 1.0, z + 1.0), &color, 1),
            Vertex::new(&(x - 1.0, y - 1.0, z + 1.0), &color, 1),

            // left
            Vertex::new(&(x - 1.0, y - 1.0, z - 1.0), &color, 2),
            Vertex::new(&(x - 1.0, y + 1.0, z - 1.0), &color, 2),
            Vertex::new(&(x - 1.0, y + 1.0, z + 1.0), &color, 2),
            Vertex::new(&(x - 1.0, y - 1.0, z + 1.0), &color, 2),

            // right
            Vertex::new(&(x + 1.0, y - 1.0, z - 1.0), &color, 3),
            Vertex::new(&(x + 1.0, y + 1.0, z - 1.0), &color, 3),
            Vertex::new(&(x + 1.0, y + 1.0, z + 1.0), &color, 3),
            Vertex::new(&(x + 1.0, y - 1.0, z + 1.0), &color, 3),

            // front
            Vertex::new(&(x - 1.0, y - 1.0, z + 1.0), &color, 4),
            Vertex::new(&(x + 1.0, y - 1.0, z + 1.0), &color, 4),
            Vertex::new(&(x + 1.0, y + 1.0, z + 1.0), &color, 4),
            Vertex::new(&(x - 1.0, y + 1.0, z + 1.0), &color, 4),

            // back
            Vertex::new(&(x - 1.0, y - 1.0, z - 1.0), &color, 5),
            Vertex::new(&(x + 1.0, y - 1.0, z - 1.0), &color, 5),
            Vertex::new(&(x + 1.0, y + 1.0, z - 1.0), &color, 5),
            Vertex::new(&(x - 1.0, y + 1.0, z - 1.0), &color, 5)
        )
    }
}

pub struct CubeIndices;

impl CubeIndices {
    pub const fn new(
        i: VertexData
    ) -> CubeIndicesData {
        let i = i * VERTEX_COUNT;

        (
            // top
            Quad::new(i +  0, i +  1, i +  2, i +  3),
            // bottom
            Quad::new(i +  7, i +  6, i +  5, i +  4),
            // left
            Quad::new(i +  8, i +  9, i + 10, i + 11),
            // right
            Quad::new(i + 15, i + 14, i + 13, i + 12),
            // front
            Quad::new(i + 19, i + 18, i + 17, i + 16),
            // back
            Quad::new(i + 20, i + 21, i + 22, i + 23)
        )
    }
}

