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

use super::moon::bullet::Bullet;

const PLAYER_SPAWN_POS: Vec2 = Vec2::new(0., -250.);
const PLAYER_Z: f32 = 1.;

const PLAYER_SHAPE: Triangle2d = Triangle2d::new(
    Vec2::new(-20., -10.),
    Vec2::new(0., 24.),
    Vec2::new(20., -10.),
);
const PLAYER_HITBOX_RADIUS: f32 = 10.;
const PLAYER_SPEED: f32 = 200.;

pub(super) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, collide_with_bullet)
            .observe(on_spawn_player);
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
            Movement {
                speed: PLAYER_SPEED,
            },
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

fn collide_with_bullet(
    mut collision_event_reader: EventReader<CollisionStarted>,
    player_query: Query<Entity, With<Player>>,
    bullet_query: Query<Entity, With<Bullet>>,
) {
    if let Ok(player) = player_query.get_single() {
        for &CollisionStarted(entity1, entity2) in collision_event_reader.read() {
            if entity1 == player {
                if bullet_query.iter().any(|e| e == entity2) {
                    println!("hit");
                }
            } else if entity2 == player {
                if bullet_query.iter().any(|e| e == entity1) {
                    println!("hit");
                }
            }
        }
    }
}
