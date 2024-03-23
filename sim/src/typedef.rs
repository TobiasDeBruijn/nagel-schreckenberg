use std::{
    ops::{AddAssign, Deref, SubAssign},
    time::Duration,
};


#[derive(Debug, Clone)]
pub struct Road {
    pub len: u8,
    pub deceleration_probability: f32,
    pub vehicles: Vec<Vehicle>,
    pub speed_per_lane: Vec<Velocity>,
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

#[derive(Debug, Clone)]
pub struct IterationInfo {
    pub iteration: usize,
    pub time: Duration,
    pub average_speed: f32,
    pub average_speed_per_lane: Vec<f32>,
    pub vehicle_count: usize,
    pub density: f32,
    pub lane_change_probability: f32,
    pub deceleration_probability: f32,
    pub max_speed_per_lane: Vec<u8>,
    pub flow: f32,
}

pub struct SimulationWriter {
    pub file_path: String,
}

#[derive(Debug, Clone)]
pub enum SimulationType {
    Density(f32, f32, f32),
    LaneChange(f32, f32, f32),
    Deceleration(f32, f32, f32),
}

pub struct SimulationsHandler {
    pub num_simulations: usize,
    pub iterations_per_simulation: usize,
    pub sim_type: SimulationType,
    pub simulation_writer: SimulationWriter,
    pub verbose: bool,
}

//The simulation runs for a certain number of simulations, we take the average of the results
//The amount of simulations is determined by the start, end, and step values
//Each simulation runs for a certain number of iterations
//Each iteration is a step in the simulation
