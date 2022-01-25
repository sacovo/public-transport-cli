use clap::Parser;
use colored::control;

#[derive(Debug, Parser)]
#[clap(
    author = "Sandro Covo",
    version = "0.1.4",
    about = "CLI tool to query connections for swiss public transport",
    long_about = None
)]
pub struct Args {
    pub from: String,
    pub to: String,
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
    #[clap(
        short,
        long,
        default_value_t = 4,
        help = "Max. number of results, between 1 and 16"
    )]
    pub limit: usize,
    #[clap(short, long, help = "Always output colors")]
    color: bool,
    #[clap(short, long, help = "Never output colors")]
    no_color: bool,
}

impl Args {
    pub(crate) fn set_color(&self) -> () {
        if self.color {
            control::set_override(true);
        }
        if self.no_color {
            control::set_override(false);
        }
    }
}
