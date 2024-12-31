//! Ship definitions.

use parts::ShipPart;
pub mod parts;

pub struct ShipMakeup {
    parts: Vec<Box<dyn ShipPart>>,
}
