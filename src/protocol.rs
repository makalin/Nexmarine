use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use nmea::ParseResult;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeabindMessage {
    pub path: String,
    pub timestamp: i64,
    pub value: SeabindValue,
    pub unit: Option<String>,
    pub source: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SeabindValue {
    Number(f64),
    Text(String),
    Boolean(bool),
    Object(HashMap<String, SeabindValue>),
    Array(Vec<SeabindValue>),
}

impl SeabindMessage {
    pub fn new(path: String, value: SeabindValue) -> Self {
        Self {
            path,
            timestamp: chrono::Utc::now().timestamp_millis(),
            value,
            unit: None,
            source: None,
        }
    }

    pub fn with_unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }

    pub fn with_source(mut self, source: impl Into<String>) -> Self {
        self.source = Some(source.into());
        self
    }
}

// NMEA to Seabind conversion traits
pub trait ToSeabind {
    fn to_seabind(&self) -> Vec<SeabindMessage>;
}

// Example implementation for NMEA sentences
impl ToSeabind for ParseResult {
    fn to_seabind(&self) -> Vec<SeabindMessage> {
        match self {
            ParseResult::RMC(rmc) => vec![
                SeabindMessage::new(
                    "nav.position.latitude".into(),
                    SeabindValue::Number(rmc.latitude.unwrap_or(0.0))
                ).with_unit("degrees"),
                SeabindMessage::new(
                    "nav.position.longitude".into(),
                    SeabindValue::Number(rmc.longitude.unwrap_or(0.0))
                ).with_unit("degrees"),
                SeabindMessage::new(
                    "nav.speed.knots".into(),
                    SeabindValue::Number(rmc.speed.unwrap_or(0.0))
                ).with_unit("knots"),
            ],
            ParseResult::GGA(gga) => vec![
                SeabindMessage::new(
                    "nav.position.latitude".into(),
                    SeabindValue::Number(gga.latitude.unwrap_or(0.0))
                ).with_unit("degrees"),
                SeabindMessage::new(
                    "nav.position.longitude".into(),
                    SeabindValue::Number(gga.longitude.unwrap_or(0.0))
                ).with_unit("degrees"),
                SeabindMessage::new(
                    "nav.position.altitude".into(),
                    SeabindValue::Number(gga.altitude.unwrap_or(0.0))
                ).with_unit("meters"),
            ],
            _ => vec![],
        }
    }
} 