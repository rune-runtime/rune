use crate::RuneRuntimeState;

#[async_trait::async_trait]
impl crate::rune::runtime::debug::Host for RuneRuntimeState {
    async fn log(&mut self, msg: String) {
        web_sys::console().log(msg);
    }

    async fn warn(&mut self, msg: String) {
        web_sys::console().warn(msg);
    }

    async fn error(&mut self, msg: String) {
        web_sys::console().error(msg);
    }
}
