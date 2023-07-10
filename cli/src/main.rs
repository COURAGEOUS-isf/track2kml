use std::{
    collections::{hash_map::RandomState, HashMap},
    process::ExitCode,
};

use track2kml_cli::{CourageousInitializer, Format};

fn main() -> ExitCode {
    let formats: HashMap<&str, Format, RandomState> = {
        let arr = [(
            "courageous-v0.2",
            Format::new(CourageousInitializer, "json", "COURAGEOUS (v0.2)"),
        )];

        HashMap::from_iter(arr.into_iter())
    };

    track2kml_cli::run(&formats, false)
}
