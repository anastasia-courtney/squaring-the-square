use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use rand::seq::SliceRandom;
use std::thread::available_parallelism;
//import config
use crate::squares::Config;
use crate::exhaustive::*;
use crate::squares::*;
use std::fs::File;
use std::io::Write;

//import Integer


pub fn Coordinator(size : Integer) -> (u128) {

    let start = std::time::Instant::now();
    let (to_coord, rcv_coord) = channel();
    let NTHREADS = 10; //available_parallelism().unwrap().get();
    //println!("Number of threads: {}", NTHREADS);
    //create an hashmap that contains tuples of threads and senders:
    let mut threads: HashMap<usize, thread::JoinHandle<()>> = HashMap::new();

    //queue:
    let mut queue: Vec<(Config, usize)> = Vec::new();

    //threadcount:
    let mut i = 0;
    let mut total_squares = 0;

    //random number generator:
    let mut rng = rand::thread_rng();

    //create the first thread:
    let to_co = to_coord.clone();
    let new_thread = thread::spawn(move || {
        //let start = std::time::Instant::now();
        solve_cc( &to_co,  size);
        //let end = std::time::Instant::now();
        //println!("Time elapsed: {}ms", (end - start).as_millis());
        //println!("Thread {} disconnecting...", i);
        to_co.send(Message::ThreadDeath(i, 0)).unwrap();
    });
    threads.insert(i, new_thread);
    //println!("Solve called, number of threads: {}", threads.len());
    
    //sleep for a bit:
    thread::sleep(std::time::Duration::from_millis(10));

    let m = rcv_coord.recv().unwrap();
        match m {
            Message::ThreadDeath(index, squares_placed) => {
                //println!("Thread {} disconnected", index);
                threads.remove(&index);
                total_squares += squares_placed;
                //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
            },
            Message::WorkUnit(unit) => {
                //add to queue:
                queue.push(unit);
                //println!("Work unit recieved, queue length: {}", queue.len());
            },
        }
        
        for received in rcv_coord.try_iter() {
            match received {
                Message::ThreadDeath(index, squares_placed) => {
                    //println!("Thread {} disconnected", index);
                    threads.remove(&index);
                    total_squares += squares_placed;
                    //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
                },
                Message::WorkUnit(unit) => {
                    //add to queue:
                    queue.push(unit);
                    //println!("Work unit recieved, queue length: {}", queue.len());
                },
                _ => {
                    //println!("Message recieved: unknown");
                }
            }
        }
    println!("Work units: {}", queue.len());

    //While there is more than one thread:
    while queue.len() > 0 {
        //println!("Number of threads: {}, workunits: {}", threads.len(), queue.len());
        //if there could be more threads:
        if threads.len() < NTHREADS {
            //println!("Number of threads: {}, workunits: {}", threads.len(), queue.len());
            for _ in 0..(NTHREADS - threads.len()) {
            //println!("Queue length: {}", queue.len());
                match queue.pop() { //TODO: change this to finite length
                    //if queue is not empty:
                    Some(unit) => {
                        //create a new thread:
                        i += 1;
                        let u = unit.clone();
                        let (to_thread, rcv_thread) = channel::<()>();
                        let to_co: std::sync::mpsc::Sender<Message> = to_coord.clone();
                        let new_thread = thread::spawn(move || {
                            //time the lifetime of the thread:
                            //let start = std::time::Instant::now();
                            let (mut config, plate_id) = u;
                            //initial_decompose_cc(&to_co, &rcv_thread, &mut config, plate_id);
                            decompose(&mut config, plate_id);
                            //let end = std::time::Instant::now();
                            //println!("Time elapsed: {}ms", (end - start).as_millis());
                            to_co.send(Message::ThreadDeath(i, config.net_squares)).unwrap();
                        });
                        threads.insert(i, new_thread);
                        //println!("New thread: {}, threads: {:?}", i, threads.keys());
                    },
                    //if queue is empty:
                    //   randomly select thread, and send it a message to produce a work unit
                    None => {} /*
                        //randomly select a key from threads:
                        let k = threads.keys().cloned().collect::<Vec<usize>>();
                        if k.len() > 0 {
                            let key = k.iter().min().unwrap();//k.choose(&mut rand::thread_rng()).unwrap();
                            //send a message to the thread:
                            match threads.get(&key){
                                Some((_, to_thread)) => {
                                    //println!("Sending request to thread {}", key);
                                    match to_thread.send(()) {
                                        Ok(_) => {/*//println!("Request sent to: {}", key)*/},
                                                
                                        Err(_) => {/*//println!("Request failed, thread {} disconnected", key);
                                                            threads.remove(&key);*/}
                                    }
                                },
                                None => {
                                    //Thread killed in between selection and sending
                                    //println!("Thread {} disconnected", key);
                                    threads.remove(&key);
                                }
                            }
                        }
                    },*/
                }
            }
        }
        //Process incoming messages:
        //It is possible a thread kills if a message is sent after it's last split opportunity.
        //That thread will still send a message, to die.
        //therefore it is safe to wait for a message.

        //little sleep:
        //thread::sleep(std::time::Duration::from_millis(10));



        if queue.len() != 0{
            let m = rcv_coord.recv().unwrap();
            match m {
                Message::ThreadDeath(index, squares_placed) => {
                    //println!("Thread {} disconnected", index);
                    total_squares += squares_placed;
                    threads.remove(&index);
                    //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
                },
                Message::WorkUnit(unit) => {
                    //add to queue:
                    queue.push(unit);
                    //println!("Work unit recieved, queue length: {}", queue.len());
                },
            }
            for received in rcv_coord.try_iter() {
                match received {
                    Message::ThreadDeath(index, squares_placed) => {
                        //println!("Thread {} disconnected", index);
                        total_squares += squares_placed;
                        threads.remove(&index);
                        //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
                    },
                    /*Message::WorkUnit(unit) => {
                        //add to queue:
                        queue.push(unit);
                        println!("Work unit recieved, queue length: {}", queue.len());
                    }*/
                    _ => {
                        println!("Message recieved: unknown");
                    }
                }
            }
        }
    }
    let mut f = File::options().append(true).open("timings-170523.txt").unwrap();
    write!(&mut f, "{} {}", size, (std::time::Instant::now() - start).as_millis()).unwrap();
    
    if threads.len() != NTHREADS {
        for _ in 0..(NTHREADS - threads.len()) {
            write!(&mut f, " {}", (std::time::Instant::now() - start).as_millis()).unwrap();
        }
    }
    while threads.len() > 0 {
        match rcv_coord.recv().unwrap() {
            Message::ThreadDeath(index, squares_placed) => {
                //println!("Thread {} disconnected", index);
                threads.remove(&index);
                total_squares += squares_placed;
            },
            _ => {
                println!("Message recieved: unknown");
            }
        }
        write!(&mut f, " {}", (std::time::Instant::now() - start).as_millis()).unwrap();

    }
    writeln!(&mut f, "").unwrap();
    //println!("sp {} {}", size, total_squares);
    total_squares
}

#[derive(Debug)]
pub enum Message {
    ThreadDeath(usize, u128),
    WorkUnit((Config, usize)),
}

pub fn coordinator_continuous(min_size : Integer, max_size : Integer) -> u128{
    let start = std::time::Instant::now();
    let mut size = min_size;
    let mut total_squares = 0;
    let (to_coord, rcv_coord) = channel();
    let NTHREADS = 10; //available_parallelism().unwrap().get();
    //println!("Number of threads: {}", NTHREADS);
    //create an hashmap that contains tuples of threads and senders:
    let mut threads: HashMap<usize, Integer> = HashMap::new();

    //queue:
    let mut queue: Vec<(Config, usize)> = Vec::new();
    //threadcount:
    let mut i = 0;
    let mut f = File::options().append(true).open("timings-170523.txt").unwrap();

    while size <= max_size{
        println!("{} start {}", size, (std::time::Instant::now() - start).as_millis());
        writeln!(&mut f, "{} start {}", size, (std::time::Instant::now() - start).as_millis()).unwrap();


    
        //random number generator:
        let mut rng = rand::thread_rng();
    
        //create the first thread:
        let to_co = to_coord.clone();
        let new_thread = thread::spawn(move || {
            //let start = std::time::Instant::now();
            solve_cc( &to_co, size);
            //let end = std::time::Instant::now();
            //println!("Time elapsed: {}ms", (end - start).as_millis());
            //println!("Thread {} disconnecting...", i);
            to_co.send(Message::ThreadDeath(i, 0)).unwrap();
        });
        threads.insert(i, size);
        //println!("Solve called, number of threads: {}", threads.len());
        

        //sleep for a bit:
        thread::sleep(std::time::Duration::from_millis(10));
    
        let m = rcv_coord.recv().unwrap();
        //println!("Message recieved: {:?}", m);
        match m {
            Message::ThreadDeath(index, squares_placed) => {
                //println!("Thread {} disconnected", index);
                let s = threads.get(&index).unwrap().clone();
                threads.remove(&index);
                if s < size {
                    if !threads.values().any(|&val| val == s){
                        writeln!(&mut f, "{} end {}", s, (std::time::Instant::now() - start).as_millis()).unwrap();
                        println!("{} end {}", s, (std::time::Instant::now() - start).as_millis());
                    }
                    else{
                        //println!("TODO threads still working on size {}", s);
                    }
                }
                total_squares += squares_placed;
                //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
            },
            Message::WorkUnit(unit) => {
                //add to queue:
                queue.push(unit);
                //println!("Work unit recieved, queue length: {}", queue.len());
            },
        }
            
        for received in rcv_coord.try_iter() {
            //println!("Message recieved: {:?}", received);
            match received {
                Message::ThreadDeath(index, squares_placed) => {
                    //println!("Thread {} disconnected", index);
                    //println!("Threads {:?}", threads);
                    let s = threads.get(&index).unwrap().clone();
                    threads.remove(&index);
                    if s < size {
                        if !threads.values().any(|&val| val == s){
                            writeln!(&mut f, "{} end {}", s, (std::time::Instant::now() - start).as_millis()).unwrap();
                            println!("{} end {}", s, (std::time::Instant::now() - start).as_millis());
                        }
                        else{
                            //println!("TODO threads still working on size {}", s);
                        }
                    }
                    total_squares += squares_placed;
                    //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
                },
                Message::WorkUnit(unit) => {
                    //add to queue:
                    queue.push(unit);
                    //println!("Work unit recieved, queue length: {}", queue.len());
                },
                _ => {
                    //println!("Message recieved: unknown");
                }
            }
        }
        println!("Work units: {}", queue.len());
    
        //While there is more than one thread:
        while queue.len() > 0 {
            //println!("Number of threads: {}, workunits: {}", threads.len(), queue.len());
            //if there could be more threads:
            if threads.len() < NTHREADS {
                //println!("Number of threads: {}, workunits: {}", threads.len(), queue.len());
                for _ in 0..(NTHREADS - threads.len()) {
                //println!("Queue length: {}", queue.len());
                    match queue.pop() { //TODO: change this to finite length
                        //if queue is not empty:
                        Some(unit) => {
                            //create a new thread:
                            let u = unit.clone();
                            let to_co: std::sync::mpsc::Sender<Message> = to_coord.clone();
                            threads.insert(i, u.0.size);
                            let new_thread = thread::spawn(move || {
                                //time the lifetime of the thread:
                                //let start = std::time::Instant::now();
                                let (mut config, plate_id) = u;
                                //initial_decompose_cc(&to_co, &rcv_thread, &mut config, plate_id);
                                decompose(&mut config, plate_id);
                                //let end = std::time::Instant::now();
                                //println!("Time elapsed: {}ms", (end - start).as_millis());
                                //println!("Thread {} disconnecting...", i);
                                to_co.send(Message::ThreadDeath(i, config.net_squares)).unwrap();
                            });
                            i += 1;
                            //println!("New thread: {}, threads: {:?}", i, threads.keys());
                        },
                        //if queue is empty:
                        //   randomly select thread, and send it a message to produce a work unit
                        None => {} /*
                            //randomly select a key from threads:
                            let k = threads.keys().cloned().collect::<Vec<usize>>();
                            if k.len() > 0 {
                                let key = k.iter().min().unwrap();//k.choose(&mut rand::thread_rng()).unwrap();
                                //send a message to the thread:
                                match threads.get(&key){
                                    Some((_, to_thread)) => {
                                        //println!("Sending request to thread {}", key);
                                        match to_thread.send(()) {
                                            Ok(_) => {/*//println!("Request sent to: {}", key)*/},
                                                    
                                            Err(_) => {/*//println!("Request failed, thread {} disconnected", key);
                                                                threads.remove(&key);*/}
                                        }
                                    },
                                    None => {
                                        //Thread killed in between selection and sending
                                        //println!("Thread {} disconnected", key);
                                        threads.remove(&key);
                                    }
                                }
                            }
                        },*/
                    }
                }
            }
            //Process incoming messages:
            //It is possible a thread kills if a message is sent after it's last split opportunity.
            //That thread will still send a message, to die.
            //therefore it is safe to wait for a message.
    
            //little sleep:
            //thread::sleep(std::time::Duration::from_millis(10));
    
    
    
            if queue.len() != 0{
                let m = rcv_coord.recv().unwrap();
                //println!("Message recieved: {:?}", m);
                match m {
                    Message::ThreadDeath(index, squares_placed) => {
                        //println!("Thread {} disconnected", index);
                        //println!("Threads {:?}", threads);
                        total_squares += squares_placed;
                        let s = threads.get(&index).unwrap().clone();
                        threads.remove(&index);
                        if s < size {
                            if !threads.values().any(|&val| val == s){
                                writeln!(&mut f, "{} end {}", s, (std::time::Instant::now() - start).as_millis()).unwrap();
                                println!("{} end {}", s, (std::time::Instant::now() - start).as_millis());
                            }
                            else{
                                //println!("TODO threads still working on size {}", s);
                            }
                        }
                        //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
                    },
                    Message::WorkUnit(unit) => {
                        //add to queue:
                        queue.push(unit);
                        //println!("Work unit recieved, queue length: {}", queue.len());
                    },
                }
                for received in rcv_coord.try_iter() {
                    //println!("Message recieved: {:?}", received);
                    match received {
                        Message::ThreadDeath(index, squares_placed) => {
                            //println!("Thread {} disconnected", index);
                            //println!("Threads {:?}", threads);
                            total_squares += squares_placed;
                            let s = threads.get(&index).unwrap().clone();
                            threads.remove(&index);
                            if s < size {
                                if !threads.values().any(|&val| val == s){
                                    writeln!(&mut f, "{} end {}", s, (std::time::Instant::now() - start).as_millis()).unwrap();
                                    println!("{} end {}", s, (std::time::Instant::now() - start).as_millis());
                                }
                                else{
                                    //println!("TODO threads still working on size {}", s);
                                }
                            }
                            //println!("Number of threads: {}, work units: {}", threads.len(), queue.len());
                        },
                        /*Message::WorkUnit(unit) => {
                            //add to queue:
                            queue.push(unit);
                            println!("Work unit recieved, queue length: {}", queue.len());
                        }*/
                        _ => {
                            println!("Message recieved: unknown");
                        }
                    }
                }
            }
        } 
        writeln!(&mut f, "{} queueempty {}", size, (std::time::Instant::now() - start).as_millis()).unwrap();
        println!("{} queueempty {}", size, (std::time::Instant::now() - start).as_millis());
        
        if size == max_size {
            while threads.len() > 0 {
                match rcv_coord.recv().unwrap() {
                    Message::ThreadDeath(index, squares_placed) => {
                        //println!("Thread {} disconnected", index);
                        //println!("Threads {:?}", threads);
                        let s = threads.get(&index).unwrap().clone();
                        threads.remove(&index);
                        if !threads.values().any(|&val| val == s){
                            writeln!(&mut f, "{} end {}", s, (std::time::Instant::now() - start).as_millis()).unwrap();
                            println!("{} end {}", s, (std::time::Instant::now() - start).as_millis());
                        }
                        else{
                            //println!("TODO threads still working on size {}", s);
                        }
                        total_squares += squares_placed;
                    },
                    _ => {
                        println!("Message recieved: unknown");
                    }
                }
        
            }
        }

        size += 1;
    }
    total_squares
}