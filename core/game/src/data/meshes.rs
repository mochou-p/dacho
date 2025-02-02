// dacho/core/game/src/data/meshes.rs

use std::collections::HashMap;

use dacho_components::Mesh;


#[derive(Default)]
#[non_exhaustive]
pub struct Meshes {
    pub updated: bool,
    pub data:    HashMap<u32, Vec<f32>>
}

impl Meshes {
    pub fn push(&mut self, mesh: Mesh) {
        self.data
            .entry(mesh.id)
            .and_modify(|vec| vec.extend(
                mesh.model_matrix
                    .to_cols_array()
            ))
            .or_insert(
                mesh.model_matrix
                    .to_cols_array()
                    .into()
            );

        self.updated = true;
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }
}

