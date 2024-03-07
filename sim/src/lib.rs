use crate::model::Model;
use crate::transformers::{Accelerator, Decelerator, LaneMover, Mover, Randomizer};
use crate::typedef::Road;

pub mod model;
pub mod transformers;
pub mod typedef;

pub fn step(road: Road) -> Road {
    Model::new(road)
        .apply(Accelerator)
        .apply(Decelerator)
        .apply(Randomizer)
        .apply(LaneMover)
        .apply(Mover)
        .finish()
}
