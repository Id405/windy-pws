use serde::Serialize;

use crate::station::Station;
use crate::observation::Observation;

/// Represents a single request made to the Windy PWS Api
#[derive(Serialize)]
pub struct WindyRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    stations: Option<Vec<Station>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    observations: Option<Vec<Observation>>,
}

impl WindyRequest {
    pub fn from_observation(observation: Observation) -> Self {
        WindyRequestBuilder::from_observation(observation).build()
    }

    pub fn from_observations(observations: Vec<Observation>) -> Self {
        WindyRequestBuilder::from_observations(observations).build()
    }

    pub fn from_station(station: Station) -> Self {
        WindyRequestBuilder::from_station(station).build()
    }

    pub fn from_stations(stations: Vec<Station>) -> Self {
        WindyRequestBuilder::from_stations(stations).build()
    }

    pub fn from_station_observation(station: Station, observation: Observation) -> Self {
        WindyRequestBuilder::from_station_observation(station, observation).build()
    }

    pub fn from_stations_observationss(stations: Vec<Station>, observations: Vec<Observation>) -> Self {
        WindyRequestBuilder::from_stations_observations(stations, observations).build()
    }
}

/// A builder to easily build a request to the Windy Api
#[derive(Default, Clone)]
pub struct WindyRequestBuilder {
    stations: Option<Vec<Station>>,
    observations: Option<Vec<Observation>>,
}

impl WindyRequestBuilder {
    pub fn from_observation(observation: Observation) -> Self {
        Self::default().observation(observation).clone()
    }

    pub fn from_observations(observations: Vec<Observation>) -> Self {
        Self::default().observations(observations).clone()
    }

    pub fn from_station(station: Station) -> Self {
        Self::default().station(station).clone()
    }

    pub fn from_stations(stations: Vec<Station>) -> Self {
        Self::default().stations(stations).clone()
    }

    pub fn from_station_observation(station: Station, observation: Observation) -> Self {
        Self::default().station(station).observation(observation).clone()
    }

    pub fn from_stations_observations(stations: Vec<Station>, observations: Vec<Observation>) -> Self {
        Self::default().stations(stations).observations(observations).clone()
    }

    pub fn observation(&mut self, observation: Observation) -> &mut Self {
        match self.observations.as_mut() {
            Some(observations) => observations.push(observation),
            None => self.observations = Some(vec![observation]),
        }
        self
    }

    pub fn station(&mut self, station: Station) -> &mut Self {
        match self.stations.as_mut() {
            Some(stations) => stations.push(station),
            None => self.stations = Some(vec![station]),
        }
        self
    }

    /// if self.stations exists, appends to it
    pub fn stations(&mut self, stations_new: Vec<Station>) -> &mut Self {
        match self.stations.as_mut() {
            Some(stations) => stations.extend(stations_new),
            None => self.stations = Some(stations_new),
        }
        self
    }

    /// if self.observations exists, appends to it
    pub fn observations(&mut self, observations_new: Vec<Observation>) -> &mut Self {
        match self.observations.as_mut() {
            Some(observations) => observations.extend(observations_new),
            None => self.observations = Some(observations_new),
        }
        self
    }

    pub fn build(&self) -> WindyRequest {
        WindyRequest {
            stations: self.stations.clone(),
            observations: self.observations.clone(),
        }
    }
}