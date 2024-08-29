use bevy::{prelude::*, render::render_asset::RenderAssetUsages};
use std::time::Instant;
use std::{fs, ffi::OsStr, path::{Path, PathBuf}};
use bevy::render::mesh::{Mesh, PrimitiveTopology};
use las::Reader;

#[derive(Resource)]
pub struct PointCloudMesh {
    pub mesh: Mesh,
    pub min_x: f32,
    pub min_y: f32,
}

#[derive(Resource)]
pub struct World {
    pub meshes: Vec<PointCloudMesh>,
    pub origin_x: f32,
    pub origin_y: f32,
}

impl World {
    pub fn new(dir: &str) -> World {
        let now = Instant::now();

        let path = Path::new(dir);
        let extension: &OsStr = OsStr::new("las");
        let mut meshes: Vec<PointCloudMesh> = vec![];

        if path.is_dir() {
            println!("Loading dataset from /{}...", path.display());
            
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
        min_y: header.min_y as f32 }
}

