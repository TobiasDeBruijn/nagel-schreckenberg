use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct Randomizer;

impl Transformer<Road> for Randomizer {
    fn transform(self, r: Road) -> Road {
        // TODO
        r
    }
}
