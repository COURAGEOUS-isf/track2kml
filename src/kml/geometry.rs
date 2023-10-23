use std::io::Write;

use courageous_format::Position3d;
use map_3d::{aer2geodetic, Ellipsoid};
use quick_xml::{events::BytesText, Writer};


pub fn ray_from_bearing(
    x: &mut Writer<impl Write>,
    cuas_origin: Position3d,
    bearing: f64,
    ray_length: f64,
) -> Result<(), quick_xml::Error> {
    let target = distance_from_position(cuas_origin, bearing, 0., ray_length);
    x.create_element("LineString").write_inner_content(|x| {
        x.create_element("extrude")
            .write_text_content(BytesText::new("0"))?;
        x.create_element("tessellate")
            .write_text_content(BytesText::new("0"))?;
        x.create_element("altitudeMode")
            .write_text_content(BytesText::new("clampToGround"))?;
        x.create_element("coordinates")
            .write_text_content(BytesText::new(&format!(
                "{},{},{} {},{},{}",
                cuas_origin.lon,
                cuas_origin.lat,
                cuas_origin.height,
                target.lon,
                target.lat,
                target.height
            )))?;

        Ok(())
    })?;
    Ok(())
}

pub fn ray_from_bearing_elevation(
    x: &mut Writer<impl Write>,
    cuas_origin: Position3d,
    bearing: f64,
    elevation: f64,
    ray_length: f64,
) -> Result<(), quick_xml::Error> {
    let target = distance_from_position(cuas_origin, bearing, elevation, ray_length);
    x.create_element("LineString").write_inner_content(|x| {
        x.create_element("extrude")
            .write_text_content(BytesText::new("0"))?;
        x.create_element("tessellate")
            .write_text_content(BytesText::new("0"))?;
        x.create_element("altitudeMode")
            .write_text_content(BytesText::new("absolute"))?;
        x.create_element("coordinates")
            .write_text_content(BytesText::new(&format!(
                "{},{},{} {},{},{}",
                cuas_origin.lon,
                cuas_origin.lat,
                cuas_origin.height,
                target.lon,
                target.lat,
                target.height
            )))?;

        Ok(())
    })?;
    Ok(())
}

pub fn point_from_bearing_elevation_distance(
    x: &mut Writer<impl Write>,
    cuas_origin: Position3d,
    bearing: f64,
    elevation: f64,
    distance: f64,
) -> Result<(), quick_xml::Error> {
    let pos = distance_from_position(cuas_origin, bearing, elevation, distance);
    x.create_element("Point").write_inner_content(|x| {
        x.create_element("extrude")
            .write_text_content(BytesText::new("0"))?;
        x.create_element("altitudeMode")
            .write_text_content(BytesText::new("absolute"))?;
        x.create_element("coordinates")
            .write_text_content(BytesText::new(&format!(
                "{},{},{}",
                pos.lon, pos.lat, pos.height
            )))?;

        Ok(())
    })?;
    Ok(())
}

pub fn create_arc_polygon(
    x: &mut Writer<impl Write>,
    bearing_from: f64,
    bearing_to: f64,
    static_cuas_origin: Position3d,
) -> Result<(), quick_xml::Error> {
    x.create_element("Polygon").write_inner_content(|x| {
        x.create_element("extrude")
            .write_text_content(BytesText::new("0"))?;
        x.create_element("tessellate")
            .write_text_content(BytesText::new("1"))?;
        x.create_element("altitudeMode")
            .write_text_content(BytesText::new("clampToGround"))?;
        x.create_element("outerBoundaryIs")
            .write_inner_content(|x| {
                x.create_element("LinearRing").write_inner_content(|x| {
                    const ARC_POINT_COUNT: usize = 64;
                    const ARC_RADIUS: f64 = 100.;

                    let arc_points = (0..ARC_POINT_COUNT).map(|idx| {
                        let angle_deg = bearing_from
                            + (bearing_to - bearing_from) * (idx as f64 / ARC_POINT_COUNT as f64);
                        (angle_deg.to_radians(), 0., ARC_RADIUS)
                    });

                    let points = std::iter::once((
                        static_cuas_origin.lat,
                        static_cuas_origin.lon,
                        static_cuas_origin.height,
                    ))
                    .chain(arc_points.map(|(az, el, slant_range)| {
                        let (lat, lon, alt) = aer2geodetic(
                            az,
                            el,
                            slant_range,
                            static_cuas_origin.lat.to_radians(),
                            static_cuas_origin.lon.to_radians(),
                            static_cuas_origin.height,
                            Ellipsoid::WGS84,
                        );
                        (lat.to_degrees(), lon.to_degrees(), alt)
                    }));

                    let coordinates_str = points
                        .map(|(lat, lon, _height)| format!("{},{} ", lon, lat))
                        .collect::<String>();

                    x.create_element("coordinates")
                        .write_text_content(BytesText::new(&coordinates_str))?;
                    Ok(())
                })?;
                Ok(())
            })?;

        Ok(())
    })?;
    Ok(())
}

pub fn distance_from_position(
    origin: Position3d,
    azimuth: f64,
    elevation: f64,
    distance: f64,
) -> Position3d {
    let (lat, lon, height) = aer2geodetic(
        azimuth.to_radians(),
        elevation.to_radians(),
        distance,
        origin.lat.to_radians(),
        origin.lon.to_radians(),
        origin.height,
        Ellipsoid::WGS84,
    );
    Position3d {
        lat: lat.to_degrees(),
        lon: lon.to_degrees(),
        height,
    }
}
