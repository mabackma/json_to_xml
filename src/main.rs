use json_to_xml::generate_xml::json_to_xml;

use std::fs;

/* fn main() {
    let json_string = fs::read_to_string("file_forestpropertydata.json").expect("Could not read the JSON file");
    let file_new_xml = json_to_xml(&json_string, "ForestPropertyData");

    // Save the new XML content to files
    std::fs::write("file_back_to_xml.xml", &file_new_xml).expect("Unable to write data");
} */

fn main() {
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
}
