# CSV_PARSER_MOSHKOVSKYI
Realisation of basic csv parser
# Technical description
This parser is parsing a csv file.
He will parse file, show it in structured way and check file on correctnes.
# Usage
The result can be used for basic data analytics
# Grammar
WHITESPACE = _{ " " | "\t" | "\n" | "\r" }
NEWLINE = _{ "\n" | "\r\n" }
csv = { (WHITESPACE | NEWLINE)* ~ record ~ (NEWLINE ~ record)* ~ (WHITESPACE | NEWLINE)* }
record = { field ~ ("," ~ field)* }
field = { empty_field | quoted_field | unquoted_field }
empty_field = _{ "," }
quoted_field = _{ "\"" ~ (!"\"" ~ ANY | "\"" ~ "\"")* ~ "\"" }
unquoted_field = _{ (!("," | NEWLINE | " ") ~ ANY)+ }