use std::marker::PhantomData;

use avian2d::prelude::*;
use bevy::{
    color::palettes::css::BLUE,
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{screen::Screen, AppSet};

pub(super) struct BulletPlugin;

impl Plugin for BulletPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                tick_life_timers.in_set(AppSet::TickTimers),
                (destroy_timed_out, move_bullets::<Simple>)
                    .chain()
                    .in_set(AppSet::Update),
            ),
        )
        .observe(on_spawn_bullet::<Simple>);
    }
}

pub trait BulletType: Component + Default + Clone {
    fn life() -> f32;
    fn speed() -> f32;
    fn hitbox_radius() -> f32;
    fn mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle;
    fn material(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial>;
}

#[derive(Event)]
pub struct SpawnBullet<B: BulletType> {
    parent: Entity,
    transform: Transform,
    _phantom_data: PhantomData<B>,
}

impl<B: BulletType> SpawnBullet<B> {
    pub fn new(parent: Entity, transform: Transform) -> Self {
        Self {
            parent,
            transform,
            _phantom_data: PhantomData,
        }
    }
}

#[derive(Component)]
pub struct Bullet {
    life_timer: Timer,
}

impl Bullet {
    pub fn new(life: f32) -> Self {
        Self {
            life_timer: Timer::from_seconds(life, TimerMode::Once),
        }
    }
}

#[derive(Component, Default, Clone)]
pub struct Simple;

impl BulletType for Simple {
    fn life() -> f32 {
        5.
    }

    fn speed() -> f32 {
        100.
    }

    fn hitbox_radius() -> f32 {
        5.
    }

    fn mesh(meshes: &mut ResMut<Assets<Mesh>>) -> Mesh2dHandle {
        Mesh2dHandle(meshes.add(Circle::new(Self::hitbox_radius())))
    }

    fn material(materials: &mut ResMut<Assets<ColorMaterial>>) -> Handle<ColorMaterial> {
        materials.add(Color::from(BLUE))
    }
}

fn on_spawn_bullet<B: BulletType>(
    trigger: Trigger<SpawnBullet<B>>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mesh = B::mesh(&mut meshes);
    let material = B::material(&mut materials);
    let transform = trigger.event().transform;

    commands
        .entity(trigger.event().parent)
        .with_children(|parent| {
            parent.spawn((
                Bullet::new(B::life()),
                B::default(),
                MaterialMesh2dBundle {
                    mesh,
                    material,
                    transform: transform.with_translation(transform.translation.with_z(10.)),
                    ..Default::default()
                },
                StateScoped(Screen::Playing),
                Collider::circle(B::hitbox_radius()),
                Sensor,
            ));
        });
}

fn tick_life_timers(mut bullet_query: Query<&mut Bullet>, time: Res<Time<Fixed>>) {
    for mut bullet in &mut bullet_query {
        bullet.life_timer.tick(time.delta());
    }
}

fn destroy_timed_out(mut commands: Commands, bullet_query: Query<(Entity, &Bullet)>) {
    for (entity, bullet) in &bullet_query {
        if bullet.life_timer.just_finished() {
            commands.entity(entity).despawn_recursive();
        }
    }
}

fn move_bullets<B: BulletType>(
    mut bullet_query: Query<&mut Transform, (With<Bullet>, With<B>)>,
    time: Res<Time<Fixed>>,
) {
    let delta = time.delta_seconds();
    for mut transform in &mut bullet_query {
        transform.translation += B::speed() * delta * Vec3::X;
    }
}
