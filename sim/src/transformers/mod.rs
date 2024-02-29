mod accelerate;
mod decelerate;
mod random;

pub use accelerate::Accelerator;
pub use decelerate::Decelerator;
pub use random::Randomizer;

pub trait Transformer<R> {
    fn transform(self, r: R) -> R;
}
