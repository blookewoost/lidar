use drone::{main_loop, Drone};
use files::dist_to_origin;
use las::Reader;
use core::f64;
use std::fs;
use std::{ffi::OsStr, path::{Path, PathBuf}};
mod files;
mod drone;

#[derive(Debug, Clone)]
struct Sector {
    data_file: PathBuf,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

#[derive(Default, Debug, Clone)]
struct Grid {
    sectors: Vec<Sector>,
    origin_x: f64,
    origin_y: f64,
}

impl Grid {
    pub fn new(sectors: Vec<Sector>) -> Grid {
        
        // Yuck! Generate iterators over the sectors and collapse them to the global min/max
        let origin_x = sectors.iter().map(|s| s.max_x).fold(f64::NEG_INFINITY, |a, s| a.max(s));
        let origin_y = sectors.iter().map(|s| s.max_y).fold(f64::NEG_INFINITY, |a, s| a.max(s));

        Grid { sectors, origin_x, origin_y }
    }
}


fn generate_grid(dir: &str) -> Grid {

    let path = Path::new(dir);
    let extension: &OsStr = OsStr::new("las");
    let mut sectors: Vec<Sector> = vec![];

    if path.is_dir() {
        println!("Loading dataset from {}", path.display());
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();
            let path = file.path();

            if path.is_file() && path.extension() == Some(extension) {
                let sector = generate_sector(path);
                sectors.push(sector);
            }
        }
    }   
    return Grid::new(sectors);
}

fn generate_sector(path: PathBuf) -> Sector {

    let reader = Reader::from_path(path.clone()).unwrap();
    
    // Read the boundary values from the header.
    let header = reader.header().clone();
    let head = header.into_raw().unwrap();

    let min_x: f64 = head.min_x;
    let min_y: f64 = head.min_y;
    let max_x: f64 = head.max_x;
    let max_y: f64 = head.max_y;

    Sector { data_file: path, min_x, min_y, max_x, max_y }
}

fn main() {
    
    let grid: Grid = generate_grid("data");
    println!("grid origin (southeast) is ({},{})", grid.origin_x, grid.origin_y);
    // dist_to_origin(grid);

    let mut drone = Drone::new(1000, 1000);
    main_loop(&mut drone);
    
}
