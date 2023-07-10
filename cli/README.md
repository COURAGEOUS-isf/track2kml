# track2kml

Track2kml is an application that converts various drone tracking formats into KML (readable by [Google Earth Pro](https://earth.google.com/intl/earth/download/ge/agree.html)[^1]).
It also converts to the [COURAGEOUS format](https://grvc.us.es/courageous/).

[^1]: The KML file is intended to be viewed on Google Earth Pro. As a result, it may not work as intended on the web version.

---

## Currently supported formats

For security concerns, only the appropiate formats on a given build will be available. For specific information about this build's formats, enter `--help` and check the `--hint` parameter's valid values.

### Specific to the 2023 March [COURAGEOUS](https://courageous-isf.eu/) tests (Greece)

- GPX
- Datcon log files
- HGH log files
- ART tracking (+ identification) log files
- ART detection log files
- Senhive log files

### Specific to the 2023 October [COURAGEOUS](https://courageous-isf.eu/) and subsequent tests

- COURAGEOUS Format (as described in <https://grvc.us.es/courageous/>) version 0.2.0

---

## CLI Usage

The CLI application is really simple to use, and most times will only require the path of the file to convert, e.g.:

```sh
track2kml ./test_data/ART_detection_test.log
```

However, it also has a few other options that can be tweaked, and in some cases their inclusion may be required to convert a file:

`--origin`, which describes the radar's static position, is required on all formats except for COURAGEOUS JSON files. Its format is as follows:

```
--origin latitude,longitude,height
```

Where latitude and longitude are given in decimal degrees and height is given in meters and AMSL. For instance, `--origin 4.3341194,51.4507167,15`.

`--hint` is only necessary if the program fails to identify the format.

`-c` is required when converting to a COURAGEOUS JSON file instead of a KML one.

The full list of options can be found using `track2kml --help`:

```
Usage: track2kml [OPTIONS] <INPUT_PATH>

Arguments:
  <INPUT_PATH>
          The path of the file to convert to KML

Options:
      --hint <HINT>
          Hint the program as to which format the input file is in

          [possible values: hgh-log, senhive, hgh-v2-log, datcon-log, courageous-v0.2, gpx, art-detection-log, art-tracking-log]

      --origin <DETECTION_ORIGIN>
          Specify the detection origin (Radar position) in GPS coordinates `lat,lon,height`.
          
          Values must be formatted as longitude,latitude[,altitude (AMSL)]. If the altitude is omitted, it will default to 0 (ground level).

  -c
          Convert to COURAGEOUS format (v0.2.0) rather than to KML

      --no-track-icons
          When exporting to KML: Hide all track icons, and only show their path or ray

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## Examples

Convert `detection_file.log` into the COURAGEOUS format, using 4º20'2.83" as latitude, 51º27'2.58" as longitude and 15m as altitude above sea level:

```sh
track2kml --origin 4.3341194,51.4507167,15 -c detection_file.log
```

Convert `tracks.json` into KML, using the radar position written in the file itself (Assuming it is a COURAGEOUS file):

```sh
track2kml detection_file.json
```

---

## Support

If you have any problems getting the application or crate to work, you can contact the [lead developer](https://github.com/aleokdev) via email. (The address should be below the profile picture and description).
