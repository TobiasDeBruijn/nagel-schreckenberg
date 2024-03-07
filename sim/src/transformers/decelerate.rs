use crate::transformers::Transformer;
use crate::typedef::{Road, Velocity};
use std::cmp::max;
use std::collections::HashMap;
use tracing::trace;

pub struct Decelerator;

impl Transformer<Road> for Decelerator {
    fn transform(self, mut r: Road) -> Road {
        for lane in 0..3_u8 {
            apply_lane(&mut r, lane);
        }

        r
    }
}

fn apply_lane(r: &mut Road, lane: u8) {
    let mut vs = r.get_vehicles_in_lane_mut(lane);
    vs.sort_by(|a, b| a.position.cmp(&b.position));

    // Figure out which vehicle needs to be slowed down (due to other traffic ahead)
    // Return those as a HashMap<Position, Velocity>, where the value will be the new velocity
    let mut to_decel = vs
        .windows(2)
        .into_iter()
        .map(|vs| {
            let v = &vs[0];
            match vs.get(1) {
                Some(vnext) => {
                    trace!("vnext {}, {}", vnext.position.x, vnext.position.y);
                    trace!("vthis {}, {}", v.position.x, v.position.y);

                    let delta_x = v.position.distance_1d(&vnext.position);
                    trace!("dx : {delta_x}");
                    trace!("velocity  : {}", v.velocity.into_inner());

                    if delta_x <= v.velocity.into_inner() && v.velocity.into_inner() != 0 {
                        (
                            v.position.clone(),
                            Velocity::new(max(delta_x as i8 - 1, 0) as u8),
                        )
                    } else {
                        (v.position.clone(), v.velocity)
                    }
                }
                None => (v.position.clone(), v.velocity),
            }
        })
        .collect::<HashMap<_, _>>();

    vs.iter_mut()
        .for_each(|vehicle| match to_decel.remove(&vehicle.position) {
            Some(velocity) => vehicle.velocity = velocity,
            None => {}
        });
}
