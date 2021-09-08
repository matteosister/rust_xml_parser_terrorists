use quick_xml::de::from_str;
use serde::Deserialize;
use std::io::Read;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename = "ArrayOfConsolidatedList")]
struct Individuals {
    #[serde(rename(deserialize = "ConsolidatedList"))]
    individuals: Vec<Individual>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Individual {
    name1: String,
    name2: String,
    name3: String,
    name4: String,
    name5: String,
    #[serde(rename(deserialize = "Name6"))]
    name6: String,
    #[serde(rename(deserialize = "FullName"))]
    full_name: String,
    #[serde(rename(deserialize = "OtherInformation"))]
    other_info: String,
    #[serde(rename(deserialize = "DateOfBirth"))]
    date: String,
}

fn main() {
    let mut xml = String::new();
    let mut file = std::fs::File::open("ConList.xml").unwrap();
    file.read_to_string(&mut xml).unwrap();

    let list: Individuals = from_str(xml.as_str()).unwrap();
    dbg!(list);
}
