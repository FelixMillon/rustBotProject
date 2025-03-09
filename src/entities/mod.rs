use crate::id_generator::IDGenerator;

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
    pub available : u16,
    pub consumed: u16,
}

pub struct Bot {
    pub mission: Mission,
}

pub enum Nature {
    Bot { mission: Mission},
    Resource { available: u16, consumed: u16},
}

pub enum Mission {
    Scout,
    Gatherer
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
        x: u32,
        y: u32,
        mission_str: &str,
        id_generator: &mut IDGenerator
    ) -> Option<Self> {
        if let Some(mission) = Mission::from_str(mission_str) {
            let id = id_generator.generate_id();
            let loc = Localization { x, y };
            let nature = Nature::Bot { mission };
            let display = if mission_str == "scout" { 'S' } else { 'R' };
            Some(Self { id, loc, nature, display })
        } else {
            None
        }
    }
}
