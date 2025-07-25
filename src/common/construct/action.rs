//! Part action events

use bevy::{
    ecs::{
        entity::Entity,
        event::{Event, EventReader, EventWriter},
        observer::Trigger,
        system::{Commands, In, IntoSystem, Query},
    },
    log::info,
    reflect::Reflect,
};

use crate::common::construct::{part::ConstructParts, slot::PartInfo};

/// A part action event.
#[derive(Event)]
pub struct PartAction {
    /// The action tag of this event.
    ///
    /// Nominally identifies this action. Used for parts to determine whether
    /// they want to consume this event or not.
    ///
    /// Try to be unambiguous with its meaning, but terse. Use snake_case.
    ///
    /// Examples:
    /// * `"fire_weapon"`
    /// * `"thrust"`
    /// * `"steer"`
    pub action_tag: String,

    /// Any data passed to the action handler.
    ///
    /// For example, a `"fire_weapon"` event may have its data be a
    /// WeaponFireOptions struct, which informs a weapon
    /// * The desired position to shoot at, if possible, assuming the weapon can
    ///   back-calculate requested power (Newtons) and angle (radians) from this
    /// * A descriptor or selector for which ammunition type to shoot if
    ///   available
    ///   TODO: transplant the below into the WeaponFireArgs documentation
    ///   * May cascade with fallbacks. For example,
    ///     ```
    ///     [
    ///       'cannonball 40mm incendiary',
    ///       'cannonball 40mm propeller_gum',
    ///       'cannonball 40mm'
    ///     ]
    ///     ```
    ///     tries to find 40mm with the incendiary charge and propeller gum
    ///     modifiers first, and fires a vanilla round if not found.
    ///   * Ammunition that is incompatible is ignored (e.g. cannon and
    ///     cannonball with mismatching callibers)
    pub data: Box<dyn Reflect>,
}

impl Clone for PartAction {
    fn clone(&self) -> Self {
        PartAction {
            action_tag: self.action_tag.clone(),
            data: self.data.reflect_clone().unwrap(),
        }
    }
}

/// An action request that a construct should dispatch to its parts.
#[derive(Event)]
pub struct PartActionDispatchRequest {
    /// A reference to the construct that will dispatch this action event to
    /// its parts.
    pub construct_ref: Entity,

    /// Selects parts to which to dispatch this action event.
    ///
    /// A part is only dispatched to if its part tag is inside this vector.
    ///
    /// If the vector is empty (as by default), this event is dispatched to all
    /// parts of the construct.
    pub part_tag_selectors: Vec<String>,

    /// The shared action data of this event.
    pub action: PartAction,
}

pub fn ev_dispatch_part_actions(
    mut commands: Commands,
    mut all_events: EventReader<PartActionDispatchRequest>,
    list_parts_query: Query<&ConstructParts>,
    part_info_query: Query<&PartInfo>,
) {
    for construct_event in all_events.read() {
        let target = construct_event.construct_ref;
        if let Ok(parts) = list_parts_query.get(target) {
            for &part_id in parts.iter() {
                // If the part tag selector is empty, skip matching check
                if !construct_event.part_tag_selectors.is_empty() {
                    let part_info = part_info_query.get(part_id).unwrap();
                    // Skip parts that don't match any part tag selector
                    if !construct_event
                        .part_tag_selectors
                        .iter()
                        .any(|tag| part_info.tags.contains(&tag))
                    {
                        continue;
                    }
                }

                commands
                    .entity(part_id)
                    .trigger(construct_event.action.clone());
            }
        }
    }
}

#[derive(Reflect)]
pub struct DebugPrintPart {
    extra_message: Option<String>,
}

pub fn dispatch_action(
    construct_ref: Entity,
    commands: &mut Commands,
    part_tag_selectors: Vec<String>,
    action: PartAction,
) {
    fn _inner(
        In((construct_ref, part_tag_selectors, action)): In<(Entity, Vec<String>, PartAction)>,
        mut writer: EventWriter<PartActionDispatchRequest>,
    ) {
        writer.write(PartActionDispatchRequest {
            construct_ref,
            part_tag_selectors,
            action,
        });
    }
    commands.run_system_cached_with(_inner, (construct_ref, part_tag_selectors, action));
}

// Observer
pub fn obs_debug_part_action(trigger: Trigger<PartAction, PartInfo>, query: Query<&PartInfo>) {
    let part_info = query.get(trigger.target()).unwrap();
    if let Some(data) = trigger.data.downcast_ref::<DebugPrintPart>() {
        info!(
            "Part with tags {:?} received debug action: {}",
            part_info.tags,
            data.extra_message.clone().unwrap_or("".to_owned())
        );
    }
}
