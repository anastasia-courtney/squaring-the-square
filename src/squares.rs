use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::mem;

pub type Integer = i32;

#[derive(Clone)]
pub struct Config {
    pub squares: Vec<Integer>,
    pub size: Integer,
    pub plates: Vec<Plate>
}

impl Config{
    pub fn new(size: Integer) -> Self {
        let s = Vec::new();
        let mut p = Vec::new();
        //First plate: height size + 1, width 1
        p.push(Plate{height: size + 1, width: 1});
        //Second plate: height 0, width size
        p.push(Plate{height: 0, width: size});
        //Third plate: height size + 1, width 1
        p.push(Plate{height: size + 1, width: 1});
        Self {
            squares: s,
            size: size,
            plates: p
        }
    }
    pub fn num_plates(&self) -> usize {
        self.plates.len()
    }

    pub fn has_no(&self, square: Integer) -> bool {
        !self.squares.contains(&square)
    }

    pub fn add_square_quick(&mut self, square: Integer, plate_id: usize) -> () {
        //add_square without merge checks.
        self.squares.push(square);
        let original_plate_height = self.plates[plate_id].height;
        self.plates.insert(plate_id, Plate{height: square + original_plate_height, width: square});
        //take the width of the square from the original plate
        self.plates[plate_id + 1].width -= square;
        //////eprintln!("c+ {}", self);
    }

    pub fn horizontal_extension(&mut self, plate_id: usize) -> () { //extends the plate on the left by adding a square
        let square = self.plates[plate_id - 1].height - self.plates[plate_id].height;
        self.squares.push(square);
        self.plates[plate_id - 1].width += square;
        self.plates[plate_id].width -= square;
        ////eprintln!("b+ {}", self);
    }

    pub fn reverse_horizontal_extension(&mut self, plate_id: usize) -> () { //remove extension
        let square = self.plates[plate_id - 1].height - self.plates[plate_id].height;
        self.squares.pop();
        self.plates[plate_id - 1].width -= square;
        self.plates[plate_id].width += square;
        ////eprintln!("b- {}", self);
    }

    pub fn vertical_extension(&mut self,  plate_id: usize) -> () {
        let square = self.plates[plate_id].width;
        self.squares.push(square);
        self.plates[plate_id].height += square;
        if self.plates[plate_id].height == self.plates[plate_id + 1].height {
            //merge the two plates:
            self.plates[plate_id].width += self.plates[plate_id + 1].width;
            self.plates.remove(plate_id + 1);
        }
        if self.plates[plate_id].height == self.plates[plate_id - 1].height {
            //merge the two plates:
            self.plates[plate_id - 1].width += self.plates[plate_id].width;
            self.plates.remove(plate_id);
        }   
        ////eprintln!("a+ {}", self);
    }     

    pub fn remove_square(&mut self, plate_id: usize) -> () {
        //remove_square, which makes it's entire own plate, and merges with the next plate
        let square = self.plates[plate_id].width;
        self.squares.pop();
        self.plates.remove(plate_id);
        self.plates[plate_id].width += square;
        ////eprintln!("c- {}", self);

    }
}

impl Debug for Config{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config: size: {}, squares: {:?}, plates: {:?}", self.size, self.squares, self.plates)
    }
}

impl Display for Config{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Config: size: {}, squares: {:?}, plates: {:?}", self.size, self.squares, self.plates)
    }
}

#[derive(Clone)] //useful later for threads
pub struct Square {
    squares: HashSet<Integer>,
    // todo: bouwkcamp / table code
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square: {:?}", self.squares)
    }
}

#[derive(Clone)]
pub struct Plate {
    pub height: Integer,
    pub width: Integer,
}
impl Debug for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}