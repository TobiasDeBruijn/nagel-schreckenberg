use crate::typedef::{IterationInfo, Road};
use std::{io::Write, time::Duration};

impl IterationInfo {
    pub fn new(iteration: usize, time: Duration, road: Road) -> Self {
        let average_speed = road.get_average_speed();
        let average_speed_per_lane = road.get_average_speed_per_lane();
        let vehicle_count = road.vehicles.len();
        let density = road.get_density();
        let lane_change_probability = road.vehicles[0].move_left_chance;
        let deceleration_probability = road.deceleration_probability;
        let max_speed_per_lane = road
            .speed_per_lane
            .iter()
            .map(|v| v.into_inner())
            .collect::<Vec<_>>();
        let flow = road.get_flow();

        Self {
            iteration,
            time,
            average_speed,
            average_speed_per_lane,
            vehicle_count,
            density,
            lane_change_probability,
            deceleration_probability,
            max_speed_per_lane,
            flow,
        }
    }
}
