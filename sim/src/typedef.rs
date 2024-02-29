pub struct Road {
    lanes: Vec<Lane>,
}

pub struct Lane {
    vehicles: Vec<Vehicle>,
    max_velocity: Velocity,
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

impl Lane {}

impl Vehicle {}
