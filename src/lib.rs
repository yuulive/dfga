#![warn(missing_docs)]
#![warn(missing_doc_code_examples)]

//! The `funky` crate provides functionality for several functions that are commonly
//! used to benchmark new optimization algorithms. More specifically, function is part of a struct
//! that contains the objective function as well as other important information (bounds of the
//! canonical problem, the known minimum value, and a function that returns the global minimizer.

pub mod multi;
pub mod single;

/// These are constants used for testing
const LOW_D: usize = 2;
const HIGH_D: usize = 137;

/// This is a trait that ensures consistent implementation of single objective benchmark functions
pub trait SingleObjective {
    /// The global minimum is constant and zero
    const MINIMUM: f64;

    /// Function for evaluating the objective function
    fn f(x: Vec<f64>) -> f64;

    /// This function returns the minimizer (argument that will return the global minimum)
    fn minimizer(n: usize) -> Vec<f64>;

    /// This function is used for testing, and checks the correctness of the minimizer
    fn check_minimizer(d: usize) {
        assert_eq!(Self::f(Self::minimizer(d)), Self::MINIMUM)
    }
}

/// This is a trait that ensures consistent implementation of multi-objective benchmark functions
pub trait MultiObjective {
    /// Function for evaluating the set of objective functions
    fn f(x: Vec<f64>) -> Vec<f64>;
}

/// This is a trait that ensures consistent implementation of bounded benchmark functions
pub trait Bounded {
    /// The bounds of the canonical optimization problem
    const BOUNDS: (f64, f64);

    /// Function to check bounds
    fn in_bounds(x: Vec<f64>) -> bool {
        let mut in_bounds = true;
        for element in x {
            if (element < Self::BOUNDS.0) || (element > Self::BOUNDS.1) {
                in_bounds = false;
                break;
            }
        }
        in_bounds
    }
}

/// This is a trait that ensures consistent implementation of unbounded benchmark functions
pub trait UnBounded {
    /// The bounds of the canonical optimization problem
    const BOUNDS: (f64, f64) = (f64::INFINITY, f64::INFINITY);

    /// Function to check bounds
    fn in_bounds(_x: Vec<f64>) -> bool {
        true
    }
}

/// This is a trait that ensures consistent implementation of constrained benchmark functions
pub trait Constrained {
    /// This constant indicates a constrained function
    const CONSTRAINED: bool = true;

    /// This constant indicates the number of equality functions
    const NH: usize;

    /// This constant indicates the number of inequality functions
    const NG: usize;

    /// This function returns the value of equality constraints
    fn equality_constraints(x: Vec<f64>) -> Vec<f64>;

    /// This function returns the value of inequality constraints
    fn inequality_constraints(x: Vec<f64>) -> Vec<f64>;

    /// This is an alias for the equality constraint function
    fn h(x: Vec<f64>) -> Vec<f64> {
        Self::equality_constraints(x)
    }

    /// This is an alias for the inequality constraint function
    fn g(x: Vec<f64>) -> Vec<f64> {
        Self::inequality_constraints(x)
    }
}

/// This is a trait that ensure consistent implementation of unconstrained benchmark functions
pub trait UnConstrained {
    const CONSTRAINED: bool = false;
}

/// This is a trait that ensures consistent implementation of N-dimensional benchmark functions
pub trait NDimensional {
    /// This is a constant containing the correct dimensionality for the function
    const D: usize = usize::MAX;
}

/// This is a trait that ensures consistent implementation of benchmark functions with fixed dimensionality
pub trait FixedDimensional {
    /// This is a constant containing the correct dimensionality for the function
    const D: usize;

    /// This function is used to check inputs
    fn check_input(x: Vec<f64>){
        if x.len() != Self::D {
            panic!("A vector with size {} was used with a function of dimensionality {}.", x.len(), Self::D);
        }
    }
}
