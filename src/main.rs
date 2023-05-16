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


    for s in 32..70 {
        let size = s;
        coordinator::Coordinator(size);
    }
    println!("{}", ((std::time::Instant::now() - start).as_millis()));
}