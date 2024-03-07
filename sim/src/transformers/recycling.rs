use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct Recycler;

impl Transformer<Road> for Recycler {
    fn transform(self, mut r: Road) -> Road {
        let len = r.road_length as u128;
        for v in r.get_vehicles_mut() {
            if v.position.x >= len {
                v.position.x = 0;
            }
        }

        r
    }
}