use super::WriteAsKmlOptions;
use quick_xml::{
    events::{BytesCData, BytesText, Event},
    Writer,
};

const TRACK_ICON_URL: &'static str =
    "http://earth.google.com/images/kml-icons/track-directional/track-0.png";
const ORIGIN_ICON_URL: &str = "http://maps.google.com/mapfiles/kml/pushpin/red-pushpin.png";
const CUAS_ICON_URL: &str = "http://maps.google.com/mapfiles/kml/paddle/blu-circle.png";
const UAV_HOME_ICON_URL: &str = "https://maps.google.com/mapfiles/kml/paddle/H.png";

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
    x.create_element("Style")
        .with_attribute(("id", "cuas_style"))
        .write_inner_content(|x| {
            x.create_element("IconStyle").write_inner_content(|x| {
                x.create_element("Icon").write_inner_content(|x| {
                    x.create_element("href")
                        .write_text_content(BytesText::new(CUAS_ICON_URL))?;
                    Ok(())
                })?;
                x.create_element("scale")
                    .write_text_content(BytesText::new("0.5"))?;
                Ok(())
            })?;
            x.create_element("BalloonStyle").write_inner_content(|x| {
                x.create_element("text").write_inner_content(|x| {
                    x.write_event(Event::CData(BytesCData::new(
                        "<b>CUAS</b></br>
                Static Location of the CUAS.",
                    )))?;
                    Ok(())
                })?;
                Ok(())
            })?;
            Ok(())
        })?;
    x.create_element("Style")
        .with_attribute(("id", "uav_home_style"))
        .write_inner_content(|x| {
            x.create_element("IconStyle").write_inner_content(|x| {
                x.create_element("Icon").write_inner_content(|x| {
                    x.create_element("href")
                        .write_text_content(BytesText::new(UAV_HOME_ICON_URL))?;
                    Ok(())
                })?;
                x.create_element("scale")
                    .write_text_content(BytesText::new("0.5"))?;
                Ok(())
            })?;
            x.create_element("BalloonStyle").write_inner_content(|x| {
                x.create_element("text").write_inner_content(|x| {
                    x.write_event(Event::CData(BytesCData::new(
                        "<b>UAV Home location</b></br>
                        The home location of the UAV intercepted by the CUAS.",
                    )))?;
                    Ok(())
                })?;
                Ok(())
            })?;
            Ok(())
        })?;
    Ok(())
}
