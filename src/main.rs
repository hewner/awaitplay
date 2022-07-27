use futures::executor::block_on;
use futures::join;
use async_timer::oneshot::{ Timer, Oneshot};
use std::error::Error;
use std::time::Duration;
use console::Term;
use std::io::Write;

fn main()  -> Result<(), Box<dyn Error>> {

    let mut term = Term::stdout();


    let future = async {

        let wait_time = Duration::from_secs(1);
        for i in 0..30 {
            term.move_cursor_to(i,0).unwrap();
            term.write(b"?").unwrap();
            let work = Timer::new(wait_time);
            work.await;
        }
        
    };

    let mut term2 = Term::stdout();

    
    let future2 = async {

        let wait_time = Duration::from_secs(1);
        for i in 0..30 {
            term2.move_cursor_to(0,i).unwrap();
            term2.write(b"?").unwrap();
            let work = Timer::new(wait_time);
            work.await;
        }
        
    };

    let all = async { join!(future, future2); };

    block_on(all);
    Ok(())

}
