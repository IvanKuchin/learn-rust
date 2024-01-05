pub trait Room {
    fn render(&self);
}

pub trait Maze {
    type ImplRoom: Room;

    fn rooms(&self) -> Vec<Self::ImplRoom>;

    fn render(&self) {
        for room in self.rooms() {
            room.render();
        }
    }
}