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
    y: u8,
}

pub struct Velocity(u8);

impl Road {
    pub fn get_vehicles_in_lane(&self, lane: u8) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|vehicle| vehicle.position.y == lane)
            .collect::<Vec<_>>()
    }
}

impl Vehicle {}
