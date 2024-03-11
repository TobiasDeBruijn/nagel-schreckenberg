use core::panic;
use std::cmp::{max, min};
use std::io::{stdout, Write};
use std::ops::{AddAssign, Deref, SubAssign};

use rand::Rng;

pub struct Road {
    pub len: u8,
    pub deceleration_probability: f32,
    pub vehicles: Vec<Vehicle>,
    speed_per_lane: Vec<Velocity>,
}

impl SubAssign<u32> for Velocity {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs as u8;
    }
}

#[derive(Debug, Clone)]
pub struct Vehicle {
    pub position: Position,
    pub velocity: Velocity,
    pub move_left_chance: f32,
    pub move_right_chance: f32,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

impl AddAssign<Velocity> for Position {
    fn add_assign(&mut self, rhs: Velocity) {
        self.x += rhs.into_inner();
    }
}

impl Position {
    pub fn new(x: u8, y: u8) -> Self {
        Self { x, y }
    }

    ///Find the distance between two positions in the x direction
    ///
    /// # Arguments
    /// * self - The first position
    /// * rhs - The second position
    pub fn distance_1d(&self, rhs: &Self) -> u8 {
        (self.x as i8 - rhs.x as i8).abs() as u8
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
    pub fn new(
        len: u8,
        deceleration_probability: f32,
        vehicles: Vec<Vehicle>,
        speed_per_lane: Vec<Velocity>,
    ) -> Self {
        Self {
            len,
            deceleration_probability,
            vehicles,
            speed_per_lane,
        }
    }

    ///Find the distance between two vehicles
    /// # Arguments
    /// * `x1` - The x position of the first vehicle
    /// * `x2` - The x position of the second vehicle
    /// # Returns
    /// The distance between the two vehicles (vehicle in front - vehicle in back))
    pub fn dist_between_vehicles(&self, x1: u8, x2: u8) -> u8 {
        ((x1 as i16 - x2 as i16 + self.len as i16 - 1) % self.len as i16) as u8
    }

    pub fn update_vehicles(&mut self) {
        self.vehicles = self
            .vehicles
            .clone()
            .into_iter()
            .map(|vehicle| vehicle.update_vehicle(&self))
            .collect::<Vec<_>>();
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

    pub fn get_max_velocity_on_position(&self, pos: Position) -> Velocity {
        let dist_to_next_vehicle = self.distance_to_next_vehicle(pos.clone());

        let max_velocity = min(
            dist_to_next_vehicle,
            self.get_max_velocity_in_lane(pos.y).unwrap().into_inner(),
        );

        Velocity::new(max_velocity)
    }

    pub fn distance_to_next_vehicle(&self, position: Position) -> u8 {
        let mut vehicles_in_lane = self.get_vehicles_in_lane(position.y);

        //Remove self from the list of vehicles
        vehicles_in_lane.retain(|v| v.position.x != position.x);

        //Check if there are any vehicles in the lane
        if vehicles_in_lane.is_empty() {
            return u8::MAX;
        }

        vehicles_in_lane.sort_by(|a, b| {
            self.dist_between_vehicles(a.position.x, position.x)
                .cmp(&self.dist_between_vehicles(b.position.x, position.x))
        });

        //Take the first vehicle from vehicles_in_lane
        let next_vehicle = vehicles_in_lane.first().unwrap();

        self.dist_between_vehicles(next_vehicle.position.x, position.x)
    }

    pub fn find_previous_vehicle(&self, position: Position) -> Option<&Vehicle> {
        let mut vehicles_in_lane = self.get_vehicles_in_lane(position.y);

        //Remove self from the list of vehicles
        vehicles_in_lane.retain(|v| v.position.x != position.x);

        //Check if there are any vehicles in the lane
        if vehicles_in_lane.is_empty() {
            return None;
        }

        vehicles_in_lane.sort_by(|a, b| {
            self.dist_between_vehicles(position.x, a.position.x)
                .cmp(&self.dist_between_vehicles(position.x, b.position.x))
        });

        //Take the first vehicle from vehicles_in_lane
        let vehicle_behind = vehicles_in_lane.first().unwrap();

        Some(vehicle_behind)
    }

    pub fn pretty_print_lane(&self, lane: u8, strides: bool) -> String {
        (0..self.len)
            .into_iter()
            .map(|f| {
                match self
                    .vehicles
                    .iter()
                    .find(|v| v.position.x == f && v.position.y == lane)
                {
                    Some(v) => v.velocity.into_inner().to_string(),
                    None => " ".to_string(),
                }
            })
            .enumerate()
            .map(|(idx, c)| {
                if strides && idx % 4 == 0 && c == " " {
                    "-".to_string()
                } else {
                    c
                }
            })
            .collect()
    }

    pub fn get_strides(&self) -> String {
        (0..self.len)
            .map(|i| if i % 4 == 0 { "-" } else { " " }.to_string())
            .collect()
    }

    pub fn pretty_print(&self) {
        print!("\x1B[2J\x1B[1;1H");

        const SIDE_OF_ROAD_STR: &str = "#";

        let s = vec![
            SIDE_OF_ROAD_STR.repeat(self.len as usize),
            self.pretty_print_lane(2, false) + "\t3",
            self.get_strides(),
            self.pretty_print_lane(1, false) + "\t2",
            self.get_strides(),
            self.pretty_print_lane(0, false) + "\t1",
            SIDE_OF_ROAD_STR.repeat(self.len as usize),
        ]
        .join("\n");

        println!("{s}");

        println!(
            "Average speed:\t\t\t{:.2}",
            self.vehicles
                .iter()
                .map(|v| v.velocity.into_inner() as f32)
                .sum::<f32>()
                / self.vehicles.len() as f32
        );

        self.print_lane_speed_avg(0);
        self.print_lane_speed_avg(1);
        self.print_lane_speed_avg(2);

        println!("Total vehicles: \t\t{}", self.vehicles.len());

        let mut r = self
            .vehicles
            .iter()
            .map(|f| (f.position.x, f.position.y))
            .collect::<Vec<_>>();

        let mut v: Vec<_> = r.clone();
        r.dedup();
        println!("Unique vehicles: \t\t{}", v.len());

        // if r.len() != v.len() {
        //     v.sort();
        //     dbg!(v);
        //     panic!()
        // }

        stdout().flush().expect("Flush stdout");
    }

    fn print_lane_speed_avg(&self, lane: u8) {
        let vs = self
            .vehicles
            .iter()
            .filter(|v| v.position.y == lane)
            .collect::<Vec<_>>();
        let avg = vs
            .iter()
            .map(|v| v.velocity.into_inner() as f32)
            .sum::<f32>()
            / vs.len() as f32;

        println!(
            "Average speed per lane {}:\t{:.2}",
            lane + 1,
            if avg.is_nan() { 0.0 } else { avg }
        );
    }
}

impl Vehicle {
    pub fn new(position: Position, move_left_chance: f32, move_right_chance: f32) -> Self {
        Self {
            position,
            velocity: Velocity::new(0),
            move_left_chance,
            move_right_chance,
        }
    }

    fn go_left(&self) -> Position {
        Position::new(self.position.x, self.position.y + 1)
    }

    fn go_right(&self) -> Position {
        Position::new(self.position.x, self.position.y - 1)
    }

    // 1. Car checks maximum speed it can achieve on it's current position (x, lane) and adjacent lane (x, lane+1).
    // 2. If the potential maximal speed on lane+1 is higher it checks safe conditions:
    // 3. Distance to previous car on lane+1 is greater that it's speed to avoid emergency braking of previous car.
    // 4. Change lane with probability P.
    pub fn update_vehicle(self, road: &Road) -> Self {
        self.update_lane(road).update_x(road)
    }

    pub fn update_x(mut self, road: &Road) -> Self {
        self.velocity = min(
            road.get_max_velocity_on_position(self.position.clone()),
            Velocity::new(self.velocity.into_inner() + 1),
        );
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();

        if r < road.deceleration_probability && self.velocity.into_inner() > 0 {
            self.velocity -= 1;
        }

        self.position.x = (self.position.x + self.velocity.into_inner()) % road.len;

        self
    }

    //Update the lane of the vehicle
    pub fn update_lane(mut self, road: &Road) -> Self {
        let mut rng = rand::thread_rng();

        if self.willing_to_move_right(road) {
            if rng.gen_bool(self.move_right_chance as f64) {
                self.position = self.go_right();
            }
            return self;
        }
        if self.willing_to_move_left(road) {
            if rng.gen_bool(self.move_left_chance as f64) {
                self.position = self.go_left();
            }
            return self;
        }

        self
    }

    fn willing_to_move_left(&self, road: &Road) -> bool {
        if self.can_go_left() {
            self.willing_to_change_lane(road, self.position.y + 1)
        } else {
            false
        }
    }

    fn willing_to_move_right(&self, road: &Road) -> bool {
        if self.can_go_right() {
            self.willing_to_change_lane(road, self.position.y - 1)
        } else {
            false
        }
    }

    fn willing_to_change_lane(&self, road: &Road, lane: u8) -> bool {
        let src_lane_speed = road.get_max_velocity_on_position(self.position.clone());
        let dst_lane_speed =
            road.get_max_velocity_on_position(Position::new(self.position.x, lane));

        if dst_lane_speed <= src_lane_speed {
            return false;
        }

        let pos = &self.position;
        let previous_vehicle = road.find_previous_vehicle(pos.clone());

        match previous_vehicle {
            Some(v) => {
                let distance_to_previous_vehicle = road.dist_between_vehicles(pos.x, v.position.x);
                // let distance_to_previous_vehicle =
                //     ((pos.x as i16 - v.position.x as i16 + road.len as i16 - 1) % road.len as i16)
                //         as u8;

                distance_to_previous_vehicle > v.velocity.into_inner()
            }
            None => true,
        }
    }

    /// Check if the vehicle can go left by checking if it is in bounds
    /// # Arguments
    /// * `road` - The road to check if the vehicle can go left on
    fn can_go_left(&self) -> bool {
        self.position.y < 2
    }

    /// Check if the vehicle can go right by checking if it is in bounds
    /// # Arguments
    /// * `road` - The road to check if the vehicle can go right on
    /// # Returns
    /// True if the vehicle can go right, false otherwise
    fn can_go_right(&self) -> bool {
        self.position.y > 0
    }
}
