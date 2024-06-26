use crate::{
    iterations_runner::run_iterations,
    road::create_road,
    typedef::{IterationInfo, MetaData, SimulationType, SimulationWriter, SimulationsHandler},
};
use indicatif::{ProgressBar, ProgressStyle};

impl SimulationsHandler {
    pub fn new(
        num_simulations: usize,
        iterations_per_simulation: usize,
        deceleration_probability: f32,
        lane_change_probability: f32,
        sim_type: SimulationType,
        simulation_writer: SimulationWriter,
        verbose: bool,
        lane_speeds: Vec<u8>,
        pretty_print: bool,
    ) -> Self {
        Self {
            num_simulations,
            iterations_per_simulation,
            deceleration_probability,
            lane_change_probability,
            sim_type,
            simulation_writer,
            verbose,
            lane_speeds,
            pretty_print,
        }
    }

    pub fn run_simulation(
        &self,
        iterations_per_simulation: usize,
        sim_type: SimulationType,
    ) -> Vec<IterationInfo> {
        let road_length = 100;
        let standard_density = 0.3;
        // let deceleration_probability = 0.4;
        // let lane_change_probability = 0.8;

        let mut iteration = 0;
        let mut iteration_infos = Vec::new();

        match sim_type {
            SimulationType::Density(start, end, step) => {
                let float_range = float_range_step(start, end, step);

                let bar = ProgressBar::new(float_range.len() as u64);

                //set width of progress bar
                bar.set_style(
                    ProgressStyle::with_template(
                        "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
                    )
                    .unwrap()
                    .progress_chars("##-"),
                );

                for i in float_range {
                    bar.inc(1);
                    iteration += 1;
                    let road = create_road(
                        road_length,
                        i,
                        self.lane_speeds.clone(),
                        self.deceleration_probability,
                        self.lane_change_probability,
                        true,
                        true,
                    );
                    let iteration_info = run_iterations(iteration, iterations_per_simulation, road, self.pretty_print);
                    iteration_infos.push(iteration_info);
                }

                bar.finish();
            }
            SimulationType::LaneChange(start, end, step) => {
                let float_range = float_range_step(start, end, step);
                let bar = ProgressBar::new(float_range.len() as u64);

                for i in float_range {
                    bar.inc(1);
                    iteration += 1;
                    let road = create_road(
                        road_length,
                        standard_density,
                        self.lane_speeds.clone(),
                        self.deceleration_probability,
                        i,
                        true,
                        true,
                    );
                    let iteration_info = run_iterations(iteration, iterations_per_simulation, road, self.pretty_print);
                    iteration_infos.push(iteration_info);
                }

                bar.finish();
            }
            SimulationType::Deceleration(start, end, step) => {
                let float_range = float_range_step(start, end, step);

                let bar = ProgressBar::new(float_range.len() as u64);

                for i in float_range {
                    bar.inc(1);
                    iteration += 1;
                    let road = create_road(
                        road_length,
                        standard_density,
                        self.lane_speeds.clone(),
                        i,
                        self.deceleration_probability,
                        true,
                        true,
                    );
                    let iteration_info = run_iterations(iteration, iterations_per_simulation, road, self.pretty_print);
                    iteration_infos.push(iteration_info);
                }

                bar.finish();
            }
        };

        iteration_infos
    }

    pub fn run_simulations(&self) -> Vec<IterationInfo> {
        let mut sim_infos = Vec::new();

        for _ in 0..self.num_simulations {
            if self.verbose {
                print!(
                    "Running simulation {} of {}\n",
                    sim_infos.len() + 1,
                    self.num_simulations
                );
            }
            let simulation_results =
                self.run_simulation(self.iterations_per_simulation, self.sim_type.clone());
            sim_infos.push(simulation_results);
        }

        self.average_of_simulations(sim_infos)
    }

    fn average_of_simulations(&self, sims: Vec<Vec<IterationInfo>>) -> Vec<IterationInfo> {
        if self.verbose {
            print!("Calculating averages of simulations\n");
        }

        let mut average_infos: Vec<IterationInfo> = Vec::new();

        let num_of_rows = sims[0].len();

        for i in 0..num_of_rows {
            let mut sum_time: f32 = 0.0;
            let mut sum_speed: f32 = 0.0;
            let mut sum_speed_per_lane: Vec<f32> = vec![0.0, 0.0, 0.0];
            let mut sum_flow: f32 = 0.0;

            let iter_info = &sims[0][i];

            for j in 0..self.num_simulations {
                let current_sim = &sims[j];
                let current_sim_time = current_sim[i].time.as_secs_f32();
                let current_sim_speed = current_sim[i].average_speed;
                let current_sim_speed_per_lane = current_sim[i].average_speed_per_lane.clone();
                let current_sim_flow = current_sim[i].flow;

                sum_time += current_sim_time;
                sum_speed += current_sim_speed;
                sum_speed_per_lane[0] += current_sim_speed_per_lane[0];
                sum_speed_per_lane[1] += current_sim_speed_per_lane[1];
                sum_speed_per_lane[2] += current_sim_speed_per_lane[2];
                sum_flow += current_sim_flow;
            }

            // let average_time = sum_time / self.num_simulations as f32;
            let average_speed = sum_speed / self.num_simulations as f32;
            let average_speed_per_lane = vec![
                sum_speed_per_lane[0] / self.num_simulations as f32,
                sum_speed_per_lane[1] / self.num_simulations as f32,
                sum_speed_per_lane[2] / self.num_simulations as f32,
            ];
            let average_flow = sum_flow / self.num_simulations as f32;

            let average_info = iter_info.clone().add_averages_to_info(
                sum_time,
                average_speed,
                average_speed_per_lane,
                average_flow,
            );

            average_infos.push(average_info);
        }

        average_infos
    }

    pub fn save_simulation_results(
        &self,
        iteration_infos: &Vec<IterationInfo>,
        metadata: &MetaData,
    ) {
        if self.verbose {
            print!("Writing simulation results to csv\n");
        }
        self.simulation_writer
            .save_csv_and_metadata(iteration_infos, metadata);
    }
}

fn float_range_step(start: f32, end: f32, step: f32) -> Vec<f32> {
    let mut range = Vec::new();
    let mut i = start;
    while i < end {
        range.push(i);
        i += step;
    }

    range
}
