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

#[derive(Clone, Copy)]
pub struct Velocity(u8);

impl Road {
    pub fn get_vehicles_in_lane(&self, lane: u8) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|vehicle| vehicle.position.y == lane)
            .collect::<Vec<_>>()
    }

    pub fn get_max_velocity_in_lane(&self, lane: u8) -> Option<Velocity> {
        self.speed_per_lane.get(lane as usize).cloned()
    }

    pub fn set_max_velocity_in_lane(&mut self, lane: u8, velocity: Velocity) {
        self.speed_per_lane
            .iter_mut()
            .enumerate()
            .map(|(idx, old_velocity)| {
                if idx == lane as usize {
                    *old_velocity = velocity
                }
            })
            .collect()
    }
}

impl Vehicle {}
