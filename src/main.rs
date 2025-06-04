mod event;

use std::{fs::File, io::BufReader};
use csv::ReaderBuilder;
use event::Event;

fn read_csv_file(file_path: &str) -> Result<Vec<csv::StringRecord>, csv::Error> {
    let file = File::open(file_path)?;
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(BufReader::new(file));

    let mut records = Vec::new();
    for result in reader.records() {
        records.push(result?);
    }
    Ok(records)
}

fn main() {
    let file_path = "./data/raw/events_2025-05-30.csv";
    let records = read_csv_file(file_path)
        .expect("Failed to read CSV file");
    for record in records {
        let event = Event::from_csv_record(&record);
        match event {
            Ok(event) => println!("{:?}", event),
            Err(e) => eprintln!("Error parsing record: {}", e),
        }
    }
}
