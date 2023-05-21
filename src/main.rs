mod game;
mod io;

fn main() {
    let input = io::input::StdinInput;
    game::session::start(&input);
}
