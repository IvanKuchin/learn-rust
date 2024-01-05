use crate::maze::Room;
use crate::maze::Maze;

#[derive(Clone)]
pub struct MagickRoom {
    title: String,
}

impl MagickRoom {
    pub fn new(title: String) -> Self {
        MagickRoom{title}
    }
}

impl Room for MagickRoom {
    fn render(&self) {
        println!("MagickRoom ({})", self.title);
    }
}

pub struct MagickMaze {
    rooms: Vec<MagickRoom>,
}

impl MagickMaze {
    pub fn new() -> Self {
        Self { rooms: vec![
            MagickRoom::new("Infinite room".into()),
            MagickRoom::new("Mirror room".into()),
            MagickRoom::new(String::from("Alternative room")),
        ] }
    }
}

impl Maze for MagickMaze {
    type ImplRoom = MagickRoom;

    fn rooms(&self) -> Vec<Self::ImplRoom> {
        self.rooms.clone()
    }
}
