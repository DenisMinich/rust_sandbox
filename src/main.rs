extern crate biplanes;

use biplanes::plane::*;
use std::f64::consts::*;

fn main() {
    let mut p: Plane = PlaneBuilder::new().
        position(2, 4).
        velocity(0., PI).
        build();
    println!("Plane before: {:?}", p);
    p.traverse();
    println!("Plane after: {:?}", p);
}
