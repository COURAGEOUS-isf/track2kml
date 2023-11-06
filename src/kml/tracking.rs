use courageous_format::{Arc, Location, Position3d, Track, TrackingRecord};
use quick_xml::{events::BytesText, Writer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::{
    ext_data::{write_gxtrack_extended_data, write_tracking_extended_data},
    geometry::{
        create_arc_polygon, point_from_bearing_elevation_distance, ray_from_bearing,
        ray_from_bearing_elevation,
    },
    uav_home_location::write_uav_home_location,
};

pub fn write_track_set(
    x: &mut Writer<impl std::io::Write>,
    set: &[Track],
    static_cuas_origin: Position3d,
    cuas_range: f64,
) -> Result<(), quick_xml::Error> {
    x.create_element("Folder").write_inner_content(|x| {
        x.create_element("name")
            .write_text_content(BytesText::new("Tracks"))?;

        for detection in set {
            write_track(x, detection, static_cuas_origin, cuas_range)?;
        }

        Ok(())
    })?;

    Ok(())
}

pub fn write_track<W: std::io::Write>(
    x: &mut Writer<W>,
    track: &Track,
    static_cuas_origin: Position3d,
    cuas_range: f64,
) -> Result<(), quick_xml::Error> {
    x.create_element("Folder").write_inner_content(|x| {
        x.create_element("name").write_text_content(BytesText::new(
            track.name.as_deref().unwrap_or("Unnamed track"),
        ))?;

        let records = track.records.iter().peekable();

        if let Some(uav_home_location) = track.uav_home_location.clone() {
            write_uav_home_location(x, uav_home_location.clone())?;
        };

        for record in records {
            if matches!(
                record.location,
                Location::Position2d(_) | Location::Position3d(_)
            ) {
                continue;
            }

            x.create_element("Placemark").write_inner_content(|x| {
                x.create_element("name").write_text_content(BytesText::new(
                    track.name.as_deref().unwrap_or("Unnamed track"),
                ))?;

                let time_str =
                    OffsetDateTime::from_unix_timestamp_nanos(record.time as i128 * 1_000_000)
                        .unwrap()
                        .format(&Rfc3339)
                        .unwrap();

                x.create_element("TimeStamp").write_inner_content(|x| {
                    x.create_element("when")
                        .write_text_content(BytesText::new(&time_str))?;

                    Ok(())
                })?;

                match record.location {
                    Location::BearingElevationDistance {
                        bearing,
                        elevation,
                        distance,
                    } => {
                        point_from_bearing_elevation_distance(
                            x,
                            record.cuas_location.unwrap_or(static_cuas_origin),
                            bearing,
                            elevation,
                            distance,
                        )?;
                    }
                    Location::BearingElevation { bearing, elevation } => {
                        ray_from_bearing_elevation(
                            x,
                            record.cuas_location.unwrap_or(static_cuas_origin),
                            bearing,
                            elevation,
                            cuas_range,
                        )?;
                    }
                    Location::Bearing { bearing } => {
                        ray_from_bearing(
                            x,
                            record.cuas_location.unwrap_or(static_cuas_origin),
                            bearing,
                            cuas_range,
                        )?;
                    }
                    Location::Quad { quad } => {
                        let (bearing_from, bearing_to) = match quad {
                            courageous_format::Quad::North => (45., -45.),
                            courageous_format::Quad::East => (135., 45.),
                            courageous_format::Quad::South => (225., 135.),
                            courageous_format::Quad::West => (315., 225.),
                        };
                        create_arc_polygon(
                            x,
                            bearing_from,
                            bearing_to,
                            static_cuas_origin,
                            cuas_range,
                        )?;
                    }
                    Location::Arc(Arc { from, to }) => {
                        create_arc_polygon(x, from, to, static_cuas_origin, cuas_range)?;
                    }
                    // Positions are processed on the next step
                    Location::Position2d(_) | Location::Position3d(_) => unreachable!(),
                }
                write_tracking_extended_data(x, record)?;
                Ok(())
            })?;
        }

        let pos_records = track.records.iter().filter(|record| {
            matches!(
                record.location,
                Location::Position3d(_) | Location::Position2d(_)
            )
        });

        // Group all point positions into a multitrack
        // Place outside the MultiGeometry element because Google Earth doesn't seem to like tracks
        // or MultiTracks inside MultiGeometry (It doesn't show them, and you cannot interact with
        // them)
        if pos_records.count() != 0 {
            x.create_element("Placemark").write_inner_content(|x| {
                x.create_element("styleUrl")
                    .write_text_content(BytesText::new("track_style"))?;

                x.create_element("name").write_text_content(BytesText::new(
                    track.name.as_deref().unwrap_or("Unnamed track"),
                ))?;

                x.create_element("gx:MultiTrack").write_inner_content(|x| {
                    x.create_element("altitudeMode")
                        .write_text_content(BytesText::new("absolute"))?;
                    let mut records = track.records.iter().peekable();
                    while let Some(record) = records.next() {
                        match record.location {
                            Location::Position2d(_) | Location::Position3d(_) => {
                                // HACK: This mostly assumes all records are either Position2d or Position3d
                                let mut contiguous_pos_records = vec![record];
                                while let Some(
                                    &record @ TrackingRecord {
                                        // HACK: This will result in points at the sea level if Position3d and Position2d are mixed
                                        location: Location::Position3d(_) | Location::Position2d(_),
                                        ..
                                    },
                                ) = records.peek()
                                {
                                    contiguous_pos_records.push(record);
                                    records.next();
                                }
                                // Append all position records into a track
                                x.create_element("gx:Track").write_inner_content(|x| {
                                    x.create_element("altitudeMode").write_text_content(
                                        BytesText::new(
                                            if matches!(record.location, Location::Position3d(_)) {
                                                "absolute"
                                            } else {
                                                "clampToGround"
                                            },
                                        ),
                                    )?;
                                    for record in contiguous_pos_records.iter() {
                                        x.create_element("when").write_text_content(
                                            BytesText::new(
                                                &OffsetDateTime::from_unix_timestamp_nanos(
                                                    record.time as i128 * 1_000_000,
                                                )
                                                .unwrap()
                                                .format(&Rfc3339)
                                                .unwrap(),
                                            ),
                                        )?;
                                        let pos = match record.location {
                                            Location::Position3d(pos) => pos,
                                            Location::Position2d(pos) => Position3d {
                                                lat: pos.lat,
                                                lon: pos.lon,
                                                height: 0.,
                                            },
                                            _ => unreachable!(),
                                        };
                                        x.create_element("gx:coord").write_text_content(
                                            BytesText::new(&format!(
                                                "{} {} {}",
                                                pos.lon, pos.lat, pos.height
                                            )),
                                        )?;
                                    }
                                    write_gxtrack_extended_data(x, &contiguous_pos_records)?;

                                    Ok(())
                                })?;
                            }
                            _ => (),
                        }
                    }

                    Ok(())
                })?;

                Ok(())
            })?;
        }

        Ok(())
    })?;

    Ok(())
}
