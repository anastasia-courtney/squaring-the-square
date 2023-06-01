use crate::squares::Config;
use std::fs::File;
use std::collections::HashSet;
use std::io::{self, BufRead};
use std::path::Path;


mod squares;
mod exhaustive;
mod concurrency_dummy;
mod coordinator;

fn main() {
    //time the exhaustive search
    //set start time:
    //create a file for the output:
    const order : usize = 18;
    const width : i32 = 112;

    let s : Vec<i32>= vec![1,2,6,8,9,11,13,15,16,22,23,24,28,29,30,34,50,59];

    let mut sset : HashSet<squares::Integer> = HashSet::new();
    //insert 2, 8, 14, 16, 18, 20, 28, 30, 36);
    for i in 0..s.len() {
        sset.insert(s[i]);
    }
    //exhaustive::solve_restricted(width, sset);

    mass_solve();


    /*
    for s in 32..120 {
        let size = s;
        coordinator::Coordinator(size);
    }
    */
}

fn mass_solve() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut file = File::create("Gambini_Code_149+.txt").unwrap();


    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() != 2 {
            println!("Invalid line format");
            continue;
        }

        let width: i32 = match parts[0].parse() {
            Ok(val) => val,
            Err(_) => {
                println!("Failed to parse i32");
                continue;
            }
        };

        let numbers_list_str = &parts[1][1..parts[1].len()-1];  // Remove braces
        let set: HashSet<i32> = match numbers_list_str
            .split(',')
            .map(|s| s.parse::<i32>())
            .collect::<Result<HashSet<_>, _>>() {
            Ok(val) => val,
            Err(_) => {
                println!("Failed to parse comma separated i32s");
                continue;
            }
        };
        exhaustive::solve_restricted(width, set);
    }

    Ok(())
}