use csv::Writer;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::{self, Write};

use csv::ReaderBuilder;
use serde::Deserialize;
use std::error::Error;

// Define a struct to represent the data in CSV
#[derive(Debug, Deserialize)]
struct CompanyDonor {
    #[serde(rename = "Purchaser Name")]
    name: String,
    #[serde(rename = "Denomination")]
    denomination: u64,
}

#[derive(Debug, Serialize)]
struct CompanyDonorWithPercentage {
    name: String,
    denomination: u64,
    percentage: f64,
}

pub fn company_donation_parcentage() -> Result<(), Box<dyn Error>> {
    // Read data from CSV file
    let mut total_denomination: u64 = 0;
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("output/company_donation_agg.csv")?;
    let mut parties_with_percentage: Vec<CompanyDonorWithPercentage> = Vec::new();

    // Iterate through each row to calculate total_denomination
    for result in csv_reader.deserialize::<CompanyDonor>() {
        let record = result?;
        total_denomination += record.denomination;
    }

    println!("{}", total_denomination);

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("output/company_donation_agg.csv")?;

    // Iterate through each row to calculate percentage for each party
    for result in csv_reader.deserialize::<CompanyDonor>() {
        let mut record = result?;
        let percentage = (record.denomination as f64 / total_denomination as f64) * 100.0;
        parties_with_percentage.push(CompanyDonorWithPercentage {
            name: record.name.clone(),
            denomination: record.denomination,
            percentage,
        });
    }

    // Output as JSON
    let json_output = serde_json::to_string_pretty(&parties_with_percentage)?;
    let mut json_file = File::create("output/company_donors_denominations_with_percentage.json")?;
    json_file.write_all(json_output.as_bytes())?;

    Ok(())
}
