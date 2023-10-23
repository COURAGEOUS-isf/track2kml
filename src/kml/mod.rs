use crate::Database;
use quick_xml::Writer;

use self::{
    cuas::write_cuas_origin, detection::write_detection_set, ext_data::write_schema,
    style::write_style, tracking::write_track_set,
};

mod cuas;
mod detection;
mod ext_data;
mod geometry;
mod style;
mod tracking;

const KML_DOCUMENT_ATTRIBUTES: [(&str, &str); 2] = [
    ("xmlns", "http://www.opengis.net/kml/2.2"),
    ("xmlns:gx", "http://www.google.com/kml/ext/2.2"),
];

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct WriteAsKmlOptions {
    pub disable_track_icons: bool,
    pub cuas_range: f64,
}

impl WriteAsKmlOptions {
    pub fn disable_track_icons(mut self, val: bool) -> WriteAsKmlOptions {
        self.disable_track_icons = val;
        self
    }
    pub fn cuas_range(mut self, val: f64) -> WriteAsKmlOptions {
        self.cuas_range = val;
        self
    }
}

pub fn write_as_kml(
    database: Database,
    writer: impl std::io::Write,
    options: WriteAsKmlOptions,
) -> anyhow::Result<()> {
    let mut xml = Writer::new(writer);
    let cuas_range = options.cuas_range;
    xml.write_bom()?;
    xml.create_element("kml")
        .with_attributes(KML_DOCUMENT_ATTRIBUTES)
        .write_inner_content(|x| {
            x.create_element("Document").write_inner_content(|x| {
                write_schema(x)?;
                write_style(x, &options)?;

                write_detection_set(
                    x,
                    &database.detection,
                    database.static_cuas_location.clone(),
                    cuas_range,
                )?;
                write_track_set(
                    x,
                    &database.tracks,
                    database.static_cuas_location.clone(),
                    cuas_range,
                )?;
                write_cuas_origin(x, database.static_cuas_location.clone())?;

                Ok(())
            })?;

            Ok(())
        })?;

    Ok(())
}
