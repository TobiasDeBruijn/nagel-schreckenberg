pub struct Road(Vec<Lane>);

pub struct Lane(Vec<Vehicle>);

pub struct Vehicle {
    position: Position,
    velocity: Velocity,
}

pub struct Position {
    x: u64,
    y: u64,
}

pub struct Velocity(u8);

impl Road {}

impl Lane {}

impl Vehicle {}
