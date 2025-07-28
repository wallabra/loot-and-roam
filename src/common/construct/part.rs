//! Construct-part relationship.

use bevy::ecs::{component::Component, entity::Entity};

/// Component present in all installed construct parts.
///
/// Wraps a reference to the construct to which this part is installed.
#[derive(Component)]
#[relationship(relationship_target = ConstructParts)]
pub struct PartInstalledOn(Entity);

impl PartInstalledOn {
    pub fn get(&self) -> Entity {
        self.0
    }

    pub(crate) fn new(construct_id: Entity) -> Self {
        Self(construct_id)
    }
}

/// Component present in all constructs.
///
/// Lists the parts that are currently installed on this construct.
#[derive(Component)]
#[relationship_target(relationship = PartInstalledOn)]
pub struct ConstructParts(Vec<Entity>);

impl ConstructParts {
    pub fn iter(&self) -> std::slice::Iter<'_, bevy::prelude::Entity> {
        self.0.iter()
    }
}
