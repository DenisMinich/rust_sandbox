extern crate biplanes;

use biplanes::plane::*;
use std::f64::consts::*;

fn main() {
    let mut p: Plane = Plane {
        position: Position { x: 1, y: -2 },
        velocity: Velocity { value: 10., angle: FRAC_PI_4 },
    };
    println!("Plane before: {:?}", p);
    p.traverse();
    println!("Plane after: {:?}", p);
}
