use csv::Writer;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CompanyDonation {
    date_of_purchase: String,
    purchaser_name: String,
    denomination: String,
}

pub fn test_fn() -> Result<(), Box<dyn Error>> {
    // let xml_data = r#"<root>
    //                     <p>Date of Purchase</p>
    //                     <p>Purchaser Name</p>
    //                     <p>Denomination</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>A B C INDIA LIMITED</p>
    //                     <p>1,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>A B C INDIA LIMITED</p>
    //                     <p>1,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>A B C INDIA LIMITED</p>
    //                     <p>10,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>INNOCENT MERCHANDISE PVT LTD</p>
    //                     <p>1,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>INNOCENT MERCHANDISE PVT LTD</p>
    //                     <p>1,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>INNOCENT MERCHANDISE PVT LTD</p>
    //                     <p>10,00,000</p>
    //                     <h1 style="page-break-before:always; "></h1>
    //                     <p>12/Apr/2019</p>
    //                     <p>INNOCENT MERCHANDISE PVT LTD</p>
    //                     <p>1,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>INNOCENT MERCHANDISE PVT LTD</p>
    //                     <p>1,00,000</p>
    //                     <p>12/Apr/2019</p>
    //                     <p>INNOCENT MERCHANDISE PVT LTD</p>
    //                     <p>10,00,000</p>
    //                 </root>"#;

    let mut file = File::open("files/donors_list_clean.xml")?;
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data)?;

    let mut reader = Reader::from_str(&xml_data);
    // let mut reader = Reader::from_str(xml_data);
    reader.trim_text(true);

    let mut company_donations: Vec<CompanyDonation> = Vec::new();
    let mut count = 0;

    let mut donation = CompanyDonation {
        date_of_purchase: String::new(),
        purchaser_name: String::new(),
        denomination: String::new(),
    };
    let mut paragraph_index = 0;
    loop {
        match reader.read_event() {
            Ok(Event::Start(e)) if e.name().as_ref() == b"p" => {
                let txt = reader
                    .read_text(e.name())
                    .expect("cannot decode text value");
                // println!("{:?}", txt);
                match paragraph_index {
                    0 => donation.date_of_purchase = txt.into_owned(),
                    1 => donation.purchaser_name = txt.into_owned(),
                    2 => {
                        donation.denomination = txt.into_owned();
                        // Add donation to the vector every three paragraphs
                        company_donations.push(donation.clone());
                        count += 1;
                    }
                    _ => {}
                }
                paragraph_index = (paragraph_index + 1) % 3;
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    // Add donation to the vector if it's not yet added (for the last set of paragraphs)
    if paragraph_index != 0 {
        company_donations.push(donation);
        count += 1;
    }

    println!("Total company donations processed: {}", count);
    // for donation in &company_donations {
    //     println!("{:?}", donation);
    // }

    // Write data to CSV
    let mut csv_writer = Writer::from_path("company_donations.csv")?;
    csv_writer.write_record(&["Date of Purchase", "Purchaser Name", "Denomination"])?;
    for donation in &company_donations {
        csv_writer.write_record(&[
            &donation.date_of_purchase,
            &donation.purchaser_name,
            &donation.denomination,
        ])?;
    }
    csv_writer.flush()?;

    // Write data to JSON
    let json_data = serde_json::to_string_pretty(&company_donations)?;
    let mut json_file = File::create("company_donations.json")?;
    json_file.write_all(json_data.as_bytes())?;

    Ok(())
}
