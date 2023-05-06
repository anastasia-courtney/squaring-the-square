use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use rand::seq::SliceRandom;
use std::thread::available_parallelism;
//import config
use crate::squares::Config;
use crate::exhaustive::*;
use crate::squares::*;
//import Integer


pub fn Coordinator(size : Integer) -> () {
    let (to_coord, rcv_coord) = channel();
    let NTHREADS = 10; //available_parallelism().unwrap().get();
    //println!("Number of threads: {}", NTHREADS);
    //create an hashmap that contains tuples of threads and senders:
    let mut threads: HashMap<usize, (thread::JoinHandle<()>, std::sync::mpsc::Sender<()>)> = HashMap::new();

    //queue:
    let mut queue: Vec<(Config, usize)> = Vec::new();

    //threadcount:
    let mut i = 0;

    //random number generator:
    let mut rng = rand::thread_rng();

    //create the first thread:
    let (to_thread, rcv_thread) = channel::<()>();
    let to_co = to_coord.clone();
    let new_thread = thread::spawn(move || {
        let start = std::time::Instant::now();
        solve_cc( &to_co, &rcv_thread, size);
        let end = std::time::Instant::now();
        //println!("Time elapsed: {}ms", (end - start).as_millis());
        //println!("Thread {} disconnecting...", i);
        to_co.send(Message::ThreadDeath(i)).unwrap();
    });
    threads.insert(i, (new_thread, to_thread));
    //println!("Solve called, number of threads: {}", threads.len());
    
    //sleep for a bit:
    //thread::sleep(std::time::Duration::from_millis(300));

    //While there is more than one thread:
    while threads.len() > 0 || queue.len() > 0 {
        //println!("Number of threads: {}, workunits: {}", threads.len(), queue.len());
        //if there could be more threads:
        if threads.len() < NTHREADS {
            //println!("Number of threads: {}, workunits: {}", threads.len(), queue.len());
            for _ in 0..(NTHREADS - threads.len()) {
            //println!("Queue length: {}", queue.len());
                match queue.pop() {
                    //if queue is not empty:
                    Some(unit) => {
                        //create a new thread:
                        i += 1;
                        let u = unit.clone();
                        let (to_thread, rcv_thread) = channel::<()>();
                        let to_co = to_coord.clone();
                        let new_thread = thread::spawn(move || {
                            //time the lifetime of the thread:
                            let start = std::time::Instant::now();
                            let (mut config, plate_id) = unit;
                            initial_decompose_cc(&to_co, &rcv_thread, &mut config, plate_id);
                            let end = std::time::Instant::now();
                            //println!("Time elapsed: {}ms", (end - start).as_millis());
                            to_co.send(Message::ThreadDeath(i)).unwrap();
                        });
                        threads.insert(i, (new_thread, to_thread));
                        //println!("New thread: {}, threads: {:?}", i, threads.keys());
                    },
                    //if queue is empty:
                    //   randomly select thread, and send it a message to produce a work unit
                    None => {
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
                    },
                }
            }
        }
        //Process incoming messages:
        //It is possible a thread kills if a message is sent after it's last split opportunity.
        //That thread will still send a message, to die.
        //therefore it is safe to wait for a message.

        let m = rcv_coord.recv().unwrap();
        match m {
            Message::ThreadDeath(index) => {
                //println!("Thread {} disconnected", index);
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
                Message::ThreadDeath(index) => {
                    //println!("Thread {} disconnected", index);
                    threads.remove(&index);
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
    }
    
}


pub enum Message {
    ThreadDeath(usize),
    WorkUnit((Config, usize)),
}