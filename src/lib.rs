use serde::{Deserialize, Serialize};

pub trait Entity {
    fn new(address: &'static str, port: u16) -> Self;
    fn send(&self) -> String;
    fn receive(&self) -> String;
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Headers {
    pub buffer_size: usize,
}
