use std::io::Write;

use courageous_format::Record;
use quick_xml::{
    events::{BytesCData, BytesText, Event},
    Writer,
};

pub fn write_schema(x: &mut Writer<impl Write>) -> Result<(), quick_xml::Error> {
    x.create_element("Schema")
        .with_attribute(("name", "schema_name"))
        .with_attribute(("id", "schema"))
        .write_inner_content(|x| {
            x.create_element("SimpleField")
                .with_attribute(("name", "record_number"))
                .with_attribute(("type", "uint"))
                .write_inner_content(|x| {
                    x.create_element("displayName").write_inner_content(|x| {
                        x.write_event(Event::CData(BytesCData::new("<b>Record Number</b>")))
                    })?;
                    Ok(())
                })?;
            x.create_element("SimpleField")
                .with_attribute(("name", "classification"))
                .with_attribute(("type", "string"))
                .write_inner_content(|x| {
                    x.create_element("displayName").write_inner_content(|x| {
                        x.write_event(Event::CData(BytesCData::new("<b>Classification</b>")))
                    })?;
                    Ok(())
                })?;
            x.create_element("SimpleField")
                .with_attribute(("name", "alarm"))
                .with_attribute(("type", "bool"))
                .write_inner_content(|x| {
                    x.create_element("displayName").write_inner_content(|x| {
                        x.write_event(Event::CData(BytesCData::new("<b>Alarm</b>")))
                    })?;
                    Ok(())
                })?;
            x.create_element("SimpleField")
                .with_attribute(("name", "alarm_certainty"))
                .with_attribute(("type", "float"))
                .write_inner_content(|x| {
                    x.create_element("displayName").write_inner_content(|x| {
                        x.write_event(Event::CData(BytesCData::new("<b>Alarm Certainty</b>")))
                    })?;
                    Ok(())
                })?;
            x.create_element("SimpleField")
                .with_attribute(("name", "identification"))
                .with_attribute(("type", "string"))
                .write_inner_content(|x| {
                    x.create_element("displayName").write_inner_content(|x| {
                        x.write_event(Event::CData(BytesCData::new("<b>Identification</b>")))
                    })?;
                    Ok(())
                })?;
            Ok(())
        })?;
    Ok(())
}

pub fn write_extended_data(
    x: &mut Writer<impl Write>,
    record: &Record,
) -> Result<(), quick_xml::Error> {
    x.create_element("ExtendedData").write_inner_content(|x| {
        x.create_element("SchemaData")
            .with_attribute(("schemaUrl", "#schema"))
            .write_inner_content(|x| {
                x.create_element("SimpleData")
                    .with_attribute(("name", "record_number"))
                    .write_text_content(BytesText::new(&format!("{}", record.record_number)))?;
                x.create_element("SimpleData")
                    .with_attribute(("name", "classification"))
                    .write_text_content(BytesText::new(match record.classification {
                        courageous_format::Classification::Gcs => "GCS",
                        courageous_format::Classification::Other => "Other",
                        courageous_format::Classification::Uav => "UAV",
                        courageous_format::Classification::Unknown => "Unknown",
                    }))?;
                x.create_element("SimpleData")
                    .with_attribute(("name", "alarm"))
                    .write_text_content(BytesText::new(if record.alarm { "On" } else { "Off" }))?;
                x.create_element("SimpleData")
                    .with_attribute(("name", "alarm_certainty"))
                    .write_text_content(BytesText::new(&format!(
                        "{:.0}",
                        record.alarm_certainty * 100.
                    )))?;
                x.create_element("SimpleData")
                    .with_attribute(("name", "identification"))
                    .write_text_content(BytesText::new(
                        record.identification.as_deref().unwrap_or("<i>empty</i>"),
                    ))?;
                Ok(())
            })?;

        Ok(())
    })?;
    Ok(())
}

pub fn write_track_extended_data(
    x: &mut Writer<impl Write>,
    records: &[&Record],
) -> Result<(), quick_xml::Error> {
    x.create_element("ExtendedData").write_inner_content(|x| {
        x.create_element("SchemaData")
            .with_attribute(("schemaUrl", "#schema"))
            .write_inner_content(|x| {
                x.create_element("gx:SimpleArrayData")
                    .with_attribute(("name", "record_number"))
                    .write_inner_content(|x| {
                        for record in records.iter() {
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(&format!(
                                    "{}",
                                    record.record_number
                                )))?;
                        }
                        Ok(())
                    })?;
                x.create_element("gx:SimpleArrayData")
                    .with_attribute(("name", "classification"))
                    .write_inner_content(|x| {
                        for record in records.iter() {
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(
                                    match record.classification {
                                        courageous_format::Classification::Gcs => "GCS",
                                        courageous_format::Classification::Other => "Other",
                                        courageous_format::Classification::Uav => "UAV",
                                        courageous_format::Classification::Unknown => "Unknown",
                                    },
                                ))?;
                        }
                        Ok(())
                    })?;
                x.create_element("gx:SimpleArrayData")
                    .with_attribute(("name", "alarm"))
                    .write_inner_content(|x| {
                        for record in records.iter() {
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(if record.alarm {
                                    "On"
                                } else {
                                    "Off"
                                }))?;
                        }
                        Ok(())
                    })?;
                x.create_element("gx:SimpleArrayData")
                    .with_attribute(("name", "alarm_certainty"))
                    .write_inner_content(|x| {
                        for record in records.iter() {
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(&format!(
                                    "{:.0}",
                                    record.alarm_certainty * 100.
                                )))?;
                        }
                        Ok(())
                    })?;
                x.create_element("gx:SimpleArrayData")
                    .with_attribute(("name", "identification"))
                    .write_inner_content(|x| {
                        for record in records.iter() {
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(
                                    record.identification.as_deref().unwrap_or("<i>empty</i>"),
                                ))?;
                        }
                        Ok(())
                    })?;
                Ok(())
            })?;

        Ok(())
    })?;
    Ok(())
}
