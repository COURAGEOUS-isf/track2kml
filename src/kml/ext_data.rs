use std::io::Write;

use courageous_format::{
    Alarm, Classification, CoordENU, DetectionRecord, Location, Position3d, TrackingRecord,
};
use quick_xml::{
    events::{BytesCData, BytesText, Event},
    Writer,
};

pub struct Record {
    /// UTC time as an Unix millisecond timestamp.
    pub time: u64,
    /// A unique number that identifies this record between all other ones present in the document.
    pub record_number: u64,
    /// Classification of the record.
    pub classification: Classification,
    /// If the record has Alarm data on this record, it may be specified here.
    pub alarm: Option<Alarm>,
    /// The UAS location, which may be given in one of several declaration types.
    pub location: Option<Location>,
    /// The UAS velocity given in ENU coordinates (given from the UAS position and given in meters per second).
    pub velocity: Option<CoordENU>,
    /// Free form text, possibly describing the model or configuration of the UAS identified.
    pub identification: Option<String>,
    /// The 3D GPS location of the CUAS recorded on this instant. Overrides the document's
    /// static_cuas_location.
    ///
    /// If the CUAS is located at the static_cuas_location on this instant, you may skip this field.
    pub cuas_location: Option<Position3d>,
}

impl From<DetectionRecord> for Record {
    fn from(value: DetectionRecord) -> Self {
        Record {
            time: value.time,
            record_number: value.record_number,
            classification: value.classification,
            alarm: value.alarm,
            location: value.location,
            velocity: value.velocity,
            identification: value.identification,
            cuas_location: value.cuas_location,
        }
    }
}
impl From<TrackingRecord> for Record {
    fn from(value: TrackingRecord) -> Self {
        Record {
            time: value.time,
            record_number: value.record_number,
            classification: value.classification,
            alarm: Some(value.alarm),
            location: Some(value.location),
            velocity: value.velocity,
            identification: value.identification,
            cuas_location: value.cuas_location,
        }
    }
}

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
            x.create_element("SimpleField")
                .with_attribute(("name", "velocity"))
                .with_attribute(("type", "string"))
                .write_inner_content(|x| {
                    x.create_element("displayName").write_inner_content(|x| {
                        x.write_event(Event::CData(BytesCData::new("<b>Velocity</b>")))
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
                let alarm = record.alarm.map_or(false, |a| a.active);
                x.create_element("SimpleData")
                    .with_attribute(("name", "alarm"))
                    .write_text_content(BytesText::new(if alarm { "On" } else { "Off" }))?;
                let certainty = record.alarm.map_or(0., |a| a.certainty);
                x.create_element("SimpleData")
                    .with_attribute(("name", "alarm_certainty"))
                    .write_text_content(BytesText::new(&format!("{:.0}", certainty * 100.)))?;
                x.create_element("SimpleData")
                    .with_attribute(("name", "identification"))
                    .write_text_content(BytesText::new(
                        record.identification.as_deref().unwrap_or("<i>empty</i>"),
                    ))?;
                x.create_element("SimpleData")
                    .with_attribute(("name", "velocity"))
                    .write_text_content(BytesText::new(
                        &record
                            .velocity
                            .map(|v| {
                                let ew = if v.east.is_sign_positive() {
                                    "east"
                                } else {
                                    "west"
                                };
                                let ns = if v.north.is_sign_positive() {
                                    "north"
                                } else {
                                    "south"
                                };
                                let ud = if v.up.is_sign_positive() {
                                    "up"
                                } else {
                                    "down"
                                };

                                format!(
                                    "{} m/s {}, {} m/s {}, {} m/s {}",
                                    v.east.abs(),
                                    ew,
                                    v.north.abs(),
                                    ns,
                                    v.up.abs(),
                                    ud,
                                )
                            })
                            .unwrap_or("<i>not given</i>".to_owned()),
                    ))?;
                Ok(())
            })?;

        Ok(())
    })?;
    Ok(())
}

pub fn write_track_extended_data(
    x: &mut Writer<impl Write>,
    records: &[&TrackingRecord],
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
                            let alarm = record.alarm.active;
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(if alarm {
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
                            let certainty = record.alarm.certainty;
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(&format!(
                                    "{:.0}",
                                    certainty * 100.
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
                x.create_element("gx:SimpleArrayData")
                    .with_attribute(("name", "velocity"))
                    .write_inner_content(|x| {
                        for record in records.iter() {
                            x.create_element("gx:value")
                                .write_text_content(BytesText::new(
                                    &record
                                        .velocity
                                        .map(|v| {
                                            let ud = if v.up.is_sign_positive() {
                                                "up"
                                            } else {
                                                "down"
                                            };
                                            let ew = if v.east.is_sign_positive() {
                                                "east"
                                            } else {
                                                "west"
                                            };
                                            let ns = if v.north.is_sign_positive() {
                                                "north"
                                            } else {
                                                "south"
                                            };

                                            format!(
                                                "{} m/s {}, {} m/s {}, {} m/s {}",
                                                v.up, ud, v.east, ew, v.north, ns
                                            )
                                        })
                                        .unwrap_or("<i>not given</i>".to_owned()),
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
