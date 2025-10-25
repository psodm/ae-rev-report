mod data_operations;
mod executive;
mod forecast;

use crate::data_operations::DataOperations;

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut csv_data = data_operations::CsvDataOperations {
        forecast_file_path: "./test_data/Forecast_Dashboard_-_Details.csv".to_string(),
        executive_file_path: "./test_data/Unsaved_24702205.csv".to_string(),
        forecast_rows: Vec::new(),
        executive_projects: Vec::new(),
    };
    let _ = csv_data.load_data();

    println!(
        "{:90}{:>15}{:>15}{:>15}{:>15}{:>15}{:>15}",
        "Customer", "Nov 25", "Dec 25", "Jan 26", "Feb 26", "Mar 26", "Apr 26"
    );
    println!("{}", "-".repeat(180));
    for forecast in &csv_data.forecast_rows {
        println!(
            "{:90}{:>15.2}{:>15.2}{:>15.2}{:>15.2}{:>15.2}{:>15.2}",
            forecast.project_name,
            forecast.month1_labor_revenue_commit,
            forecast.month2_labor_revenue_commit,
            forecast.month3_labor_revenue_commit,
            forecast.month4_labor_revenue_commit,
            forecast.month5_labor_revenue_commit,
            forecast.month6_labor_revenue_commit
        );
    }
    println!("\nExecutive Projects:");
    println!("{:15}{}", "Project ID", "Account Executive");
    println!("{}", "-".repeat(40));
    for project in &csv_data.executive_projects {
        println!("{:15}{}", project.project_id, project.account_executive);
    }
    let executives = csv_data.get_list_of_executives();
    for exec in executives {
        println!("{}", exec);
        let projects =
            executive::get_projects_by_executive(exec.clone(), &csv_data.executive_projects);
        for project_id in projects {
            println!("  - {}", project_id);
        }
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
