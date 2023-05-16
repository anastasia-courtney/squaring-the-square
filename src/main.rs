use crate::squares::Config;
use std::fs::File;

mod squares;
mod exhaustive;
mod concurrency_dummy;
mod coordinator;

fn main() {
    //time the exhaustive search
    //set start time:
    //create a file for the output:
    let mut file = File::create("output.txt").unwrap();
    let mut file = File::create("timings.txt").unwrap();
    let start = std::time::Instant::now();
    let mut squares_placed = 0;


    for s in 30..80 {
        let start_s = std::time::Instant::now();
        let size = s;
        let squares_placed_s = coordinator::Coordinator(size);
        squares_placed += squares_placed_s;
        //println!("Total squares placed: {}", squares_placed);
        let end_s = std::time::Instant::now();
        //println!("time {} {}", size, (end_s - start_s).as_millis());
        //println!("Squares/millis: {}", squares_placed_s / (end_s - start_s).as_millis());
    }
    let end = std::time::Instant::now();
    println!("{}", ((end- start).as_millis()));
    println!("Squares (total): {}", squares_placed);
}