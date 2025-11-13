use json_to_xml::generate_xml::json_to_xml;

use std::fs;

fn main() {
    let json_string = fs::read_to_string("file_forestpropertydata2.json").expect("Could not read the JSON file");
    let json_value: serde_json::Value = serde_json::from_str(&json_string).unwrap();
    let file_new_xml = json_to_xml(&json_value, "People");

    // Save the new XML content to files
    std::fs::write("file_back_to_xml.xml", &file_new_xml).expect("Unable to write data");
}
