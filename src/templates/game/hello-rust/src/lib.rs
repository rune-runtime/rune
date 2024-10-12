wit_bindgen::generate!({
    world: "runtime",
    path: ".rune/wit/runtime",
    exports: {
        "rune:runtime/guest": Game
    },
});

use crate::exports::rune::runtime::guest::Guest;
use crate::rune::runtime::debug::log;

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
