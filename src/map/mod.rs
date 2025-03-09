use std::collections::HashMap;
use noise::{NoiseFn, Perlin};
use rand::prelude::*;

pub struct Map {
    pub cols: u32,
    pub rows: u32,
    pub entities: HashMap<u32, Entity>,  
    pub map_matrix: Vec<Vec<char>>
}

pub struct Localization {
    pub x: u32,
    pub y: u32,
}

pub struct Entity {
    pub id: u32,
    pub loc: Localization,
    pub nature: Nature,
}

pub struct Resource {
    pub available : u16,
    pub consumed: u16,
    pub display: char,
}

pub struct Bot {
    pub function: u8,
    pub display: char,
}

pub enum Nature {
    Bot { function: u8, display: char },
    Resource { available: u16, consumed: u16, display: char },
}


impl Map {
    pub fn new(rows: u32, cols: u32, seed: u64) -> Self {
        let entities = HashMap::new();
        Self {
            rows,
            cols,
            entities,
            map_matrix: vec![vec![' '; cols as usize]; rows as usize],
        }
    }

    pub fn generate_display(&mut self) -> Vec<Vec<char>>{
        let mut result_map = self.map_matrix.clone();
        for (_, entity) in &self.entities {
            let x = entity.loc.x as usize;
            let y = entity.loc.y as usize;
    
            let display = match &entity.nature {
                Nature::Bot { display, .. } => display,
                Nature::Resource { display, .. } => display,
            };
            result_map[x][y] = *display;
        }
        result_map
    }
    pub fn generate_map_obstacles(&mut self, seed: u64) {
        let perlin = Perlin::new();
        let scale = ((self.rows + self.cols) as f64) / 10.0;
        let mut rng = StdRng::seed_from_u64(seed);

        let threshold = perlin.get([seed as f64 / 100.0, seed as f64 / 100.0]);

        let safe_zone_size = (self.rows.min(self.cols) as f64 * 0.3) as u32;
        let center_x = self.rows / 2;
        let center_y = self.cols / 2;

        for i in 0..self.rows {
            for j in 0..self.cols {
                if (i as i32 - center_x as i32).abs() < 3 / 2
                    && (j as i32 - center_y as i32).abs() < 3 / 2 {
                    continue;
                }

                let noise_value = perlin.get([i as f64 / scale, j as f64 / scale]);

                if noise_value > threshold + 0.2 {
                    self.map_matrix[i as usize][j as usize] = '8';
                }
            }
        }
    }
}
