use crate::Grid;

pub fn dist_to_origin(grid: Grid) {
    for sector in grid.sectors {
        let mut reader = las::Reader::from_path(sector.data_file).unwrap();
        for wrapped_point in reader.points() {
            let point = wrapped_point.unwrap();
            let dx = point.x - grid.origin_x;
            let dy = point.y - grid.origin_y;

            println!("This point is ({},{}) relative to the grid origin at Z: {}", dx, dy, point.z);
        }
    }
}



