use chrono::prelude::*;
use serde::Serialize;
/// Represents a weather observation as exposed by the Windy API, if you include this struct in an uploaded WindyReport windy will interpret it as weather data
#[derive(Default, Serialize, Builder, Clone)]
pub struct Observation {
    /// Station Id
    station: u32,
    /// Date
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    dateutc: Option<DateTime<Utc>>,
    /// Tempurature in Celsius
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    temp: Option<f32>,
    /// Wind speed in Meters per Second
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    wind: Option<f32>,
    /// wind direction in Degrees
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "winddir")]
    #[builder(setter(into, strip_option), default)]
    wind_direction: Option<i32>,
    /// wind gust speed in Meters Per Second
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    gust: Option<f32>,
    /// relative humidity in Percent
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "rh")]
    #[builder(setter(into, strip_option), default)]
    relative_humidity: Option<f32>,
    /// Dew Point in Celsius
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "dewpoint")]
    #[builder(setter(into, strip_option), default)]
    dew_point: Option<f32>,
    /// Pressure in Pascal
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    pressure: Option<f32>,
    /// Precipitation over the past hour in Millimeters
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "precip")]
    #[builder(setter(into, strip_option), default)]
    precipitation: Option<f32>,
    /// Uv in index
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into, strip_option), default)]
    uv: Option<i32>,
}
