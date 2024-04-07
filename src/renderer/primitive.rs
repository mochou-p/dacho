// dacho/src/renderer/primitive.rs

use super::{
    color::Color,
    vertex::{CubePosition, Vertex}
};

pub type CubePosUnit      = i16;
pub type VertexData       = u32;
type     TriangleData     = (VertexData, VertexData, VertexData);
type     QuadData         = (TriangleData, TriangleData);
pub type CubeIndicesData  = (QuadData, QuadData, QuadData, QuadData, QuadData, QuadData);
pub type CubeVerticesData = (Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex);

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

        let center = CubePosition(x, y, z);

        (
            Vertex::new(&CubePosition(x - 1, y + 1, z - 1), &center, &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z - 1), &center, &color),
            Vertex::new(&CubePosition(x + 1, y + 1, z + 1), &center, &color),
            Vertex::new(&CubePosition(x - 1, y + 1, z + 1), &center, &color),
            Vertex::new(&CubePosition(x - 1, y - 1, z - 1), &center, &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z - 1), &center, &color),
            Vertex::new(&CubePosition(x + 1, y - 1, z + 1), &center, &color),
            Vertex::new(&CubePosition(x - 1, y - 1, z + 1), &center, &color)
        )
    }
}

pub struct CubeIndices;

impl CubeIndices {
    pub const fn new(
        i: VertexData
    ) -> CubeIndicesData {
        let i = i * 8;

        (
            Quad::new(i + 0, i + 1, i + 2, i + 3),
            Quad::new(i + 7, i + 6, i + 5, i + 4),
            Quad::new(i + 4, i + 5, i + 1, i + 0),
            Quad::new(i + 6, i + 7, i + 3, i + 2),
            Quad::new(i + 0, i + 3, i + 7, i + 4),
            Quad::new(i + 2, i + 1, i + 5, i + 6)
        )
    }
}

