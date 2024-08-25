use bevy::prelude::*;
use las::Reader;
use crate::Sector;

pub fn bevy_test() {
    App::new().add_systems(Update, some_function)
    .run();
}

fn some_function() {
    println!("Hello World!");
}

fn render_sector(sector: Sector) {
    let reader = Reader::from_path(sector.data_file).unwrap();
}