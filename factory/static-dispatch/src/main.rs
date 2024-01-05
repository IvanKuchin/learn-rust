mod maze;
mod magick_maze;
mod ordinary_maze;

// use magick_maze::MagickRoom;
use magick_maze::MagickMaze;
use ordinary_maze::OrdinaryMaze;

fn render_a_maze(maze: &impl maze::Maze) {
    maze.render();
}

fn main() {
    println!("Program start");
    // let room = MagickRoom::new(String::from_str("Infinite room").unwrap());
    // room.render();
    let maze1 = MagickMaze::new();
    render_a_maze(&maze1);
    
    let maze2 = OrdinaryMaze::new();
    render_a_maze(&maze2);
    
    println!("Program finish");
}
