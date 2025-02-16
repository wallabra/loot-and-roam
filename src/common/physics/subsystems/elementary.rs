/*!
 * Elementary physics systems.
 */

use itertools::izip;
use ultraviolet::Vec3;

use crate::common::{config::Configurable, physics::PhysicsRegistry};

use super::PhysicsSubsystem;
 
/**
 * Applies gravity to all points in the physics registry.
 */
pub struct GravitySystem {
    gravity: Vec3,
}

impl GravitySystem {
    pub fn get_gravity(&self) -> Vec3 {
        self.gravity
    }
}

impl PhysicsSubsystem for GravitySystem {
    fn apply_subsystem(&self, registry: &mut PhysicsRegistry, delta_time: f32) {
        registry.velocities.iter_mut().for_each(|vel| *vel += self.gravity * delta_time);
    }
}

impl Configurable for GravitySystem {
    fn check_config_change(&mut self, config_name: &str, value: crate::common::config::ConfigValue) {
        if config_name != "phys.gravity" {
            return;
        }
    
        if let Ok(value) = value.coerce_vector() {
            self.gravity = value;
        }
    }
}

/**
 * Applies drag to all volumed points in the physics registry.
 */
pub struct DragSystem {
    water_level: f32,
    water_drag_factor: f32,
    air_drag_factor: f32,
}

impl DragSystem {
    pub fn get_air_drag_factor(&self) -> f32 {
        self.air_drag_factor
    }
    
    pub fn get_water_drag_factor(&self) -> f32 {
        self.water_drag_factor
    }
}

impl PhysicsSubsystem for DragSystem {
    fn apply_subsystem(&self, registry: &mut PhysicsRegistry, delta_time: f32) {
        registry.volumes.iter_mut().for_each(|vol| {
            
        });
    }
}

impl Configurable for DragSystem {
    fn check_config_change(&mut self, config_name: &str, value: crate::common::config::ConfigValue) {
        if config_name == "phys.water_level" {
            self.water_level = value.coerce_float().unwrap_or(self.water_level);
        }
        
        if config_name == "phys.drag.air" {
            self.air_drag_factor = value.coerce_float().unwrap_or(self.air_drag_factor);
        }
        
        if config_name == "phys.drag.water" {
            self.water_drag_factor = value.coerce_float().unwrap_or(self.water_drag_factor);
        }
    }
}