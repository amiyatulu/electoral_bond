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
struct PoliticalParty {
    #[serde(rename = "Political Party")]
    name: String,
    #[serde(rename = "Denomination")]
    denomination: u64,
}

#[derive(Debug, Serialize)]
struct PoliticalPartyWithPercentage {
    name: String,
    denomination: u64,
    percentage: f64,
}

pub fn party_money_parcentage() -> Result<(), Box<dyn Error>> {
    // Read data from CSV file
    let mut total_denomination: u64 = 0;
    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("output/political_party_encashment_agg.csv")?;
    let mut parties_with_percentage: Vec<PoliticalPartyWithPercentage> = Vec::new();

    // Iterate through each row to calculate total_denomination
    for result in csv_reader.deserialize::<PoliticalParty>() {
        let record = result?;
        total_denomination += record.denomination;
    }

    println!("{}", total_denomination);

    let mut csv_reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path("output/political_party_encashment_agg.csv")?;

    // Iterate through each row to calculate percentage for each party
    for result in csv_reader.deserialize::<PoliticalParty>() {
        let mut record = result?;
        let percentage = (record.denomination as f64 / total_denomination as f64) * 100.0;
        parties_with_percentage.push(PoliticalPartyWithPercentage {
            name: record.name.clone(),
            denomination: record.denomination,
            percentage,
        });
    }

    // Output as JSON
    let json_output = serde_json::to_string_pretty(&parties_with_percentage)?;
    let mut json_file =
        File::create("output/party_enchashment_denominations_with_percentage.json")?;
    json_file.write_all(json_output.as_bytes())?;

    Ok(())
}
