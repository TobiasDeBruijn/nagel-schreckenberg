use std::fs;
use std::io::Write;
use std::path::PathBuf;

use crate::typedef::{IterationInfo, MetaData, SimulationWriter};

const CSV_DELIMITER: &str = ",";

impl SimulationWriter {
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: PathBuf::from(".").join(PathBuf::from("data").join(file_path)),
        }
    }

    pub fn initialize_csv(&self) {
        let header = format!(
            "iteration{d}time{d}density{d}average_speed{d}average_speed_lane_0{d}average_speed_lane_1{d}average_speed_lane_2{d}lane_change_probability{d}deceleration_probability{d}max_speed_lane_1{d}max_speed_lane_2{d}max_speed_lane_3{d}flow{d}vehicle_count\n",
            d = CSV_DELIMITER
        );

        let mut f = fs::File::create(&self.file_path).unwrap();
        f.write_all(header.as_bytes()).unwrap();
    }

    pub fn save_iteration_to_csv(&self, i_inf: &IterationInfo) {
        let csv = format!(
            "{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}{d}{}\n",
            i_inf.iteration,
            i_inf.time.as_secs_f32(),
            i_inf.density,
            i_inf
                .average_speed
                .is_nan()
                .then(|| 0.0)
                .unwrap_or_else(|| i_inf.average_speed),
            i_inf.average_speed_per_lane[0]
                .is_nan()
                .then(|| 0.0)
                .unwrap_or_else(|| i_inf.average_speed_per_lane[0]),
            i_inf.average_speed_per_lane[1]
                .is_nan()
                .then(|| 0.0)
                .unwrap_or_else(|| i_inf.average_speed_per_lane[1]),
            i_inf.average_speed_per_lane[2]
                .is_nan()
                .then(|| 0.0)
                .unwrap_or_else(|| i_inf.average_speed_per_lane[2]),
            i_inf.lane_change_probability,
            i_inf.deceleration_probability,
            i_inf.max_speed_per_lane[0],
            i_inf.max_speed_per_lane[1],
            i_inf.max_speed_per_lane[2],
            i_inf.flow,
            i_inf.vehicle_count,
            d = CSV_DELIMITER,
        );

        let mut file = fs::OpenOptions::new()
            .write(true)
            .append(true)
            .create(true)
            .open(&self.file_path)
            .unwrap();

        file.write_all(csv.as_bytes()).unwrap();
    }

    pub fn save_csv_and_metadata(&self, iteration_infos: &Vec<IterationInfo>, metadata: &MetaData) {
        self.write_iteration_infos_to_csv(iteration_infos);
        self.write_metadata_to_file(metadata);
    }

    pub fn write_iteration_infos_to_csv(&self, iteration_infos: &Vec<IterationInfo>) {
        self.initialize_csv();
        for iteration_info in iteration_infos {
            self.save_iteration_to_csv(iteration_info);
        }
    }

    pub fn write_metadata_to_file(&self, metadata: &MetaData) {
        //Remove .csv extension
        let mut file_path = self.file_path.with_extension("");
        file_path.set_extension("metadata");

        let mut file = fs::File::create(file_path).unwrap();

        let metadata = format!("Road Length: {}\nNumber of Simulations: {}\nIterations per Simulation: {}\nSimulation Type: {:?}\nSpeeds per lane: {} {} {}",
            metadata.road_len,
            metadata.num_simulations,
            metadata.iterations_per_simulation,
            metadata.sim_type,
            metadata.speeds_per_lane[0],
            metadata.speeds_per_lane[1],
            metadata.speeds_per_lane[2]
        );

        file.write_all(metadata.as_bytes()).unwrap();
    }
}
