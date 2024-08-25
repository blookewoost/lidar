use bevy::{prelude::*, render::{mesh::VertexAttributeValues, render_asset::RenderAssetUsages}, transform::commands};
use las::{point, Reader};
use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

use crate::Sector;

#[derive(Resource)]
struct PointCloudMesh {
    mesh: Mesh
}

pub fn bevy_test(sector: Sector) {
    let mut commands: Commands;
    let pc_mesh = create_pc_mesh(sector);
    

    App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(PointCloudMesh{ mesh: pc_mesh })
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, newmesh: Res<PointCloudMesh>) {

    commands.spawn(PbrBundle {
        mesh: meshes.add(newmesh.mesh.clone()),
        ..Default::default()
    });

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(950000.0, 820000.0, 1000.0).looking_at(Vec3::ZERO, Vec3::Y),
        
        ..Default::default()
    });
}

fn some_function() {
    println!("Hello World!");
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

fn render_sector(sector: Sector) {
    let reader = Reader::from_path(sector.data_file).unwrap();

}