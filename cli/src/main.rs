use std::process::ExitCode;

use clap::{CommandFactory, Parser};
use colored::Colorize;
use courageous_format::Position3d;
use std::path::PathBuf;
use textwrap::Options;

use track2kml_cli::process_to_kml;

mod clap_util;

fn main() -> ExitCode {
    let start_time = std::time::Instant::now();

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// The path of the file to convert to KML.
        input_path: PathBuf,

        /// Where to place the resulting KML file. (Default: input_path with the "kml" extension)
        #[arg(short = 'o')]
        output_path: Option<PathBuf>,

        /// Specify the detection origin (Radar position) in GPS coordinates `lat,lon,height`.
        ///
        /// Values must be formatted as longitude,latitude[,altitude (AMSL)]. If the altitude is omitted,
        /// it will default to 0 (ground level).
        #[arg(value_enum, long = "origin", value_parser = clap_util::Position3dParser)]
        detection_origin: Option<Position3d>,

        /// When exporting to KML: Hide all track icons, and only show their path or ray.
        #[arg(long)]
        no_track_icons: bool,

        /// When exporting to KML: Ignore the CUAS origin in the file, do not include it.
        #[arg(long, short = 'i')]
        ignore_cuas_origin: bool,

        /// Maximum distance from the C-UAS where objects can be detected, in meters.
        ///
        /// Used for the length of rays and radii of arcs in systems that represent position with BearingElevation, Bearing,
        /// Arc or Quad.
        #[arg(long, short = 'r')]
        cuas_range: Option<f64>,
    }

    let cmd = Args::command()
        .name("track2kml")
        .help_template(include_str!("help_template"));

    let matches = cmd.clone().get_matches();

    match process_to_kml(&matches) {
        Ok(output_path) => {
            textwrap::wrap(
                &format!(
                    "Finished in {}ms.\nSaved result into {:?}",
                    (std::time::Instant::now() - start_time).as_millis(),
                    output_path
                ),
                Options::new(80)
                    .initial_indent(&format!("{}{} ", "OK".green().bold(), ":".bold()))
                    .subsequent_indent("    "),
            )
            .into_iter()
            .for_each(|line| println!("{}", line));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("{}{} {}", "Error".red().bold(), ":".bold(), err);

            ExitCode::FAILURE
        }
    }
}
