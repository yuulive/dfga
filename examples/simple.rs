// Import the benchmark library
use benchfun::*;

fn main() {
    // Print some info about the ackley function
    println!("Minmimum: {:?}", benchfun::single::Ackley::MINIMUM);
    println!("Minmizer: {:?}", benchfun::single::Ackley::minimizer(5));
    println!("Minmizer: {:?}", benchfun::single::Ackley::BOUNDS);
}
