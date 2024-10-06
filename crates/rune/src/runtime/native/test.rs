use std::io::Write;

use crate::RuneRuntimeState;
use termcolor::{StandardStream, ColorChoice, Color, ColorSpec, WriteColor};
use wasmtime::{Result, WasmBacktrace};

#[async_trait::async_trait]
impl crate::rune::runtime::test::Host for RuneRuntimeState {
    async fn log(&mut self, msg: String) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(127, 127, 127)))).unwrap();
        writeln!(&mut stdout, "{}", msg).unwrap();
    }

    async fn warn(&mut self, msg: String) {
        let mut stdout = StandardStream::stdout(ColorChoice::Always);
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow))).unwrap();
        writeln!(&mut stdout, "{}", msg).unwrap();
    }

    async fn error(&mut self, msg: String) {
        let mut stderr = StandardStream::stderr(ColorChoice::Always);
        stderr.set_color(ColorSpec::new().set_fg(Some(Color::Red))).unwrap();
        writeln!(&mut stderr, "{}", msg).unwrap();
    }
}