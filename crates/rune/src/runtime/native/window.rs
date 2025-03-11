use super::state::RuneRuntimeState;

impl crate::rune::runtime::window::Host for RuneRuntimeState {
    async fn dimensions(&mut self) -> (u32, u32) {
        (self.window_size.width, self.window_size.height)
    }
}
