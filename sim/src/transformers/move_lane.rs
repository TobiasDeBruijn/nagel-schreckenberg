use std::collections::HashMap;
use rand::Rng;
use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct LaneMover;

impl Transformer<Road> for LaneMover {
    fn transform(self, mut r: Road) -> Road {
        let mut occupied_spaces = r.vehicles
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
                    occupied_spaces
                        .iter_mut()
                        .for_each(|(x, y)| if *x == v.position.x && *y == v.position.y {
                            *y += 1;
                        });
                    v.position.y += 1;
                }
            } else if v.position.y != 0 {
                let is_right = occupied_spaces.contains(&(v.position.x, v.position.y - 1));
                if !is_right {
                    occupied_spaces
                        .iter_mut()
                        .for_each(|(x, y)| if *x == v.position.x && *y == v.position.y {
                            *y -= 1;
                        });
                    v.position.y -= 1;
                }
            }
        }

        let mut seen = HashMap::new();
        for v in &r.vehicles {
            seen.entry((v.position.x, v.position.y))
                .and_modify(|v| *v += 1)
                .or_insert(1);
        }

        if seen.values().find(|v| **v > 1).is_some() {
            panic!("Lol");
        }

        r
    }
}