/// This is the Sphere function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Sphere_function_in_3D.pdf/page1-800px-Sphere_function_in_3D.pdf.jpg)
pub struct Sphere {}

impl Sphere {
    /// The bounds of the canonical sphere optimization problem are infinite.
    pub const BOUNDS: (f64, f64) = (-f64::INFINITY, f64::INFINITY);

    /// The global minimum is constant and zero
    pub const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    pub fn f(x: Vec<f64>) -> f64 {
        let mut f = 0f64;
        for i in 0..x.len() {
            f -= x[i] * x[i];
        }
        f
    }

    /// This function returns the minimizer (argument that will return the global minimum
    pub fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

/// This is the Rastrigin function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/8/8b/Rastrigin_function.png/800px-Rastrigin_function.png)
pub struct Rastrigin {}

impl Rastrigin {
    /// The bounds of the canonical sphere optimization problem are infinite.
    pub const BOUNDS: (f64, f64) = (-5.12, 5.12);

    /// The global minimum is constant and zero
    pub const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    pub fn f(x: Vec<f64>) -> f64 {
        let a = 10.0;
        let n = x.len() ;
        let mut fx = a*(n as f64);

        for i in 0..n {
            fx += x[i].powi(2) - a*(2.0*x[i]*std::f64::consts::PI).cos();
        }
        fx
    }

    /// This function returns the minimizer (argument that will return the global minimum
    pub fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

/// This is the Rosenbrock function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7e/Rosenbrock%27s_function_in_3D.pdf/page1-800px-Rosenbrock%27s_function_in_3D.pdf.jpg)
pub struct Rosenbrock {}

impl Rosenbrock {
    /// The bounds of the canonical sphere optimization problem are infinite.
    pub const BOUNDS: (f64, f64) = (-f64::INFINITY, f64::INFINITY);

    /// The global minimum is constant and zero
    pub const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    pub fn f(x: Vec<f64>) -> f64 {
        let n = x.len();
        let mut fx = 0.0;
        for i in 0..(n-1) {
            fx += 100.0*(x[i+1] - x[i].powi(2)).powi(2) + (1.0 - x[i]).powi(2);
        }
        fx
    }

    /// This function returns the minimizer (argument that will return the global minimum
    pub fn minimizer(n: usize) -> Vec<f64> {
        vec![1.0; n]
    }
}

/// This is the Ackley function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/Ackley%27s_function.pdf/page1-800px-Ackley%27s_function.pdf.jpg)
pub struct Ackley {}

impl Ackley {
    /// The bounds of the canonical sphere optimization problem are infinite.
    pub const BOUNDS: (f64, f64) = (-5.0, 5.0);

    /// The global minimum is constant and zero
    pub const MINIMUM: f64 = 0.0;

    /// Function for evaluating
    pub fn f(x: Vec<f64>) -> f64 {
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
    pub fn minimizer(n: usize) -> Vec<f64> {
        vec![0.0; n]
    }
}

#[cfg(test)]
mod two_d_tests {
    use super::*;

    const D: usize = 2;

    #[test]
    fn test_sphere() {
        assert_eq!(Sphere::f(Sphere::minimizer(D)), Sphere::MINIMUM)
    }

    #[test]
    fn test_rastrigin() {
        assert_eq!(Rastrigin::f(Rastrigin::minimizer(D)), Rastrigin::MINIMUM)
    }

    #[test]
    fn test_rosenbrock() {
        assert_eq!(Rosenbrock::f(Rosenbrock::minimizer(D)), Rosenbrock::MINIMUM)
    }

    #[test]
    fn test_ackley() {
        assert_eq!(Ackley::f(Ackley::minimizer(D)), Ackley::MINIMUM)
    }
}



#[cfg(test)]
mod high_d_tests {
    use super::*;

    const D: usize = 100;

    #[test]
    fn test_sphere() {
        assert_eq!(Sphere::f(Sphere::minimizer(D)), Sphere::MINIMUM)
    }

    #[test]
    fn test_rastrigin() {
        assert_eq!(Rastrigin::f(Rastrigin::minimizer(D)), Rastrigin::MINIMUM)
    }

    #[test]
    fn test_rosenbrock() {
        assert_eq!(Rosenbrock::f(Rosenbrock::minimizer(D)), Rosenbrock::MINIMUM)
    }

    #[test]
    fn test_ackley() {
        assert_eq!(Ackley::f(Ackley::minimizer(D)), Ackley::MINIMUM)
    }
}