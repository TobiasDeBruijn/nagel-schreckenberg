use std::io::Write;

use crate::typedef::{IterationInfo, SimulationWriter};

impl SimulationWriter {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }

    pub fn initialize_csv(&self) {
        let delimiter = ",";
        let header = format!("iteration{delimiter}time{delimiter}density{delimiter}average_speed{delimiter}average_speed_lane_0{delimiter}average_speed_lane_1{delimiter}average_speed_lane_2{delimiter}lane_change_probability{delimiter}deceleration_probability{delimiter}max_speed_lane_1{delimiter}max_speed_lane_2{delimiter}max_speed_lane_3{delimiter}flow{delimiter}vehicle_count\n");

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        file.write_all(header.as_bytes()).unwrap();
    }

    pub fn save_iteration_to_csv(&self, iteration_info: &IterationInfo) {
        let delimiter = ",";

        let csv = format!(
            "{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}{delimiter}{}\n",
            iteration_info.iteration,
            iteration_info.time.as_secs_f32(),
            iteration_info.density,
            //Also check if Nan 
            if iteration_info.average_speed.is_nan() {0.0} else {iteration_info.average_speed},
            if iteration_info.average_speed_per_lane[0].is_nan() {0.0} else {iteration_info.average_speed_per_lane[0]},
            if iteration_info.average_speed_per_lane[1].is_nan() {0.0} else {iteration_info.average_speed_per_lane[1]},
            if iteration_info.average_speed_per_lane[2].is_nan() {0.0} else {iteration_info.average_speed_per_lane[2]},
            iteration_info.lane_change_probability,
            iteration_info.deceleration_probability,
            iteration_info.max_speed_per_lane[0],
            iteration_info.max_speed_per_lane[1],
            iteration_info.max_speed_per_lane[2],
            iteration_info.flow,
            iteration_info.vehicle_count,
        );

        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        file.write_all(csv.as_bytes()).unwrap();
    }

    pub fn write_iteration_infos_to_csv(&self, iteration_infos: &Vec<IterationInfo>) {
        self.initialize_csv();
        for iteration_info in iteration_infos {
            self.save_iteration_to_csv(iteration_info);
        }
    }
}
