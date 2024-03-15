use csv::Writer;
use quick_xml::events::Event;
use quick_xml::reader::Reader;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PartyEncashment {
    date_of_encashment: String,
    political_party: String,
    denomination: String,
}

pub fn political_party_encashment() -> Result<(), Box<dyn Error>> {
    let mut file = File::open("files/political_party_clean.xml")?;
    let mut xml_data = String::new();
    file.read_to_string(&mut xml_data)?;

    let mut reader = Reader::from_str(&xml_data);
    // let mut reader = Reader::from_str(xml_data);
    reader.trim_text(true);

    let mut party_encashment: Vec<PartyEncashment> = Vec::new();
    let mut count = 0;

    let mut donation = PartyEncashment {
        date_of_encashment: String::new(),
        political_party: String::new(),
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
                    0 => {
                        let parts: Vec<&str> = txt.split_whitespace().collect();
                        let date = parts[0];
                        let party_name = parts[1..].join(" ");
                        donation.date_of_encashment = date.to_owned();
                        donation.political_party = party_name.to_owned();
                    }
                    1 => {
                        donation.denomination = txt
                            .into_owned()
                            .trim()
                            .trim()
                            .replace(',', "")
                            .parse()
                            .unwrap();

                        // Add donation to the vector every three paragraphs
                        party_encashment.push(donation.clone());
                        count += 1;
                    }

                    _ => {}
                }
                paragraph_index = (paragraph_index + 1) % 2;
            }
            Ok(Event::Eof) => break,
            Err(e) => panic!("Error at position {}: {:?}", reader.buffer_position(), e),
            _ => (),
        }
    }

    // Add donation to the vector if it's not yet added (for the last set of paragraphs)
    if paragraph_index != 0 {
        party_encashment.push(donation);
        count += 1;
    }

    println!("Total company donations processed: {}", count);
    // for donation in &party_encashment {
    //     println!("{:?}", donation);
    // }

    // Write data to CSV
    let mut csv_writer = Writer::from_path("output/party_encashment.csv")?;
    csv_writer.write_record(&["Date of Encashment", "Political Party", "Denomination"])?;
    for donation in &party_encashment {
        csv_writer.write_record(&[
            &donation.date_of_encashment,
            &donation.political_party,
            &donation.denomination,
        ])?;
    }
    csv_writer.flush()?;

    // Write data to JSON
    let json_data = serde_json::to_string_pretty(&party_encashment)?;
    let mut json_file = File::create("output/party_encashment.json")?;
    json_file.write_all(json_data.as_bytes())?;

    Ok(())
}
