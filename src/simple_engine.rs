use async_trait::async_trait;
use tokio::sync::mpsc;
//use std::sync::mpsc::{Sender, Receiver};
use futures::Future;
use std::time::Duration;
use async_timer::oneshot::Timer;
use async_timer::Oneshot;
use tokio::task::JoinHandle;
use terminal;
use terminal::Terminal;
use std::sync::{Arc,Mutex};
use std::io::Stdout;
use super::Engine;

#[derive(Clone)]
pub struct SimpleEngine {
    mutex : Arc<Mutex<Terminal<Stdout>>>,
    spawned_channel : mpsc::UnboundedSender<JoinHandle<()>>
}

impl SimpleEngine {

    pub fn new() -> (SimpleEngine, impl futures::Future<Output = ()>) {
        //        let (tx, rx): (Sender<Box<Future>>, Receiver<Box<Future>>) = mpsc::channel();
        let (tx, mut rx) = mpsc::unbounded_channel();

        let mutex = Arc::new(Mutex::new(terminal::stdout()));
        let result = SimpleEngine { mutex : mutex, spawned_channel : tx};
        (result, async move {

            loop {
                match rx.recv().await {
                    Some(handle) => {
        //                println!("got handle\n");
                        handle.await.unwrap();
                    }
                    None => {
         //               println!("no more handles");
                        break;
                    }
                }
                
            }
        })
    }
    


}

#[async_trait]
impl Engine for SimpleEngine {
    async fn draw_glyph(&mut self, glyph : char, row: usize, col: usize) {


        let mut terminal = self.mutex.lock().unwrap();
        use std::io::Write;
        terminal.batch(terminal::Action::MoveCursorTo(col.try_into().unwrap(), row.try_into().unwrap())).unwrap();
        
        
        
        //self.term.move_cursor_to(col, row).unwrap();
        let mut b = [0; 4];
        let result = glyph.encode_utf8(&mut b);
        terminal.write(result.as_bytes()).unwrap();
        //TODO: error handling that deals with these unwraps
        terminal.flush_batch().unwrap();
        
    }


    async fn wait(&mut self, secs : f64) {
        let wait_time = Duration::from_nanos((secs * 1_000_000_000.0) as u64);
        Timer::new(wait_time).await;

    }

    fn spawn<F: Future<Output=()> + std::marker::Send + 'static>(&mut self, f:F) {
        let handle = tokio::spawn(f);
        // println!("sending handle\n");
        self.spawned_channel.send(handle).unwrap();
    }
}
