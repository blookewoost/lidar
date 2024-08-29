use bevy::prelude::*;
use std::time::Instant;
mod resources;

use resources::World;

fn main() {
    let world = World::new("data");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(world)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, world: Res<World>) {

    println!("Loading meshes...");
    let now = Instant::now();
    for idx in 0..world.meshes.len() {
        // Load the mesh.
        commands.spawn(PbrBundle {
            mesh: meshes.add(world.meshes[idx].mesh.clone()),
            ..Default::default()
        });
    }

    println!("Loaded meshes in {:.2?} seconds.", now.elapsed());

    commands.spawn(TransformBundle::from_transform(Transform::from_xyz(world.origin_x, world.origin_y, 1500.0)))
    .insert(Player)
    .with_children(|player| {
        player.spawn( Camera3dBundle{
            transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        });
    });
}

fn player_movement(keys: Res<ButtonInput<KeyCode>>, time: Res<Time>, mut query: Query<&mut Transform, With<Player>>) {
    for mut transform in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            direction.y += 10.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            direction.x += 10.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            direction.y -= 10.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            direction.x -= 10.0;
        }
        if keys.pressed(KeyCode::ArrowUp) {
            direction.z += 5.0;
        }
        if keys.pressed(KeyCode::ArrowDown) {
            direction.z -= 5.0;
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
            transform.translation += direction * time.delta_seconds() * 200.0;
        }
    }
}
