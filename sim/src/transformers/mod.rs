mod accelerate;
mod decelerate;
mod mover;
mod random;

pub use accelerate::Accelerator;
pub use decelerate::Decelerator;
pub use mover::Mover;
pub use random::Randomizer;

pub trait Transformer<R> {
    fn transform(self, r: R) -> R;
}
