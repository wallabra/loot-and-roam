/*!
 * Physics subsystems.
 *
 * A physics subsystem operates on physics elements, giving them life.
 *
 * There are a few types of physics subsystems:
 *
 * * Point: operate on internal points.
 *
 * * Surface and Volume: operate on volumetric primitives.
 *
 * * Springs: operate on springs between points.
 */

use super::PhysicsRegistry;

pub mod elementary;

pub trait PhysicsSubsystem {
    fn apply_subsystem(&self, registry: &mut PhysicsRegistry, delta_time: f32);
}