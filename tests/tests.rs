use std::{fs, io::Write};

use track2kml::{write_as_kml, WriteAsKmlOptions};

#[test]
fn read_courageous() {
    let file: courageous_format::Document =
        serde_json::de::from_str(include_str!("test_data.json")).unwrap();

    let mut res = Vec::new();
    write_as_kml(file, &mut res, WriteAsKmlOptions::default()).unwrap();
    // This doesn't actually check if the resulting KML is correct
}
