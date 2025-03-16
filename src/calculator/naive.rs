use std::collections::VecDeque;
use std::f64;

use crate::models::StatsResponse;

use super::Calculator;

pub struct NaiveCalculator {
    buffer: VecDeque<f64>,
    capacity: usize,
}

impl NaiveCalculator {
    pub fn new(capacity: usize) -> Self {
        NaiveCalculator {
            buffer: VecDeque::with_capacity(capacity),
            capacity,
        }
    }
}

impl Calculator for NaiveCalculator {
    fn append(&mut self, values: &[f64]) {
        for i in values {
            if self.buffer.len() == self.capacity {
                self.buffer.pop_back();
            }
            self.buffer.push_front(*i);
        }
    }

    fn calculate_stats(&self, k: u8) -> crate::models::StatsResponse {
        if self.buffer.is_empty() {
            return StatsResponse::default();
        }

        let mut window: usize = 10_usize.pow(k.into());
        let max_window_size = self.buffer.len();
        window = std::cmp::min(window, max_window_size);

        let window_iter = self.buffer.iter().take(window);

        let mut min = f64::MAX;
        let mut max = f64::MIN;
        let mut sum = 0.0;
        let last: f64 = *self.buffer.front().unwrap_or(&0.0);
        let mut count = 0;

        for &value in window_iter {
            min = min.min(value);
            max = max.max(value);
            sum += value;
            count += 1;
        }

        if count == 0 {
            return StatsResponse::default();
        }

        let avg = sum / (count as f64);
        let mut variance_sum = 0.0;
        for &value in self.buffer.iter().take(window) {
            variance_sum += (value - avg) * (value - avg);
        }
        let var = variance_sum / (count as f64);

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

    #[test]
    fn test_naive_calculator_buffer_overflow() {
        let mut calc = NaiveCalculator::new(3);
        calc.append(&[1.0, 2.0, 3.0, 4.0]);
        assert_eq!(calc.buffer.len(), 3);
        assert_eq!(calc.buffer[0], 4.0);
        assert_eq!(calc.buffer[1], 3.0);
        assert_eq!(calc.buffer[2], 2.0);
    }

    #[test]
    fn test_naive_calculator_1000_params() {
        let mut calc = NaiveCalculator::new(10);
        let mut values = Vec::new();
        for i in 0..1000 {
            values.push(i as f64);
        }
        calc.append(&values);
        assert_eq!(calc.buffer.len(), 10);
        assert_eq!(calc.buffer[0], 999.0);
        assert_eq!(calc.buffer[9], 990.0);
    }

    #[test]
    fn test_naive_calculator_10_3() {
        let mut calc = NaiveCalculator::new(3);
        calc.append(&[1.0, 2.0, 3.0]);
        let stats = calc.calculate_stats(1);
        assert_eq!(stats.min, 1.0);
        assert_eq!(stats.max, 3.0);
        assert_eq!(stats.last, 3.0);
        assert_eq!(stats.avg, 2.0);
        assert_eq!(stats.var, 0.6666666666666666);
    }

    #[test]
    fn test_naive_calculator_1_mln_elements() {
        let mut calc = NaiveCalculator::new(1000000);
        let mut values = Vec::new();
        for i in 0..1000000 {
            values.push(i as f64);
        }
        calc.append(&values);
        let stats = calc.calculate_stats(6);
        assert_eq!(stats.min, 0.0);
        assert_eq!(stats.max, 999999.0);
        assert_eq!(stats.last, 999999.0);
        assert_eq!(stats.avg, 499999.5);
        assert_eq!(stats.var, 83333333333.91629);
    }
}
