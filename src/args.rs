use clap::Parser;
use colored::control;

#[derive(Debug, Parser)]
#[clap(
    author = "Sandro Covo",
    version = "0.1.3",
    about = "CLI tool to query connections for swiss public transport",
    long_about = None
)]
pub struct Args {
    pub from: String,
    pub to: String,
    pub via: Option<String>,
    #[clap(short, long)]
    pub date: Option<String>,
    #[clap(short, long)]
    pub time: Option<String>,
    #[clap(short, long)]
    pub is_arrival_time: bool,
    #[clap(short, long)]
    color: bool,
    #[clap(short, long)]
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
