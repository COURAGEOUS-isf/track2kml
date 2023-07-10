use super::WriteAsKmlOptions;
use quick_xml::{
    events::{BytesCData, BytesText, Event},
    Writer,
};

const TRACK_ICON_URL: &'static str =
    "http://earth.google.com/images/kml-icons/track-directional/track-0.png";
const ORIGIN_ICON_URL: &str = "http://maps.google.com/mapfiles/kml/pushpin/red-pushpin.png";

pub fn write_style(
    x: &mut Writer<impl std::io::Write>,
    options: &WriteAsKmlOptions,
) -> Result<(), quick_xml::Error> {
    x.create_element("Style")
        .with_attribute(("id", "track_style"))
        .write_inner_content(|x| {
            x.create_element("IconStyle").write_inner_content(|x| {
                if options.disable_track_icons {
                    x.create_element("color")
                        .write_text_content(BytesText::new("00ffffff"))?;
                }
                x.create_element("scale")
                    .write_text_content(BytesText::new("0.5"))?;
                x.create_element("Icon").write_inner_content(|x| {
                    x.create_element("href")
                        .write_text_content(BytesText::new(TRACK_ICON_URL))?;
                    Ok(())
                })?;
                Ok(())
            })?;
            x.create_element("LineStyle").write_inner_content(|x| {
                x.create_element("colorMode")
                    .write_text_content(BytesText::new("random"))?;
                x.create_element("width")
                    .write_text_content(BytesText::new("5"))?;
                Ok(())
            })?;
            x.create_element("BalloonStyle").write_inner_content(|x| {
                x.create_element("text").write_inner_content(|x| {
                    x.write_event(Event::CData(BytesCData::new(include_str!(
                        "track_balloon_text.html"
                    ))))?;
                    Ok(())
                })?;

                Ok(())
            })?;

            Ok(())
        })?;
    x.create_element("Style")
        .with_attribute(("id", "origin_style"))
        .write_inner_content(|x| {
            x.create_element("IconStyle").write_inner_content(|x| {
                x.create_element("Icon").write_inner_content(|x| {
                    x.create_element("href")
                        .write_text_content(BytesText::new(ORIGIN_ICON_URL))?;
                    Ok(())
                })?;
                x.create_element("scale")
                    .write_text_content(BytesText::new("0.5"))?;
                Ok(())
            })?;
            x.create_element("BalloonStyle").write_inner_content(|x| {
                x.create_element("text").write_inner_content(|x| {
                    x.write_event(Event::CData(BytesCData::new(include_str!(
                        "detection_balloon_text.html"
                    ))))?;
                    Ok(())
                })?;
                Ok(())
            })?;
            Ok(())
        })?;
    Ok(())
}