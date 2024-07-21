use bevy::prelude::*;

pub mod sfx;
pub mod soundtrack;

use sfx::SfxPlugin;
use soundtrack::SoundtrackPlugin;

pub(super) struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SfxPlugin, SoundtrackPlugin));
    }
}
