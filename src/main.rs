mod api;
mod request;

#[macro_use]
extern crate derive_builder;

use anyhow::{Context, Result};

use crate::{api::ConnectionResponse, request::ConnectionRequest};
use clap::Parser;
use pager::Pager;

fn main() -> Result<()> {
    Pager::new().setup();
    let client = reqwest::blocking::Client::new();

    let connection_request = ConnectionRequest::parse();

    let response = client
        .get("http://transport.opendata.ch/v1/connections")
        .query(&connection_request)
        .send()
        .context("Could not make query")?;

    let u: ConnectionResponse =
        serde_json::from_str(&response.text().context("Error reading response")?)
            .context("No connections found")?;

    println!("{}", u);
    Ok(())
}
