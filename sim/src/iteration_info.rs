use crate::typedef::{IterationInfo, Road};
use std::{io::Write, time::Duration};

impl IterationInfo {
    pub fn new(iteration: usize, time: Duration, road: Road) -> Self {
        Self {
            iteration,
            time,
            road,
        }
    }

    pub fn save_iteration_to_csv(&self, file_path: &str) {
        let average_speed = self.road.get_average_speed();
        let average_speed_per_lane = self.road.get_average_speed_per_lane();
        let vehicle_count = self.road.vehicles.len();
        let csv = format!(
            "{},{},{},{},{},{},{}\n",
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
            .open(file_path)
            .unwrap();

        file.write_all(csv.as_bytes()).unwrap();
    }

    pub fn initialize_csv(file_path: &str) {
        let header = "iteration,time,average_speed,average_speed_lane_0,average_speed_lane_1,average_speed_lane_2,vehicle_count\n";
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)
            .unwrap();

        file.write_all(header.as_bytes()).unwrap();
    }
}
