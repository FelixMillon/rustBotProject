use crate::id_generator::IDGenerator;
use crate::events::*;

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
    fn move_to(&self);
}

impl BotActions for Bot {
    fn move_to(&self) {
        match self.mission {
            Mission::Scout => println!("J'explore."),
            Mission::Gatherer => println!("Je ramasse"),
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

impl Bot {
    pub fn on_event(&mut self, event: &EventType) {
        match event {
            EventType::Tick => self.move_to(),
        }
    }
}