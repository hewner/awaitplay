use async_trait::async_trait;
use console::Term;
use async_timer::oneshot::{ Timer, Oneshot};
use std::time::Duration;


#[async_trait]
pub trait Engine {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize);
    async fn wait(&mut self, secs : f64);
        
}

pub struct SimpleEngine {
    term : Term
}

impl SimpleEngine {

    pub fn new() -> SimpleEngine {
        SimpleEngine { term : Term::stdout() }
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

    
}
