use courageous_format::Position3d;
use quick_xml::{events::BytesText, Writer};

pub fn write_uav_home_location(
    x: &mut Writer<impl std::io::Write>,
    uav_home_location: Position3d,
) -> Result<(), quick_xml::Error> {
    x.create_element("Placemark").write_inner_content(|x| {
        x.create_element("name")
            .write_text_content(BytesText::new("UAV Home Location"))?;

        x.create_element("styleUrl")
            .write_text_content(BytesText::new("uav_home_style"))?;

        x.create_element("Point").write_inner_content(|x| {
            x.create_element("extrude")
                .write_text_content(BytesText::new("false"))?;
            x.create_element("altitudeMode")
                .write_text_content(BytesText::new("absolute"))?;
            x.create_element("coordinates")
                .write_text_content(BytesText::new(&format!(
                    "{},{},{}",
                    uav_home_location.lon, uav_home_location.lat, uav_home_location.height
                )))?;

            Ok(())
        })?;
        Ok(())
    })?;
    Ok(())
}
