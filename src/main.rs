mod squares;
mod exhaustive;

fn main() {
    println!("Hello, world!");
    //time the exhaustive search
    //set start time:
    let start = std::time::Instant::now();
    exhaustive::solve(i32::try_from(40).unwrap());
    //set end time:
    let end = std::time::Instant::now();
    //print time elapsed:
    println!("Time elapsed: {}ms", (end - start).as_millis());
}
