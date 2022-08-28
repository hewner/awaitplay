use awaitplay::Engine;
use awaitplay::simple_engine::SimpleEngine;
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


async fn move_x<E:Engine + 'static>(mut engine: E) {


    let row_start : i16;
    let col_start : i16;

    {
    
    let mut rng = thread_rng();
  
    row_start = rng.gen_range(0..40);
    col_start = rng.gen_range(0..80);

    }

    
    let mut start = pos(row_start,col_start);
    for _ in 0..10 {
        start = move_step(&mut engine, start).await;
        if !is_valid(&start) { break; }
    }
    
}


async fn fading<E:Engine>(mut engine : E, cur : Pos) {

    engine.back_color(0,100,0);
    engine.draw_glyph('X', cur.row.try_into().unwrap(), cur.col.try_into().unwrap()).await;
    engine.wait(0.1).await;
    
    for i in 1..20 {
        engine.back_color(0,100 - i * 5,100 - i * 5);
        engine.draw_glyph(' ', cur.row.try_into().unwrap(), cur.col.try_into().unwrap()).await;
        engine.wait(0.1).await;
    }
    
}

async fn move_step<E:Engine + 'static>(engine : &mut E, mut cur : Pos) -> Pos {



    let dir : usize;
    {
    
        let mut rng = thread_rng();
        dir = rng.gen_range(0..4);
    }
    

    let deltas = vec![pos(0,1), pos(1,0), pos(0,-1), pos(-1,0)];



    
    
    for _i in 0..5 {

        
        cur = cur + deltas[dir];
        if !is_valid(&cur) { return cur };
        engine.spawn(fading(engine.clone(), cur.clone()));
        engine.wait(0.1).await;
    }
    cur
}

#[tokio::main]
async fn main() {

    let (mut engine, waiter) = SimpleEngine::new();

    for _ in 0..3 {
        engine.spawn ( move_x(engine.clone() ) );
    }
    
    drop(engine);
//    println!("awaiting waiter");
    waiter.await;

}
