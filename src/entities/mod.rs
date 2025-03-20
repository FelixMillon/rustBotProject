use rand::prelude::*;

use crate::id_generator::IDGenerator;
use crate::events::*;
use crate::map::*;

pub struct Localization {
    pub x: u32,
    pub y: u32,
}

pub struct Entity {
    pub id: u32,
    pub loc: Localization,
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
pub trait BotActions {
    fn move_to(&mut self, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, seed: u64);
}

impl BotActions for Entity {
    fn move_to(&mut self, map_matrix: &Vec<Vec<Cell>>, rows: u32, cols: u32, seed: u64) {
        if let Nature::Bot(bot) = &mut self.nature {
            let mut rng = StdRng::seed_from_u64(seed.wrapping_add(self.id as u64));

            let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];

            loop {
                let mut shuffled_directions = directions;
                shuffled_directions.shuffle(&mut rng);

                for &(delta_x, delta_y) in &shuffled_directions {
                    let new_x = self.loc.x as i32 + delta_x;
                    let new_y = self.loc.y as i32 + delta_y;

                    if new_x >= 0 && new_x < rows as i32 && new_y >= 0 && new_y < cols as i32 {
                        let new_x = new_x as u32;
                        let new_y = new_y as u32;

                        if map_matrix[new_x as usize][new_y as usize].display != '8' {
                            self.loc.x = new_x;
                            self.loc.y = new_y;
                            return;
                        }
                    }
                }
            }
        }
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
                nature: Nature::Bot(Bot { mission }),
                display,
            }
        )
    }
}
