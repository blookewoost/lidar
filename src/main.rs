use las::Reader;
use std::{ffi::OsStr, fs, path::{Path, PathBuf}};

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

    let mut reader = Reader::from_path(path.clone()).unwrap();
    println!("Loading dataset: {}...", path.display());
    
    // Initialize our sector's boundary values.
    let mut min_x: f64;
    let mut min_y: f64;
    let mut max_x: f64;
    let mut max_y: f64;

    // Grab an xy pair from the dataset
    let (x, y) = (reader.read_point().unwrap().unwrap().x, reader.read_point().unwrap().unwrap().y);
    (min_x, min_y, max_x, max_y) = (x, y, x, y);

    for wrapped_point in reader.points() {
        let point = wrapped_point.unwrap();
        
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }

    Sector { min_x, min_y, max_x, max_y }
}

fn main() {
    
    let grid: Grid = generate_grid("data");
    for sector in grid.sectors {
        println!("SE: ({}, {}), NW ({},{})", sector.min_x, sector.min_y, sector.max_x, sector.max_y);
    }
}
