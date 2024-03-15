use polars::prelude::*;
use std::fs::File;

pub fn party_encashment() -> PolarsResult<()> {
    let df_csv = CsvReader::from_path("output/party_encashment.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    // println!("{}", df_csv);

    let df_cast = df_csv
        .clone()
        .lazy()
        .select([
            col("Date of Encashment"),
            col("Political Party"),
            col("Denomination").cast(DataType::Int64),
        ])
        .collect()?;
    // println!("{}", df_cast);
    let mut df_sum = df_cast
        .clone()
        .lazy()
        .group_by(["Political Party"])
        .agg([(col("Denomination").sum())])
        .sort(
            "Denomination",
            SortOptions {
                descending: true,
                nulls_last: true,
                ..Default::default()
            },
        )
        .collect()?;
    let mut file = File::create("output/political_party_encashment_agg.csv").expect("could not create file");
    CsvWriter::new(&mut file)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df_sum)?;

    let mut file = std::fs::File::create("output/political_party_encashment_agg.json").unwrap();

    // json
    JsonWriter::new(&mut file)
        .with_json_format(JsonFormat::JsonLines)
        .finish(&mut df_sum)
        .unwrap();

    println!("{}", df_sum);
    Ok(())
}
