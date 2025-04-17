use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, channel};
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::f64;
use std::sync::{Arc, Mutex};
use std::thread;
use std::fs::{create_dir_all, OpenOptions};
use std::io::Write;
use std::path::Path;
use crate::id_generator::IDGenerator;
use crate::gatherers::*;
use crate::scouts::*;
use crate::resources::*;
use crate::events::*;

pub struct Map {
    pub cols: u32,
    pub rows: u32,
    pub seed: u64,
    pub scouts: HashMap<u32, Scout>,
    pub scout_senders: HashMap<u32, Sender<EventType>>,
    pub scout_receivers: HashMap<u32, Receiver<EventType>>,
    pub gatherers: HashMap<u32, Gatherer>,
    pub resources: HashMap<u32, Resource>,
    pub finded_resources: Vec<u32>,  
    pub map_matrix: Vec<Vec<Cell>>,
    pub age: u32,
    pub base: Base,
}

#[derive(Debug, Clone, Copy)]
pub struct Base {
    pub loc: Localization,
    pub crystal: u16,
    pub energy: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct Localization {
    pub x: u32,
    pub y: u32,
}
impl Localization {
    pub fn same_loc(&self, other: &Localization) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub display: char,
    pub explore: i8,
}

impl Base {
    pub fn new(rows: u32, cols: u32) -> Self {
        let loc = Localization{x: ((rows + 1) / 2), y: ((cols + 1) / 2)};
        Self {
            loc,
            crystal: 0,
            energy: 0,
        }
    }
}

impl Map {
    pub fn new(rows: u32, cols: u32, seed: u64) -> Self {
        let scouts = HashMap::new();
        let scout_senders = HashMap::new();
        let scout_receivers = HashMap::new();
        let gatherers = HashMap::new();
        let resources = HashMap::new();
        let mut map_matrix = Vec::new();
        let finded_resources = Vec::new();
        for _ in 0..rows {
            let mut row = Vec::new();
            for _ in 0..cols {
                row.push(Cell { display: ' ', explore: -1 });
            }
            map_matrix.push(row);
        }
        Self {
            rows,
            cols,
            seed,
            scouts,
            scout_senders,
            scout_receivers,
            gatherers,
            resources,
            finded_resources,
            map_matrix,
            age: 0,
            base: Base::new(rows, cols),
        }
    }

    pub fn add_scout(
        &mut self,
        x: u32,
        y: u32,
        id_generator: &mut IDGenerator
    ) {
        let loc = Localization { x, y };

        if let Some(mut scout) = Scout::new(loc, id_generator) {
            let (map_sender, map_receiver): (Sender<EventType>, Receiver<EventType>) = channel();
            let (scout_sender, scout_receiver): (Sender<EventType>, Receiver<EventType>) = channel();
            let map_matrix = self.map_matrix.clone();
            let rows = self.rows;
            let cols = self.cols;
            let seed = self.seed;
            let scout_id = scout.id;

            let mut cloned_scout = scout.clone();
            self.scout_senders.insert(scout_id, scout_sender);
            self.scout_receivers.insert(scout_id, map_receiver);
            self.scouts.insert(scout_id, scout);
            let log_dir = Path::new("logs");
            if !log_dir.exists() {
                create_dir_all(log_dir).expect("Failed to create log directory");
            }
            thread::spawn(move || {
                let thread_id = std::thread::current().id();
                cloned_scout.handle_events(map_matrix, rows, cols, seed, thread_id, scout_receiver, map_sender);
            });

        }
    }

    pub fn add_gatherer(
        &mut self,
        x: u32,
        y: u32,
        id_generator: &mut IDGenerator
    ) {
        let loc = Localization{ x, y };
        if let Some(gatherer) = Gatherer::new(loc, id_generator) {
            self.gatherers.insert(gatherer.id, gatherer);
        }
    }

    pub fn add_resource(
        &mut self,
        resource_kind_str: &str,
        initial_quantity: u16,
        id_generator: &mut IDGenerator
    ) {
        if let Some(kind) = ResourceKind::from_str(resource_kind_str) {
            let loc = self.find_free_localization();
            if let Some(resource) = Resource::new_resource(loc, kind, initial_quantity, id_generator) {
                self.resources.insert(resource.id, resource);
            }
        } else {
            eprintln!("Unknown resource kind : {}", resource_kind_str);
        }
    }

    pub fn find_free_localization(&self) -> Localization {
        let mut rng = StdRng::seed_from_u64(self.seed.wrapping_add(
            self.age.pow(2) as u64 * 13
        ));
        let mut loc = Localization { x: 0, y: 0 };
        loop {
            rng = StdRng::seed_from_u64(rng.gen::<u64>().wrapping_add(11));
            let x = rng.gen_range(0..self.rows);
            let y = rng.gen_range(0..self.cols);
            let cell = &self.map_matrix[x as usize][y as usize];
    
            if cell.display != '#' && cell.display != '8' {
                let mut is_free = true;
                for resource in self.resources.values() {
                    if resource.loc.x == x && resource.loc.y == y {
                        is_free = false;
                        break;
                    }
                }

                if is_free {
                    loc = Localization { x, y };
                    break;
                }
            }
        }
        return loc;
    }

    pub fn update_explore_matrix(&mut self) {

        let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("logs/explore.log")
        .expect("Impossible d'ouvrir le fichier explore.log");

        for scout in self.scouts.values() {
            let x = scout.loc.x as i32;
            let y = scout.loc.y as i32;
            let _ = writeln!(file, "  Scout {}: ({}, {})", scout.id, x, y);
            for delta_x in -1..=1 {
                for delta_y in -1..=1 {
                    let dx = x + delta_x;
                    let dy = y + delta_y;

                    if dx >= 0 && dx < self.rows as i32 && dy >= 0 && dy < self.cols as i32 {
                        self.map_matrix[dx as usize][dy as usize].explore = 30;
                        if let Some(resource) = self.find_resource_by_loc(dx as u32, dy as u32) {
                            if !self.finded_resources.contains(&resource.id) {
                                self.finded_resources.push(resource.id);
                            }
                        }
                    }
                }
            }
        }
    }

    pub fn decay_passage_counters(&mut self) {
        let center_x = self.rows / 2;
        let center_y = self.cols / 2;
    
        for row in 0..self.map_matrix.len() {
            for col in 0..self.map_matrix[row].len() {
                if !(row >= (center_x - 1) as usize && row <= (center_x + 1) as usize &&
                     col >= (center_y - 1) as usize && col <= (center_y + 1) as usize) {
                    if self.map_matrix[row][col].explore > 0 {
                        self.map_matrix[row][col].explore -= 1;
                    }
                }
            }
        }
    }

    pub fn find_resource_by_loc(&self, x: u32, y: u32) -> Option<&Resource> {
        for resource in self.resources.values() {
            if resource.loc.x == x && resource.loc.y == y {
                return Some(resource);
            }
        }
        None
    }

    pub fn delete_resource_by_id(&mut self, resource_id: u32) -> bool {
        self.resources.remove(&resource_id).is_some()
    }

    pub fn handle_event(&mut self, event: EventType) {
        if let EventType::Tick = event {
            self.age += 1;
    
            for tx in self.scout_senders.values() {
                let _ = tx.send(EventType::Tick);
            }
    
            for (id, rx) in self.scout_receivers.iter() {
                if let Ok(response) = rx.recv() {
                    if let EventType::ScoutMoved(_, new_loc) = response {
                        if let Some(scout) = self.scouts.get_mut(id) {
                            scout.loc = new_loc;
                        }
                    }
                }
            }
        
            let map_matrix = &self.map_matrix;
            let finded_resources = &self.finded_resources;
            let resources = &mut self.resources;
            let base = &mut self.base;
    
            
            for gatherer in self.gatherers.values_mut() {
                gatherer.choose(finded_resources, resources, self.seed, map_matrix, base);
            }
    
            self.clear_empty_resources();
    
            self.decay_passage_counters();
    
            self.update_explore_matrix();
        }
    }

    pub fn generate_display(&mut self) -> Vec<Vec<Cell>> {
        let mut result_map = self.map_matrix.clone();

        for x in 0..self.rows as usize {
            for y in 0..self.cols as usize {
                if self.map_matrix[x][y].explore == -1 {
                    result_map[x][y].display = ' ';
                }
            }
        }

        for (_, resource) in &self.resources {
            let x = resource.loc.x as usize;
            let y = resource.loc.y as usize;
    
            if self.map_matrix[x][y].explore != -1 {
                result_map[x][y].display = match resource.kind {
                    ResourceKind::Crystal => 'C',
                    ResourceKind::Energy => 'E',
                };
            } else {
                result_map[x][y].display = ' ';
            }
        }

        for (_, scout) in &self.scouts {
            let x = scout.loc.x as usize;
            let y = scout.loc.y as usize;
    
            let display = scout.display;
            result_map[x][y].display = display;
        }
        for (_, gatherer) in &self.gatherers {
            let x = gatherer.loc.x as usize;
            let y = gatherer.loc.y as usize;
    
            let display = gatherer.display;
            result_map[x][y].display = display;
        }

        result_map
    }

    pub fn generate_resources(&mut self, id_generator: &mut IDGenerator, number: u8) {
        for i in 0..number {
            if i % 2 == 0 {
                self.add_resource("crystal", 40, id_generator);
            } else {
                self.add_resource("energy", 40, id_generator);
            }
        }
    }

    fn clear_empty_resources(&mut self) {
        let ids_to_remove: Vec<u32> = self.resources.iter()
            .filter_map(|(id, resource)| {
                if resource.remaining_quantity == 0 {
                    Some(*id)
                } else {
                    None
                }
            })
            .collect();
        for id in ids_to_remove.iter() {
            self.resources.remove(id);
            self.finded_resources.retain(|&resource_id| resource_id != *id);
        }
    }

    pub fn generate_map_obstacles(&mut self) {
        let perlin = Perlin::new();
        let scale = ((self.rows + self.cols) as f64) / 10.0;
        let mut rng = StdRng::seed_from_u64(self.seed);
    
        let threshold = perlin.get([self.seed as f64 / 100.0, self.seed as f64 / 100.0]);
    
        for i in 0..self.rows {
            for j in 0..self.cols {
                let noise_value = perlin.get([i as f64 / scale, j as f64 / scale]);
                if noise_value > threshold + 0.2 {
                    self.map_matrix[i as usize][j as usize].display = '8'; // Ajouter un obstacle
                }
            }
        }
    
        let center_x = self.rows / 2;
        let center_y = self.cols / 2;
    
        let safe_zone_size = 6;
        let mut safe_zone_noise = vec![vec![false; self.cols as usize]; self.rows as usize];
    
        for i in 0..self.rows {
            for j in 0..self.cols {
                let dist_x = (i as f64 - center_x as f64).abs();
                let dist_y = (j as f64 - center_y as f64).abs();
                let dist = (dist_x.powi(2) + dist_y.powi(2)).sqrt();
    
                let safe_zone_threshold = safe_zone_size as f64 + perlin.get([i as f64 / scale, j as f64 / scale]) * 5.0;
    
                if dist < safe_zone_threshold {
                    safe_zone_noise[i as usize][j as usize] = true;
                }
            }
        }
    
        for i in 0..self.rows {
            for j in 0..self.cols {
                if safe_zone_noise[i as usize][j as usize] {
                    self.map_matrix[i as usize][j as usize].display = ' '; // Zone vide
                }
            }
        }
    
        for i in (center_x - 1) as i32..=(center_x + 1) as i32 {
            for j in (center_y - 1) as i32..=(center_y + 1) as i32 {
                if i >= 0 && j >= 0 && i < self.rows as i32 && j < self.cols as i32 {
                    self.map_matrix[i as usize][j as usize].display = '#';
                    self.map_matrix[i as usize][j as usize].explore = 30;
                }
            }
        }
    }
}
