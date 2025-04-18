use crate::game::Localization;

pub enum EventType {
    Tick,
    Moved(Localization),
    Deposit((u16, u16)),
    Collect((u16, u16)),
    Extract(u32,(u16, f32)),
    Nothing,
}