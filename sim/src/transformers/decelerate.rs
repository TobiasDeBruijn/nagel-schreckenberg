use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct Decelerator;

impl Transformer<Road> for Decelerator {
    fn transform(self, r: Road) -> Road {
        // TODO
        r
    }
}
