use crate::typedef::{Position, Road, Vehicle, Velocity};
use std::cmp::min;
use std::io::{stdout, Write};

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
            .map(|vehicle| vehicle.update(&self))
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

        let v: Vec<_> = r.clone();
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