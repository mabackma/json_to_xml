# JSON to XML

## Parameters

- `json_string`: The input JSON string to be converted into XML. It can contain objects, arrays, and strings.  
- `root`: The name of the root. It will become the root element of the XML if the JSON contains top-level `@` attributes. 

## Returns

A string containing the XML representation of the input JSON, including necessary XML attributes.  

## Notes

- This function works recursively to handle nested structures and arrays.  
- Attributes are prefixed with `@` in the JSON input and are converted to XML attributes.  
- The order of attributes in the XML elements may differ.  
- The root start and end tags will included only if the top-level JSON object contains `@` attributes.

## Example

```rust
use json_to_xml::generate_xml::json_to_xml;

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

let xml_string = json_to_xml(&json_string, "People");

println!("{}", xml_string);
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