pub mod naive;
mod optimized;

pub trait Calculator {
    fn append(&mut self, values: &[f64]);
    fn calculate_stats(&self, k: u8) -> crate::models::StatsResponse;
}

pub fn naive() -> impl Calculator {
    naive::NaiveCalculator::new(10_usize.pow(8))
}

pub fn optimized() -> impl Calculator {
    optimized::OptimizedCalculator::new(8)
}