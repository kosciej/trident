use dashmap::DashMap;
use std::sync::Arc;

use calculator_lib::{optimized, Calculator, StatsResponse};

#[derive(Clone)]
pub struct AppState {
    calculators: Arc<DashMap<String, Box<dyn Calculator + Send + Sync>>>,
}

pub fn new() -> AppState {
    AppState {
        calculators: Arc::new(DashMap::new()),
    }
}

impl AppState {
    pub fn append(&self, symbol: String, values: &[f64]) {
        self.calculators
            .entry(symbol)
            .and_modify(|calc| calc.append(values))
            .or_insert_with(|| {
                let mut calc = optimized();
                calc.append(values);
                Box::new(calc)
            });
    }

    pub fn stats(&self, symbol: &str, k: u32) -> Option<StatsResponse> {
        self.calculators
            .get_mut(symbol)
            .map(|mut entry| entry.value_mut().calculate_stats(k as u8))
    }
}
