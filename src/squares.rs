use std::collections::HashSet;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use std::mem;
use num;

pub type Integer = i8;

#[derive(Clone)] //useful later for threads
pub struct Square {
    squares: HashSet<Integer>,
    // todo: bouwkcamp / table code
}

pub struct Plates{
    plates: Vec<Plate>,
}

impl Debug for Square {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Square: {:?}", self.squares)
    }
}
impl Plates{
    pub fn new(size: Integer) -> Self {
        let mut p = Vec::new();
        //First plate: height size + 1, width 1
        p.push(Plate{height: size + 1, width: 1});
        //Second plate: height 0, width size
        p.push(Plate{height: 0, width: size});
        //Third plate: height size + 1, width 1
        p.push(Plate{height: size + 1, width: 1});
        Self {
            plates: p,
        }
    }
    fn add_plate(&mut self, plate: Plate) {
        unimplemented!();

    }
    fn remove_plate(&mut self, plate: Plate) {
        unimplemented!();
    }
}
impl Debug for Plates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plates (width, height): {:?}", self.plates)
    }
}

struct Plate {
    height: Integer,
    width: Integer,
}
impl Debug for Plate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.width, self.height)
    }
}