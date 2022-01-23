mod api;
mod args;
mod request;

extern crate derive_builder;

use anyhow::{Context, Result};

use crate::{api::ConnectionResponse, args::Args, request::ConnectionRequest};
use clap::Parser;

fn main() -> Result<()> {
    let client = reqwest::blocking::Client::new();

    let args = Args::parse();
    args.set_color();

    let connection_request = ConnectionRequest::from(args);

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
