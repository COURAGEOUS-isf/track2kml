pub use courageous_format::{
    Arc, Classification, Detection, DetectionRecord, Document as Database, Location, Position2d,
    Position3d, Quad, Track, TrackingRecord,
};

mod kml;
pub use kml::{write_as_kml, WriteAsKmlOptions};
