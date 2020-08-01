use benchfun::*;

fn main() {
    println!["{}", benchfun::single::Ackley::f(benchfun::single::Ackley::minimizer(37))];
}
