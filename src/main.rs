use std::time::Duration;
use std::thread::sleep;
use std::io::stdout;
use std::io::Write;
use std::time::Instant;

struct Star {
    position: [u128; 3],    //kilometers (km)
    rotation: f64,          //degrees
    rotation_period: u32,   //seconds    (s)
    axial_tilt: f64,        //degrees
    mass: u128,             //kilograms  (kg)
    radius: u32,            //kilometers (km)
}

struct Planet {
    position: [u128; 3],    //kilometers (km)
    rotation: f64,          //degrees
    rotation_period: u32,   //seconds    (s)
    axial_tilt: f64,        //degrees
    mass: u128,             //kilograms  (kg)
    radius: u32,            //kilometers (km)
}

const SPEED: u32 = 1000;

fn main() {
    let mut sun: Star = Star {
        position: [0, 0, 0],
        rotation: 0.0,
        rotation_period: 2192832,
        axial_tilt: 7.25,
        mass: 1989 * u128::pow(10, 27),
        radius: 696340,
    };

    let mut earth: Planet = Planet {
        position: [0, 0, 0],
        rotation: 0.0,
        rotation_period: 86400, //86400
        axial_tilt: 23.44,
        mass: 5972 * u128::pow(10, 20),
        radius: 6371,
    };
    let mut tick = 0;

    loop {
        let start = Instant::now();

        earth.rotation += 360.0 / earth.rotation_period as f64;
        if earth.rotation >= 360.0{
            earth.rotation = 0.0
        }

        tick += 1;

        if tick == 64 {
            println!("SUN: {} EARTH: {}", sun.rotation, earth.rotation);
            tick = 0;
        }

        sleep(Duration::from_millis(1));

        let elapsed = start.elapsed().as_micros();
        println!("{}", elapsed)
    }
}
