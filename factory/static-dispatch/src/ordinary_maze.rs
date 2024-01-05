use crate::maze::{Room, Maze};

#[derive(Clone)]
pub struct OrdinaryRoom {
    id: u32,
}

impl OrdinaryRoom {
    fn new(id: u32) -> Self {
        Self{ id }
    }
}

impl Room for OrdinaryRoom {
    fn render(&self) {
        println!("Oridanry room {}", self.id);
    }
}

pub struct OrdinaryMaze {
    rooms: Vec<OrdinaryRoom>,
}

impl OrdinaryMaze {
    pub fn new() -> Self {
        Self {
            rooms: vec![
                OrdinaryRoom::new(33),
                OrdinaryRoom::new(69),
            ]
        }
    }
}

impl Maze for OrdinaryMaze {
    type ImplRoom = OrdinaryRoom;

    fn rooms(&self) -> Vec<Self::ImplRoom> {
        let mut rooms = self.rooms.clone();
        rooms.reverse();
        rooms
    }
}

