use tetrust::engine::engine;
use tetrust::view;
use tetrust::playfield;

fn main() {
    let playfield = playfield::Playfield{storage: playfield::Storage::default()};
    let view = view::ConsoleView{};
    let mut game = engine::new_game(playfield, &view);

    engine::calculate_frame(&mut game);
    engine::draw_frame(&game);
}
