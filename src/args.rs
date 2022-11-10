use clap::Parser;
use colored::control;

use crate::config::CONFIG;

#[derive(Debug, Parser)]
#[clap(
    author = "Sandro Covo",
    version = "0.1.5",
    about = "CLI tool to query connections for swiss public transport",
    long_about = None
)]
pub struct Args {
    pub to: String,
    #[clap(short, long, help = "Starting point of journey, can be configured")]
    pub from: Option<String>,
    pub via: Option<String>,
    #[clap(
        short,
        long,
        help = "Date of connection, otherwise the current date is used"
    )]
    pub date: Option<String>,
    #[clap(
        short,
        long,
        help = "Time of connection, otherwise the current time is used"
    )]
    pub time: Option<String>,
    #[clap(
        short,
        long,
        help = "If set the given time is treated as arrival time, otherwise as departure time"
    )]
    pub is_arrival_time: bool,
    #[clap(short, long, help = "Max. number of results, between 1 and 16")]
    pub limit: Option<usize>,
    #[clap(short, long, help = "Always output colors")]
    color: bool,
    #[clap(short, long, help = "Never output colors")]
    no_color: bool,
}

impl Args {
    pub(crate) fn set_color(&self) {
        let config = CONFIG.get().expect("Error accessing config");
        if let Some(color) = config.color {
            control::set_override(color);
        }
        if self.color {
            control::set_override(true);
        }
        if self.no_color {
            control::set_override(false);
        }
    }
}
