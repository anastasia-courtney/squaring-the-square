use crate::squares::Config;

mod squares;
mod exhaustive;
mod concurrency_dummy;
mod coordinator;

fn main() {
    println!("Hello, world!");
    //time the exhaustive search
    //set start time:
    let start = std::time::Instant::now();

    let queue: Vec<Config> = Vec::new();

    coordinator::Coordinator(90);
    //exhaustive::solve(i32::try_from(65).unwrap());
    //set end time:
    let end = std::time::Instant::now();
    //print time elapsed:
    println!("Time elapsed: {}ms", (end - start).as_millis());
}