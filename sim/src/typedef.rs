use std::io::{stdout, Write};
use std::ops::{AddAssign, Deref, SubAssign};

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

#[derive(Debug)]
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

    // 1. Car checks maximum speed it can achieve on it's current position (x, lane) and adjacent lane (x, lane+1).
    // 2. If the potential maximal speed on lane+1 is higher it checks safe conditions:
    // 3. Distance to previous car on lane+1 is greater that it's speed to avoid emergency braking of previous car.
    // 4. Change lane with probability P.
    pub fn update_vehicle(&mut self, road: &Road) -> &Self {
        self.update_lane(road).update_x(road)
    }

    //Update the lane of the vehicle
    pub fn update_lane(&mut self, road: &Road) -> &mut Self {
        let max_speed = road.get_max_velocity_in_lane(self.position.y).unwrap();
        let max_speed_next_lane = road
            .get_max_velocity_in_lane(self.position.y + 1)
            .unwrap_or(max_speed);
        let max_speed_prev_lane = road
            .get_max_velocity_in_lane(self.position.y - 1)
            .unwrap_or(max_speed);

        if max_speed_next_lane > max_speed {
            if self.position.y < 2 {
                let next_lane = road.get_vehicles_in_lane(self.position.y + 1);
                if next_lane.is_empty() {
                    self.position.y += 1;
                } else {
                    let next_vehicle = next_lane.iter().min_by_key(|v| v.position.x).unwrap();
                    if self.position.x + (max_speed.into_inner() as u8)
                        < next_vehicle.position.x - next_vehicle.velocity.into_inner() as u8
                    {
                        self.position.y += 1;
                    }
                }
            }
        } else if max_speed_prev_lane > max_speed {
            if self.position.y > 0 {
                let prev_lane = road.get_vehicles_in_lane(self.position.y - 1);
                if prev_lane.is_empty() {
                    self.position.y -= 1;
                } else {
                    let prev_vehicle = prev_lane.iter().min_by_key(|v| v.position.x).unwrap();
                    if self.position.x + (max_speed.into_inner() as u8)
                        < prev_vehicle.position.x - prev_vehicle.velocity.into_inner() as u8
                    {
                        self.position.y -= 1;
                    }
                }
            }
        }

        self
    }

    fn willing_to_move_left(&self, road: &Road) -> bool {
        self.can_go_left(road) && self.willing_to_change_lane(road, self.position.y + 1)
    }

    fn willing_to_move_right(&self, road: &Road) -> bool {
        self.can_go_right(road) && self.willing_to_change_lane(road, self.position.y - 1)
    }

    fn willing_to_change_lane(&self, road: &Road, lane: u8) -> bool {
        let max_speed = road.get_max_velocity_in_lane(self.position.y).unwrap();
        let max_speed_next_lane = road
            .get_max_velocity_in_lane(self.position.y + 1)
            .unwrap_or(max_speed);
        let max_speed_prev_lane = road
            .get_max_velocity_in_lane(self.position.y - 1)
            .unwrap_or(max_speed);

        if max_speed_next_lane > max_speed {
            if self.position.y < 2 {
                let next_lane = road.get_vehicles_in_lane(self.position.y + 1);
                if next_lane.is_empty() {
                    return true;
                } else {
                    let next_vehicle = next_lane.iter().min_by_key(|v| v.position.x).unwrap();
                    if self.position.x + (max_speed.into_inner() as u8)
                        < next_vehicle.position.x - next_vehicle.velocity.into_inner() as u8
                    {
                        return true;
                    }
                }
            }
        } else if max_speed_prev_lane > max_speed {
            if self.position.y > 0 {
                let prev_lane = road.get_vehicles_in_lane(self.position.y - 1);
                if prev_lane.is_empty() {
                    return true;
                } else {
                    let prev_vehicle = prev_lane.iter().min_by_key(|v| v.position.x).unwrap();
                    if self.position.x + (max_speed.into_inner() as u8)
                        < prev_vehicle.position.x - prev_vehicle.velocity.into_inner() as u8
                    {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// Check if the vehicle can go left by checking if it is in bounds
    /// # Arguments
    /// * `road` - The road to check if the vehicle can go left on
    fn can_go_left(&self, road: &Road) -> bool {
        self.position.y > 1
    }

    /// Check if the vehicle can go right by checking if it is in bounds
    /// # Arguments
    /// * `road` - The road to check if the vehicle can go right on
    /// # Returns
    /// True if the vehicle can go right, false otherwise
    fn can_go_right(&self, road: &Road) -> bool {
        self.position.y < 1
    }

    pub fn update_x(&mut self, road: &Road) -> &mut Self {
        let max_speed = road.get_max_velocity_in_lane(self.position.y).unwrap();
        self.velocity = Velocity::new(max_speed.into_inner());
        self.position += self.velocity;
        self
    }
}
