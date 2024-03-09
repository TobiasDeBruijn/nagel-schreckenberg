use crate::model::Model;
use crate::transformers::{Accelerator, Decelerator, LaneMover, Mover, Randomizer};
use crate::typedef::Road;

pub mod model;
pub mod transformers;
pub mod typedef;

// 1. Car checks maximum speed it can achieve on it's current position (x, lane) and adjacent lane (x, lane+1).
// 2. If the potential maximal speed on lane+1 is higher it checks safe conditions:
// 3. Distance to previous car on lane+1 is greater that it's speed to avoid emergency braking of previous car.
// 4. Change lane with probability P.
// Same steps for lane-1

/// Step the simulation forward by one time step
/// 1. Car checks maximum speed it can achieve on it's current position (x, lane) and adjacent lane (x, lane+1).
/// 2. If the potential maximal speed on lane+1 is higher it checks safe conditions:
/// 3. Distance to previous car on lane+1 is greater that it's speed to avoid emergency braking of previous car.
/// 4. Change lane with probability P.
/// Same steps for lane-1
/// # Arguments
/// * `road` - The road to step forward
/// # Returns
/// The road after the time step
/// # Example
/// ```
/// use sim::{Road, Vehicle, Position, Velocity};
/// let road = Road::new(
///     100,
///     0.0,
///     (0..10)
///         .into_iter()
///         .map(|x| Vehicle::new(Position::new(x, 0), 0.9, 0.1))
///         .collect::<Vec<_>>(),
///     vec![Velocity::new(30), Velocity::new(30), Velocity::new(30)],
///     )
/// let new_road = sim::step(road);
/// ```
pub fn step(road: Road) -> Road {
    Model::new(road)
        .apply(Accelerator)
        .apply(Decelerator)
        .apply(Randomizer)
        .apply(LaneMover)
        .apply(Mover)
        .finish()
}
