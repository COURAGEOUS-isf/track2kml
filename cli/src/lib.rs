use clap::ArgMatches;
use std::{
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use colored::Colorize;
use textwrap::Options;

use courageous_format::{Document, Position3d};
use track2kml::{write_as_kml, WriteAsKmlOptions};

mod clap_util;

pub fn process_to_kml(args: &ArgMatches) -> Result<PathBuf, anyhow::Error> {
    let input_path: &PathBuf = args.get_one("input_path").unwrap();
    let database = read_input_file(args, input_path)?;
    let output_path = input_path.with_extension("kml");
    let output_file = BufWriter::new(File::create(&output_path)?);
    let disable_track_icons = args.get_flag("no_track_icons");
    let cuas_range = *args
        .try_get_one("cuas_range")
        .ok()
        .flatten()
        .unwrap_or(&100.);

    write_as_kml(
        database,
        output_file,
        WriteAsKmlOptions::default()
            .disable_track_icons(disable_track_icons)
            .cuas_range(cuas_range),
    )?;

    Ok(output_path)
}

fn read_input_file(
    args: &ArgMatches,
    input_path: &Path,
) -> Result<track2kml::Database, anyhow::Error> {
    if input_path.extension().as_ref() != Some(&OsStr::new("json")) {
        textwrap::wrap(
            &format!("Unexpected file extention. Expected JSON file.",),
            Options::new(80)
                .initial_indent(&format!("{}{} ", "Warning".yellow().bold(), ":".bold()))
                .subsequent_indent("    "),
        )
        .into_iter()
        .for_each(|line| println!("{}", line));
    }

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);
    let parser: Result<Document, anyhow::Error> =
        serde_json::from_reader(reader).map_err(anyhow::Error::from);
    match parser {
        Ok(mut database) => {
            if let Some(origin) = args.get_one::<Position3d>("detection_origin") {
                database.static_cuas_location = *origin;
            };
            return Ok(database);
        }
        Err(err) => Err(anyhow::anyhow!(
            "Could not load input file.\n\
        Tried loading it as a {} file, but got the following error: {}",
            "COURAGEOUS (v0.4)", // ToDo: pass value through external parameter that depends on the format schema version
            err
        )),
    }
}
