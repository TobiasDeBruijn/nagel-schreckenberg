use std::ops::{AddAssign, Deref};

pub struct Road {
    vehicles: Vec<Vehicle>,
    speed_per_lane: Vec<Velocity>,
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

impl Position {
    pub fn new(x: u128, y: u8) -> Self {
        Self { x, y }
    }

    pub fn distance_1d(&self, rhs: &Self) -> u128 {
        rhs.x - self.x
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
    pub fn new(vehicles: Vec<Vehicle>, speed_per_lane: Vec<Velocity>) -> Self {
        Self {
            vehicles,
            speed_per_lane,
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
        self.vehicles
            .iter()
            .map(|vehicle| vehicle.position.x)
            .max()
            .unwrap_or(0)
    }

    pub fn pretty_print(&self) {
        const SIDE_OF_ROAD_STR: &str = "#";
        const ROAD_MARGIN: usize = 20;
        let road_length = self.get_length_of_road() as usize + ROAD_MARGIN * 2;

        let mut road = vec![vec![' '; road_length]; 3];

        //Print a '-' every 4th position in the second lane of the road
        for (idx, char) in road[1].iter_mut().enumerate() {
            if idx % 4 == 0 {
                *char = '-';
            }
        }

        for vehicle in &self.vehicles {
            road[vehicle.position.y as usize][vehicle.position.x as usize + ROAD_MARGIN] = 'o';
        }
        println!("Road:");

        //Print SIDE_OF_ROAD_CHAR 100 times
        println!("{}", SIDE_OF_ROAD_STR.repeat(road_length));

        for row in road {
            println!("{}", row.into_iter().collect::<String>());
        }

        //Print SIDE_OF_ROAD_CHAR 100 times
        println!("{}", SIDE_OF_ROAD_STR.repeat(road_length));
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
