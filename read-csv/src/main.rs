use csv;
use std::error::Error;

fn read_csv_file_from_path(path: &str) -> Result<(), Box<dyn Error>> {
    let mut csv_reader = csv::Reader::from_path(path)?;
    for record in csv_reader.records() {
        println!("record:{:?}", record?);
    }
    Ok(())
}
fn main() {
    if let Err(e) = read_csv_file_from_path("./demo.csv") {
        eprintln!("err:{:?}", e)
    }
}
