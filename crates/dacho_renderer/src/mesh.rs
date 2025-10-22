// dacho/crates/dacho_renderer/src/mesh.rs

pub type   Vertex = [f32;   VERTEX_SIZE];
pub type    Index = [u32;    INDEX_SIZE];
pub type Instance = [f32; INSTANCE_SIZE];

pub const   VERTEX_SIZE: usize = 2;
pub const    INDEX_SIZE: usize = 3;
pub const INSTANCE_SIZE: usize = 2;

pub trait Mesh {
    fn vertices() -> impl IntoIterator<Item = Vertex>;
    fn  indices() -> impl IntoIterator<Item =  Index>;
}

pub struct Quad;
impl Mesh for Quad {
    fn vertices() -> impl IntoIterator<Item = Vertex> {
        [
            [-0.5, -0.5],
            [-0.5,  0.5],
            [ 0.5, -0.5],
            [ 0.5,  0.5]
        ]
    }

    fn indices() -> impl IntoIterator<Item = Index> {
        [
            [0, 1, 2],
            [2, 1, 3]
        ]
    }
}

pub struct Circle;
impl Mesh for Circle {
    fn vertices() -> impl IntoIterator<Item = Vertex> {
        [
            [ 0.0, -0.50],
            [ 0.0,  0.00],
            [ 0.5, -0.25],
            [ 0.5,  0.25],
            [ 0.0,  0.50],
            [-0.5,  0.25],
            [-0.5, -0.25]
        ]
    }

    fn indices() -> impl IntoIterator<Item = Index> {
        [
            [0, 1, 2],
            [2, 1, 3],
            [3, 1, 4],
            [4, 1, 5],
            [5, 1, 6],
            [6, 1, 0]
        ]
    }
}

