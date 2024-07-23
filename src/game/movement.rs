use avian2d::prelude::*;
use bevy::prelude::*;
use utils::input::Pressed;

use crate::{
    controls::{MoveDown, MoveUp, RotateLeft, RotateRight},
    AppSet,
};

pub(super) struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        // Record directional input as movement controls.
        app.register_type::<MovementController>()
            .add_systems(
                Update,
                record_movement_controller.in_set(AppSet::RecordInput),
            )
            // Apply movement based on controls.
            .register_type::<Movement>()
            .add_systems(
                FixedUpdate,
                (apply_movement, apply_rotation).in_set(AppSet::Update),
            );
    }
}

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct MovementController(pub Vec2);

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Movement {
    pub speed: f32,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Rotation {
    pub speed: f32,
}

fn record_movement_controller(
    input: Res<ButtonInput<KeyCode>>,
    mut controller_query: Query<&mut MovementController>,
) {
    // Collect directional input.
    let mut intent = Vec2::ZERO;
    if MoveUp::pressed(&input) {
        intent.y += 1.0;
    }
    if MoveDown::pressed(&input) {
        intent.y -= 1.0;
    }
    if RotateLeft::pressed(&input) {
        intent.x -= 1.0;
    }
    if RotateRight::pressed(&input) {
        intent.x += 1.0;
    }

    intent = intent.normalize_or_zero();

    // Apply movement intent to controllers.
    for mut controller in &mut controller_query {
        controller.0 = intent;
    }
}

fn apply_movement(
    mut movement_query: Query<(&MovementController, &Movement, &mut LinearVelocity)>,
) {
    for (controller, movement, mut velocity) in &mut movement_query {
        velocity.y = movement.speed * controller.0.y;
    }
}

fn apply_rotation(
    mut rotation_query: Query<(&MovementController, &Rotation, &mut AngularVelocity)>,
) {
    for (controller, rotation, mut velocity) in &mut rotation_query {
        velocity.0 = -rotation.speed * controller.0.x;
    }
}
