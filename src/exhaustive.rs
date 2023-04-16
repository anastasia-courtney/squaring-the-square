use crate::squares::{Integer, *};

pub fn solve (size: Integer) -> (){
    println!{"Solving for size: {}", size};
    let mut plates = Plates::new(size);
    println!("{:?}", plates);

}