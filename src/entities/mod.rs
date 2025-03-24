use rand::prelude::*;
use std::collections::{VecDeque, HashMap};

use crate::id_generator::IDGenerator;
use crate::events::*;
use crate::map::*;

#[derive(Debug, Clone, Copy)]
pub struct Localization {
    pub x: u32,
    pub y: u32,
}

pub struct Entity {
    pub id: u32,
    pub loc: Localization,
    pub prev_loc: Option<Localization>,
    pub nature: Nature,
    pub display: char,
}

pub struct Resource {
    pub kind: ResourceKind,
    pub available : u16,
    pub consumed: u16,
}

pub struct Bot {
    pub mission: Mission,
}

pub enum Nature {
    Bot(Bot),
    Resource(Resource),
}

pub enum Mission {
    Scout,
    Gatherer
}

pub enum ResourceKind {
    Crystal,
    Energy
}

pub trait ScoutActions {
    fn explore(&mut self, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, seed: u64);
    fn try_move_to_best_cell(&mut self, circle_cells: &Vec<(i32, i32)>, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, rng: &mut StdRng) -> bool;
    fn try_move_to_any_cell(&mut self, circle_cells: &Vec<(i32, i32)>, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, rng: &mut StdRng) -> bool;
    fn attempt_movement(&mut self, cells: &mut Vec<(i32, i32)>, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, rng: &mut StdRng) -> bool;
    fn swap_with_previous_location(&mut self);
}

pub trait BotActions: ScoutActions {
    fn move_to(&mut self, x: u32, y: u32);
}

impl ScoutActions for Entity {
    fn explore(&mut self, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, seed: u64) {
        if let Nature::Bot(_) = &mut self.nature {
            let mut rng = self.initialize_rng(seed);
            let circle_cells = get_circle_cells(self.loc.x as i32, self.loc.y as i32, rows as i32, cols as i32);
            
            if self.try_move_to_best_cell(&circle_cells, map_matrix, rows, cols, &mut rng) {
                return;
            }
            
            if self.try_move_to_any_cell(&circle_cells, map_matrix, rows, cols, &mut rng) {
                return;
            }
            
            self.swap_with_previous_location();
        }
    }
    fn try_move_to_best_cell(
        &mut self,
        circle_cells: &Vec<(i32, i32)>,
        map_matrix: &Vec<Vec<Cell>>,
        rows: u32,
        cols: u32,
        rng: &mut StdRng
    ) -> bool {
        let min_explore = circle_cells.iter()
            .filter(|&&(i, j)| map_matrix[i as usize][j as usize].display != '8')
            .map(|&(i, j)| map_matrix[i as usize][j as usize].explore)
            .min()
            .unwrap_or(i8::MAX);

        let mut best_cells: Vec<(i32, i32)> = circle_cells.iter()
            .cloned()
            .filter(|&(i, j)| map_matrix[i as usize][j as usize].explore == min_explore && map_matrix[i as usize][j as usize].display != '8')
            .collect();

        self.attempt_movement(&mut best_cells, map_matrix, rows, cols, rng)
    }

    fn try_move_to_any_cell(
        &mut self,
        circle_cells: &Vec<(i32, i32)>,
        map_matrix: &Vec<Vec<Cell>>,
        rows: u32,
        cols: u32,
        rng: &mut StdRng,
    ) -> bool {
        let mut retry_cells: Vec<(i32, i32)> = circle_cells.iter()
            .cloned()
            .filter(|&(i, j)| map_matrix[i as usize][j as usize].display != '8')
            .collect();
        
        self.attempt_movement(&mut retry_cells, map_matrix, rows, cols, rng)
    }

    fn attempt_movement(
        &mut self,
        cells: &mut Vec<(i32, i32)>,
        map_matrix: &Vec<Vec<Cell>>,
        rows: u32,
        cols: u32,
        rng: &mut StdRng,
    ) -> bool {
        while !cells.is_empty() {
            if let Some(&(target_x, target_y)) = cells.choose(rng) {
                if let Some(path) = find_shortest_path(
                    (self.loc.x as i32, self.loc.y as i32),
                    (target_x, target_y),
                    map_matrix,
                    rows,
                    cols,
                ) {
                    for &(step_x, step_y) in &path {
                        if map_matrix[step_x as usize][step_y as usize].display != '8' {
                            self.move_to(step_x as u32, step_y as u32);
                            return true;
                        }
                    }
                }
                cells.retain(|&(x, y)| !(x == target_x && y == target_y));
            }
        }
        false
    }

    fn swap_with_previous_location(&mut self) {
        if let Some(prev) = self.prev_loc {
            self.prev_loc = Some(self.loc);
            self.loc = prev;
        }
    }
}

impl BotActions for Entity {
    fn move_to(&mut self, x: u32, y: u32) {
        self.prev_loc = Some(self.loc);
        self.loc.x = x;
        self.loc.y = y;
    }

}
impl Mission {
    pub fn from_str(mission_str: &str) -> Option<Mission> {
        match mission_str.to_lowercase().as_str() {
            "scout" => Some(Mission::Scout),
            "gatherer" => Some(Mission::Gatherer),
            _ => None,
        }
    }
}

impl Entity {
    pub fn new_bot(
        loc: Localization,
        mission: Mission,
        id_generator: &mut IDGenerator,
    ) -> Option<Self> {
        let id = id_generator.generate_id();
        let display = match mission {
            Mission::Scout => 'S',
            Mission::Gatherer => 'G',
        };

        Some(
            Self {
                id,
                loc,
                prev_loc: Some(loc),
                nature: Nature::Bot(Bot { mission }),
                display,
            }
        )
    }

    fn initialize_rng(&self, seed: u64) -> StdRng {
        StdRng::seed_from_u64(seed.wrapping_add(
            self.id.pow(5) as u64 * 31 + self.loc.x as u64 * 17 + self.loc.y as u64 * 13
        ))
    }


}
fn get_circle_cells(x: i32, y: i32, rows: i32, cols: i32) -> Vec<(i32, i32)> {
    let mut cells = Vec::new();

    for i in (x - 2)..=(x + 2) {
        for j in (y - 2)..=(y + 2) {
            if i >= 0 && i < rows && j >= 0 && j < cols {
                if (i - x).pow(2) + (j - y).pow(2) == 4 {
                    cells.push((i, j));
                }
            }
        }
    }

    cells
}


fn find_shortest_path(start: (i32, i32), target: (i32, i32), map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32) -> Option<Vec<(i32, i32)>> {
    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut queue = VecDeque::new();
    let mut came_from: HashMap<(i32, i32), (i32, i32)> = HashMap::new();
    
    queue.push_back(start);
    came_from.insert(start, start);

    while let Some((x, y)) = queue.pop_front() {
        if (x, y) == target {
            let mut path = Vec::new();
            let mut current = target;
            while current != start {
                path.push(current);
                current = came_from[&current];
            }
            path.reverse();
            return Some(path);
        }

        for &(dx, dy) in &directions {
            let next_x = x + dx;
            let next_y = y + dy;

            if next_x >= 0 && next_x < rows as i32 && next_y >= 0 && next_y < cols as i32 {
                if map_matrix[next_x as usize][next_y as usize].display != '8' && !came_from.contains_key(&(next_x, next_y)) {
                    queue.push_back((next_x, next_y));
                    came_from.insert((next_x, next_y), (x, y));
                }
            }
        }
    }
    None
}