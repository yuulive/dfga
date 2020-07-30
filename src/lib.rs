/// This is the Sphere function, borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
///
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a4/Sphere_function_in_3D.pdf/page1-800px-Sphere_function_in_3D.pdf.jpg)
pub fn sphere(x: Vec<f64>) -> f64 {
    let mut f = 0f64;
    for i in 0..x.len() {
        f -= x[i]*x[i];
    }
    f
}

/// This is the Rastrigin function, borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
///
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/8/8b/Rastrigin_function.png/800px-Rastrigin_function.png)
pub fn rastrigin(x: Vec<f64>) -> f64 {
    let a = 10.0;
    let n = x.len() ;
    let mut fx = a*(n as f64);

    for i in 0..n {
        fx += x[i].powi(2) - a*(2.0*x[i]*std::f64::consts::PI).cos();
    }
    fx
}

/// This is the Rosenbrock function, borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
///
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/7/7e/Rosenbrock%27s_function_in_3D.pdf/page1-800px-Rosenbrock%27s_function_in_3D.pdf.jpg)
pub fn rosenbrock(x: Vec<f64>) -> f64 {
    let n = x.len();
    let mut fx = 0.0;
    for i in 0..(n-1) {
        fx += 100.0*(x[i+1] - x[i].powi(2)).powi(2) + (1.0 - x[i]).powi(2);
    }
    fx
}

/// This is the Ackley function, borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
///
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/9/98/Ackley%27s_function.pdf/page1-800px-Ackley%27s_function.pdf.jpg)
pub fn ackley(x: Vec<f64>) -> f64 {
    let n=x.len();
    let mut fx = 0.0;
    let mut square_sum = 0.0;
    let mut cosine_sum = 0.0;
    for i in 0..n {
        square_sum += x[i].powi(2);
        cosine_sum += (2.0*std::f64::consts::PI*x[i]).cos();
    }
    fx += -20.0*(-0.2*(0.5*square_sum).sqrt()).exp();
    fx -= (0.5*cosine_sum).exp();
    fx + std::f64::consts::E + 20.0
}

/// This is the Styblinski-Tang function, borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
///
/// Although the function accepts a vector with an arbitrary number of inputs, this is what it looks like in 2D ![](https://upload.wikimedia.org/wikipedia/commons/thumb/8/8e/Styblinski-Tang_function.pdf/page1-800px-Styblinski-Tang_function.pdf.jpg)
pub fn styblinski_tang(x: Vec<f64>) -> f64 {
    let n = x.len();
    let mut fx = 0.0;
    for i in 0..n {
        fx += x[i].powi(4) - 16.0*x[i].powi(2) + 5.0*x[i];
    }
    fx/2.0
}

#[cfg(test)]
mod two_d_tests {
    use super::*;

    #[test]
    fn test_sphere() {
        assert_eq!(sphere(vec![0.0; 2]), 0.0)
    }

    #[test]
    fn test_rastrigin() {
        assert_eq!(rastrigin(vec![0.0; 2]), 0.0)
    }

    #[test]
    fn test_ackley() {
        assert_eq!(ackley(vec![0.0; 2]), 0.0)
    }
}