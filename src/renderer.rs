use bevy::{ecs::query, prelude::*, render::render_asset::RenderAssetUsages};
use las::Reader;
use bevy::render::mesh::{Mesh, PrimitiveTopology};

use crate::Sector;

#[derive(Resource)]
struct PointCloudMesh {
    mesh: Mesh
}

#[derive(Component)]
struct Player;

pub fn bevy_test(sector: Sector) {
    let mut commands: Commands;
    let pc_mesh = create_pc_mesh(sector);
    

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(PointCloudMesh{ mesh: pc_mesh })
    .add_systems(Startup, setup)
    .add_systems(Update, player_movement)
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, newmesh: Res<PointCloudMesh>) {

    commands.spawn(PbrBundle {
        mesh: meshes.add(newmesh.mesh.clone()),
        ..Default::default()
    });

    commands.spawn(TransformBundle::from_transform(Transform::from_xyz(950000.0, 820000.0, 500.0)))
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

fn create_pc_mesh(sector: Sector) -> Mesh {
    let mut reader = Reader::from_path(sector.data_file).unwrap();
    let points: Vec<[f32; 3]> = reader.points().map(|point|{
        let point = point.unwrap();
        [point.x as f32, point.y as f32, point.z as f32]
    }).collect();

    let mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default())
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points);
    mesh
}   
