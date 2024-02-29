pub struct Road {
    vehicles: Vec<Vehicle>,
    speed_per_lane: Vec<Velocity>,
}

pub struct Vehicle {
    position: Position,
    velocity: Velocity,
}

pub struct Position {
    x: u128,
    y: u128,
}

pub struct Velocity(u8);

impl Road {}

impl Vehicle {}
