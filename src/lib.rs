//! The `benchmark_functions` crate provides functionality for several functions that are commonly
//! used to benchmark new optimization algorithms. More specifically, function is part of a struct
//! that contains the objective function as well as other important information (bounds of the
//! canonical problem, the known minimum value, and a function that returns the global minimizer.

/// This is a trait that ensures consistent implementation of different benchmark functions
pub trait Function {
    /// The bounds of the canonical optimization problem
    const BOUNDS: (f64, f64);

    /// The global minimum is constant and zero
    const MINIMUM: f64;

    /// Function for evaluating the objective function
    fn f(x: Vec<f64>) -> f64;

    /// This function returns the minimizer (argument that will return the global minimum)
    fn minimizer(n: usize) -> Vec<f64>;
}


/// This is a constant used for low-dimensional testing
const LOW_D: usize = 2;
const HIGH_D: usize = 137;

/// This is the Sphere function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Sphere_function_in_3D.pdf/page1-800px-Sphere_function_in_3D.pdf.jpg)
pub struct Sphere {}

impl Function for Sphere {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-f64::INFINITY, f64::INFINITY);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let mut f = 0f64;
        for i in 0..x.len() {
            f -= x[i] * x[i];
        }
        f
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Sphere::f(Sphere::minimizer(LOW_D)), Sphere::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Sphere::f(Sphere::minimizer(HIGH_D)), Sphere::MINIMUM)
    }
}

/// This is the Rastrigin function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/8/8b/Rastrigin_function.png/800px-Rastrigin_function.png)
pub struct Rastrigin {}

impl Function for Rastrigin {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-5.12, 5.12);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let a = 10.0;
        let n = x.len() ;
        let mut fx = a*(n as f64);

        for i in 0..n {
            fx += x[i].powi(2) - a*(2.0*x[i]*std::f64::consts::PI).cos();
        }
        fx
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod rastrigin_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Rastrigin::f(Rastrigin::minimizer(LOW_D)), Rastrigin::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Rastrigin::f(Rastrigin::minimizer(HIGH_D)), Rastrigin::MINIMUM)
    }
}

/// This is the Rosenbrock function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7e/Rosenbrock%27s_function_in_3D.pdf/page1-800px-Rosenbrock%27s_function_in_3D.pdf.jpg)
pub struct Rosenbrock {}

impl Function for Rosenbrock {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-f64::INFINITY, f64::INFINITY);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n = x.len();
        let mut fx = 0.0;
        for i in 0..(n-1) {
            fx += 100.0*(x[i+1] - x[i].powi(2)).powi(2) + (1.0 - x[i]).powi(2);
        }
        fx
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![1.0; n]
    }
}

#[cfg(test)]
mod rosenbrock_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Rosenbrock::f(Rosenbrock::minimizer(LOW_D)), Rosenbrock::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Rosenbrock::f(Rosenbrock::minimizer(HIGH_D)), Rosenbrock::MINIMUM)
    }
}

/// This is the Ackley function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/Ackley%27s_function.pdf/page1-800px-Ackley%27s_function.pdf.jpg)
pub struct Ackley {}

impl Function for Ackley {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-5.0, 5.0);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n=x.len();
        let mut fx = 0.0;
        let mut square_sum = 0.0;
        let mut cosine_sum = 0.0;
        for i in 0..n {
            square_sum += x[i].powi(2);
            cosine_sum += (2.0*std::f64::consts::PI*x[i]).cos();
        }
        fx += -20.0*(-0.2*(0.5*square_sum).sqrt()).exp();
        fx -= (cosine_sum/(n as f64)).exp();
        fx + std::f64::consts::E + 20.0
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod ackley_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Ackley::f(Ackley::minimizer(LOW_D)), Ackley::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Ackley::f(Ackley::minimizer(HIGH_D)), Ackley::MINIMUM)
    }
}

/// This is the Ackley function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/6/63/Matyas_function.pdf/page1-800px-Matyas_function.pdf.jpg)
pub struct Matyas {}

impl Function for Matyas {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-10.0, 10.0);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n=x.len();
        let mut square_sum = 0.0;
        let mut prod = 1.0;
        for i in 0..n {
            square_sum += x[i].powi(2);
            prod *= x[i];
        }
        0.26*square_sum - 0.48*prod
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod matyas_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Matyas::f(Matyas::minimizer(LOW_D)), Matyas::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Matyas::f(Matyas::minimizer(HIGH_D)), Matyas::MINIMUM)
    }
}

/// This is the Griewank function.
///
/// The function is borrowed from [here](http://benchmarkfcns.xyz/benchmarkfcns/griewankfcn.html).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](http://benchmarkfcns.xyz/benchmarkfcns/plots/griewankfcn_10_0.png)
pub struct Griewank {}

impl Function for Griewank {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-600.0, 600.0);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n=x.len();
        let mut cosine_prod = 1.0;
        let mut square_sum = 0.0;
        for i in 0..n {
            square_sum += x[i].powi(2);
            cosine_prod *= (x[i]/((i+1) as f64).sqrt()).cos();
        }
        1.0 + square_sum/4000.0 - cosine_prod
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod griewank_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Griewank::f(Griewank::minimizer(LOW_D)), Griewank::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Griewank::f(Griewank::minimizer(HIGH_D)), Griewank::MINIMUM)
    }
}

/// This is the Ridge function.
///
/// The function is borrowed from [here](http://benchmarkfcns.xyz/benchmarkfcns/ridgefcn.html).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](http://benchmarkfcns.xyz/benchmarkfcns/plots/ridgefcn.png)
pub struct Ridge {}

impl Function for Ridge {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-5.0, 5.0);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = -5.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n=x.len();
        let D = 1.0;
        let alpha = 0.0;
        let mut square_sum = 0.0;
        for i in 1..n {
            square_sum += x[i].powi(2);
        }
        -1.0 + x[0] + D *square_sum.powf(alpha)
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        let mut v = vec![0.0; n];
        v[0] = -5.0;
        v
    }
}

#[cfg(test)]
mod ridge_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Ridge::f(Ridge::minimizer(LOW_D)), Ridge::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Ridge::f(Ridge::minimizer(HIGH_D)), Ridge::MINIMUM)
    }
}

/// This is the Zakharov function.
///
/// The function is borrowed from [here](http://benchmarkfcns.xyz/benchmarkfcns/zakharov.html).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](http://benchmarkfcns.xyz/benchmarkfcns/plots/zakharovfcn.png)
pub struct Zakharov {}

impl Function for Zakharov {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-5.0, 10.0);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n=x.len();
        let mut square_sum = 0.0;
        let mut sum_ixi = 0.0;
        for i in 0..n {
            square_sum += x[i].powi(2);
            sum_ixi += 0.5*x[i]*(i as f64);
        }
        square_sum + sum_ixi.powi(2) + sum_ixi.powi(4)
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod zakharov_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Zakharov::f(Zakharov::minimizer(LOW_D)), Zakharov::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Zakharov::f(Zakharov::minimizer(HIGH_D)), Zakharov::MINIMUM)
    }
}

/// This is the Salomon function.
///
/// The function is borrowed from [here](http://benchmarkfcns.xyz/benchmarkfcns/salomonfcn.html).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks
/// like in 2D:
///
/// ![](http://benchmarkfcns.xyz/benchmarkfcns/plots/salomonfcn.png)
pub struct Salomon {}

impl Function for Salomon {
    /// The bounds of the canonical sphere optimization problem are infinite.
    const BOUNDS: (f64, f64) = (-100.0, 100.0);

    /// The global minimum is constant and zero
    const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    fn f(x: Vec<f64>) -> f64 {
        let n=x.len();
        let mut square_sum = 0.0;
        for i in 0..n {
            square_sum += x[i].powi(2);
        }
        1.0 - (2.0*std::f64::consts::PI*square_sum.sqrt()).cos() + 0.1*square_sum.sqrt()
    }

    /// This function returns the minimizer (argument that will return the global minimum
    fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod salomon_tests {
    use super::*;

    #[test]
    fn low_d() {
        assert_eq!(Salomon::f(Salomon::minimizer(LOW_D)), Salomon::MINIMUM)
    }

    #[test]
    fn high_d() {
        assert_eq!(Salomon::f(Salomon::minimizer(HIGH_D)), Salomon::MINIMUM)
    }
}