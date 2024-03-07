use crate::transformers::Transformer;
use crate::typedef::{Road, Velocity};
use std::cmp::max;
use rand::Rng; // Add this line to import the `Rng` trait from the `rand` crate

pub struct Randomizer;

impl Transformer<Road> for Randomizer {
    fn transform(self, r: Road) -> Road {
        //foreach vehicle on the road, generate a random number and if it is less than r.deceleration_probability, decelerate the vehicle
        let mut r = r;
        let mut rng = rand::thread_rng();

        let deceleration_probability = r.deceleration_probability;
        for vs in r.get_vehicles_mut() {
            let r = rng.gen::<f32>();
            if r < deceleration_probability {
                vs.velocity = Velocity::new(max(vs.velocity.into_inner() as i8  - 1, 0) as u8);
            }
        }
        r
    }
}
