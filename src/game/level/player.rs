use avian2d::prelude::*;
use bevy::{
    color::palettes::css::LIGHT_GREEN,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    game::movement::{Movement, MovementController},
    screen::Screen,
};

const PLAYER_SPAWN_POS: Vec2 = Vec2::new(0., -250.);
const PLAYER_Z: f32 = 1.;

const PLAYER_SHAPE: Triangle2d = Triangle2d::new(
    Vec2::new(-20., -10.),
    Vec2::new(0., 24.),
    Vec2::new(20., -10.),
);
const PLAYER_HITBOX_RADIUS: f32 = 10.;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.observe(on_spawn_player);
    }
}

#[derive(Event)]
pub struct SpawnPlayer;

#[derive(Component)]
pub struct Player;

fn on_spawn_player(
    _trigger: Trigger<SpawnPlayer>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = Mesh2dHandle(meshes.add(PLAYER_SHAPE));
    let material = materials.add(Color::from(LIGHT_GREEN));

    commands
        .spawn((
            Player,
            MovementController::default(),
            Movement { speed: 100. },
            MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(PLAYER_SPAWN_POS.extend(PLAYER_Z)),
                ..Default::default()
            },
            StateScoped(Screen::Playing),
            RigidBody::Dynamic,
            Collider::circle(PLAYER_HITBOX_RADIUS),
            LockedAxes::ROTATION_LOCKED.lock_translation_x(),
        ))
        .with_children(|p| {
            let mesh = Mesh2dHandle(meshes.add(Circle::new(PLAYER_HITBOX_RADIUS)));
            let material = materials.add(Color::WHITE.with_alpha(0.5));

            p.spawn(MaterialMesh2dBundle {
                mesh,
                material,
                transform: Transform::from_translation(Vec2::ZERO.extend(PLAYER_Z + 1.)),
                ..Default::default()
            });
        });
}
