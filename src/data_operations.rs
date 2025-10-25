pub trait DataOperations {
    fn load_data(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn get_list_of_executives(&self) -> Vec<String>;
}

pub struct CsvDataOperations {
    pub forecast_file_path: String,
    pub executive_file_path: String,
    pub forecast_rows: Vec<super::forecast::ForecastRow>,
    pub executive_projects: Vec<super::executive::ExecutiveProject>,
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
        super::executive::get_list_of_executives(&self.executive_projects)
    }
}

fn read_forecast_data(rdr: &mut csv::Reader<std::fs::File>) -> Vec<super::forecast::ForecastRow> {
    skip_header(rdr);
    rdr.deserialize().filter_map(|result| result.ok()).collect()
}

fn read_executive_data(
    rdr: &mut csv::Reader<std::fs::File>,
) -> Vec<super::executive::ExecutiveProject> {
    skip_header(rdr);
    rdr.deserialize().filter_map(|result| result.ok()).collect()
}

fn skip_header(rdr: &mut csv::Reader<std::fs::File>) {
    rdr.records().next();
    rdr.records().next();
}
