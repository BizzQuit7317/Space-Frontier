use rand::Rng;
use serde::{Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct Planet {
    Name:String,
    Mass:f32,
    Radius:f32,
    Rotation_Speed:f32,
    Inhabited:bool,
    Agressive:bool,
    Level:f32,
    Resources:f32,
    Health:f32,
    Skin_Path:String,    
}

impl Planet {
    fn new(name: &str, skin_path: &str) -> Self {
        let mut rng = rand::thread_rng();

        let inhab = rng.r#gen();
        let mut agg = false;
        if inhab {
            agg = rng.r#gen();
        }

        /*
            This will need to read the player ship json and make the level
            dependant on the ship level
        */
        let level = rng.gen_range(1.0..100.0);
        let resources_shift = rng.gen_range(-10.0..10.0);
        let res = resources_shift + level;
        let health = rng.gen_range(100.0..150.0);

        Planet {
            Name:name.to_string(),
            Mass:rng.gen_range(10.0..20.0),
            Radius:rng.gen_range(0.0..5.0),
            Rotation_Speed:rng.gen_range(0.01..0.5),
            Inhabited:inhab,
            Agressive:agg,
            Level:level,
            Resources:res,
            Health:health,
            Skin_Path:skin_path.to_string(),
        }
    }
}

fn main() {
    let planets = vec![
        Planet::new("Alpha 1", "res://planet_1.png"),
        Planet::new("Alpha 2", "res://planet_2.png"),
        Planet::new("Alpha 3", "res://planet_3.png"),
        Planet::new("Alpha 4", "res://planet_4.png"),
        Planet::new("Alpha 5", "res://planet_5.png"),
        Planet::new("Alpha 6", "res://planet_6.png"),
        Planet::new("Alpha 7", "res://planet_7.png"),
        Planet::new("Alpha 8", "res://planet_8.png"),
        Planet::new("Alpha 9", "res://planet_9.png"),
        Planet::new("Alpha 10", "res://planet_10.png"),
        Planet::new("Alpha 11", "res://planet_11.png"),
        Planet::new("Alpha 12", "res://planet_12.png"),
    ];

    let json = serde_json::to_string_pretty(&planets).expect("Failed to serialize");

    let mut file = File::create("planets.json").expect("Failed to create file");
    file.write_all(json.as_bytes()).expect("Failed to write to file");

    println!("Planets Generted!!!");
}
