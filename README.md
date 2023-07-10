# track2kml

A CLI tool and [Rust](https://www.rust-lang.org/) crate to convert from the [COURAGEOUS data format](https://github.com/COURAGEOUS-isf/format) to [KML](https://developers.google.com/kml/documentation/kmlreference) for representation on Google Earth.

## Building

### CLI application

Make sure you have a recent Rust toolchain installed. Otherwise, check [the official site](https://www.rust-lang.org/tools/install) for more information on how to install one.

Clone the repository:

```sh
git clone https://github.com/COURAGEOUS-isf/track2kml
```

Change directory into the project folder, then run

```sh
cargo build -p track2kml-cli --release
```

to build an optimized, stripped binary onto the `target/release` folder, named `track2kml` (or `track2kml.exe`). Feel free to move it to another location, as other files in the `target` directory are just intermediate products and artefacts that the executable doesn't need to run.

### Rust crate

The following is only neccessary if you require using the format to KML library within your own Rust application. If you only want to build the CLI application, you do not need to do this.

Add the following line to your `Cargo.toml`:

```toml
track2kml = { git = "https://github.com/COURAGEOUS-isf/track2kml.git", tag = "<version>" }
```

replacing the `<version>` placeholder with the tag of the version you wish to use.

Although the documentation available is barebones, it is recommended to always have it to check available types and methods. Remember that you can open it in your browser using `cargo doc --no-deps -p track2kml --open`.

## CLI Usage

The CLI application is really simple to use, and most times will only require the path of the file to convert, e.g.:

```sh
track2kml ./test_data/test-courageous-file.json
```

However, it also has a few other options that can be tweaked, and in some cases their inclusion may be required to convert a file. You can check the full list of options using `track2kml --help`.

## Support

If you have any problems getting the application or crate to work, feel free to post an issue here on GitHub. For other questions, you can contact the [lead developer](https://github.com/aleokdev) via email. (The address should be below the profile picture and description)
