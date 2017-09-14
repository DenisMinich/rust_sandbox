#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


#[derive(Debug)]
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
