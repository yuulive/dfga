use benchmark_functions::*;

fn main() {
    println!["{}", benchmark_functions::singleobjective::Ackley::f(benchmark_functions::singleobjective::Ackley::minimizer(37))];
}
