use crate::map::Localization;

pub enum EventType {
    Tick,
    ScoutMoved(u32, Localization),
}