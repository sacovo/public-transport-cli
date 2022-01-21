use clap::Parser;
use serde::Serialize;

#[derive(Debug, Builder, Serialize, Parser)]
#[serde(rename_all = "camelCase")]
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
