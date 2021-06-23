//! Provides functions related to factorial calculations (e.g. binomial
//! coefficient, factorial, multinomial)
use crate::factorial::gamma;
use std::sync::Once;

/// The maximum factorial representable
/// by a 64-bit floating point without
/// overflowing
pub const MAX_ARG: u64 = 170;

/// Computes the factorial function `x -> x!` for
/// `170 >= x >= 0`. All factorials larger than `170!`
/// will overflow an `f64`.
///
/// # Remarks
///
/// Returns `f64::INFINITY` if `x > 170`
pub fn factorial(x: u64) -> f64 {
    if x > MAX_ARG {
        f64::INFINITY
    } else {
        get_fcache()[x as usize]
    }
}

/// Computes the logarithmic factorial function `x -> ln(x!)`
/// for `x >= 0`.
///
/// # Remarks
///
/// Returns `0.0` if `x <= 1`
pub fn ln_factorial(x: u64) -> f64 {
    if x <= 1 {
        0.0
    } else if x > MAX_ARG {
        gamma::ln_gamma(x as f64 + 1.0)
    } else {
        get_fcache()[x as usize].ln()
    }
}

/// Computes the binomial coefficient `n choose k`
/// where `k` and `n` are non-negative values.
///
/// # Remarks
///
/// Returns `0.0` if `k > n`
pub fn binomial(n: u64, k: u64) -> f64 {
    if k > n {
        0.0
    } else {
        (0.5 + (ln_factorial(n) - ln_factorial(k) - ln_factorial(n - k)).exp()).floor()
    }
}

/// Computes the natural logarithm of the binomial coefficient
/// `ln(n choose k)` where `k` and `n` are non-negative values
///
/// # Remarks
///
/// Returns `f64::NEG_INFINITY` if `k > n`
pub fn ln_binomial(n: u64, k: u64) -> f64 {
    if k > n {
        f64::NEG_INFINITY
    } else {
        ln_factorial(n) - ln_factorial(k) - ln_factorial(n - k)
    }
}


// Initialization for pre-computed cache of 171 factorial
// values 0!...170!
const CACHE_SIZE: usize = 171;

static mut FCACHE: &'static mut [f64; CACHE_SIZE] = &mut [1.0; CACHE_SIZE];
static START: Once = Once::new();

fn get_fcache() -> &'static [f64; CACHE_SIZE] {
    unsafe {
        START.call_once(|| {
            (1..CACHE_SIZE).fold(FCACHE[0], |acc, i| {
                let fac = acc * i as f64;
                FCACHE[i] = fac;
                fac
            });
        });
        FCACHE
    }
}

#[cfg_attr(rustfmt, rustfmt_skip)]
#[cfg(test)]
mod test {
    use approx::assert_abs_diff_eq;

    #[test]
    fn test_factorial_and_ln_factorial() {
        let mut factorial = 1.0;
        for i in 1..171 {
            factorial *= i as f64;
            assert_eq!(super::factorial(i), factorial);
            assert_eq!(super::ln_factorial(i), factorial.ln());
        }
    }

    #[test]
    fn test_factorial_overflow() {
        assert_eq!(super::factorial(172), f64::INFINITY);
        assert_eq!(super::factorial(u64::MAX), f64::INFINITY);
    }

    #[test]
    fn test_ln_factorial_does_not_overflow() {
        assert_eq!(super::ln_factorial(1 << 10), 6078.2118847500501140);
        assert_abs_diff_eq!(super::ln_factorial(1 << 12), 29978.648060844048236, epsilon=1e-11);
        assert_eq!(super::ln_factorial(1 << 15), 307933.81973375485425);
        assert_eq!(super::ln_factorial(1 << 17), 1413421.9939462073242);
    }

    #[test]
    fn test_binomial() {
        assert_eq!(super::binomial(1, 1), 1.0);
        assert_eq!(super::binomial(5, 2), 10.0);
        assert_eq!(super::binomial(7, 3), 35.0);
        assert_eq!(super::binomial(1, 0), 1.0);
        assert_eq!(super::binomial(0, 1), 0.0);
        assert_eq!(super::binomial(5, 7), 0.0);
    }

    #[test]
    fn test_ln_binomial() {
        assert_eq!(super::ln_binomial(1, 1), 1f64.ln());
        assert_abs_diff_eq!(super::ln_binomial(5, 2), 10f64.ln(), epsilon=1e-14);
        assert_abs_diff_eq!(super::ln_binomial(7, 3), 35f64.ln(), epsilon=1e-14);
        assert_eq!(super::ln_binomial(1, 0), 1f64.ln());
        assert_eq!(super::ln_binomial(0, 1), 0f64.ln());
        assert_eq!(super::ln_binomial(5, 7), 0f64.ln());
    }
}
