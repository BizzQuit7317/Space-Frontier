use serde::{Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Ship {
    Mass:f32,
    Speed:f32,
    Level:f32,
    AP:f32,
    DP:f32,
    Health:f32,   
}

impl Ship {
    fn new() -> Self {
        Ship {
            Mass:10.0,
            Speed:10.0,
            Level:1.0,
            AP:10.0,
            DP:10.0,
            Health:100.0
        }
    }
}

fn main() {
    let ship = vec![
        Ship::new(),
    ];

    let json = serde_json::to_string_pretty(&ship).expect("Failed to serialize");

    let mut file = File::create("Ship.json").expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");

    println!("Ship Generted!!!");
}
