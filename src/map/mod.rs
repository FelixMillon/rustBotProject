use std::collections::HashMap;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;
use std::f64;

use crate::id_generator::IDGenerator;
use crate::robots::*;
use crate::resources::*;
use crate::events::*;

pub struct Map {
    pub cols: u32,
    pub rows: u32,
    pub seed: u64,
    pub robots: HashMap<u32, Robot>,
    pub resources: HashMap<u32, Resource>,  
    pub map_matrix: Vec<Vec<Cell>>,
    pub age: u32
}

#[derive(Debug, Clone, Copy)]
pub struct Localization {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub display: char,
    pub explore: i8,
}

impl Map {
    pub fn new(rows: u32, cols: u32, seed: u64) -> Self {
        let robots = HashMap::new();
        let resources = HashMap::new();
        let mut map_matrix = Vec::new();
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
            robots,
            resources,
            map_matrix: map_matrix,
            age: 0,
        }
    }
    pub fn add_bot(
        &mut self,
        x: u32,
        y: u32,
        mission_str: &str,
        id_generator: &mut IDGenerator
    ){
        if let Some(mission) = Mission::from_str(mission_str) {
            let loc = Localization { x, y };
            if let Some(robot) = Robot::new_bot(loc, mission, id_generator) {
                self.robots.insert(robot.id, robot);
            }
        } else {
            eprintln!("Unknown mission : {}", mission_str);
        }
    }

    pub fn add_resource(
        &mut self,
        resource_kind_str: &str,
        initial_quantity: u16,
        id_generator: &mut IDGenerator
    ){
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
        for robot in self.robots.values() {
            if let Mission::Scout = robot.mission {
                let x = robot.loc.x as i32;
                let y = robot.loc.y as i32;
                for delta_x in -1..=1 {
                    for delta_y in -1..=1 {
                        let dx = x + delta_x;
                        let dy = y + delta_y;
    
                        if dx >= 0 && dx < self.rows as i32 && dy >= 0 && dy < self.cols as i32 {
                            self.map_matrix[dx as usize][dy as usize].explore = 30;
                        }
                    }
                }
            }
        }
    }

    fn decay_passage_counters(&mut self) {
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
            
            let bot_ids: Vec<u32> = self.robots.keys().cloned().collect();
    
            let seed = self.seed;
            let map_matrix = &self.map_matrix;
            let rows = self.rows;
            let cols = self.cols;
    
            for id in bot_ids {
                if let Some(robot) = self.robots.get_mut(&id) {
                    robot.explore(map_matrix, rows, cols, seed);
                }
            }
    
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

        for (_, robot) in &self.robots {
            let x = robot.loc.x as usize;
            let y = robot.loc.y as usize;
    
            let display = robot.display;
            result_map[x][y].display = display;
        }

        result_map
    }

    pub fn generate_resources(&mut self, id_generator: &mut IDGenerator, number: u8) {
        for i in 0..number {
            if i % 2 == 0 {
                self.add_resource("crystal", 100, id_generator);
            } else {
                self.add_resource("energy", 100, id_generator);
            }
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
