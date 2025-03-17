use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use crate::{
    calculator::{optimized, Calculator},
    models::StatsResponse,
};

#[derive(Clone)]
pub struct AppState {
    calculators: Arc<Mutex<HashMap<String, Box<dyn Calculator + Send>>>>,
}

pub fn new() -> AppState {
    let calculators: HashMap<String, Box<dyn Calculator + Send>> = HashMap::new();

    AppState {
        calculators: Arc::new(Mutex::new(calculators)),
    }
}

impl AppState {
    pub fn append(&self, symbol: String, values: &[f64]) {
        let mut map = self.calculators.lock().expect("Poisoned mutex");
        map.entry(symbol)
            .and_modify(|e| e.append(values))
            .or_insert_with(|| {
                let mut calc = optimized();
                calc.append(values);
                Box::new(calc)
            });
    }

    pub fn stats(&self, symbol: &str, k: u32) -> Option<StatsResponse> {
        self.calculators
            .lock()
            .expect("Poisoned mutex")
            .get_mut(symbol)
            .map(|c| c.calculate_stats(k as u8))
    }
}
