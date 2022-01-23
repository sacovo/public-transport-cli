use serde::Serialize;

use crate::args::Args;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionRequest {
    from: String,
    to: String,
    via: Option<String>,
    date: Option<String>,
    time: Option<String>,
    is_arrival_time: usize,
}

impl From<Args> for ConnectionRequest {
    fn from(args: Args) -> Self {
        ConnectionRequest {
            from: args.from,
            to: args.to,
            via: args.via,
            date: args.date,
            time: args.time,
            is_arrival_time: if args.is_arrival_time { 1 } else { 0 },
        }
    }
}
