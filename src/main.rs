use futures::executor::block_on;
use futures::join;
use std::error::Error;
use awaitplay::{SimpleEngine, Engine};
use std::ops::Add;


use rand::Rng;
use rand::thread_rng;


#[derive(Copy, Clone)]
struct Pos {
    row : i16,
    col : i16
}

fn pos(r : i16, c : i16) -> Pos {
    Pos { row : r, col : c }
}


impl Add<Pos> for Pos {
    type Output = Pos;
    fn add(self, other : Pos) -> Pos {
        Pos { row: self.row + other.row, col : self.col + other.col }
    }
}

fn is_valid(cur: &Pos) -> bool {
    (cur.row >= 0) & (cur.row < 80) & (cur.col >= 0) & (cur.col < 80)
}


async fn moveX() {

    let mut rng = thread_rng();
    
    let mut engine = SimpleEngine::new();
    let row_start: i16 = rng.gen_range(0..40);
    let col_start: i16 = rng.gen_range(0..80);

    let mut start = pos(row_start,col_start);
    for i in 0..10 {
        start = moveStep(&mut engine, start).await;
        if !is_valid(&start) { break; }
    }
    
}


async fn moveStep(engine : &mut SimpleEngine, mut cur : Pos) -> Pos {



    
    let mut rng = thread_rng();

    let dir: usize = rng.gen_range(0..4);
    let deltas = vec![pos(0,1), pos(1,0), pos(0,-1), pos(-1,0)];


    
    
    for _i in 0..5 {

        engine.draw_glyph(' ', cur.row.try_into().unwrap(), cur.col.try_into().unwrap()).await;
        cur = cur + deltas[dir];
        if !is_valid(&cur) { return cur };
        engine.draw_glyph('X', cur.row.try_into().unwrap(), cur.col.try_into().unwrap()).await;
        engine.wait(0.1).await;
    }
    cur
}


fn main()  -> Result<(), Box<dyn Error>> {
    

    let all = async { join!(moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX(), moveX()); };

    block_on(all);
    Ok(())

}
