use csv::{ReaderBuilder, StringRecord};

mod forecast;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("Forecast_Dashboard_-_Details.csv")?;
    rdr.records().next();
    rdr.records().next();
    println!("{:90}{:>15}{:>15}{:>15}{:>15}{:>15}{:>15}", "Customer", "Nov 25", "Dec 25", "Jan 26", "Feb 26", "Mar 26", "Apr 26");
    println!("{}", "-".repeat(180));
    for result in rdr.deserialize() {
        let record: forecast::ForecastRow = result?;
        println!("{:90}{:>15.2}{:>15.2}{:>15.2}{:>15.2}{:>15.2}{:>15.2}", record.projectName, record.month1LaborRevenueCommit, record.month2LaborRevenueCommit, record.month3LaborRevenueCommit, record.month4LaborRevenueCommit, record.month5LaborRevenueCommit, record.month6LaborRevenueCommit);
    };
    Ok(())
}

fn main() {
    let err = run();
    if let Err(e) = err {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
