// dacho/core/ecs/src/world.rs

use super::{
    entity::{Entity, Tuple},
    query::{Query, QueryT}
};

type System = Box<dyn Fn(&Vec<Entity>)>;

pub struct World {
    entities: Vec<Entity>,
    systems:  Vec<System>,
    temp:     bool
}

impl World {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            entities: vec![],
            systems:  vec![],
            temp:     true
        }
    }

    pub fn spawn<T>(&mut self, components: T)
    where
        T: Tuple
    {
        let mut entity = Entity::new();
        components.insert_into(&mut entity.components);

        self.entities.push(entity);
    }

    pub fn add_system<T>(&mut self, system: fn(Query<T>))
    where
        T: QueryT + 'static
    {
        self.systems.push(Box::new(move |entities| {
            let mut query = Query::<T>::new();

            for entity in entities {
                if T::check(&entity.components) {
                    query.add(entity);
                }
            }

            if !query.entities.is_empty() {
                system(query);
            }
        }));
    }

    pub fn run(&mut self) {
        for system in &self.systems {
            system(&self.entities);
        }
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

