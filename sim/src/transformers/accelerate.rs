use crate::transformers::Transformer;
use crate::typedef::{Road, Velocity};

pub struct Accelerator;

impl Transformer<Road> for Accelerator {
    fn transform(self, mut r: Road) -> Road {
        for lane in 0..3_u8 {
            apply_lane(&mut r, lane);
        }

        r
    }
}

fn apply_lane(r: &mut Road, lane: u8) {
    let vmax = r
        .get_max_velocity_in_lane(lane)
        .expect("Getting vmax for a lane");
    let mut vs = r.get_vehicles_in_lane_mut(lane);

    let mut to_accelerate = vs
        .windows(2)
        .into_iter()
        .filter(|vs| {
            let v = &vs[0];
            println!("vpos {} {}", v.position.x, v.position.y);
            match vs.get(1) {
                Some(vnext) => {
                    v.velocity < vmax
                        && v.position.distance_1d(&vnext.position)
                            > (v.velocity.into_inner() + 1) as u128
                }
                None => v.velocity < vmax,
            }
        })
        .map(|f| f[0].position.clone())
        .collect::<Vec<_>>();

    if vs.len() % 2 != 0 {
        match vs.last() {
            Some(v) if v.velocity < vmax => to_accelerate.push(v.position.clone()),
            Some(_) | None => {}
        }
    }

    vs.iter_mut()
        .filter(|f| to_accelerate.contains(&f.position))
        .for_each(|f| f.velocity += Velocity::new(1));
}

#[cfg(test)]
mod test {
    use crate::transformers::accelerate::apply_lane;
    use crate::typedef::{Position, Road, Vehicle, Velocity};

    #[test]
    fn simple() {
        let mut r = Road::new(
            vec![
                Vehicle::new(Position::new(0, 0)),
                Vehicle::new(Position::new(20, 0)),
            ],
            vec![Velocity::new(10)],
        );

        for _ in 0..=10 {
            apply_lane(&mut r, 0);
        }

        let lane = r.get_vehicles_in_lane(0);
        assert_eq!(lane[0].velocity, Velocity::new(10));
    }
}
