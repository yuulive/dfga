use benchmark_functions::*;

fn main() {
    println!["{}", benchmark_functions::Ackley::f(benchmark_functions::Ackley::minimizer(37))];
}
