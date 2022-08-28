use async_trait::async_trait;
use futures::Future;

#[async_trait]
pub trait Engine : Clone + Send {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize);
    async fn wait(&mut self, secs : f64);
    fn back_color(&mut self, r : u8, g : u8, b : u8);
    fn spawn<F:Future<Output = ()> + std::marker::Send + 'static>(&mut self, f:F);
}

pub mod simple_engine;
