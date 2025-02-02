// dacho/core/game/src/data/mod.rs

pub mod commands;
pub mod meshes;

use {commands::Commands, meshes::Meshes};

use dacho_components::Camera;


#[derive(Default)]
#[non_exhaustive]
pub struct Data<GD> {
    pub game:   GD,
    pub engine: EngineData
}

#[derive(Default)]
#[non_exhaustive]
pub struct EngineData {
    pub camera:   Camera,
    pub commands: Commands,
    pub meshes:   Meshes
}

