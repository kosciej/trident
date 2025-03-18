//! Statistical calculator library
//!
//! Provides two implementations for calculating streaming statistics:
//! - Naive calculator: Simple reference implementation
//! - Optimized calculator: Performance-optimized version
//!
//! # Examples
//! ```
//! use calculator_lib::{naive, Calculator};
//!
//! let mut calc = naive();
//! calc.append(&[1.0, 2.0, 3.0]);
//! let stats = calc.calculate_stats(1);
//! ```

pub mod naive;
pub mod optimized;

/// Trait defining calculator operations
pub trait Calculator {
    /// Append values to the calculator's dataset
    ///
    /// # Arguments
    /// * `values` - Slice of f64 values to add
    fn append(&mut self, values: &[f64]);
    /// Calculate statistical measures for the dataset
    ///
    /// # Arguments
    /// * `k` - Number of last values to use in stats calculation 1e{k}
    ///
    /// # Returns
    /// [`StatsResponse`] containing calculated statistics
    fn calculate_stats(&self, k: u8) -> crate::StatsResponse;
}

/// Statistical measures container
///
/// Contains calculated statistics for a dataset with precision control.
#[derive(Debug, serde::Serialize, Default)]
pub struct StatsResponse {
    /// Last value added to the dataset
    pub last: f64,
    /// Minimum value in the dataset
    pub min: f64,
    /// Maximum value in the dataset
    pub max: f64,
    /// Arithmetic mean of the dataset
    pub avg: f64,
    /// Variance of the dataset
    pub var: f64,
}

/// Create a naive calculator implementation
///
/// Uses a simple buffer-based approach with:
/// - O(1) append operations
/// - O(n) statistical calculations
/// - Fixed buffer size of 100 million elements
///
/// # Examples
/// ```
/// use calculator_lib::naive;
///
/// let mut calc = naive();
/// calc.append(&[1.0, 2.0, 3.0]);
/// let stats = calc.calculate_stats(2);
/// ```
pub fn naive() -> impl Calculator {
    naive::NaiveCalculator::new(10_usize.pow(8))
}

/// Create an optimized calculator implementation
///
/// Uses circular buffers with:
/// - amortized O(1) append operations
/// - O(1) statistical calculations
///
/// # Examples
/// ```
/// use calculator_lib::optimized;
///
/// let mut calc = optimized();
/// calc.append(&[1.0, 2.0, 3.0]);
/// let stats = calc.calculate_stats(2);
/// ```
pub fn optimized() -> impl Calculator {
    optimized::OptimizedCalculator::new(8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};

    #[test]
    fn compare_implementations() {
        let mut rng = StdRng::seed_from_u64(42);
        let mut naive = naive();
        let mut optimized = optimized();

        // Test with 10 random batches
        for _ in 0..10 {
            let batch_size = rng.gen_range(1..=1000);
            let values: Vec<f64> = (0..batch_size)
                .map(|_| rng.gen_range(-1000.0..1000.0))
                .collect();

            naive.append(&values);
            optimized.append(&values);

            for i in 1..=8 {
                // Compare stats after each batch
                let stats_naive = naive.calculate_stats(i);
                let stats_optimized = optimized.calculate_stats(i);

                assert_f64_near!(stats_naive.last, stats_optimized.last);
                assert_f64_near!(stats_naive.min, stats_optimized.min);
                assert_f64_near!(stats_naive.max, stats_optimized.max);
                assert_float_relative_eq!(stats_naive.avg, stats_optimized.avg, 0.00000001);
                assert_float_relative_eq!(stats_naive.var, stats_optimized.var, 0.00000001);
            }
        }
    }
}
