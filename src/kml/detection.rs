use courageous_format::{Arc, Detection, Location, Position3d};
use quick_xml::{events::BytesText, Writer};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

use super::{
    ext_data::write_detection_extended_data,
    geometry::{
        create_arc_polygon, point_from_bearing_elevation_distance, ray_from_bearing,
        ray_from_bearing_elevation,
    },
};

pub fn write_detection_set(
    x: &mut Writer<impl std::io::Write>,
    set: &[Detection],
    static_cuas_origin: Position3d,
    cuas_range: f64,
) -> Result<(), quick_xml::Error> {
    x.create_element("Folder").write_inner_content(|x| {
        x.create_element("name")
            .write_text_content(BytesText::new("Detection Sets"))?;

        for detection in set {
            write_detection(x, detection, static_cuas_origin, cuas_range)?;
        }

        Ok(())
    })?;

    Ok(())
}

pub fn write_detection(
    x: &mut Writer<impl std::io::Write>,
    detection: &Detection,
    static_cuas_origin: Position3d,
    cuas_range: f64,
) -> Result<(), quick_xml::Error> {
    x.create_element("Folder").write_inner_content(|x| {
        x.create_element("name").write_text_content(BytesText::new(
            detection.name.as_deref().unwrap_or("Unnamed detection set"),
        ))?;
        x.create_element("description")
            .write_text_content(BytesText::new(&format!(
                "UAV unique ID: {}",
                detection
                    .uas_id
                    .map(|x| x.to_string())
                    .unwrap_or_else(|| "None".to_owned())
            )))?;

        for record in detection
            .records
            .iter()
            .filter(|record| record.location.is_some())
        {
            x.create_element("Placemark").write_inner_content(|x| {
                x.create_element("name")
                    .write_text_content(BytesText::new(&format!("{}", record.record_number)))?;
                let time_str =
                    OffsetDateTime::from_unix_timestamp_nanos(record.time as i128 * 1_000_000)
                        .unwrap()
                        .format(&Rfc3339)
                        .unwrap();
                write_detection_extended_data(x, record)?;

                x.create_element("styleUrl")
                    .write_text_content(BytesText::new("origin_style"))?;
                x.create_element("TimeStamp").write_inner_content(|x| {
                    x.create_element("when")
                        .write_text_content(BytesText::new(&time_str))?;

                    Ok(())
                })?;

                if let Some(location) = &record.location {
                    match location {
                        Location::Position3d(pos) => {
                            x.create_element("Point").write_inner_content(|x| {
                                x.create_element("extrude")
                                    .write_text_content(BytesText::new("false"))?;
                                x.create_element("altitudeMode")
                                    .write_text_content(BytesText::new("absolute"))?;
                                x.create_element("coordinates").write_text_content(
                                    BytesText::new(&format!(
                                        "{},{},{}",
                                        pos.lon, pos.lat, pos.height
                                    )),
                                )?;

                                Ok(())
                            })?;
                        }
                        Location::Position2d(pos) => {
                            x.create_element("Point").write_inner_content(|x| {
                                x.create_element("extrude")
                                    .write_text_content(BytesText::new("false"))?;
                                x.create_element("altitudeMode")
                                    .write_text_content(BytesText::new("clampToGround"))?;
                                x.create_element("coordinates").write_text_content(
                                    BytesText::new(&format!("{},{},0", pos.lon, pos.lat)),
                                )?;

                                Ok(())
                            })?;
                        }
                        Location::BearingElevationDistance {
                            bearing,
                            elevation,
                            distance,
                        } => {
                            point_from_bearing_elevation_distance(
                                x,
                                record.cuas_location.unwrap_or(static_cuas_origin),
                                *bearing,
                                *elevation,
                                *distance,
                            )?;
                        }
                        Location::BearingElevation { bearing, elevation } => {
                            ray_from_bearing_elevation(
                                x,
                                record.cuas_location.unwrap_or(static_cuas_origin),
                                *bearing,
                                *elevation,
                                cuas_range,
                            )?;
                        }
                        Location::Bearing { bearing } => {
                            ray_from_bearing(
                                x,
                                record.cuas_location.unwrap_or(static_cuas_origin),
                                *bearing,
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
                            create_arc_polygon(x, *from, *to, static_cuas_origin, cuas_range)?;
                        }
                    }
                }

                Ok(())
            })?;
        }

        Ok(())
    })?;

    Ok(())
}
