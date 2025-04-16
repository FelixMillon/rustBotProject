pub struct GameState {
    pub tick: u64,
    pub seed: u64,
    pub map_matrix: Vec<Vec<Cell>>,
    pub gatherers: HashMap<u32, Gatherer>,
    pub scouts: HashMap<u32, Scout>,
    pub resources: HashMap<u32, Resource>,
    pub base: Base,
    pub discovered_resources: Vec<u32>,
}

use crate::{scout::Scout, gatherer::Gatherer, resource::Resource, events::{Event, EventType}, map::{Localization, GameState, generate_map}};
use std::collections::HashMap;

pub struct Game {
    pub state: GameState,
    pub tick: u64,
}

impl Game {
    pub fn new(map_matrix: Vec<Vec<Cell>>) -> Self {
        Self {
            state: GameState::new(map_matrix),
            tick: 0,
        }
    }

    pub fn add_scout(&mut self, scout: Scout) {
        self.state.add_scout(scout);
    }

    pub fn add_gatherer(&mut self, gatherer: Gatherer) {
        self.state.add_gatherer(gatherer);
    }

    pub fn update(&mut self) {
        self.tick += 1;
        
        // Mise à jour des scouts
        for scout in self.state.scouts.values_mut() {
            if let Some(event) = scout.update(&self.state.map_matrix, self.tick) {
                self.handle_event(event);
            }
        }

        // Mise à jour des gatherers
        for gatherer in self.state.gatherers.values_mut() {
            if let Some(event) = gatherer.update(&self.state.resources, self.tick) {
                self.handle_event(event);
            }
        }
        
        // Gérer d'autres événements globaux
    }

    fn handle_event(&mut self, event: Event) {
        match event.event_type {
            EventType::ResourceGathered => {
                // Gestion de la collecte des ressources
            },
            EventType::Exploration => {
                // Gestion de l'exploration
            },
            _ => {}
        }
    }

    pub fn display(&self) {
        // Affiche la carte et les informations du jeu
    }
}