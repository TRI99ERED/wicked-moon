use avian2d::prelude::*;
use bevy::{asset::AssetMetaCheck, prelude::*};

mod audio;
mod controls;
mod game;
mod main_camera;
mod screen;
mod ui;

use audio::AudioPlugin;
use game::GamePlugin;
use main_camera::MainCameraPlugin;
use screen::ScreenPlugin;

pub struct AppPlugin;

impl Plugin for AppPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            PhysicsPlugins::default(),
            AudioPlugin,
            GamePlugin,
            MainCameraPlugin,
            ScreenPlugin,
        ))
        .configure_sets(
            Update,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        )
        .configure_sets(
            FixedUpdate,
            (AppSet::TickTimers, AppSet::RecordInput, AppSet::Update).chain(),
        )
        // DEBUG
        .add_systems(Update, utils::systems::close_on_esc);
    }
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash)]
enum AppSet {
    TickTimers,
    RecordInput,
    Update,
}
