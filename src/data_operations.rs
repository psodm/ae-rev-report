use super::models::{executive, forecast};

pub trait DataOperations {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_list_of_executives(&self) -> Vec<String>;
}

pub struct CsvDataOperations {
    pub forecast_file_path: String,
    pub executive_file_path: String,
    pub forecast_rows: Vec<forecast::ForecastRow>,
    pub executive_projects: Vec<executive::ExecutiveProject>,
}

impl DataOperations for CsvDataOperations {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Load forecast data
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.forecast_file_path)?;
        self.forecast_rows = read_forecast_data(&mut rdr);
        let mut rdr = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_path(&self.executive_file_path)?;
        self.executive_projects = read_executive_data(&mut rdr);
        Ok(())
    }
    fn get_list_of_executives(&self) -> Vec<String> {
        executive::get_list_of_executives(&self.executive_projects)
    }
}

fn read_forecast_data(rdr: &mut csv::Reader<std::fs::File>) -> Vec<forecast::ForecastRow> {
    skip_header(rdr);
    rdr.deserialize().filter_map(|result| result.ok()).collect()
}

fn read_executive_data(rdr: &mut csv::Reader<std::fs::File>) -> Vec<executive::ExecutiveProject> {
    skip_header(rdr);
    rdr.deserialize().filter_map(|result| result.ok()).collect()
}

fn skip_header(rdr: &mut csv::Reader<std::fs::File>) {
    rdr.records().next();
    rdr.records().next();
}
