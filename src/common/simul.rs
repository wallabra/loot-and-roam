//! Simulation code.

use std::any::{Any, TypeId};

use super::{obj::defs::ship::Ship, shipmakeup::ShipMakeup, terrain::base::Terrain};

pub struct Simulation {
    tickables: Vec<Box<dyn Tickable>>,
    terrain: Terrain,
    player_fleet: Vec<usize>,
}

pub struct EndOfSimulation {
    pub player_fleet_defs: Vec<ShipMakeup>,
}

impl Simulation {
    pub fn new(terrain: Terrain) -> Self {
        Simulation {
            tickables: vec![],
            terrain,
            player_fleet: vec![],
        }
    }

    pub fn tick(&mut self, delta_time: f64) {
        for tickable in self.tickables.iter_mut() {
            if tickable.skip_tick() {
                continue;
            }

            tickable.tick(delta_time);
        }

        self.tickables.retain(|tickable| !tickable.is_destroyed());
    }

    pub fn finish(self) -> EndOfSimulation {
        let mut tickables = self.tickables;
        EndOfSimulation {
            player_fleet_defs: self
                .player_fleet
                .iter()
                .filter_map(|key| {
                    let tickable = tickables.remove(*key);
                    if tickable.type_id() != TypeId::of::<Ship>() {
                        None
                    } else {
                        Some(unsafe { (Box::into_raw(tickable) as *mut Ship).read() })
                    }
                })
                .map(|ship| ship.makeup)
                .collect(),
        }
    }
}

pub trait Tickable {
    fn tick(&mut self, delta_time: f64);
    fn is_destroyed(&self) -> bool;

    fn skip_tick(&self) -> bool {
        self.is_destroyed()
    }
}
