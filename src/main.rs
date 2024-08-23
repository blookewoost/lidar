use las::Reader;

struct Boundary {
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
}

impl Boundary {
    // TODO!
}

fn main() {
    let mut reader = Reader::from_path("./data/32080_01_91.las").unwrap();
    
    let mut min_x: f64;
    let mut min_y: f64;
    let mut max_x: f64;
    let mut max_y: f64;

    // yikes
    (min_x, min_y) = (reader.read_point().unwrap().unwrap().x, reader.read_point().unwrap().unwrap().y);
    (max_x, max_y) = (reader.read_point().unwrap().unwrap().x, reader.read_point().unwrap().unwrap().y);

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

    let boundary = Boundary {min_x, min_y, max_x, max_y};

    println!("The boundaries of the dataset are btm left({:.46}, {:.46})...top right({:.46}, {:.46})", min_x, min_y, max_x, max_y);
}
