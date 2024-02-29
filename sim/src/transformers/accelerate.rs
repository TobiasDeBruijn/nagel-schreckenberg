use crate::transformers::Transformer;
use crate::typedef::Road;

pub struct Accelerator;

impl Transformer<Road> for Accelerator {
    fn transform(self, r: Road) -> Road {
        // TODO
        r
    }
}
