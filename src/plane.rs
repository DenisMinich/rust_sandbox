#[derive(Debug)]
#[derive(Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug)]
#[derive(Clone)]
pub struct Velocity {
    pub value: f64,
    pub angle: f64,
}

pub trait CanTraverse {
    fn traverse(&mut self);
}

#[derive(Debug)]
pub struct Plane{
    pub position: Position,
    pub velocity: Velocity,
}

impl CanTraverse for Plane {
    fn traverse(&mut self) {
        self.position = Position {
            x: self.position.x + (
                self.velocity.value * self.velocity.angle.cos()) as i32,
            y: self.position.y + (
                self.velocity.value * self.velocity.angle.sin()) as i32,
        }
    }
}


pub struct PlaneBuilder {
    pub position: Position,
    pub velocity: Velocity,
}

impl PlaneBuilder {
    pub fn new() -> PlaneBuilder {
        PlaneBuilder {
            position: Position { x: 0, y: 0 },
            velocity: Velocity { value: 0., angle: 0. }
        }
    }
    pub fn position(&mut self, x: i32, y: i32) -> &mut PlaneBuilder {
       self.position = Position { x: x, y: y };
       self
    }
    pub fn velocity(&mut self, value: f64, angle: f64) -> &mut PlaneBuilder {
       self.velocity = Velocity { value: value, angle: angle };
       self
    }
    pub fn build(&self) -> Plane {
        Plane { position: self.position.clone(), velocity: self.velocity.clone() }
    }
}
