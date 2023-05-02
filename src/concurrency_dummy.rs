use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use rand::Rng;
use std::thread::available_parallelism;
//import config
use crate::squares::Config;

pub fn concurrency_main(){
    //create multiple threads, each with a sender to the coordinator, and also each with a receiver from the coordinator
    let (to_coord, rcv_coord) = channel();
    let NTHREADS = available_parallelism().unwrap().get();
    println!("Number of threads: {}", NTHREADS);
    //create an hashmap that contains tuples of threads and senders:
    let mut threads: HashMap<usize, (thread::JoinHandle<()>, std::sync::mpsc::Sender<usize>)> = HashMap::new();
    println!("Creating {} threads", NTHREADS);
    for i in 0..NTHREADS {
        let (to_thread, rcv_thread) = channel::<usize>();
        let to_co = to_coord.clone();
        let new_thread = thread::spawn(move || {
            //send to coordinator
            //counter to tell messages apart :)
            let mut j = 0;
            //recieve from coordinator
            for _ in 0..10 {
                // use try_iter to not block while waiting:
                for received in rcv_thread.try_iter() {
                    //wait
                    //println!("Thread {} got: {}", i, received);
                    println!("Thread {} sending to coord: {}", i, j);
                    to_co.send(Message::WorkUnit(Config::new(1))).unwrap();
                }
                thread::sleep(Duration::from_millis(20 + i as u64 * 10));
                j+= 1;
            }
            for _ in rcv_thread.try_iter() {
                //wait
                //println!("Thread {} got: {}", i, received);
                println!("Work unit produced: {}", i);
                to_co.send(Message::WorkUnit(Config::new(1))).unwrap();
            }
            //send exit message to coordinator
            to_co.send(Message::KillThread(i)).unwrap();
        });
        threads.insert(i, (new_thread, to_thread));
    }
    //recieve the messages from the threads:
    let mut i = NTHREADS;
    let mut workunits: Vec<Config> = Vec::new();
    while threads.len() > 0 {
        //count threads:
        //vec of workunits:
        println!("Number of threads: {}", threads.len());
        for received in rcv_coord.try_iter() {
            match received {
                Message::KillThread(index) => {
                    println!("Thread {} disconnected", index);
                    threads.remove(&index);
                },
                Message::WorkUnit(config) => {
                    i+=1;
                    println!("work unit recieved");
                    workunits.push(config);
                }
            }
            //wait
            //println!("Coord got: {}", received);
        }
        //randomly select a key from threads:
        if threads.len() < NTHREADS {
            if workunits.is_empty(){
                match threads.keys().cloned().last() {
                    Some(t) => {
                        //println!("Sending to thread {}: {}", t, i);
                        match threads.get(&t).unwrap().1.send(0){
                            //if err, remove from threads, else do nothing
                            Err(_) => {
                                println!("Thread {} disconnected", t);
                                threads.remove(&t);
                            },
                            _ => {
                                println!("Work unit requested from thread {}", t);
                            }
                        }
                    },
                    None => {}
                }
            }
            while workunits.is_empty(){
                println!("workunits: {}", workunits.len());
                for received in rcv_coord.try_iter() {
                    match received {
                        Message::KillThread(index) => {
                            println!("Thread {} disconnected", index);
                            threads.remove(&index);
                        },
                        Message::WorkUnit(config) => {
                            println!("work unit recieved");
                            workunits.push(config);
        
                        }
                    }
                }
                thread::sleep(Duration::from_millis(5));
            }
            println!("Creating New thread {}", i);
            i+=1;
            let (to_thread, rcv_thread) = channel::<usize>();
            let new_thread = thread::spawn(move|| {                    //simulate work
                let mut rng = rand::thread_rng();
                let mut a = 0;
                while a < 1000000000 {
                    a += 1;
                    let _ = rng.gen::<u64>();
                }
                println!("Work unit done");
            });
            threads.insert(i, (new_thread, to_thread));
        }
        thread::sleep(Duration::from_millis(20));
    }


}

enum Message {
    KillThread(usize),
    WorkUnit(Config),
}