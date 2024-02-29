use crate::model::Model;
use crate::transformers::{Accelerator, Decelerator, Randomizer, Transformer};
use crate::typedef::Road;

mod model;
mod transformers;
mod typedef;

pub fn step(road: Road) -> Road {
    Model::new(road)
        .apply(Accelerator)
        .apply(Decelerator)
        .apply(Randomizer)
        .finish()
}
