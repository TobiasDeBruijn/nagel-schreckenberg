use std::path::PathBuf;
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
    pub original_lane: u8,
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

pub struct MetaData {
    pub road_len: u8,
    pub num_simulations: usize,
    pub iterations_per_simulation: usize,
    pub sim_type: SimulationType,
    pub speeds_per_lane: Vec<u8>,
}

pub struct SimulationWriter {
    pub file_path: PathBuf,
}

#[derive(Clone)]
pub enum SimulationType {
    Density(f32, f32, f32),
    LaneChange(f32, f32, f32),
    Deceleration(f32, f32, f32),
}

//Determine how to print the simulation type to file
impl std::fmt::Debug for SimulationType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            SimulationType::Density(start, end, step) => {
                write!(f, "Density: {} to {} by {}", start, end, step)
            }
            SimulationType::LaneChange(start, end, step) => {
                write!(f, "Lane Change: {} to {} by {}", start, end, step)
            }
            SimulationType::Deceleration(start, end, step) => {
                write!(f, "Deceleration: {} to {} by {}", start, end, step)
            }
        }
    }
}

pub struct SimulationsHandler {
    pub num_simulations: usize,
    pub iterations_per_simulation: usize,
    pub deceleration_probability: f32,
    pub lane_change_probability: f32,
    pub sim_type: SimulationType,
    pub simulation_writer: SimulationWriter,
    pub verbose: bool,
    pub lane_speeds: Vec<u8>,
    pub pretty_print: bool,
}
