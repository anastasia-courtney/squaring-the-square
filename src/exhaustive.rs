use crate::squares::*;
use std::sync::mpsc::{channel, Receiver, Sender};
use crate::coordinator::Message;
use std::io::Write;

//import config
use crate::squares::Config;
use std::fs::{self, File};


pub fn solve (size: Integer) -> (){
    //println!{"Solving for size: {}", size};
    let mut config = Config::new(size); //Creates the necessary starting plate.
    //println!("Config: {:?}", config);
    decompose(&mut config, 1);
}

pub fn solve_cc(send : &Sender<Message>, size: Integer) ->(){
    //println!{"Solving for size: {}", size};
    let mut config = Config::new(size); //Creates the necessary starting plate.
    //println!("Config: {:?}", config);
    double_nest_init(send, &mut config);
    //initial_SOLVE_decompose_cc(send, rcv, &mut config);
}

fn next_plate(config: &mut Config) -> () { //find smallest delimited plate, and decompose it
    let mut l_min : Integer = config.size + Integer::from(1); //equiv to infinity
    let mut p_min_i : usize = 0;
    //find the minimum delimited plate
    //if, in the meantime, we identify that there is only three plates, we have found a square or rectangle and should return them
    for i in 1..config.num_plates()-1 {
        let plate = &config.plates[i];
        let plate_prev = &config.plates[i-1];
        let plate_next = &config.plates[i+1];
        if plate_prev.height > plate.height && plate_next.height > plate.height && plate.width < l_min{
            l_min = plate.width;
            p_min_i = i;
        }
    }
    //print width and index of minimum delimited plate
    ////eprintln!("l_min: {}, p_min_i: {}", l_min, p_min_i);
    if l_min == config.size {
        let mut f = File::options().append(true).open("output-170523.txt").unwrap();
        if config.plates[p_min_i].height == config.size {
            //we have found a square
            //return the square and 
            //create a string of ("Found a square, width: {}, height: {}, squares: ", config.size, config.plates[p_min_i].height + config.squares:
            let mut s = String::new();
            s += "S, W: ";
            s += &config.size.to_string();
            s += ", H: ";
            s += &config.plates[p_min_i].height.to_string();
            s += ", ORDER: ";
            s += &config.order().to_string();
            s += ", SET: ";
            s += &config.squares_to_string();
            writeln!(&mut f, "{}", s).unwrap();
        }
        else {
            //we have found a rectangle
            //return the rectangle}
            //create a string of ""Found a rectangle, width: {}, height: {}, squares: ", config.size, config.plates[p_min_i].height, );
            // config.print_squares();
            let mut s = String::new();
            s += "R, W: ";
            s += &config.size.to_string();
            s += ", H: ";
            s += &config.plates[p_min_i].height.to_string();
            s += ", ORDER: ";
            s += &config.order().to_string();
            s += ", SET: ";
            s += &config.squares_to_string();
            writeln!(&mut f, "{}", s).unwrap();
            //println!("continuing search...");
            decompose(config, p_min_i);
        }
    }
    else {
        decompose(config, p_min_i);
    }
}

fn next_plate_cc(send : &Sender<Message>, rcv: &Receiver<()>, config: &mut Config) -> () { //find smallest delimited plate, and decompose it
    let mut l_min : Integer = config.size + Integer::from(1); //equiv to infinity
    let mut p_min_i : usize = 0;
    //find the minimum delimited plate
    //if, in the meantime, we identify that there is only three plates, we have found a square or rectangle and should return them
    for i in 1..config.num_plates()-1 {
        let plate = &config.plates[i];
        let plate_prev = &config.plates[i-1];
        let plate_next = &config.plates[i+1];
        if plate_prev.height > plate.height && plate_next.height > plate.height && plate.width < l_min{
            l_min = plate.width;
            p_min_i = i;
        }
    }
    //print width and index of minimum delimited plate
    ////eprintln!("l_min: {}, p_min_i: {}", l_min, p_min_i);
    if l_min == config.size {
        if config.plates[p_min_i].height == config.size {
            //we have found a square
            //return the square and 
            //println!("Found a square: {:?}", config);
        }
        else {
            //we have found a rectangle
            //return the rectangle}
            //println!("Found a rectangle: {:?}", config);
            //println!("continuing search...");
            decompose_cc(send, rcv, config, p_min_i);
        }
    }
    else {
        decompose_cc(send, rcv, config, p_min_i);
    }
}

pub fn decompose(mut config: &mut Config, plate_id: usize) -> () { //given a plate, decompose it by adding squares, then select the next plate if the plates change
    // if filling the plate with a square does not make the height greater than the size, add the square and then next plate
    //exprintln!("decomposing, config: {}, plate_id: {}, net_squares: {}", config, plate_id, config.net_squares);
    if config.has_no(config.plates[plate_id].width) && config.plates[plate_id].height + config.plates[plate_id].width <= config.size &&  (if plate_id == config.plates.len()-2 {config.plates[plate_id].width >= 5} else {true}){
        config.net_squares += 1;
        //eprintln!("a+ {}", config);
        let mut config_backup = config.clone();
        config.vertical_extension(plate_id);
        //println!("{:?}", config);
        next_plate(&mut config);
        //undo it
        config_backup.net_squares = config.net_squares;
        *config = config_backup;
        
    }
    else{
        ////eprintln!("a.");
    }
    // if the height separating the plate from the one to it's left is less than the length, extend the left plate horizontally by adding the square
    if config.has_no(config.plates[plate_id - 1].height - config.plates[plate_id].height) && config.plates[plate_id - 1].height - config.plates[plate_id].height < config.plates[plate_id].width{
        config.net_squares += 1;
        //eprintln!("b+ {}", config);
        let mut config_backup = config.clone();

        config.horizontal_extension(plate_id);
        //println!("{:?}", config);
        decompose(config, plate_id);
        //remove the square
        //config.reverse_horizontal_extension(plate_id);
        config_backup.net_squares = config.net_squares;
        *config = config_backup;
        //eprintln!("b- {}", config);

    }
    else{
        //////eprintln!("b.");
    }
    // iterate over all possible square sizes that can be added to the bottom left corner.
    //println!("{} to {}", 2, std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height) + 1);

    let min = if (config.plates[plate_id].height == 0 || plate_id == 1) {5} else {2};

    for s in min..(std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height)+1) {
        // if the square can be added to the bottom left corner, add it and then decompose the new plate)
        if config.has_no(s) && s != config.plates[plate_id-1].height - config.plates[plate_id].height{
            config.net_squares += 1;
            //print number of plates:        
            let mut config_backup = config.clone();

            config.add_square_quick(s, plate_id);
            //println!("{:?}", config);
            next_plate(&mut config);
            //undo it
            config_backup.net_squares = config.net_squares;
            *config = config_backup;
            //decompose(config, plate_id + 1);
            //remove the square
            //config.remove_square(plate_id);
            //println!("{} checked", s);
        }
        else{
            ////eprintln!("{} is not a valid square size", s)
        }
    }

}

fn decompose_cc(send : &Sender<Message>, rcv : &Receiver<()>, mut config: &mut Config, plate_id: usize) -> (){
    match rcv.try_recv() {
        Ok(_) => {
            //println!("Work unit produced: {}, plate: {}", config, plate_id);
            send.send(Message::WorkUnit((config.clone(), plate_id))).unwrap();
        },
        Err(_) => {

            // if filling the plate with a square does not make the height greater than the size, add the square and then next plate
            if config.has_no(config.plates[plate_id].width) && config.plates[plate_id].height + config.plates[plate_id].width <= config.size && (if plate_id == config.plates.len()-2 {config.plates[plate_id].width >= 5} else {true}){
                //eprintln!("a+ {}", config);
                let config_backup = config.clone();
                config.vertical_extension(plate_id);
                //println!("{:?}", config);
                next_plate_cc(send, rcv, &mut config);
                //undo it
                *config = config_backup;
            }
            else{
                ////eprintln!("a.");
            }
            // if the height separating the plate from the one to it's left is less than the length, extend the left plate horizontally by adding the square
            if config.has_no(config.plates[plate_id - 1].height - config.plates[plate_id].height) && config.plates[plate_id - 1].height - config.plates[plate_id].height < config.plates[plate_id].width{
                //eprintln!("b+ {}", config);

                config.horizontal_extension(plate_id);
                //println!("{:?}", config);
                decompose_cc(send, rcv, config, plate_id);
                //remove the square
                config.reverse_horizontal_extension(plate_id);
                //eprintln!("b- {}", config);

            }
            else{
                //////eprintln!("b.");
            }
            // iterate over all possible square sizes that can be added to the bottom left corner.
            //println!("{} to {}", 2, std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height) + 1);
        
            let min = if (config.plates[plate_id].height == 0 || plate_id == 1) {5} else {2};

            for s in min..(std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height)+1) {
                // if the square can be added to the bottom left corner, add it and then decompose the new plate)
                if config.has_no(s) && s != config.plates[plate_id-1].height - config.plates[plate_id].height{
                    //print number of plates:
                    config.add_square_quick(s, plate_id);
                    //println!("{:?}", config);
                    decompose_cc(send, rcv, config, plate_id + 1);
                    //remove the square
                    config.remove_square(plate_id);
                    //println!("{} checked", s);
                }
                else{
                    ////eprintln!("{} is not a valid square size", s)
                }
            }
        }
    }
}

pub fn initial_decompose_cc(send : &Sender<Message>, rcv : &Receiver<()>, config: &mut Config, plate_id: usize) -> (){

    { 
    //given a plate, decompose it by adding squares, then select the next plate if the plates change
    // if filling the plate with a square does not make the height greater than the size, add the square and then next plate

    if config.has_no(config.plates[plate_id].width) && config.plates[plate_id].height + config.plates[plate_id].width <= config.size && (if plate_id == config.plates.len()-2 {config.plates[plate_id].width >= 5} else {true}){
        //eprintln!("a+ {}", config);
        let config_backup = config.clone();
        config.vertical_extension(plate_id);
        next_plate_cc(send, rcv, config);
        //undo it
        *config = config_backup;
        //eprintln!("a- {}", config);
    }
    else{
        ////eprintln!("a.");
    }
    // if the height separating the plate from the one to it's left is less than the length, extend the left plate horizontally by adding the square
    if config.has_no(config.plates[plate_id - 1].height - config.plates[plate_id].height) && config.plates[plate_id - 1].height - config.plates[plate_id].height < config.plates[plate_id].width{
        //eprintln!("b+ {}", config);
        config.horizontal_extension(plate_id);
        decompose_cc(send, rcv, config, plate_id);
        //remove the square
        config.reverse_horizontal_extension(plate_id);
        //eprintln!("b- {}", config);
    }
    else{
        ////eprintln!("b.");
    }
    // iterate over all possible square sizes that can be added to the bottom left corner.
    //println!("{} to {}", 2, std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height) + 1);
    
    let min = if (config.plates[plate_id].height == 0 || plate_id == 1) {5} else {2};

    for s in min..(std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height)+1) {
        // if the square can be added to the bottom left corner, add it and then decompose the new plate)
        if config.has_no(s) && s != config.plates[plate_id-1].height - config.plates[plate_id].height{
            //print number of plates:
            config.add_square_quick(s, plate_id);
            decompose_cc(send, rcv, config, plate_id + 1);
            //remove the square
            config.remove_square(plate_id);
            //eprintln!("{} checked", s);
        }
        else{
        }
    }
    //println!("Initial decompose finished");

}
}

pub fn initial_SOLVE_decompose_cc(send : &Sender<Message>, rcv : &Receiver<()>, config: &mut Config) -> (){

    { 
    
    // iterate over all possible square sizes that can be added to the bottom left corner.
    //println!("{} to {}", 2, std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height) + 1);
    for s in (9..(config.plates[1].width/2 +1)) {
        config.add_square_quick(s, 1);
        send.send(Message::WorkUnit((config.clone(), 2))).unwrap();
        config.remove_square(1);
        //eprintln!("{} checked", s);
    }
    //println!("Initial decompose finished");

}
}

pub fn double_nest_init(send: &Sender<Message>,  config: &mut Config) -> () {
    let mut i = 1;
    //start time:
    //let mut start: std::time::Instant = std::time::Instant::now();
    config.net_squares = 2;
    for s1 in (9..(config.plates[1].width/2+1)){
        config.add_square_quick(s1, 1);
            for s2 in 5..((config.plates[2].width - 9) + 1){
                if s2 != s1 {
                    config.add_square_quick(s2, 2);
                        if s2 > s1 && s1 < config.plates[3].width {
                            send.send(Message::WorkUnit((config.clone(), 1))).unwrap();
                        }
                        else {
                            send.send(Message::WorkUnit((config.clone(), 3))).unwrap();
                        }
                    i+=1;
                    config.remove_square(2);
                }
            }
            //MY BOUND, to implement comment out
            let s2 = config.plates[2].width;
            if s2 != s1 {
                let mut c = config.clone();
                c.vertical_extension(2);
                    if s1 > s2 {
                        send.send(Message::WorkUnit((c, 2))).unwrap();
                    }
                    else{
                        send.send(Message::WorkUnit((c, 1))).unwrap();
                    }
                    i+=1;
            }

        config.remove_square(1);
    }
    //end time:
    //println!("{} work units produced", i);
    ////let end = std::time::Instant::now();
    //println!("Time elapsed producing units: {}ms", (end - start).as_millis());
}