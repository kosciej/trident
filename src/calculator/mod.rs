pub mod naive;
mod proper;

pub trait Calculator {
    fn append(&mut self, values: &[f64]);
    fn calculate_stats(&self, k: u8) -> crate::models::StatsResponse;
}

pub fn naive(capacity: usize) -> impl Calculator {
    naive::NaiveCalculator::new(capacity)
}
