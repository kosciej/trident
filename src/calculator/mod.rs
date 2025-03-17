pub mod naive;
mod optimized;

pub trait Calculator {
    fn append(&mut self, values: &[f64]);
    fn calculate_stats(&self, k: u8) -> crate::models::StatsResponse;
}

#[allow(dead_code)]
pub fn naive() -> impl Calculator {
    naive::NaiveCalculator::new(10_usize.pow(8))
}

#[allow(dead_code)]
pub fn optimized() -> impl Calculator {
    optimized::OptimizedCalculator::new(8)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_float_eq::*;
    use rand::{Rng, SeedableRng, rngs::StdRng};


    #[test]
    #[ignore = "long-running comparison test"]
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
