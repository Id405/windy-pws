use serde::Serialize;

#[derive(Serialize, Clone)]
pub enum ShareOption {
    Open,
    #[serde(rename = "Only Windy")]
    OnlyWindy,
    Private,
}

/// Represents a station as exposed by the Windy API, if you include this struct in an uploaded WindyReport the station with a matching id will have its parameters changed to the given fields
#[derive(Serialize, Clone, Builder)]
pub struct Station {
    station: u32,
    #[serde(rename = "shareOption")]
    #[builder(setter(into, strip_option), default)]
    share_option: Option<ShareOption>,
    #[builder(setter(into, strip_option), default)]
    name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    latitude: Option<f32>,
    #[builder(setter(into, strip_option), default)]
    longitude: Option<f32>,
    #[builder(setter(into, strip_option), default)]
    elevation: Option<f32>,
    #[serde(rename = "tempheight")]
    #[builder(setter(into, strip_option), default)]
    tempmeter_height: Option<f32>,
    #[serde(rename = "windheight")]
    #[builder(setter(into, strip_option), default)]
    windmeter_height: Option<f32>,
}
