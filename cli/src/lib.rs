use clap::{ArgMatches, Command, CommandFactory, Parser};
use colored::Colorize;
use courageous_format::{Document, Position3d};
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::File,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
    process::ExitCode,
};
use textwrap::Options;
use track2kml::{write_as_kml, WriteAsKmlOptions};

mod clap_util;

/// Describes any object that can initialize a format parser via CLI parameters (The clap command
/// and the argument matches). Object-safe by design.
pub trait CliFormatInitializer {
    fn init_from_cli(
        &self,
        cmd: &Command,
        args: &ArgMatches,
    ) -> Result<Box<dyn FormatParser>, anyhow::Error>;
}

pub trait FormatParser {
    fn parse_file(&mut self, file: &mut BufReader<File>) -> Result<Document, anyhow::Error>;
}

pub struct Format {
    initializer: Box<dyn CliFormatInitializer>,
    file_ext: &'static OsStr,
    name: &'static str,
}

impl Format {
    pub fn new<T: CliFormatInitializer + 'static>(
        initializer: T,
        file_ext: &'static str,
        name: &'static str,
    ) -> Self {
        Self {
            initializer: Box::new(initializer),
            file_ext: OsStr::new(file_ext),
            name,
        }
    }
}

pub fn run(formats: &HashMap<&'static str, Format>) -> ExitCode {
    let start_time = std::time::Instant::now();

    #[derive(Parser)]
    #[command(author, version, about, long_about = None)]
    struct Args {
        /// The path of the file to convert to KML.
        input_path: PathBuf,

        /// Specify the detection origin (Radar position) in GPS coordinates `lat,lon,height`.
        ///
        /// Values must be formatted as longitude,latitude[,altitude (AMSL)]. If the altitude is omitted,
        /// it will default to 0 (ground level).
        #[arg(value_enum, long = "origin", value_parser = clap_util::Position3dParser)]
        detection_origin: Option<Position3d>,

        /// When exporting to KML: Hide all track icons, and only show their path or ray.
        #[arg(long)]
        no_track_icons: bool,

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

    match process_to_kml(&cmd, &matches, &formats) {
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

fn process_to_kml(
    cmd: &Command,
    args: &ArgMatches,
    formats: &HashMap<&str, Format>,
) -> Result<PathBuf, anyhow::Error> {
    let input_path: &PathBuf = args.get_one("input_path").unwrap();
    let database = read_input_file(cmd, args, formats, input_path)?;
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
    cmd: &Command,
    args: &ArgMatches,
    formats: &HashMap<&str, Format>,
    input_path: &Path,
) -> Result<track2kml::Database, anyhow::Error> {
    let formats: Vec<_> = formats
        .values()
        .filter(|Format { file_ext, .. }| Some(file_ext) == input_path.extension().as_ref())
        .collect(); // Check extensions

    if formats.is_empty() {
        return Err(anyhow::anyhow!(
            "Could not load input file.\n\
    Cannot determine format from extension;"
        ));
    }

    let mut errors = Vec::with_capacity(formats.len());

    for Format { initializer, .. } in formats.iter() {
        match initializer.init_from_cli(cmd, args).and_then(|mut parser| {
            let file = File::open(input_path)?;
            parser.parse_file(&mut BufReader::new(file))
        }) {
            Ok(database) => {
                return Ok(database);
            }
            Err(err) => errors.push(err),
        }
    }

    if formats.len() > 1 {
        let error_str = {
            formats
                .iter()
                .zip(errors.into_iter())
                .map(|(format, error)| format!("\t- {} (Error: {})\n", format.name, error))
                .collect::<String>()
        };

        Err(anyhow::anyhow!(
            "Could not load input file.\n\
    Tried with the following formats:\n\
    {error_str}"
        ))
    } else {
        Err(anyhow::anyhow!(
            "Could not load input file.\n\
    Tried loading it as a {} file, but got the following error: {}",
            formats[0].name,
            errors[0]
        ))
    }
}

pub struct CourageousParser;

impl FormatParser for CourageousParser {
    fn parse_file(&mut self, file: &mut BufReader<File>) -> Result<Document, anyhow::Error> {
        serde_json::from_reader(file).map_err(anyhow::Error::from)
    }
}

pub struct CourageousInitializer;

impl CliFormatInitializer for CourageousInitializer {
    fn init_from_cli(
        &self,
        _cmd: &Command,
        _args: &ArgMatches,
    ) -> Result<Box<dyn FormatParser>, anyhow::Error> {
        Ok(Box::new(CourageousParser))
    }
}
