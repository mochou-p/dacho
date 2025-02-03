// dacho/core/game/src/data/mod.rs

pub mod commands;
pub mod meshes;

use {commands::Commands, meshes::Meshes};

use dacho_components::Camera;


#[derive(Default)]
#[expect(clippy::exhaustive_structs, reason = "for now created by struct expr + ..default")]
pub struct Data<GD> {
    pub game:   GD,
    pub engine: EngineData
}

#[derive(Default)]
#[expect(clippy::exhaustive_structs, reason = "for now created by struct expr + ..default")]
pub struct EngineData {
    pub camera:   Camera,
    pub commands: Commands,
    pub meshes:   Meshes
}

