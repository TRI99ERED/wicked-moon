use bevy::{color::palettes::css::LIGHT_GREEN, prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}};

const PLAYER_Z: f32 = 1.;

pub(crate) struct PlayerPlugin;

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
    let mesh = Mesh2dHandle(meshes.add(Triangle2d::new(
        Vec2::new(-50., 0.),
        Vec2::new(0., 100.),
        Vec2::new(50., 0.),
    )));
    let material = materials.add(Color::from(LIGHT_GREEN));

    commands.spawn((
        Player,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(Vec2::new(0., -250.).extend(PLAYER_Z)),
            ..Default::default()
        }
    ));
}
