use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use las::{point, reader, Reader};
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use core::f32;
use std::fs;
use std::path::PathBuf;
use std::{ffi::OsStr, path::Path};
use std::time::Instant;


use crate::Sector;

#[derive(Resource)]
struct PointCloudMesh {
    mesh: Mesh,
    min_x: f32,
    min_y: f32,
    max_x: f32,
    max_y: f32,
}

#[derive(Resource)]
struct World {
    meshes: Vec<PointCloudMesh>,
    origin_x: f32,
    origin_y: f32,

}

impl World {
    fn new(dir: &str) -> World {
        let now = Instant::now();

        let path = Path::new(dir);
        let extension: &OsStr = OsStr::new("las");
        let mut meshes: Vec<PointCloudMesh> = vec![];

        if path.is_dir() {
            println!("Loading dataset from {}...", path.display());
            for file in fs::read_dir(path).unwrap() {
                let file = file.unwrap();
                let path = file.path();

                if path.is_file() && path.extension() == Some(extension) {
                    let mesh = create_pc_mesh(path);
                    meshes.push(mesh);
                }
            }
        }

        let elapsed = now.elapsed();
        println!("Loading complete! {} meshes were loaded from disk in {:.2?} seconds", meshes.len(), elapsed);

        let mut origin_x: f32 = f32::INFINITY;
        let mut origin_y: f32 = f32::INFINITY;

        for mesh in &meshes {
            if mesh.min_x < origin_x {
                origin_x = mesh.min_x;
                origin_y = mesh.min_y; // Not the global minimum Y value, just the local minimum within the data block furthest left.
            }
        }

        World { meshes, origin_x, origin_y }
    }
}

pub fn game_loop() {
    let mut commands: Commands;
    let world = World::new("data");

    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(world)
        .add_systems(Startup, setup)
        .add_systems(Update, player_movement)
        .run();
}

#[derive(Resource)]
struct Initial_Player_Location {
    initial_x: f64,
    initial_y: f64,
}

#[derive(Component)]
struct Player;

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, world: Res<World>) {

    let mut initial_position = [f32::INFINITY, f32::INFINITY, 0.0];

    println!("Loading meshes...");
    let now = Instant::now();
    for idx in 0..world.meshes.len() {
        // Load the mesh.
        commands.spawn(PbrBundle {
            mesh: meshes.add(world.meshes[idx].mesh.clone()),
            ..Default::default()
        });

        // Find a good starting point to spawn the player.
        if world.meshes[idx].min_x < initial_position[0] {
            initial_position[0] = world.meshes[idx].min_x;
            initial_position[1] = world.meshes[idx].min_y;
        }
    }

    println!("Loaded meshes in {:.2?} seconds.", now.elapsed());
    println!("Starting view at {},{}", initial_position[0], initial_position[1]);

    commands.spawn(TransformBundle::from_transform(Transform::from_xyz(initial_position[0], initial_position[1], 1500.0)))
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

// fn create_pc_mesh(sector: &Sector) -> Mesh {
//     let mut reader = Reader::from_path(sector.data_file.clone()).unwrap();
//     let points: Vec<[f32; 3]> = reader.points().map(|point|{
//         let point = point.unwrap();
//         [point.x as f32, point.y as f32, point.z as f32]
//     }).collect();
// 
//     let mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default())
//     .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points);
//     mesh
// }   

fn create_pc_mesh(path: PathBuf) -> PointCloudMesh {
    let mut reader = Reader::from_path(path).unwrap();
    let header = reader.header().clone().into_raw().unwrap();

    let points: Vec<[f32; 3]> = reader.points().map(|point| {
        let point = point.unwrap();
        [point.x as f32, point.y as f32, point.z as f32]
    }).collect();

    let mesh = Mesh::new(PrimitiveTopology::PointList, RenderAssetUsages::default())
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, points);

    PointCloudMesh { mesh, 
        min_x: header.min_x as f32,
        min_y: header.min_y as f32,
        max_x: header.max_x as f32,
        max_y: header.max_y as f32 }
}


// pub fn bevy_test(sector: &Sector, origin_x: f64, origin_y: f64) {
// 
//     let mut commands: Commands;
//     let pc_mesh = create_pc_mesh(sector);
//     
// 
//     App::new()
//     .add_plugins(DefaultPlugins)
//     .insert_resource(Initial_Player_Location {initial_x: origin_x, initial_y: origin_y})
//     .insert_resource(PointCloudMesh{ mesh: pc_mesh })
//     .insert_resource(World {})
//     .add_systems(Startup, setup)
//     .add_systems(Update, player_movement)
//     .run();
// }