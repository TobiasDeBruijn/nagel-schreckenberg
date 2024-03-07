use crate::transformers::Transformer;
use crate::typedef::Road;
use rand::Rng;

pub struct LaneMover;

impl Transformer<Road> for LaneMover {
    fn transform(self, mut r: Road) -> Road {
        let mut occupied_spaces = r
            .vehicles
            .iter()
            .map(|v| (v.position.x, v.position.y))
            .collect::<Vec<_>>();

        let mut rng = rand::thread_rng();

        for v in r.get_vehicles_mut() {
            if v.position.y != 2 {
                let is_ahead = occupied_spaces.contains(&(v.position.x + 1, v.position.y));
                let is_left = occupied_spaces.contains(&(v.position.x, v.position.y + 1));
                let chance = rng.gen_bool(v.move_left_chance as f64);

                if is_ahead && !is_left && chance {
                    occupied_spaces = occupied_spaces
                        .into_iter()
                        .map(|(x, y)| {
                            if x == v.position.x && y == v.position.y {
                                (x, y + 1)
                            } else {
                                (x, y)
                            }
                        })
                        .collect::<Vec<_>>();
                    v.position.y += 1;

                    continue;
                }
            }

            if v.position.y != 0 {
                let is_right = occupied_spaces.contains(&(v.position.x, v.position.y - 1));
                let chance = rng.gen_bool(v.move_right_chance as f64);
                if !is_right && chance {
                    occupied_spaces = occupied_spaces
                        .into_iter()
                        .map(|(x, y)| {
                            if x == v.position.x && y == v.position.y {
                                (x, y - 1)
                            } else {
                                (x, y)
                            }
                        })
                        .collect::<Vec<_>>();
                    v.position.y -= 1;
                }
            }
        }

        r
    }
}
