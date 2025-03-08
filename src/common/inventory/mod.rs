//! # Inventory code.
//!
//! Each inventory item is an InventoryDef, which defines the type
//! of item ("part", "food", etc), and other parameters such as
//! mass and cost.

// Written by:
// * Gustavo Ramos Rehermann <rehermann6046@gmail.com>
//
// (c)2025 GameCircular. Under the Cooperative Non-Violent Public License.
//
// Loot & Roam is non-violent software: you can use, redistribute,
// and/or modify it under the terms of the CNPLv6+ as found
// in the LICENSE file in the source code root directory or
// at <https://git.pixie.town/thufie/CNPL>.
//
// Loot & Roam comes with ABSOLUTELY NO WARRANTY, to the extent
// permitted by applicable law.  See the CNPL for details.

use std::u8;

pub struct CannonDef {
    /// The minimum amount of power with which to launch a cannonball.
    pub min_power: f32,

    /// The maximum amount of power with which to launch a cannonball.
    pub max_power: f32,

    /// The inaccuracy of the cannon, in max. radians to either side.
    pub spread: f32,

    /// The interval betwen cannon shots, in centiseconds.
    pub fire_rate: u16,

    /// The caliber of the cannon, in tenths of millimeters.
    pub caliber: u8,
}

pub struct BallistaDef {
    /// The power with which to fire a ballista bolt.
    pub power: f32,

    /// The vertical inclation to fire the ballista bolts at, in radians up.
    ///
    /// Ballistas have static vertical inclination, to balance out their speed
    /// and horizontal trajectories.
    pub inclination: f32,

    /// The interval betwen bolt shots, in centiseconds.
    pub fire_rate: u16,
}

pub struct MinelayerDef {
    /// The power with which to launch a mine backward.
    pub power: f32,

    /// The interval betwen mines laid, in centiseconds.
    pub fire_rate: u16,
}

pub enum GunTypeDef {
    Cannon(CannonDef),
    Ballista(BallistaDef),
    Minelayer(MinelayerDef),
}

pub struct GunDef {
    pub gun_type: GunTypeDef,
}

pub struct EngineDef {
    /// The type of fuel used by this engine.
    ///
    /// None means a manual engine.
    pub fuel_type: Option<FuelType>,

    /// The engine power in Newtons per second.
    pub power: u32,

    /// The fuel consumption per second in thousandths of item units.
    pub fuel_consumption: u16,
}

pub struct ArmorDef {
    pub defense_factor: u8,
    pub wear_factor: u8,
    pub deflect_factor: u8,
    pub overwhelm_factor: u8,
}

pub struct VacuumDef {
    pub suck_radius: f32,
    pub suck_strength: f32,
}

pub enum PartTypeDef {
    Gun(GunDef),
    Engine(EngineDef),
    Vacuum(VacuumDef),
    Armor(ArmorDef),
}

pub enum ManningType {
    Unmanned,
    AnyManned,
    StrengthManned(u8),
}

pub struct ItemPartDef {
    pub part_type: PartTypeDef,
    pub manned: ManningType,
}

pub struct FoodDef {
    pub food_points: u8,
}

pub enum FuelType {
    Coal,
    Diesel,
}

pub struct FuelDef {
    pub fuel_type: FuelType,
}

pub struct CannonballDef {
    /// Cannonball caliber, in tenths of millimeters.
    pub caliber: u8,
}

pub struct GrenadeDef {
    /// Fuse length, in centiseconds.
    pub fuse_time: u16,

    /// Explosion power.
    pub power: f32,
}

pub struct MineDef {
    /// Proximity detection range.
    pub trigger_range: f32,

    /// Explosion power.
    pub power: f32,
}

pub enum AmmoType {
    Cannonball(CannonballDef),
    BallistaBolt,
    Grenade(GrenadeDef),
    NavalMine(MineDef),
}

pub struct AmmoDef {
    pub ammo_type: AmmoType,
    // [WIP] Projectile modifier list, to implement in a submodule.
    // pub modifiers: Vec<ProjectileModifier>,
}

pub enum ItemType {
    Part(ItemPartDef),
    Food(FoodDef),
    Fuel(FuelDef),
    Ammo(AmmoDef),
}

/// An inventory item definition.
pub struct InventoryDef {
    pub item_type: ItemType,
    pub name: String,
    pub mass: f32,
    pub unit_cost: u32,
    pub drop_chance: u8,
    pub vulnerability: u8,
    pub repair_cost_scale: u16,

    /// Amount of this item.
    pub amount: f32,
}
