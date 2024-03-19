use crate::typedef::{IterationInfo, Road};
use std::{io::Write, time::Duration};

impl IterationInfo {
    pub fn new(iteration: usize, time: Duration, road: Road, file_path: &str) -> Self {
        let this = Self {
            iteration,
            time,
            road,
            file_path: file_path.to_string(),
        };
        this.initialize_csv();
        this
    }

    pub fn save_iteration_to_csv(&self) {
        let average_speed = self.road.get_average_speed();
        let average_speed_per_lane = self.road.get_average_speed_per_lane();
        let vehicle_count = self.road.vehicles.len();

        let delimiter = ",";

        let csv = format!(
            "{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}\n",
            self.iteration,
            self.time.as_secs_f32(),
            average_speed,
            average_speed_per_lane[0],
            average_speed_per_lane[1],
            average_speed_per_lane[2],
            vehicle_count
        );

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        file.write_all(csv.as_bytes()).unwrap();
    }

    fn initialize_csv(&self) {
        let delimiter = ",";

        let header = format!("iteration{delimiter}time{delimiter}average_speed{delimiter}average_speed_lane_0{delimiter}average_speed_lane_1{delimiter}average_speed_lane_2{delimiter}vehicle_count\n");
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        file.write_all(header.as_bytes()).unwrap();
    }
}
