use bevy::prelude::*;

pub mod assets;
pub mod level;
mod movement;

use assets::AssetsPlugin;
use level::LevelPlugin;
use movement::MovementPlugin;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LevelPlugin, AssetsPlugin, MovementPlugin));
    }
}
