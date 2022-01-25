mod api;
mod args;
mod request;

extern crate derive_builder;

use anyhow::Result;

use crate::{api::ConnectionResponse, args::Args, request::ConnectionRequest};
use clap::Parser;

fn main() -> Result<()> {
    let args = Args::parse();
    args.set_color();

    let connection_request = ConnectionRequest::from(args);
    let u: ConnectionResponse = ConnectionResponse::try_from(connection_request)?;

    println!("{}", u);
    Ok(())
}
