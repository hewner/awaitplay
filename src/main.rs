use futures::executor::block_on;
use futures::join;
use async_timer::oneshot::{ Timer, Oneshot};
use std::error::Error;
use std::time::Duration;
use console::Term;
use std::io::Write;
use awaitplay::{SimpleEngine, Engine};

fn main()  -> Result<(), Box<dyn Error>> {



    let future = async {

        let mut engine = SimpleEngine::new();
        
        
        for i in 0..30 {
            engine.draw_glyph('?', 0, i).await;
            engine.wait(1./6.).await;
        }
        
    };


    let future2 = async {

        let mut engine = SimpleEngine::new();

        
        for i in 0..5 {
            engine.draw_glyph('?', i, 0).await;
            engine.wait(1.).await;
        }
        
    };

    

    let all = async { join!(future, future2); };

    block_on(all);
    Ok(())

}
