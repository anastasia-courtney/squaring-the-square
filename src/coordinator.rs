use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use rand::random;
use rand::seq::SliceRandom;
use std::thread::available_parallelism;
//import config
use crate::squares::Config;
use crate::exhaustive::*;
use crate::squares::*;
//import Integer


fn Coordinator(size : Integer) -> () {
    let (to_coord, rcv_coord) = channel();
    let NTHREADS = available_parallelism().unwrap().get();
    println!("Number of threads: {}", NTHREADS);
    //create an hashmap that contains tuples of threads and senders:
    let mut threads: HashMap<usize, (thread::JoinHandle<()>, std::sync::mpsc::Sender<usize>)> = HashMap::new();
    println!("Creating {} threads", NTHREADS);

    //queue:
    let mut queue: Vec<(Config, usize)> = Vec::new();

    //threadcount:
    let mut i = 0;

    //random number generator:
    let mut rng = rand::thread_rng();

    //create the first thread:
    let (to_thread, rcv_thread) = channel::<usize>();
    let to_co = to_coord.clone();
    let new_thread = thread::spawn(move || {
        solve_cc(size);
        to_co.send(Message::ThreadDeath(i)).unwrap();
    });
    threads.insert(i, (new_thread, to_thread));
    
    //While there is more than one thread:
    while threads.len() > 0 {
        //if there could be more threads:
        if threads.len() < NTHREADS {
            match queue.pop() {
                //if queue is not empty:
                Some(unit) => {
                    //create a new thread:
                    i += 1;
                    let (to_thread, rcv_thread) = channel::<usize>();
                    let to_co = to_coord.clone();
                    let new_thread = thread::spawn(move || {
                        Worker(unit);
                        to_co.send(Message::ThreadDeath(i)).unwrap();
                    });
                    threads.insert(i, (new_thread, to_thread));
                },
                //if queue is empty:
                //   randomly select thread, and send it a message to produce a work unit
                None => {
                    //randomly select a key from threads:
                    let k = threads.keys().cloned().collect::<Vec<usize>>();
                    if k.len() > 0 {
                        let key = k.choose(&mut rand::thread_rng()).unwrap();
                        //send a message to the thread:
                        match threads.get(&key){
                            Some((_, to_thread)) => {
                                to_thread.send(0).unwrap();
                            },
                            None => {
                                //Thread killed in between selection and sending
                                println!("Thread {} disconnected", key);
                                threads.remove(&key);
                            }
                        }
                    }
                },
            }
        }
        //Process incoming messages:
        //It is possible a thread kills if a message is sent after it's last split opportunity.
        //That thread will still send a message, to die.
        //therefore it is safe to wait for a message.
        
        for received in rcv_coord.recv() {
            match received {
                Message::ThreadDeath(index) => {
                    println!("Thread {} disconnected", index);
                    threads.remove(&index);
                },
                Message::WorkUnit(unit) => {
                    //add to queue:
                    queue.push(unit);
                },
            }
        }
    }
    
}

fn Worker(unit :(Config, usize))->(){
    let (config, plate_id) = unit;
    decompose_cc(config, plate_id);
}


enum Message {
    ThreadDeath(usize),
    WorkUnit((Config, usize)),
}