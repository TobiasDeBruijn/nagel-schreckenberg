use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct Mover;

impl Transformer<Road> for Mover {
    fn transform(self, mut r: Road) -> Road {
        for vs in r.get_vehicles_mut() {
            vs.position += vs.velocity;
        }

        r
    }
}
