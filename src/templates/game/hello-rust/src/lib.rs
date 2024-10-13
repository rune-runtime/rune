use wit_bindgen::generate;

use crate::exports::rune::runtime::guest::Guest;
use crate::rune::runtime::debug::log;

generate!({
    world: "runtime",
    path: ".rune/wit/runtime"
});
export!(Game);

struct Game;

impl Guest for Game {
    fn init() {
        log("init");
    }

    fn update(time: f64, delta_time: f64) {
        log(&format!("update: {:?}", time));
    }

    fn render(time: f64, delta_time: f64) {
        log(&format!("render: {:?}", time));
    }
}
