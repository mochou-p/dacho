// dacho/src/renderer/primitive.rs

use super::{
    color::Color,
    vertex::{CubePosition, Vertex}
};

type TriangleData = (u16, u16, u16);
type     QuadData = (TriangleData, TriangleData);

pub type CubeIndicesData  = (QuadData, QuadData, QuadData, QuadData, QuadData, QuadData);
pub type CubeVerticesData = (Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex, Vertex);

struct Triangle;

impl Triangle {
    const fn new(
        a: u16,
        b: u16,
        c: u16
    ) -> TriangleData {
        (a, b, c)
    }
}

struct Quad;

impl Quad {
    const fn new(
        a: u16,
        b: u16,
        c: u16,
        d: u16
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
        x: i16,
        y: i16,
        z: i16,
        i: usize
    ) -> CubeVerticesData {
        let color = [Color::LIGHT, Color::WHITE][i % 2];

        (
            Vertex::new(CubePosition(x - 1, y + 1, z - 1), color),
            Vertex::new(CubePosition(x + 1, y + 1, z - 1), color),
            Vertex::new(CubePosition(x + 1, y + 1, z + 1), color),
            Vertex::new(CubePosition(x - 1, y + 1, z + 1), color),
            Vertex::new(CubePosition(x - 1, y - 1, z - 1), color),
            Vertex::new(CubePosition(x + 1, y - 1, z - 1), color),
            Vertex::new(CubePosition(x + 1, y - 1, z + 1), color),
            Vertex::new(CubePosition(x - 1, y - 1, z + 1), color)
        )
    }
}

pub struct CubeIndices;

impl CubeIndices {
    pub const fn new(i: u16) -> CubeIndicesData {
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

