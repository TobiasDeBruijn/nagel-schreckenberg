use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct Mover;

impl Transformer<Road> for Mover {
    fn transform(self, mut r: Road) -> Road {
        let len = r.len;
        for vs in r.get_vehicles_mut() {
            vs.position.x = (vs.position.x + vs.velocity.into_inner()) % len;
        }

        r
    }
}
