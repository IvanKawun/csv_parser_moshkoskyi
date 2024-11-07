use pest::Parser;
use pest_derive::Parser;
use std::fs;
#[derive(Parser)]
#[grammar = "gramatyka.pest"] // Ваша граматика
pub struct CSVParser;

fn main() {
    let file_content = fs::read_to_string("C:/Haskell/parser_moshkovskyi_ivan/src/data.csv").expect("Unable to read file");
    
    match CSVParser::parse(Rule::csv, &file_content) {
        Ok(parsed) => {
            for record in parsed {
                println!("{:?}", record);
            }
        }
        Err(e) => println!("Parsing failed: {}", e),
    }
}