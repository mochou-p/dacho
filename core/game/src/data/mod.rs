// dacho/core/game/src/data/mod.rs

pub mod commands;
pub mod meshes;

use {
    super::{events::Events, Time},
    commands::Commands, meshes::Meshes
};

use dacho_components::Camera;


#[expect(clippy::exhaustive_structs, reason = "for now created by struct expr + ..default")]
pub struct Data<GD, GE> {
    pub game:   GD,
    pub engine: EngineData<GE>
}

// not derive to not expect Default from GE
impl<GD, GE> Default for Data<GD, GE>
where
    GD: Default
{
    fn default() -> Self {
        Self {
            game:   GD::default(),
            engine: EngineData::<GE>::default()
        }
    }
}

#[expect(clippy::exhaustive_structs, reason = "for now created by struct expr + ..default")]
pub struct EngineData<GE> {
    pub time:     Time,
    pub camera:   Camera,
    pub commands: Commands,
    pub events:   Events<GE>,
    pub meshes:   Meshes
}

// not derive to not expect Default from GE
impl<GE> Default for EngineData<GE> {
    fn default() -> Self {
        Self {
            time:     Time    ::default(),
            camera:   Camera  ::default(),
            commands: Commands::default(),
            events:   Events  ::default(),
            meshes:   Meshes  ::default()
        }
    }
}
