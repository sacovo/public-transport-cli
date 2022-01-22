use std::fmt::{Display, Write};

use chrono::{DateTime, Duration, Local};
use pad::PadStr;
use serde::{Deserialize, Serialize};

use colored::{ColoredString, Colorize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    id: String,
    name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checkpoint {
    station: Location,
    arrival: Option<DateTime<Local>>,
    departure: Option<DateTime<Local>>,
    platform: Option<String>,
    delay: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Journey {
    name: String,
    category: String,
    number: String,
    to: String,
    pass_list: Vec<Checkpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Walk {
    duration: Option<usize>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    journey: Option<Journey>,
    walk: Option<Walk>,
    departure: Option<Checkpoint>,
    arrival: Option<Checkpoint>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Connection {
    from: Checkpoint,
    to: Checkpoint,
    sections: Vec<Section>,
}

impl Connection {
    fn time_until(&self) -> Duration {
        match self.from.departure {
            Some(departure) => departure.signed_duration_since(Local::now()),
            None => Duration::seconds(0),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionResponse {
    connections: Vec<Connection>,
    from: Location,
    to: Location,
}
impl Display for Location {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Display for Checkpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(time) = self.departure.as_ref().or(self.arrival.as_ref()) {
            write!(f, "{}: ", time.format("%H:%M"))?;
            write!(f, "{: <40}", self.station.name)?;
            if let Some(platform) = self.platform.as_ref() {
                write!(f, " {: >8}   ", platform)?;
            } else {
                f.write_str(&" ".pad_to_width_with_char(12, ' '))?;
            }
        }
        Ok(())
    }
}

fn print_checkpoint(checkpoint: &Checkpoint) -> ColoredString {
    format!(" {}", checkpoint).white().on_blue()
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(departure) = &self.departure {
            write!(f, "{}\n", print_checkpoint(&departure))?;
        };
        write!(f, "{}", "   ┃  \n".cyan())?;
        if let Some(arrival) = &self.arrival {
            write!(f, "{}\n", print_checkpoint(&arrival))?;
        }
        write!(f, "\n")
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let duration = self.time_until();
        write!(
            f,
            "{} - {}, Leaves in  {:02}:{:02}\n\n",
            self.from.departure.as_ref().unwrap().format("%H:%M"),
            self.to.arrival.as_ref().unwrap().format("%H:%M"),
            duration.num_hours(),
            (duration.num_minutes() % 60).abs(),
        )?;
        for section in &self.sections {
            write!(f, "{}", section)?;
        }
        Ok(())
    }
}

impl Display for ConnectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!(
            "{}\n",
            &format!("{} ⇒ {}", self.from.name, self.to.name).bold()
        ))?;
        f.write_str(&"".pad_to_width_with_char(60, '━'))?;
        for (i, connection) in self.connections.iter().enumerate() {
            f.write_char('\n')?;
            write!(f, "#{}: {}", i + 1, connection)?;
            f.write_str(&"".pad_to_width_with_char(60, '━'))?;
        }

        write!(f, "")
    }
}
