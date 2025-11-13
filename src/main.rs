use json_to_xml::generate_xml::json_to_xml;

use std::fs;

fn main() {
    let json_string = fs::read_to_string("file_forestpropertydata.json").expect("Could not read the JSON file");
    let file_new_xml = json_to_xml(&json_string, "ForestPropertyData");

    // Save the new XML content to files
    std::fs::write("file_back_to_xml.xml", &file_new_xml).expect("Unable to write data");
}
