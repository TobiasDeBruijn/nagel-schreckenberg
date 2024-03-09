use crate::transformers::Transformer;
use crate::typedef::{Road, Velocity};
use tracing::trace;

pub struct Accelerator;

impl Transformer<Road> for Accelerator {
    fn transform(self, mut r: Road) -> Road {
        for lane in 0..3_u8 {
            apply_lane(&mut r, lane);
        }

        r
    }
}

/// Apply the acceleration rules to a single lane
/// This is done by iterating over the vehicles in the lane, and checking if the distance
/// to the next vehicle is greater than the current velocity. If it is, then the velocity is
/// increased by 1, unless the velocity is already at the maximum.
///
/// # Arguments
/// * `r` - The road to apply the acceleration to
/// * `lane` - The lane to apply the acceleration to
fn apply_lane(r: &mut Road, lane: u8) {
    let vmax = r
        .get_max_velocity_in_lane(lane)
        .expect("Getting vmax for a lane");
    let mut vs = r.get_vehicles_in_lane_mut(lane);
    vs.sort_by(|a, b| a.position.cmp(&b.position));

    let mut to_accelerate = vs
        .windows(2)
        .into_iter()
        .filter(|vs| {
            let v = &vs[0];
            match vs.get(1) {
                Some(vnext) => {
                    v.velocity < vmax
                        && v.position.distance_1d(&vnext.position) > (v.velocity.into_inner() + 1)
                }
                None => v.velocity < vmax,
            }
        })
        .map(|f| f[0].position.clone())
        .collect::<Vec<_>>();

    // If the number of vehicles is odd, then the last vehicle should be checked for acceleration
    // We do this because the last vehicle will not be checked in the windows iterator
    if vs.len() % 2 != 0 {
        match vs.last() {
            Some(v) if v.velocity < vmax => to_accelerate.push(v.position.clone()),
            Some(_) | None => {}
        }
    }

    trace!("Accelerating {} vehicles", to_accelerate.len());

    vs.iter_mut()
        .filter(|f| to_accelerate.contains(&f.position))
        .for_each(|f| f.velocity += Velocity::new(1));
}
