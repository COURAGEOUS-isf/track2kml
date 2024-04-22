## 2.5.0
- Remove `--hint` argument from CLI.
- Change behaviour of `--origin` such that it overwrites the internal file `static_cuas_location` member.
- Add `--ignore-cuas-origin` option

## 2.4.0
- Separate track records by classification: Create one KML track per set of records with the same classification in a COURAGEOUS track.

## 2.3.1
- Implement UAV home location representation.

## 2.3.0
- Update COURAGEOUS schema support to v0.4.0 
- Implement CUAS location representation.
- Implement `--cuas-range <RANGE>` optional parameter for specifying the length of rays and arcs drawn with
BearingElevation, Bearing, Arc and Quad locations.

## 2.2.0
- Update COURAGEOUS schema support to v0.3.1
- Display record velocity in its KML extended data balloon.

## 2.1.1
- Fix Position3d.height_amsl member still being parsed as "height".

## 2.1.0
- Update COURAGEOUS support to v0.3.0

## 2.0.3
- Internal structural changes.

## 2.0.2
- Do not allow the -c option if the application is built with only COURAGEOUS support (This is technically a breaking change, but since it makes no sense to use it, it's counted as a minor change/bugfix).
- Update README.

## 2.0.1
- Correctly display point tracks in KML files.

## 2.0.0
- Rewrite entire backend.
- Support COURAGEOUS format.
- Allow exporting to COURAGEOUS format.

## 1.5.1
- Remove debug information shown when parsing some files.

## 1.5.0
- Use points instead of rays to represent angle-based point log files (e.g. ART detection).
- Add README.

## 1.4.1
- Fix timestamps in HGH v2 format.

## 1.4.0
- Support for the second iteration of the HGH log format.

## 1.3.0
- Support for the Senhive log format.

## 1.2.1
- Fix loading GPX files without the `<extensions>` tag.

## 1.2.0
- Allow disabling track icons (`--no-track-icons`).
- Allow disabling track segmenting (`--no-segmenting`).

## 1.1.0
- Improve track line style by making them bigger and their color random.

## 1.0.0
- Initial version.
- Support for ART tracking, identification & detection logs, DATCON & HGH logs and Robin Radar GPX files.