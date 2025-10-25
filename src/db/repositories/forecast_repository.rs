use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::models::forecast::ForecastRow;

trait ForecastRepository {
    fn insert(&self, forecast: ForecastRow);
    fn find_all(&self) -> Vec<ForecastRow>;
    fn find_by_id(&self, project_id: &str) -> Option<ForecastRow>;
}

pub struct InMemoryForecastRepository {
    data: Arc<RwLock<HashMap<String, ForecastRow>>>,
}

impl InMemoryForecastRepository {
    pub fn new() -> Self {
        InMemoryForecastRepository {
            data: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn insert(&self, forecast: ForecastRow) {
        let mut data = self.data.write().unwrap();
        data.insert(forecast.project_id.clone(), forecast);
    }

    pub fn find_all(&self) -> Vec<ForecastRow> {
        let data = self.data.read().unwrap();
        data.values().cloned().collect()
    }

    pub fn find_by_id(&self, project_id: &str) -> Option<ForecastRow> {
        let data = self.data.read().unwrap();
        data.get(project_id).cloned()
    }
}
