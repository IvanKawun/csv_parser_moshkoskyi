use pest::Parser;
use pest_derive::Parser;
use std::fs;
use csv_parser_moshkovskyi::parse_csv;

fn main() {
    let file_content = fs::read_to_string("src/data.csv").expect("Unable to read file");
    match parse_csv(&file_content) {
        Ok(_) => println!("CSV parsed successfully!"),
        Err(e) => eprintln!("Error parsing CSV: {}", e),
    }
    // match Grammar::parse(Rule::csv, &file_content) {
    //     Ok(parsed) => {
    //         for record in parsed {
    //             println!("{:?}", record);
    //         }
    //     }
    //     Err(e) => println!("Parsing failed: {}", e),
    // }
}