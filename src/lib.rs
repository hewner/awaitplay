use async_trait::async_trait;
use console::Term;
use async_timer::oneshot::{ Timer, Oneshot};
use std::time::Duration;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::future::Future;

#[async_trait]
pub trait Engine<F:Future> {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize);
    async fn wait(&mut self, secs : f64);
    async fn spawn(&mut self, f:F);
}

pub struct SimpleEngine<F : Future> {
    term : Term,
    spawned_channel : mpsc::Sender<F>
}

impl<F:Future+ std::marker::Send> SimpleEngine<F> {

    pub fn new(_ : F) -> SimpleEngine<F> {
        let (tx, rx): (Sender<F>, Receiver<F>) = mpsc::channel();
        let mut result = SimpleEngine { term : Term::stdout() , spawned_channel : tx};
        result
    }
    
}

#[async_trait]
impl<F: Future + std::marker::Send> Engine<F> for SimpleEngine<F> {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize) {

        use std::io::Write;
        
        self.term.move_cursor_to(col, row).unwrap();
        let mut b = [0; 4];
        let result = glyph.encode_utf8(&mut b);
        self.term.write(result.as_bytes()).unwrap();

    }


    async fn wait(&mut self, secs : f64) {
        let wait_time = Duration::from_nanos((secs * 1_000_000_000.0) as u64);
        Timer::new(wait_time).await;

    }

    async fn spawn(&mut self, f:F) {
        f.await;
    }
}
