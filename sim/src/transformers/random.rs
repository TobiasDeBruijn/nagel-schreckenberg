use crate::transformers::Transformer;
use crate::typedef::{Road, Velocity};
use std::cmp::max;
use rand::Rng; // Add this line to import the `Rng` trait from the `rand` crate

pub struct Randomizer;

impl Transformer<Road> for Randomizer {
    fn transform(self, r: Road) -> Road {
        //foreach vehicle on the road, generate a random number and if it is less than r.deceleration_probability, decelerate the vehicle
        let mut r = r;
        let deceleration_probability = r.deceleration_probability;
        for vs in r.get_vehicles_mut() {
            let random_number = rand::thread_rng().gen::<f32>();
            if random_number < deceleration_probability {
                vs.velocity = Velocity::new(max((vs.velocity.into_inner() as i8  - 1) as u8, 0));
            }
        }
        r
    }
}
