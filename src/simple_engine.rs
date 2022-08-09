use console::Term;
use async_trait::async_trait;
use std::sync::mpsc;
//use std::sync::mpsc::{Sender, Receiver};
use futures::Future;
use std::time::Duration;
use async_timer::oneshot::Timer;
use async_timer::Oneshot;
use super::Engine;

pub struct SimpleEngine {
    term : Term,
    spawned_channel : mpsc::Sender<Box<dyn Future<Output= () > + std::marker::Send>>
}

impl SimpleEngine {

    pub fn new() -> SimpleEngine {
        //        let (tx, rx): (Sender<Box<Future>>, Receiver<Box<Future>>) = mpsc::channel();
        let (tx, rx) = mpsc::channel();
        let mut result = SimpleEngine { term : Term::stdout() , spawned_channel : tx};
        result
    }
    
}

#[async_trait]
impl Engine for SimpleEngine {
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

    async fn spawn<F: Future + std::marker::Send>(&mut self, f:F) {
        f.await;
    }
}
