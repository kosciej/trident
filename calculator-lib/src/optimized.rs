//! Optimized implementation of streaming statistics calculator
//!
//! This implementation uses specialized data structures to maintain:
//! - O(1) time complexity for statistical calculations
//! - O(1) amortized time for value insertion/eviction
// - - Incremental sum and sum of squares tracking
//! - Monotonic deques for min/max tracking

use std::collections::VecDeque;
use std::f64;

use crate::StatsResponse;

use super::Calculator;

/// Optimized streaming statistics calculator with multiple window sizes
///
/// Maintains parallel calculators for different window sizes (10^k)
/// to enable O(1) lookups of statistics for any supported k value.
pub struct OptimizedCalculator {
    calculators: Vec<InnerCalc>,
}

impl OptimizedCalculator {
    pub fn new(k_capacity: u8) -> Self {
        let calculators = (1..=k_capacity)
            .map(|k| InnerCalc::new(10_usize.pow(k.into())))
            .collect();
        OptimizedCalculator { calculators }
    }
}

impl Calculator for OptimizedCalculator {
    fn append(&mut self, values: &[f64]) {
        self.calculators.iter_mut().for_each(|c| c.append(values));
    }

    fn calculate_stats(&self, k: u8) -> crate::StatsResponse {
        let i: &InnerCalc = &self.calculators[(k - 1) as usize];
        i.calculate_stats()
    }
}

/// Inner calculator maintaining statistics for a specific window size
#[derive(Clone)]
pub struct InnerCalc {
    /// Circular buffer of values
    buffer: VecDeque<f64>,
    /// Monotonic deque for emax tracking
    max_deque: VecDeque<f64>,
    /// Monotonic deque for min tracking
    min_deque: VecDeque<f64>,
    /// Running sum of values in buffer
    sum: f64,
    /// Running sum of squared values in buffer
    sum_sq: f64,
    /// Maximum number of values to retain
    capacity: usize,
}

impl InnerCalc {
    pub fn new(capacity: usize) -> Self {
        InnerCalc {
            buffer: VecDeque::with_capacity(capacity),
            max_deque: VecDeque::with_capacity(capacity),
            min_deque: VecDeque::with_capacity(capacity),
            sum: 0.0,
            sum_sq: 0.0,
            capacity,
        }
    }
}

impl InnerCalc {
    /// Add values while maintaining statistical aggregates in O(1) amortized time
    ///
    /// # Example
    /// ```
    /// use calculator_lib::optimized::InnerCalc;
    ///
    /// let mut calc = InnerCalc::new(100);
    /// calc.append(&[1.0, 2.0, 3.0]);
    /// ```
    fn append(&mut self, values: &[f64]) {
        for v in values {
            // Maintain buffer capacity by evicting oldest values
            if self.buffer.len() == self.capacity {
                let minus = self.buffer.pop_back().unwrap();
                self.sum -= minus;
                self.sum_sq -= minus.powi(2);

                //min
                if *self.min_deque.front().unwrap() == minus {
                    self.min_deque.pop_front();
                }

                //max
                if *self.max_deque.front().unwrap() == minus {
                    self.max_deque.pop_front();
                }
            }

            self.buffer.push_front(*v);
            self.sum += *v;
            self.sum_sq += v.powi(2);

            // Maintain min deque (ascending order)
            while let Some(&back) = self.min_deque.back() {
                if back > *v {
                    self.min_deque.pop_back();
                } else {
                    break;
                }
            }
            self.min_deque.push_back(*v);

            // Maintain max deque (descending order)
            while let Some(&back) = self.max_deque.back() {
                if back < *v {
                    self.max_deque.pop_back();
                } else {
                    break;
                }
            }
            self.max_deque.push_back(*v);
        }
    }

    /// Calculates stats in O(1) time
    ///
    /// # Example
    /// ```
    /// use calculator_lib::optimized::InnerCalc;
    ///
    /// let mut calc = InnerCalc::new(100);
    /// calc.append(&[1.0, 2.0, 3.0]);
    /// calc.calculate_stats()
    /// ```
    fn calculate_stats(&self) -> crate::StatsResponse {
        if self.buffer.is_empty() {
            return StatsResponse::default();
        }

        let count = self.buffer.len();
        let last: f64 = *self.buffer.front().unwrap_or(&0.0);
        let avg = self.sum / (count as f64);
        let min = *self.min_deque.front().unwrap_or(&0.0);
        let max = *self.max_deque.front().unwrap_or(&0.0);
        let var = (self.sum_sq / (count as f64)) - (avg.powi(2));

        StatsResponse {
            min,
            max,
            last,
            avg,
            var,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;

    #[test]
    fn test_naive_calculator_10_3() {
        let mut calc = OptimizedCalculator::new(3);
        calc.append(&[1.0, 2.0, 3.0]);
        let stats = calc.calculate_stats(1);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 3.0);
        assert_eq!(stats.last, 3.0);
        assert_eq!(stats.avg, 2.0);
        assert_f64_near!(stats.var, 0.6666666666666666);
    }

    #[test]
    fn test_naive_calculator_1_mln_elements() {
        let mut calc = OptimizedCalculator::new(8);
        let mut values = Vec::new();
        for i in 0..1000000 {
            values.push(i as f64);
        }
        calc.append(&values);
        let stats = calc.calculate_stats(6);
        assert_eq!(stats.min, 0.0);
        assert_eq!(stats.max, 999999.0);
        assert_f64_near!(stats.last, 999999.0);
        assert_f64_near!(stats.avg, 499999.5);
        assert_f64_near!(stats.var, 83333333332.87756);
    }
}
