use las::Reader;
use renderer::game_loop;
use core::f64;
use std::fs;
use std::{ffi::OsStr, path::{Path, PathBuf}};

mod files;
mod renderer;

#[derive(Default, Debug, Clone)]
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
    origin_idx: usize,
    origin_x: f64,
    origin_y: f64,
}

impl Grid {
    pub fn new(sectors: Vec<Sector>) -> Grid {

        let mut origin_x = f64::INFINITY;
        let mut origin_idx: usize = 0;

        for idx in 0..sectors.len() {
            if sectors[idx].min_x < origin_x {
                origin_x = sectors[idx].min_x;
                origin_idx = idx;
            }
        }

        let origin_y = sectors[origin_idx].min_y;
        Grid { sectors, origin_idx, origin_x, origin_y }
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
    
    // let grid: Grid = generate_grid("data");
    // let origin = grid.origin_idx;
    game_loop();
    //bevy_test();
    //main_loop(&mut drone);
    
}
