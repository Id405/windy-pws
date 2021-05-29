use crate::windy_request::WindyRequest;
use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum RequestError {
    #[snafu(display("send_request failed: failed with status error {}", status))]
    StatusError { status: u16 },
    #[snafu(display("send_request failed: timeout (check your connection to stations.windy.com)"))]
    TimeOutError,
    #[snafu(display("send_request failed: {}", error))]
    OtherError { error: reqwest::Error },
}

pub struct WindyInstance {
    api_key: String,
    client: reqwest::blocking::Client,
}

impl WindyInstance {
    pub fn new(api_key: String) -> Self {
        Self {
            api_key,
            client: reqwest::blocking::Client::new(),
        }
    }

    pub fn send_request(&self, data: WindyRequest) -> Result<(), RequestError> {
        let url = format!("https://stations.windy.com/pws/update/{}", self.api_key);
        let result = self
            .client
            .post(url)
            .body(
                serde_json::to_string(&data)
                    .expect("send_request failed: failed to serialize WindyRequest to string"),
            )
            .send();
        match result {
            Ok(_) => Ok(()),
            Err(error) => {
                if error.is_status() {
                    let status = error.status().unwrap().as_u16();
                    return Err(RequestError::StatusError { status });
                }

                if error.is_timeout() {
                    return Err(RequestError::TimeOutError);
                }

                Err(RequestError::OtherError { error }) // this is like some terrible error handling but I'm not very happy with how reqwest gives the error, I'd prefer to refactor all of this code and move away from reqwest to fix this
            }
        }
    }
}
