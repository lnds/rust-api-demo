use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Instant;

type Id = usize;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Record {
    pub value: u8,
    pub count: u64,
}

pub type DataBase = HashMap<Id, Record>;

#[derive(Debug, Deserialize)]
struct CsvRecord {
    id: usize,
    value: u8,
}

/// filename is a csv file
/// has a header row
pub fn load_data(filename: &str) -> DataBase {
    let start = Instant::now();
    println!("loading data from {}", filename);
    let mut data = HashMap::new();

    let mut reader = ReaderBuilder::new().from_path(filename).unwrap();
    for row in reader.deserialize() {
        let record: CsvRecord = row.unwrap();
        data.insert(
            record.id,
            Record {
                value: record.value,
                count: 0,
            },
        );
    }
    let end = Instant::now();
    println!("data loaded in {:?}", end.duration_since(start));
    data
}
