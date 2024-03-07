use rand::Rng;
use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct LaneMover;

impl Transformer<Road> for LaneMover {
    fn transform(self, mut r: Road) -> Road {
        let occupied_spaces = r.vehicles
            .iter()
            .map(|v| (v.position.x, v.position.y))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        for v in r.get_vehicles_mut() {
            let is_ahead = occupied_spaces.contains(&(v.position.x + 1, v.position.y));
            let is_left = occupied_spaces.contains(&(v.position.x, v.position.y + 1));
            let chance = rng.gen_bool(v.move_left_chance as f64);

            if is_ahead && !is_left && v.position.y != 2 && chance {
                v.position.y += 1;
            }
        }

        r
    }
}