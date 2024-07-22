use avian2d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    game::movement::{MovementController, Rotation},
    screen::Screen,
};

const MOON_Z: f32 = 1.;

pub(super) struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut App) {
        app.observe(on_spawn_moon);
    }
}

#[derive(Event)]
pub struct SpawnMoon;

#[derive(Component)]
pub struct Moon;

fn on_spawn_moon(
    _trigger: Trigger<SpawnMoon>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(Rectangle::new(50., 50.)));
    let material = materials.add(Color::WHITE);

    commands.spawn((
        Moon,
        MovementController::default(),
        Rotation { speed: 10. },
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec2::ZERO.extend(MOON_Z)),
            ..Default::default()
        },
        StateScoped(Screen::Playing),
        RigidBody::Kinematic,
        Collider::circle(50.),
        LockedAxes::TRANSLATION_LOCKED,
    ));
}
