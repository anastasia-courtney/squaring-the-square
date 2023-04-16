mod squares;
mod exhaustive;

fn main() {
    println!("Hello, world!");
    exhaustive::solve(i8::try_from(5).unwrap());
}
