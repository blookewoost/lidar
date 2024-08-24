use std::{cmp::min, path::PathBuf};
use las::Reader;

pub fn approximate_file_as_plane(path: &str) { 
    let mut reader = Reader::from_path(path).unwrap();
    let header = reader.header().clone();
    let header = header.into_raw().unwrap();

    let min_x = header.min_x;
    let min_y = header.min_y;

    for wrapped_point in reader.points() {
        let point = wrapped_point.unwrap();

        let dx = point.x - min_x;
        let dy = point.y - min_y;
        println!("This point is {},{} from the origin, at a height of {}", dx, dy, point.z);
    }

}

pub fn write_to_las() {

}



