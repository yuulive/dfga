use crate::{FixedDimensional, Constrained, MultiObjective};

/// This is the Chankong-Haimes function.
///
/// The function is borrowed from [here](https://en.wikipedia.org/wiki/Test_functions_for_optimization).
/// This function is specificaly 2 dimensional, and has a Pareto fron that looks like this:
///
/// ![](https://upload.wikimedia.org/wikipedia/commons/thumb/a/a8/Chakong_and_Haimes_function.pdf/page1-796px-Chakong_and_Haimes_function.pdf.jpg)
pub struct ChankongHaimes {}

impl FixedDimensional for ChankongHaimes {
    const D: usize = 2;
}

impl Constrained for ChankongHaimes {
    const NH: usize = 0;
    const NG: usize = 2;

    fn equality_constraints(_x: Vec<f64>) -> Vec<f64> {
        vec![0.0; Self::NH]
    }

    fn inequality_constraints(x: Vec<f64>) -> Vec<f64> {
        let mut fx: Vec<f64> = vec![0.0; Self::NG];
        fx[0] = x[0].powi(2) + x[1].powi(2) - 225.0;
        fx[1] = x[0] - 3.0*x[1] + 10.0;
        fx
    }
}

impl MultiObjective for ChankongHaimes {
    fn f(x: Vec<f64>) -> Vec<f64> {
        Self::check_input(x.clone());
        let mut fx: Vec<f64> = vec![0.0; Self::D];
        fx[0] = 2.0 + (x[0] - 2.0).powi(2) - (x[1] - 1.0).powi(2);
        fx[1] = 9.0*x[0] - (x[1] - 1.0).powi(2);
        fx
    }
}

#[cfg(test)]
mod chankong_haimes_tests {
    use super::*;

    #[test]
    fn check_zero() {
        let x = vec![0.0; ChankongHaimes::D];
        ChankongHaimes::f(x.clone());
        ChankongHaimes::equality_constraints(x.clone());
        ChankongHaimes::inequality_constraints(x);
        assert!(true);
    }

    #[test]
    fn check_one() {
        let x = vec![1.0; ChankongHaimes::D];
        ChankongHaimes::f(x.clone());
        ChankongHaimes::equality_constraints(x.clone());
        ChankongHaimes::inequality_constraints(x);
        assert!(true);
    }
}
