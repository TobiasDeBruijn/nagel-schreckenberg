mod accelerate;
mod decelerate;
mod mover;
mod random;
mod recycling;
mod move_lane;

pub use accelerate::Accelerator;
pub use decelerate::Decelerator;
pub use mover::Mover;
pub use random::Randomizer;
pub use recycling::Recycler;
pub use move_lane::LaneMover;

pub trait Transformer<R> {
    fn transform(self, r: R) -> R;
}
