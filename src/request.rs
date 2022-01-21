use clap::Parser;
use serde::Serialize;

#[derive(Debug, Builder, Serialize, Parser)]
#[serde(rename_all = "camelCase")]
#[clap(
    author = "Sandro Covo",
    version = "0.1",
    about = "CLI tool to query connections for swiss public transport",
    long_about = None
)]
pub struct ConnectionRequest {
    from: String,
    to: String,
    #[builder(setter(into, strip_option), default)]
    via: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[clap(short, long)]
    date: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[clap(short, long)]
    time: Option<String>,
    #[builder(setter(into, strip_option), default)]
    #[clap(short, long, default_value_t = 0)]
    is_arrival_time: usize,
}
