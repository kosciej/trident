use dashmap::DashMap;
use std::sync::Arc;

use calculator_lib::{optimized, Calculator};

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

    pub fn stats(&self, symbol: &str, k: u32) -> Option<crate::models::StatsResponse> {
        self.calculators
            .get_mut(symbol)
            .map(|mut entry| entry.value_mut().calculate_stats(k as u8).into())
    }
}

#[cfg(test)]
mod tests {
    use assert_float_eq::assert_f64_near;

    use super::*;
    use std::sync::Barrier;

    #[test]
    fn concurrent_appends_and_stats() {
        let state = new();
        let symbols = vec!["A".to_string(), "B".to_string()];
        let thread_count = 10;
        let barrier = Arc::new(Barrier::new(thread_count * symbols.len()));

        let handles: Vec<_> = symbols
            .iter()
            .flat_map(|symbol| {
                (0..thread_count)
                    .map(|i| {
                        let state = state.clone();
                        let symbol = symbol.clone();
                        let barrier = barrier.clone();
                        std::thread::spawn(move || {
                            barrier.wait();
                            let values = vec![i as f64, (i + 1) as f64];
                            state.append(symbol, &values);
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for handle in handles {
            handle.join().unwrap();
        }

        for symbol in &symbols {
            let stats = state.stats(symbol, 2).unwrap();
            assert_f64_near!(stats.avg, 5.0);
        }
    }
}
