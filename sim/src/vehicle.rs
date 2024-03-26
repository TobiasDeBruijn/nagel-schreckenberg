use crate::typedef::{Position, Road, Vehicle, Velocity};

use rand::Rng;
use std::cmp::min;

impl Vehicle {
    pub fn new(
        position: Position,
        mut vel: Option<Velocity>,
        move_left_chance: f32,
        move_right_chance: f32,
    ) -> Self {
        if vel == None {
            vel = Some(Velocity::new(0));
        }
        let velocity = vel.unwrap();

        Self {
            original_lane: position.y,
            position,
            velocity,
            move_left_chance,
            move_right_chance,
        }
    }

    fn go_left(&self) -> Position {
        Position::new(self.position.x, self.position.y + 1)
    }

    fn go_right(&self) -> Position {
        Position::new(self.position.x, self.position.y - 1)
    }

    /// Check if the vehicle can go left by checking if it is in bounds
    /// # Arguments
    /// * `road` - The road to check if the vehicle can go left on
    fn can_go_left(&self) -> bool {
        self.position.y < 2
    }

    /// Check if the vehicle can go right by checking if it is in bounds
    /// # Arguments
    /// * `road` - The road to check if the vehicle can go right on
    /// # Returns
    /// True if the vehicle can go right, false otherwise
    fn can_go_right(&self) -> bool {
        self.position.y > 0
    }

    // 1. Car checks maximum speed it can achieve on it's current position (x, lane) and adjacent lane (x, lane+1).
    // 2. If the potential maximal speed on lane+1 is higher it checks safe conditions:
    // 3. Distance to previous car on lane+1 is greater that it's speed to avoid emergency braking of previous car.
    // 4. Change lane with probability P.
    pub fn update(self, road: &Road) -> Self {
        self.update_lane(road).update_x(road)
    }

    pub fn update_x(self, road: &Road) -> Self {
        self.accelerate(road).decelerate(road).update_position(road)
    }

    fn accelerate(mut self, road: &Road) -> Self {
        self.velocity = min(
            road.get_max_velocity_on_position(self.position.clone()),
            Velocity::new(self.velocity.into_inner() + 1),
        );
        self
    }

    fn decelerate(mut self, road: &Road) -> Self {
        let mut rng = rand::thread_rng();
        let r = rng.gen::<f32>();

        if r < road.deceleration_probability && self.velocity.into_inner() > 0 {
            self.velocity -= 1;
        }

        self
    }

    fn update_position(mut self, road: &Road) -> Self {
        self.position.x = (self.position.x + self.velocity.into_inner()) % road.len;
        self
    }

    //Update the lane of the vehicle
    fn update_lane(mut self, road: &Road) -> Self {
        let mut rng = rand::thread_rng();

        if self.willing_to_move_right(road) {
            if rng.gen_bool(self.move_right_chance as f64) {
                self.position = self.go_right();
            }
            return self;
        }
        if self.willing_to_move_left(road) {
            if rng.gen_bool(self.move_left_chance as f64) {
                self.position = self.go_left();
            }
            return self;
        }

        self
    }

    fn willing_to_move_left(&self, road: &Road) -> bool {
        if self.can_go_left() {
            self.willing_to_change_lane(road, self.position.y + 1)
        } else {
            false
        }
    }

    fn willing_to_move_right(&self, road: &Road) -> bool {
        if self.can_go_right() {
            self.willing_to_change_lane(road, self.position.y - 1)
        } else {
            false
        }
    }

    fn willing_to_change_lane(&self, road: &Road, lane: u8) -> bool {
        let src_lane_speed = road.get_max_velocity_on_position(self.position.clone());
        let dst_lane_speed =
            road.get_max_velocity_on_position(Position::new(self.position.x, lane));

        if dst_lane_speed <= src_lane_speed {
            return false;
        }

        let pos = &self.position;
        let previous_vehicle = road.find_previous_vehicle(pos.clone());

        match previous_vehicle {
            Some(v) => {
                let distance_to_previous_vehicle = road.dist_between_vehicles(pos.x, v.position.x);
                distance_to_previous_vehicle > v.velocity.into_inner()
            }
            None => true,
        }
    }
}
