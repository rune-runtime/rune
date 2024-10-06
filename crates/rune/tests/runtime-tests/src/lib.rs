use wasm_bindgen::prelude::*;

wit_bindgen::generate!({
    world: "rune:tests/tests",
    path: [
        "../../wit/runtime",
        "../../wit/tests",
    ],
    generate_all
});

use crate::exports::rune::tests::guest::Guest;
use crate::rune::runtime::debug::log;
// use crate::exports::rune::tests::guest::

struct RuntimeTests;

impl Guest for RuntimeTests {
    fn register() -> Vec<String> {
        [
            "example_test"
        ].iter().map(|&s| s.to_owned()).collect()
    }
}

#[wasm_bindgen]
pub fn example_test() {
    log("example test");
}

export!(RuntimeTests);
