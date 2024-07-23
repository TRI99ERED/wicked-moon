use std::f32::consts::{FRAC_PI_4, FRAC_PI_6, PI};

use avian2d::prelude::{Collider, LockedAxes, RigidBody};
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

pub mod bullet;
pub mod spawner;

use bullet::{BulletPlugin, Simple};
use spawner::{SpawnSpawner, Spawner, SpawnerPlugin, Straight};

use crate::{
    game::movement::{MovementController, Rotation},
    screen::Screen,
};

const MOON_Z: f32 = 1.;

pub(super) struct MoonPlugin;

impl Plugin for MoonPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((BulletPlugin, SpawnerPlugin))
            .observe(on_spawn_moon);
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

    let moon = commands
        .spawn((
            Moon,
            MovementController::default(),
            Rotation { speed: 1. },
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
        ))
        .id();

    #[inline(always)]
    fn spawn_straight_spawner(commands: &mut Commands, moon: Entity, cd: f32, angle: f32) {
        commands.trigger(SpawnSpawner::new(
            Spawner::<Simple>::new(cd),
            Straight,
            moon,
            Transform::from_rotation(Quat::from_axis_angle(Vec3::Z, angle)),
        ));
    }

    spawn_straight_spawner(&mut commands, moon, 1., 0.);
    spawn_straight_spawner(&mut commands, moon, 1., -FRAC_PI_4);
    spawn_straight_spawner(&mut commands, moon, 1., FRAC_PI_4);
    spawn_straight_spawner(&mut commands, moon, 1., -FRAC_PI_6);
    spawn_straight_spawner(&mut commands, moon, 1., FRAC_PI_6);
    spawn_straight_spawner(&mut commands, moon, 1., -PI / 12.);
    spawn_straight_spawner(&mut commands, moon, 1., PI / 12.);
}
