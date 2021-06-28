//
// This example interprets from a raw mqtt stream of data from an Acurite 5in1 weather station and uploads to windy.com
//

extern crate measurements;
extern crate paho_mqtt as mqtt;
extern crate windy_pws;

use std::{
    env,
    time::{Duration, Instant},
};

use measurements::{Speed, Temperature};
use mqtt::QOS_0;
use serde::Deserialize;

use windy_pws::observation::{Observation, ObservationBuilder};

#[derive(Deserialize, Debug)]
struct WeatherDataPoint {
    time: String, // This data never gets used because the time is system time of the raspberry pi recieving the data? not UTC
    model: String,
    channel: String,
    battery_ok: u32,
    wind_avg_km_h: Option<f32>,
    wind_dir_deg: Option<f32>,
    rain_in: Option<f32>,
    #[serde(rename = "temperature_F")]
    temperature_f: Option<f32>,
    humidity: Option<f32>,
    mic: String,
}

trait IntoObservation {
    fn to_observation(self, station: u32) -> Observation;
}

fn average(iter: impl Iterator<Item = Option<f32>>) -> f32 {
    let collection: Vec<f32> = iter
        .filter(|value| match value {
            Some(_) => true,
            None => false,
        })
        .map(|value| value.unwrap())
        .collect();

    collection.iter().sum::<f32>() / collection.len() as f32
}

impl IntoObservation for Vec<WeatherDataPoint> {
    // TODO rain
    fn to_observation(self, station: u32) -> Observation {
        let wind_data_points: Vec<f32> = self
            .iter()
            .map(|data_point| data_point.wind_avg_km_h)
            .filter(|value| match value {
                Some(_) => true,
                None => false,
            })
            .map(|value| value.unwrap())
            .collect();

        let wind_avg = Speed::from_kilometers_per_hour(
            (wind_data_points.iter().sum::<f32>() / wind_data_points.len() as f32) as f64,
        );

        let gust = Speed::from_kilometers_per_hour(
            *wind_data_points
                .iter()
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap() as f64,
        );

        let wind_direction_avg = average(self.iter().map(|data_point| data_point.wind_dir_deg));

        let temperature = Temperature::from_fahrenheit(average(
            self.iter().map(|data_point| data_point.temperature_f),
        ) as f64);

        let humidity = average(self.iter().map(|data_point| data_point.humidity));

        ObservationBuilder::default()
            .wind(wind_avg.as_meters_per_second() as f32)
            .gust(gust.as_meters_per_second() as f32)
            .wind_direction(wind_direction_avg.round() as i32)
            .temp(temperature.as_celsius() as f32)
            .relative_humidity(humidity)
            .station(station)
            .build()
            .unwrap()
    }
}

fn main() {
    let mut history: Vec<WeatherDataPoint> = vec![];

    let url = env::var("MQTT_URL").unwrap();
    let topic = env::var("MQTT_TOPIC").unwrap();
    let api_key = env::var("WINDY_KEY").unwrap();

    let windy_client = windy_pws::windy_instance::WindyInstance::new(api_key);

    println!("pulling from: {} topic: {}", url, topic);

    let mut mqtt_client = mqtt::Client::new(url).unwrap();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(std::time::Duration::from_secs(20))
        .clean_session(true)
        .finalize();

    let consumer = mqtt_client.start_consuming();

    mqtt_client.connect(conn_opts).unwrap();

    mqtt_client.subscribe(&topic, QOS_0).unwrap();

    let mut last_measurement = Instant::now();

    for msg in consumer.iter() {
        if Instant::now() - last_measurement > Duration::new(300, 0) {
            let observation = history.to_observation(0);
            history = vec![];

            windy_client
                .send_request(windy_pws::windy_request::WindyRequest::from_observation(
                    observation,
                ))
                .unwrap(); // This unwrap is EVIL

            last_measurement = Instant::now();
        }

        match msg {
            Some(msg) => {
                history.push(serde_json::from_str(&msg.payload_str()).unwrap());
            }
            None => {
                if !mqtt_client.is_connected() {
                    std::thread::sleep(std::time::Duration::from_secs(20));
                    mqtt_client.reconnect().unwrap();
                }
            }
        }
    }
}
