use json_to_xml::generate_xml::json_to_xml_with_root;

use std::fs;

fn main() {
    let json = r#"{
        "@version": "2.0",
        "@xmlns:corp": "http://example.com/corp",
        "Clients": [
            {
                "Name": "AlphaCorp",
                "Projects": [
                    { "@priority": "high", "ProjectName": "AlphaOne", "Budget": 100000 },
                    { "ProjectName": "AlphaTwo", "Budget": null }
                ]
            },
            {
                "Name": "BetaCorp",
                "Projects": []
            },
            "UnknownClient"
        ],
        "Platform": {
            "Metrics": {
                "Uptime": 99.99,
                "Requests": [1200, 1350, 1425, null],
                "Errors": []
            },
            "Services": [
                "Compute",
                {
                    "@deprecated": "true",
                    "Name": "Storage",
                    "Instances": [
                        { "Type": "S3", "Count": 100 },
                        { "Type": "Block", "Count": 50, "Options": ["fast", "redundant"] }
                    ]
                },
                "LegacyService"
            ],
            "Teams": [
                {
                    "TeamName": "DevOps",
                    "Members": ["Alice", "Bob", "Charlie"]
                },
                {
                    "TeamName": "QA",
                    "Members": []
                }
            ]
        },
        "Notes": []
    }"#;

    let xml_string = json_to_xml_with_root(&json, "Posts").unwrap();

    fs::write("examples/large_json_output.xml", xml_string.as_bytes()).expect("Failed to write XML file");
    println!("{}", xml_string);
}