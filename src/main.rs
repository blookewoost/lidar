use las::{point::Classification, Point, Reader};
use core::f64;
use std::{default, ffi::OsStr, fs::{self, read}, path::{Path, PathBuf}};

#[derive(Default, Debug, Clone, Copy)]
struct Sector {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

#[derive(Default, Debug, Clone)]
struct Grid {
    sectors: Vec<Sector>,
    global_min_x: f64,
    global_min_y: f64,
    global_max_x: f64,
    global_max_y: f64,
}

impl Grid {
    pub fn new(sectors: Vec<Sector>) -> Grid {
        
        // Yuck! Generate iterators over the sectors and collapse them to the global min/max
        let global_max_x = sectors.iter().map(|s| s.max_x).fold(f64::NEG_INFINITY, |a, s| a.max(s));
        let global_max_y = sectors.iter().map(|s| s.max_y).fold(f64::NEG_INFINITY, |a, s| a.max(s));
        let global_min_x = sectors.iter().map(|s| s.min_x).fold(f64::INFINITY, |a, s| a.min(s));
        let global_min_y: f64 = sectors.iter().map(|s| s.min_y).fold(f64::INFINITY, |a, s| a.min(s));

        Grid { sectors, global_min_x, global_min_y, global_max_x, global_max_y }
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

    Sector { min_x, min_y, max_x, max_y }
}

fn main() {
    
    let grid: Grid = generate_grid("data");
    println!("grid origin (southeast) is ({},{})", grid.global_min_x, grid.global_min_y);
    println!("grid maxima (northwest) is ({},{})", grid.global_max_x, grid.global_max_y); 
    
}
