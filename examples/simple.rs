// Import the benchmark library
use ka::*;

fn main() {
    // Print some info about the ackley function
    println!("Minmimum: {:?}", Ackley::MINIMUM);
    println!("Minmizer: {:?}", Ackley::minimizer(5));
    println!("Minmizer: {:?}", single::Ackley::BOUNDS);
}
