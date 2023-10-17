use courageous_format::Position3d;
use quick_xml::{events::BytesText, Writer};

pub fn write_cuas_origin(
    x: &mut Writer<impl std::io::Write>,
    static_cuas_origin: Position3d,
) -> Result<(), quick_xml::Error> {
    x.create_element("Placemark").write_inner_content(|x| {
        x.create_element("name")
            .write_text_content(BytesText::new("Static CUAS Location"))?;

        x.create_element("styleUrl")
            .write_text_content(BytesText::new("cuas_style"))?;

        x.create_element("Point").write_inner_content(|x| {
            x.create_element("extrude")
                .write_text_content(BytesText::new("false"))?;
            x.create_element("altitudeMode")
                .write_text_content(BytesText::new("absolute"))?;
            x.create_element("coordinates")
                .write_text_content(BytesText::new(&format!(
                    "{},{},{}",
                    static_cuas_origin.lon, static_cuas_origin.lat, static_cuas_origin.height
                )))?;

            Ok(())
        })?;
        Ok(())
    })?;
    Ok(())
}
