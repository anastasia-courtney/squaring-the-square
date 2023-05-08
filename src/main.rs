use crate::squares::Config;

mod squares;
mod exhaustive;
mod concurrency_dummy;
mod coordinator;

fn main() {
    //time the exhaustive search
    //set start time:
    let mut start: std::time::Instant = std::time::Instant::now();
    let mut end = std::time::Instant::now();

    for s in 60..120 {
        let size = s;
        start = std::time::Instant::now();
        coordinator::Coordinator(size);
        end = std::time::Instant::now();
        println!("{}: {}", size, (end - start).as_millis());

    }
}