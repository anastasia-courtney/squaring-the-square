use crate::squares::Config;
use std::fs::File;
use std::collections::HashSet;


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
    let s : [squares::Integer; 11]= [3, 5, 7, 12, 19, 22, 25, 28, 45, 48, 50];

    let mut sset : HashSet<squares::Integer> = HashSet::new();
    //insert 2, 8, 14, 16, 18, 20, 28, 30, 36);
    for i in 0..s.len() {
        sset.insert(s[i]);
    }
    exhaustive::solve_restricted(98, sset);


    /*
    for s in 32..120 {
        let size = s;
        coordinator::Coordinator(size);
    }
    */
}