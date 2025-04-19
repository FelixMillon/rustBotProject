use rand::prelude::*;
use std::collections::{VecDeque, HashSet, HashMap};
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{Sender, Receiver};
use crate::id_generator::IDGenerator;
use crate::events::*;
use crate::resources::*;
use crate::game::{Localization, Cell};

pub struct Gatherer {
    pub id: u32,
    pub loc: Localization,
    pub display: char,
    pub target: Option<u32>,
    pub inventory: (u16, u16),
    pub inventory_size: u16,
    pub path: Option<Vec<Localization>>,
}

impl Gatherer {
    pub fn new(
        loc: Localization,
        id_generator: &mut IDGenerator,
    ) -> Option<Self> {
        let id = id_generator.generate_id();
        let display = 'G';

        Some(
            Self {
                id,
                loc,
                target: None,
                display,
                inventory: (0, 0),
                inventory_size: 10,
                path: Some(Vec::new()),
            }
        )
    }

    fn initialize_rng(&self, seed: u64) -> StdRng {
        StdRng::seed_from_u64(seed.wrapping_add(
            self.id.pow(5) as u64 * 31 + self.loc.x as u64 * 17 + self.loc.y as u64 * 13
        ))
    }

    pub fn handle_events(
        &mut self,
        map_matrix: Arc<RwLock<Vec<Vec<Cell>>>>,
        resources: Arc<RwLock<HashMap<u32, Resource>>>,
        base_loc: Localization,
        seed: u64,
        finded_resources:  Arc<RwLock<Vec<u32>>>,
        gatherer_receiver: Receiver<EventType>,
        map_sender: Sender<EventType>,
        display_obstacle: char,
    ) {

        loop {
            if let Ok(event) = gatherer_receiver.recv() {
                match event {
                    EventType::Tick => {
                        
                        let map_matrix = map_matrix.read().unwrap();
                        let resources = resources.read().unwrap();
                        let finded_resources = finded_resources.read().unwrap();

                        let map_matrix_copy = map_matrix.clone();
                        let mut resources_copy = resources.clone();
                        let finded_resources_copy = finded_resources.clone();
                        let event = self.choose(&finded_resources_copy, &mut resources_copy, seed, &map_matrix_copy, base_loc, display_obstacle);
                        let _ = map_sender.send(event);
                    }
                    EventType::Collect(recolted) => {
                        self.inventory.0 += recolted.0;
                        self.inventory.1 += recolted.1;
                    }
                    _ => {
                    }
                }
            }
        }
    }

    pub fn choose(
        &mut self,
        finded_resources: &Vec<u32>,
        resources: &mut HashMap<u32, Resource>,
        seed: u64,
        map_matrix: &Vec<Vec<Cell>>,
        base_loc: Localization,
        display_obstacle: char,
    ) -> EventType {
    
        if self.path.as_ref().map_or(true, |p| p.is_empty()) {
            // Si la capacité de l'inventaire est pleine, se rendre à la base.
            if self.inventory.0 + self.inventory.1 >= self.inventory_size {
                self.seek(map_matrix, base_loc, display_obstacle);
                if base_loc.same_loc(&self.loc) {
                    let deposit = (self.inventory.0, self.inventory.1);
                    self.inventory = (0, 0);
                    return EventType::Deposit(deposit);
                }
            } else {
                if self.target.is_none() {
                    self.find(finded_resources, resources, seed);
                    if let Some(target_id) = self.target {
                        if let Some(resource) = resources.get(&target_id) {
                            self.seek(map_matrix, resource.loc, display_obstacle);
                            return EventType::Nothing;
                        }
                    }
                    return EventType::Nothing;
                } else {
                    if let Some(target_id) = self.target {
                        if let Some(resource) = resources.get_mut(&target_id) {
                            if self.loc.same_loc(&resource.loc) {
                                if resource.remaining_quantity == 0 {
                                    self.target = None;
                                    self.find(finded_resources, resources, seed);
                                }
                                return EventType::Extract(target_id,(10, 1.0));
                            } else {
                                self.seek(map_matrix, resource.loc, display_obstacle);
                                return EventType::Nothing;
                            }
                        } else {
                            self.target = None;
                            return EventType::Nothing;
                        }
                    }
                }
            }
        } else {
            self.step();
            return EventType::Moved(self.loc);
        }
        return EventType::Nothing;
    }

    fn find(
        &mut self,
        finded_resources: &Vec<u32>,
        resources: &HashMap<u32, Resource>,
        seed: u64
    ) {
        let mut rng = self.initialize_rng(seed);

        if let Some(&target_id) = finded_resources.choose(&mut rng) {
            if let Some(resource) = resources.get(&target_id) {
                if resource.remaining_quantity > 0 {
                    self.target = Some(target_id);
                }
            }
        }
    }

    fn seek(&mut self, map_matrix: &Vec<Vec<Cell>>, target: Localization, display_obstacle: char) {
        let start = self.loc;

        let rows = map_matrix.len();
        let cols = map_matrix[0].len();

        let directions = vec![
            (-1, 0),
            (1, 0),
            (0, -1),
            (0, 1),
        ];

        let mut queue: VecDeque<(u32, u32)> = VecDeque::new();
        queue.push_back((start.x, start.y));

        let mut visited: HashSet<(u32, u32)> = HashSet::new();
        visited.insert((start.x, start.y));

        let mut parent_map: HashMap<(u32, u32), (u32, u32)> = HashMap::new();

        while let Some((x, y)) = queue.pop_front() {
            if (x, y) == (target.x, target.y) {
                let mut path = Vec::new();
                let mut current = (x, y);

                while let Some(&parent) = parent_map.get(&current) {
                    if parent != (start.x, start.y) {
                        let loc = Localization { x: parent.0, y: parent.1 };
                        path.push(loc);
                    }
                    current = parent;
                }

                path.reverse();
                path.push(Localization { x: target.x, y: target.y });
                self.path = Some(path);

                return;
            }

            for &(dx, dy) in &directions {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;

                if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                    let new_x = new_x as u32;
                    let new_y = new_y as u32;

                    if !visited.contains(&(new_x, new_y)) && map_matrix[new_x as usize][new_y as usize].display != display_obstacle {
                        visited.insert((new_x, new_y));
                        parent_map.insert((new_x, new_y), (x, y));
                        queue.push_back((new_x, new_y));
                    }
                }
            }
        }
        self.path = Some(Vec::new());
    }

    fn step(&mut self) {
        if let Some(ref mut path) = self.path {
            if !path.is_empty() {
                let next_loc = path.remove(0);
                self.loc = next_loc;
            }
        }
    }
}
