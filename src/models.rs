use serde::Deserialize;

#[derive(Deserialize)]
pub struct AddBatchRequest {
    pub symbol: String,
    pub values: Vec<f64>,
}

#[derive(Deserialize)]
pub struct StatsQuery {
    pub symbol: String,
    pub k: Option<u32>,
}

#[derive(serde::Serialize)]
pub struct StatsResponse {
    pub last: f64,
    pub min: f64,
    pub max: f64,
    pub avg: f64,
    pub var: f64,
}

impl From<calculator_lib::StatsResponse> for StatsResponse {
    fn from(item: calculator_lib::StatsResponse) -> Self {
        StatsResponse {
            last: item.last,
            min: item.min,
            max: item.max,
            avg: item.avg,
            var: item.var,
        }
    }
}
