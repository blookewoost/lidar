use las::{point::Classification, Point, Reader};
use std::{ffi::OsStr, fs::{self, read}, path::{Path, PathBuf}};

struct Sector {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

struct Grid {
    sectors: Vec<Sector>,
}


fn generate_grid(dir: &str) -> Grid {

    let path = Path::new(dir);
    let extension: &OsStr = OsStr::new("las");
    let mut sectors: Vec<Sector> = vec![];

    if path.is_dir() {
        for file in fs::read_dir(path).unwrap() {
            let file = file.unwrap();
            let path = file.path();

            if path.is_file() && path.extension() == Some(extension) {
                let sector = generate_sector(path);
                sectors.push(sector);
            }
        }
    }   

    Grid { sectors }
}

fn generate_sector(path: PathBuf) -> Sector {

    let reader = Reader::from_path(path.clone()).unwrap();
    println!("Loading dataset: {}...", path.display());
    
    // Read the boundary values from the header.
    let header = reader.header().clone();
    let head = header.into_raw().unwrap();

    let min_x: f64 = head.min_x;
    let min_y: f64 = head.min_y;
    let max_x: f64 = head.max_x;
    let max_y: f64 = head.max_y;

    Sector { min_x, min_y, max_x, max_y }
}

fn check_fields(point: Point) {

}

fn main() {
    
    let grid: Grid = generate_grid("data");
    for sector in grid.sectors {
        println!("SE: ({}, {}), NW ({},{})", sector.min_x, sector.min_y, sector.max_x, sector.max_y);
    }
    
}
