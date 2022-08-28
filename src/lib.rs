use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait Engine : Clone {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize);
    async fn wait(&mut self, secs : f64);
    fn spawn<F:Future<Output = ()> + std::marker::Send + 'static>(&mut self, f:F);
}

pub mod simple_engine;
