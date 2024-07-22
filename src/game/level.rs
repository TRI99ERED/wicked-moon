use avian2d::prelude::{Collider, RigidBody};
use bevy::{prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

pub mod moon;
pub mod player;

use moon::{MoonPlugin, SpawnMoon};
use player::{PlayerPlugin, SpawnPlayer};

use crate::screen::Screen;

pub(super) struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((MoonPlugin, PlayerPlugin))
            .observe(on_spawn_level);
    }
}

#[derive(Event)]
pub struct SpawnLevel;

fn on_spawn_level(
    _trigger: Trigger<SpawnLevel>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.trigger(SpawnMoon);
    commands.trigger(SpawnPlayer);

    // Bottom Player stoper
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(Vec2::new(0., -500.).extend(0.))),
        StateScoped(Screen::Playing),
        RigidBody::Static,
        Collider::rectangle(50., 50.),
    ));
    
    let mesh = Mesh2dHandle(meshes.add(Annulus::new(475., 1000.)));
    let material = materials.add(Color::WHITE);

    commands.spawn(MaterialMesh2dBundle {
        mesh,
        material,
        ..Default::default()
    });
}
