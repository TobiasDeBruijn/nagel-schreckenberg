use crate::typedef::{Position, Road, Vehicle, Velocity};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::cmp::min;
use std::io::{stdout, Write};
use colored::Colorize;

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
            .into_par_iter()
            .map(|vehicle| vehicle.update(&self))
            .collect::<Vec<_>>();
    }

    pub fn get_average_speed(&self) -> f32 {
        self.vehicles
            .iter()
            .map(|v| v.velocity.into_inner() as f32)
            .sum::<f32>()
            / self.vehicles.len() as f32
    }

    pub fn get_density(&self) -> f32 {
        (self.vehicles.len() as f32 / self.len as f32) / 3.0
    }

    pub fn get_flow(&self) -> f32 {
        self.get_average_speed() * self.get_density()
    }

    pub fn get_average_speed_per_lane(&self) -> Vec<f32> {
        (0..3)
            .into_iter()
            .map(|lane| {
                let vs = self
                    .vehicles
                    .iter()
                    .filter(|v| v.position.y == lane)
                    .collect::<Vec<_>>();
                vs.iter()
                    .map(|v| v.velocity.into_inner() as f32)
                    .sum::<f32>()
                    / vs.len() as f32
            })
            .collect()
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
                    Some(v) => {
                        let text = v.velocity.into_inner().to_string();
                        let color = match v.original_lane {
                            0 => text.blue(),
                            1 => text.green(),
                            2 => text.red(),
                            _ => unreachable!(),
                        };

                        format!("{color}")
                    },
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
            format!("{}\t{}", self.pretty_print_lane(2, false), self.speed_per_lane[2].into_inner().to_string().red()),
            self.get_strides(),
            format!("{}\t{}", self.pretty_print_lane(1, false), self.speed_per_lane[1].into_inner().to_string().green()),
            self.get_strides(),
            format!("{}\t{}", self.pretty_print_lane(0, false), self.speed_per_lane[0].into_inner().to_string().blue()),
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

/// Create a new road
pub fn create_road(
    length: usize,
    density: f32,
    mut speed_per_lane: Vec<u8>,
    deceleration_probability: f32,
    lange_change_probability: f32,
    random_car_start_pos: bool,
    random_car_start_speed: bool,
) -> Road {
    let amount_of_cars = (length as f32 * density * 3.0) as usize;

    if speed_per_lane.is_empty() {
        print!("No speeds provided, defaulting to 5 for all lanes");
        speed_per_lane = vec![5, 5, 5];
    }

    if speed_per_lane.len() < 3 {
        panic!("Speed per lane must have at least 3 speeds")
    }

    //Remove any extra speeds
    speed_per_lane.truncate(3);

    let mut vehicles = Vec::new();

    for i in 0..amount_of_cars {
        let mut lane = i % 3;
        let mut x = (i / 3) as u8;
        if random_car_start_pos {
            lane = rand::random::<usize>() % 3;
            x = rand::random::<u8>() % length as u8;
            while vehicles
                .iter()
                .any(|v: &Vehicle| v.position.x == x && v.position.y == lane as u8)
            {
                lane = rand::random::<usize>() % 3;
                x = rand::random::<u8>() % length as u8;
            }
        }

        let speed = if random_car_start_speed {
            Velocity::new(rand::random::<u8>() % speed_per_lane[lane] + 1)
        } else {
            Velocity::new(0)
        };

        vehicles.push(Vehicle::new(
            Position::new(x, lane as u8),
            Some(speed),
            lange_change_probability,
            lange_change_probability,
        ));
    }

    Road::new(
        length as u8,
        deceleration_probability,
        vehicles,
        speed_per_lane.into_iter().map(Velocity::new).collect(),
    )
}
