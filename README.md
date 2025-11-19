# Convert JSON to XML with a default "Root" element.

This is a convenience function that calls `json_to_xml_with_root` with "Root" as the default root element name.

# Example

```rust
use json_to_xml::generate_xml::json_to_xml;

let json_string = r#"
{
    "book": {
        "@isbn": "978-3-16-148410-0",
        "title": "The Rust Programming Language",
        "author": "Steve Klabnik and Carol Nichols"
    }
}
"#;

let xml_string = json_to_xml(&json_string).unwrap();

println!("{}", xml_string);
```

## Expected Output (XML):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!--Generated with json_to_xml 0.1.7-->
<Root>
  <Book isbn="978-3-16-148410-0">
    <Author>Steve Klabnik and Carol Nichols</Author>
    <Title>The Rust Programming Language</Title>
  </Book>
</Root>
```

# Convert JSON to XML with a custom root element.

This function takes a JSON string and a specified root element name and converts it into an XML string.
It processes JSON objects, arrays, and primitive values recursively.
Attributes in JSON (prefixed with `@`) are converted to XML attributes.
All XML tags are capitalized.

# Example

```rust
use json_to_xml::generate_xml::json_to_xml_with_root;

let json_string = r#"
{
    "@xmlns:addr": "http://standards.fi/schemas/personData/addresses",
    "@xmlns:pr": "http://standards.fi/schemas/personData/person",
    "person": {
        "name": "John Doe",
        "age": "30",
        "@id": "1234",
        "addresses": [
            {
                "street": "123 Main St",
                "city": "Springfield",
                "@type": "primary"
            },
            {
                "street": "456 Oak Ave",
                "city": "Shelbyville",
                "@type": "secondary"
            }
        ]
    }
}
"#;

let xml_string = json_to_xml_with_root(&json_string, "People").unwrap();

println!("{}", xml_string);
```

## Expected Output (XML):

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!--Generated with json_to_xml 0.1.7-->
<People xmlns:pr="http://standards.fi/schemas/personData/person" xmlns:addr="http://standards.fi/schemas/personData/addresses">
  <Person id="1234">
    <Addresses type="primary">
      <City>Springfield</City>
      <Street>123 Main St</Street>
    </Addresses>
    <Addresses type="secondary">
      <City>Shelbyville</City>
      <Street>456 Oak Ave</Street>
    </Addresses>
    <Age>30</Age>
    <Name>John Doe</Name>
  </Person>
</People>
```

## Parameters:
- `json_string`: The input JSON string to be converted into XML.
- `root`: The name for the root element of the XML. This is especially important if the top-level JSON object contains attributes.

## Returns:
A `Result` which is either a `String` containing the XML representation of the input JSON, or a `ConversionError` if parsing or conversion fails.

## Notes:
- This function works recursively to handle nested structures and arrays.
- JSON keys starting with `@` are treated as attributes for the parent XML element.
- All XML element tags are automatically capitalized.
- Empty JSON objects (`{}`) are converted into self-closing tags (e.g., `<Tag/>`).
- Empty JSON arrays (`[]`) are converted into an empty element (e.g., `<Tag></Tag>`).
- `null` values in JSON are converted into a self-closing `<None/>` tag.