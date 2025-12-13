use std::sync::mpsc::Sender;

pub enum CompositorMessage {
    Maximize(bool),
    Fullscreen(bool),
}
