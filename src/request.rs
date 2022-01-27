use anyhow::{Context, Error};
use serde::Serialize;

use crate::{api::ConnectionResponse, args::Args, config::CONFIG};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionRequest {
    from: String,
    to: String,
    via: Option<String>,
    date: Option<String>,
    time: Option<String>,
    is_arrival_time: usize,
    limit: usize,
    page: usize,
}

impl From<Args> for ConnectionRequest {
    fn from(args: Args) -> Self {
        let config = CONFIG.get().expect("Error accessing config");
        ConnectionRequest {
            from: args
                .from
                .as_ref()
                .or(config.from.as_ref())
                .expect(
                    "No 'from' specified, either specify one with --from or in the config file.",
                )
                .clone(),
            to: args.to,
            via: args.via,
            date: args.date,
            time: args.time,
            is_arrival_time: if args.is_arrival_time { 1 } else { 0 },
            limit: args.limit.or(config.limit).unwrap_or(4),
            page: 0,
        }
    }
}

impl TryFrom<ConnectionRequest> for ConnectionResponse {
    type Error = Error;

    fn try_from(request: ConnectionRequest) -> Result<Self, Self::Error> {
        let client = reqwest::blocking::Client::new();
        let response = client
            .get("http://transport.opendata.ch/v1/connections")
            .query(&request)
            .send()
            .context("Could not make query")?;

        let response = serde_json::from_str(&response.text().context("Error reading response")?)
            .context("No connections found")?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::ConnectionResponse;

    use super::ConnectionRequest;

    #[test]
    fn make_request() {
        // given
        let request = ConnectionRequest {
            to: "Zürich".to_string(),
            from: "Basel".to_string(),
            via: None,
            date: None,
            time: None,
            is_arrival_time: 0,
            limit: 10,
            page: 0,
        };

        // when
        let response = ConnectionResponse::try_from(request).unwrap();

        // then
        assert_eq!(response.from_name(), "Basel SBB");
        assert_eq!(response.to_name(), "Zürich HB");
    }
}
