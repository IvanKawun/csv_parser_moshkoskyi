use pest::Parser;
use pest_derive::Parser;
#[derive(Parser)]
#[grammar = "gramatyka.pest"] 
pub struct Grammar;