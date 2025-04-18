use rand::prelude::*;
use std::collections::{VecDeque, HashMap};

use crate::id_generator::IDGenerator;
use crate::events::*;
use crate::game::*;

#[derive(Debug, Clone, Copy)]
pub struct Resource {
    pub id: u32,
    pub loc: Localization,
    pub display: char,
    pub kind: ResourceKind,
    pub initial_quantity: u16,
    pub remaining_quantity: u16,
}

#[derive(Debug, Clone, Copy)]
pub enum ResourceKind {
    Crystal,
    Energy,
}
impl ResourceKind {
    pub fn from_str(ressource_kind_str: &str) -> Option<ResourceKind> {
        match ressource_kind_str.to_lowercase().as_str() {
            "crystal" => Some(ResourceKind::Crystal),
            "energy" => Some(ResourceKind::Energy),
            _ => None,
        }
    }
}

// pub trait CrystalOperations {
//     fn dig(&mut self);
// }

// pub trait EnergyOperations {
//     fn reload(&mut self);
// }

// pub trait ResourceOperations: CrystalOperations + EnergyOperations {
pub trait ResourceOperations {
    fn calculate_gather(&mut self, qt: u16, gatherer_rate: f32) -> u16 ;
    fn gather(&mut self, qt: u16, gatherer_rate: f32) -> (u16,u16) ;
}

impl ResourceOperations for Resource {
    fn calculate_gather(&mut self, qt: u16, gatherer_rate: f32) -> u16 {
        let max_extractable = (qt as f32 * gatherer_rate).round() as u16;
        let extracted = if self.remaining_quantity < max_extractable {
            let extracted = self.remaining_quantity;
            self.remaining_quantity = 0;
            extracted
        } else {
            self.remaining_quantity -= max_extractable;
            max_extractable
        };
        extracted
    }
    fn gather(&mut self, qt: u16, gatherer_rate: f32) -> (u16, u16) {
        let qty = self.calculate_gather(qt, gatherer_rate);
    
        match self.kind {
            ResourceKind::Crystal => (qty, 0),
            ResourceKind::Energy => (0, qty),
        }
    }
}

impl Resource {
    pub fn new_resource(
        loc: Localization,
        kind: ResourceKind,
        initial_quantity: u16,
        id_generator: &mut IDGenerator,
    ) -> Option<Self> {
        let id = id_generator.generate_id();
        let display = match kind {
            ResourceKind::Crystal => 'C',
            ResourceKind::Energy => 'E',
        };

        Some(
            Self {
                id,
                loc,
                kind,
                display,
                initial_quantity: initial_quantity,
                remaining_quantity: initial_quantity,
            }
        )
    }

    fn initialize_rng(&self, seed: u64) -> StdRng {
        StdRng::seed_from_u64(seed.wrapping_add(
            self.id.pow(5) as u64 * 31 + self.loc.x as u64 * 17 + self.loc.y as u64 * 13
        ))
    }
}
