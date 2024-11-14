use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;
<<<<<<< HEAD

#[derive(Parser)]
#[grammar = "gramatyka.pest"]
=======
use pest::iterators::Pair;


#[derive(Parser)]
#[grammar = "gramatyka.pest"] 
>>>>>>> a0e6417339afd214f12d9739459a7a535afde8f5
pub struct Grammar;

#[derive(Error, Debug)]

pub enum CsvParseError {
    #[error("ERror")]
    InvalidCsvStructure,

    #[error("Pest parsing error: {0}")]
    PestError(#[from] pest::error::Error<Rule>),
}

pub fn parse_csv(file_content: &str) -> Result<(), CsvParseError> {
<<<<<<< HEAD
    let parsed = Grammar::parse(Rule::csv, file_content).map_err(|e| {
        println!("Parsing error: {:?}", e);
        CsvParseError::PestError(e)
    })?;
=======
    let parsed = Grammar::parse(Rule::csv, file_content)
        .map_err(|e| {
            println!("Parsing error: {:?}", e);
            CsvParseError::PestError(e)
        })?;
>>>>>>> a0e6417339afd214f12d9739459a7a535afde8f5

    println!("{}", file_content);

    for csv_pair in parsed {
        for record in csv_pair.into_inner() {
            println!("{:?}", record);
            match record.as_rule() {
                Rule::record => {
                    let fields: Vec<&str> = record
                        .into_inner()
                        .map(|field| field.as_str().trim_matches('"'))
                        .collect();
<<<<<<< HEAD

=======
                    
>>>>>>> a0e6417339afd214f12d9739459a7a535afde8f5
                    println!("Record: {:?}", fields);
                }
                _ => return Err(CsvParseError::InvalidCsvStructure),
            }
        }
    }

<<<<<<< HEAD
=======

>>>>>>> a0e6417339afd214f12d9739459a7a535afde8f5
    Ok(())
}
