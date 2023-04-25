use crate::squares::*;

pub fn solve (size: Integer) -> (){
    println!{"Solving for size: {}", size};
    let mut config = Config::new(size); //Creates the necessary starting plate.
    println!("Config: {:?}", config);
    decompose(&mut config, 1);
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
    //eprintln!("l_min: {}, p_min_i: {}", l_min, p_min_i);
    if l_min == config.size {
        if config.plates[p_min_i].height == config.size {
            //we have found a square
            //return the square and 
            println!("Found a square: {:?}", config);
        }
        else {
            //we have found a rectangle
            //return the rectangle}
            println!("Found a rectangle: {:?}", config);
            println!("continuing search...");
            decompose(config, p_min_i);
        }
    }
    else {
        decompose(config, p_min_i);
    }
}

fn decompose(config: &mut Config, plate_id: usize) -> () { //given a plate, decompose it by adding squares, then select the next plate if the plates change
    // if filling the plate with a square does not make the height greater than the size, add the square and then next plate
    if config.has_no(config.plates[plate_id].width) && config.plates[plate_id].height + config.plates[plate_id].width <= config.size{
        let config_backup = config.clone();
        config.vertical_extension(plate_id);
        next_plate(config);
        //undo it
        *config = config_backup;
        //eprintln!("a- {}", config);
    }
    else{
        //eprintln!("a.");
    }
    // if the height separating the plate from the one to it's left is less than the length, extend the left plate horizontally by adding the square
    if config.has_no(config.plates[plate_id - 1].height - config.plates[plate_id].height) && config.plates[plate_id - 1].height + config.plates[plate_id].height < config.plates[plate_id].width{
        config.horizontal_extension(plate_id);
        decompose(config, plate_id);
        //remove the square
        config.reverse_horizontal_extension(plate_id);
    }
    else{
        ////eprintln!("b.");
    }
    // iterate over all possible square sizes that can be added to the bottom left corner.
    //println!("{} to {}", 2, std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height) + 1);
    for s in 2..std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height)+1 {
        // if the square can be added to the bottom left corner, add it and then decompose the new plate)
        if config.has_no(s) && s != config.plates[plate_id-1].height - config.plates[plate_id].height{
            //print number of plates:
            config.add_square_quick(s, plate_id);
            decompose(config, plate_id + 1);
            //remove the square
            config.remove_square(plate_id);
        }
        else{
            ////eprintln!("{} is not a valid square size", s)
        }
    }
    //state search space for squares:
    //eprintln!("c fin, searched 2 to {} squares", std::cmp::min(config.plates[plate_id].width - 1, config.size - config.plates[plate_id].height));
    // undo adding the square
}