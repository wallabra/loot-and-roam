/*!
 * Elementary physics definitions.
 *
 * "Physics elements" are the most basic building blocks of all physically
 * enabled physcis objects in Loot & Roam. These include:
 *
 * * Internal points: they represent the location of different parts of an
 *                    object, and how these parts are linked together. They
 *                    also interact with point elementary forces (gravity).
 *                    All points have their own masses.
 *
 * * Springs: they represent the linkages between internal points, varying
 *            in stiffness. They always seek to preserve an "equilibrium
 *            length", achieving that by applying a constant force on all
 *            points in order to bring their real distances into convergence
 *            with spring equilibrium.
 *
 * * Volumetric primitives: they give volume to physics objects, in the eyes
 *                          of the simulation. There are multiple types of
 *                          primitives, cylinders and spheres. They interact
 *                          with volumetric elementary forces (buoyancy) and
 *                          surface elementary forces (friction, drag).
 */

use ultraviolet::Vec3;

pub mod volumes;

/**
 * A physics element that joins two [Point]s
 */
#[derive(Clone, Copy, Debug, Default)]
pub struct Spring {
    pub(crate) which_point_a: usize,
    pub(crate) which_point_b: usize,
    pub(crate) stiffness: f32,
    pub(crate) equilibrium_length: f32,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Point {
    pub(crate) pos: Vec3,
    pub(crate) mass: f32,
}
