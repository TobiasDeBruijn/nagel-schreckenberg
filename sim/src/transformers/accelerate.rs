use crate::transformers::Transformer;
use crate::typedef::{Road, Velocity};

pub struct Accelerator;

impl Transformer<Road> for Accelerator {
    fn transform(self, r: Road) -> Road {
        for lane in 0..=3_u8 {
            r = apply_lane(r)
        }
    }
}

fn apply_lane(r: &mut Road, lane: u8) {
    let vmax = r.get_max_velocity_in_lane(lane).expect("Getting vmax for a lane");
    let mut vs = r.get_vehicles_in_lane_mut(lane);

    let to_accelerate = vs
        .windows(2)
        .into_iter()
        .filter(|vs| {
            let v = &vs[0];
            match vs.get(1) {
                Some(vnext) => v.velocity < vmax && v.position.distance_1d(&vnext.position) > (v.velocity.into_inner() + 1) as u128,
                None => v.velocity < vmax
            }
        })
        .map(|f| f[0].position.clone())
        .collect::<Vec<_>>();

    vs
        .iter_mut()
        .filter(|f| to_accelerate.contains(&f.position))
        .for_each(|f| f.velocity += Velocity::new(1));
}