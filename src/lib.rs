use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait Engine {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize);
    async fn wait(&mut self, secs : f64);
    async fn spawn<F:Future<Output = ()> + std::marker::Send>(&mut self, f:F);
}

pub mod simple_engine;
