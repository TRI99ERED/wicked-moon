use bevy::prelude::*;

pub mod level;
pub mod assets;

use assets::AssetsPlugin;
use level::LevelPlugin;

pub(super) struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((LevelPlugin, AssetsPlugin));
    }
}
