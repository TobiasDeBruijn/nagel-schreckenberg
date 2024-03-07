use std::ops::{AddAssign, Deref};

pub struct Road {
    vehicles: Vec<Vehicle>,
    speed_per_lane: Vec<Velocity>,
    pub len: u128,
}

#[derive(Debug)]
pub struct Vehicle {
    pub position: Position,
    pub velocity: Velocity,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: u128,
    pub y: u8,
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.into_inner() as u128;
    }
}

impl Position {
    pub fn new(x: u128, y: u8) -> Self {
        Self { x, y }
    }

    pub fn distance_1d(&self, rhs: &Self) -> u128 {
        (self.x as i128 - rhs.x as i128).abs() as u128
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Velocity(u8);

impl AddAssign for Velocity {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0
    }
}

impl Velocity {
    pub fn new(v: u8) -> Self {
        Self(v)
    }

    pub fn into_inner(self) -> u8 {
        self.0
    }
}

impl Deref for Velocity {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Road {
    pub fn new(vehicles: Vec<Vehicle>, speed_per_lane: Vec<Velocity>, len: u128) -> Self {
        Self {
            vehicles,
            speed_per_lane,
            len
        }
    }

    pub fn get_vehicles_in_lane(&self, lane: u8) -> Vec<&Vehicle> {
        self.vehicles
            .iter()
            .filter(|vehicle| vehicle.position.y == lane)
            .collect::<Vec<_>>()
    }

    pub fn get_vehicles_in_lane_mut(&mut self, lane: u8) -> Vec<&mut Vehicle> {
        self.vehicles
            .iter_mut()
            .filter(|vehicle| vehicle.position.y == lane)
            .collect::<Vec<_>>()
    }

    pub fn get_vehicles_mut(&mut self) -> &mut [Vehicle] {
        self.vehicles.as_mut_slice()
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

    fn get_length_of_road(&self) -> u128 {
        let xs = self
            .vehicles
            .iter()
            .map(|v| v.position.x)
            .collect::<Vec<_>>();

        xs.iter().max().unwrap_or(&0_u128) - xs.iter().min().unwrap_or(&0_u128)
    }

    pub fn pretty_print_lane(&self, lane: u8, strides: bool) -> String {
        (0..self.len).into_iter()
            .map(|f| match self.vehicles.iter().find(|v| v.position.x == f && v.position.y == lane) {
                Some(v) => v.velocity.into_inner().to_string(),
                None => " ".to_string()
            })
            .enumerate()
            .map(|(idx, c)| if strides && idx % 4 == 0 && c == " " {
                "-".to_string()
            } else {
                c
            })
            .collect()
    }

    pub fn pretty_print(&self) {
        const SIDE_OF_ROAD_STR: &str = "#";

        let s = vec![
            SIDE_OF_ROAD_STR.repeat(self.len as usize),
            self.pretty_print_lane(2, false),
            self.pretty_print_lane(1, true),
            self.pretty_print_lane(0, false),
            SIDE_OF_ROAD_STR.repeat(self.len as usize),
        ].join("\n");

        println!("{s}");
    }
}

impl Vehicle {
    pub fn new(position: Position) -> Self {
        Self {
            position,
            velocity: Velocity::new(0),
        }
    }
}
