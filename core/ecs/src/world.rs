// dacho/core/ecs/src/world.rs

use std::rc::Rc;

use super::entity::{Entity, Tuple};

pub struct World {
    pub entities: Vec<Rc<Entity>>,
        temp:     bool
}

impl World {
    #[expect(clippy::new_without_default, reason = "default would just be empty")]
    pub fn new() -> Self {
        Self {
            entities: vec![],
            temp:     true
        }
    }

    pub fn spawn<T>(&mut self, components: T)
    where
        T: Tuple
    {
        let mut entity = Entity::new();
        components.insert_into(&mut entity.components);

        self.entities.push(Rc::new(entity));
    }

    pub fn get_updated_mesh_instances(&mut self) -> Vec<(u32, Vec<f32>)> {
        if self.temp {
            self.temp = false;

            return vec![
                (
                    0,
                    dacho_mesh::Mesh::quad(
                        dacho_types::V3::ZERO,
                        dacho_types::V2::ONE
                    ).model_matrix.to_cols_array().to_vec()
                )
            ];
        }

        vec![]
    }
}

