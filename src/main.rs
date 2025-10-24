mod forecast;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path("Forecast_Dashboard_-_Details.csv")?;
    rdr.records().next();
    rdr.records().next();
    println!(
        "{:90}{:>15}{:>15}{:>15}{:>15}{:>15}{:>15}",
        "Customer", "Nov 25", "Dec 25", "Jan 26", "Feb 26", "Mar 26", "Apr 26"
    );
    println!("{}", "-".repeat(180));
    for result in rdr.deserialize() {
        let record: forecast::ForecastRow = result?;
        println!(
            "{:90}{:>15.2}{:>15.2}{:>15.2}{:>15.2}{:>15.2}{:>15.2}",
            record.project_name,
            record.month1_labor_revenue_commit,
            record.month2_labor_revenue_commit,
            record.month3_labor_revenue_commit,
            record.month4_labor_revenue_commit,
            record.month5_labor_revenue_commit,
            record.month6_labor_revenue_commit
        );
    }
    Ok(())
}

fn main() {
    let err = run();
    if let Err(e) = err {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
