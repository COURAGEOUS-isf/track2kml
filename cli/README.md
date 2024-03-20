# track2kml

Track2kml is an application that converts files in the [COURAGEOUS format](https://grvc.us.es/courageous/) into KML (readable by [Google Earth Pro](https://earth.google.com/intl/earth/download/ge/agree.html)[^1]).

[^1]: The KML file is intended to be viewed on Google Earth Pro. As a result, it may not work as intended on the web version.

---

## CLI Usage

The CLI application is really simple to use, and most times will only require the path of the file to convert, e.g.:

```sh
track2kml ./courageous_test_file.json
```

However, it also has a few other options that can be tweaked, for instance:

`--origin`, which overrides the radar's static position (`static_cuas_location` on the COURAGEOUS format). Its format is as follows:

```
--origin latitude,longitude,height
```

Where latitude and longitude are given in decimal degrees and height is given in meters and AMSL. For instance, `--origin 4.3341194,51.4507167,15`.

The full list of options can be found using `track2kml --help`.

## Examples
Convert `tracks.json` into KML, using the radar position written in the file itself:

```sh
track2kml detection_file.json
```

Convert `data_20_3_24.json` into KML, overriding the origin to 4.3341194º lat, 51.4507167º lon, 15m AMSL, hiding track icons:

```sh
track2kml --origin 4.3341194,51.4507167,15 --no-track-icons data_20_3_24.json
```
---

## Support

If you have any problems getting the application or crate to work, you can contact the [lead developer](https://github.com/aleokdev) via email. (The address should be below the profile picture and description).
