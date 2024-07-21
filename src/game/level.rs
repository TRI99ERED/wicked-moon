use bevy::prelude::*;

pub mod player;

use player::{PlayerPlugin, SpawnPlayer};

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((PlayerPlugin,))
        .observe(on_spawn_level);
    }
}

#[derive(Event)]
pub struct SpawnLevel;

fn on_spawn_level(_trigger: Trigger<SpawnLevel>, mut commands: Commands) {
    commands.trigger(SpawnPlayer);
}
