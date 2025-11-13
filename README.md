# JSON to XML

## Example

```rust
use json_to_xml::generate_xml::json_to_xml;

let json_data = r#"
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

let xml_output = json_to_xml(&json_data, "People");

println!("{}", xml_output);
````

## Output

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!--Generated with json_to_xml 0.1.0-->
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