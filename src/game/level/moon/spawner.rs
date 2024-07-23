use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{screen::Screen, AppSet};

use super::bullet::{BulletType, Simple, SpawnBullet};

pub(super) struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (
                tick_spawner_timer::<Simple>.in_set(AppSet::TickTimers),
                shoot_bullet::<Simple>.in_set(AppSet::Update),
            ),
        )
        .observe(on_spawn_spawner::<Simple, Straight>);
    }
}

pub trait SpawnerType: Component + Clone {}

#[derive(Event)]
pub struct SpawnSpawner<B: BulletType, S: SpawnerType> {
    spawner: Spawner<B>,
    spawner_type: S,
    parent: Entity,
    transform: Transform,
}

impl<B: BulletType, S: SpawnerType> SpawnSpawner<B, S> {
    pub fn new(spawner: Spawner<B>, spawner_type: S, parent: Entity, transform: Transform) -> Self {
        Self {
            spawner,
            spawner_type,
            parent,
            transform,
        }
    }
}

#[derive(Component, Clone)]
pub struct Spawner<B: BulletType> {
    spawn_timer: Timer,
    _phantom_data: PhantomData<B>,
}

impl<B: BulletType> Spawner<B> {
    pub fn new(spawn_cd: f32) -> Self {
        Self {
            spawn_timer: Timer::from_seconds(spawn_cd, TimerMode::Repeating),
            _phantom_data: PhantomData,
        }
    }
}

#[derive(Component, Clone)]
pub struct Straight;

impl SpawnerType for Straight {}

#[derive(Component, Clone)]
pub struct Spin {
    speed: f32,
}

impl Spin {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl SpawnerType for Spin {}

#[derive(Component, Clone)]
pub struct Oscilating {
    speed: f32,
}

impl Oscilating {
    pub fn new(speed: f32) -> Self {
        Self { speed }
    }
}

impl SpawnerType for Oscilating {}

fn on_spawn_spawner<B: BulletType, S: SpawnerType>(
    trigger: Trigger<SpawnSpawner<B, S>>,
    mut commands: Commands,
) {
    commands
        .entity(trigger.event().parent)
        .with_children(|parent| {
            parent.spawn((
                trigger.event().spawner.clone(),
                trigger.event().spawner_type.clone(),
                SpatialBundle::from_transform(trigger.event().transform),
                StateScoped(Screen::Playing),
            ));
        });
}

fn tick_spawner_timer<B: BulletType>(
    mut spawner_query: Query<&mut Spawner<B>>,
    time: Res<Time<Fixed>>,
) {
    for mut spawner in &mut spawner_query {
        spawner.spawn_timer.tick(time.delta());
    }
}

fn shoot_bullet<B: BulletType>(
    mut commands: Commands,
    spawner_query: Query<(&Parent, &Transform, &Spawner<B>)>,
) {
    for (parent, transform, spawner) in &spawner_query {
        if spawner.spawn_timer.just_finished() {
            commands.trigger(SpawnBullet::<B>::new(parent.get(), transform.clone()));
        }
    }
}
