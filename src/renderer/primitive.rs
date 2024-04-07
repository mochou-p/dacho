// dacho/src/renderer/primitive.rs

use super::{
    color::Color,
    vertex::{CubePosition, Vertex}
};

pub type CubePosUnit      = i16;
pub type VertexData       = u32;
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
    const VERTEX_COUNT: u32   = 24;

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
    pub const fn new(
        x: CubePosUnit,
        y: CubePosUnit,
        z: CubePosUnit,
        i: usize
    ) -> CubeVerticesData {
        let color = [
            [Color::DARKER,  Color::DARK ],
            [Color::LIGHTER, Color::LIGHT]
        ][(y.abs() / 2 % 2) as usize][i % 2];

        (
            // top
            Vertex::new(&CubePosition(x - 1, y + 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z + 1), &color),
            Vertex::new(&CubePosition(x - 1, y + 1, z + 1), &color),

            // bottom
            Vertex::new(&CubePosition(x - 1, y - 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z + 1), &color),
            Vertex::new(&CubePosition(x - 1, y - 1, z + 1), &color),

            // left
            Vertex::new(&CubePosition(x - 1, y - 1, z - 1), &color),
            Vertex::new(&CubePosition(x - 1, y + 1, z - 1), &color),
            Vertex::new(&CubePosition(x - 1, y + 1, z + 1), &color),
            Vertex::new(&CubePosition(x - 1, y - 1, z + 1), &color),

            // right
            Vertex::new(&CubePosition(x + 1, y - 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z + 1), &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z + 1), &color),

            // front
            Vertex::new(&CubePosition(x - 1, y - 1, z + 1), &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z + 1), &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z + 1), &color),
            Vertex::new(&CubePosition(x - 1, y + 1, z + 1), &color),

            // back
            Vertex::new(&CubePosition(x - 1, y - 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z - 1), &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z - 1), &color),
            Vertex::new(&CubePosition(x - 1, y + 1, z - 1), &color)
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

