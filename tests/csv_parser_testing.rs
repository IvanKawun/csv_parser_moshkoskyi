use csv_parser_moshkovskyi::Grammar;
use pest::Parser;
use csv_parser_moshkovskyi::Rule;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_csv() {
        let file_content = r#"
            "name","age","city"
            "John Doe",30,"New York"
            "Jane Smith",25,"Los Angeles"
            "Sam Brown",22,"Chicago"
        "#;

        let parsed = Grammar::parse(Rule::csv, file_content);
        
        assert!(parsed.is_ok());
        let parsed = parsed.unwrap();

        assert_eq!(parsed.len(), 4);
    }

    #[test]
    fn test_invalid_csv() {
        let file_content = r#"
            "name","age","city"
            "John Doe",30,"New York"
            "Jane Smith",25
            "Sam Brown",22,"Chicago"
        "#;

        let parsed = Grammar::parse(Rule::csv, file_content);
        assert!(parsed.is_err());
    }
}