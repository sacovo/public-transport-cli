use std::fmt::{Display, Write};

use chrono::{DateTime, Local};
use pad::PadStr;
use serde::{Deserialize, Serialize};

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
                write!(f, " {: >10}", platform)?;
            }
        }
        Ok(())
    }
}

impl Display for Section {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(departure) = &self.departure {
            write!(f, " {}\n", departure)?;
        };
        write!(f, "   |  \n")?;
        if let Some(arrival) = &self.arrival {
            write!(f, " {}\n", arrival)?;
        }
        write!(f, "\n")
    }
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} - {}\n\n",
            self.from.departure.as_ref().unwrap().format("%H:%M"),
            self.to.arrival.as_ref().unwrap().format("%H:%M"),
        )?;
        for section in &self.sections {
            write!(f, "{}", section)?;
        }
        Ok(())
    }
}

impl Display for ConnectionResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&"".pad_to_width_with_char(60, '*'))?;
        f.write_char('\n')?;
        for (i, connection) in self.connections.iter().enumerate() {
            f.write_char('\n')?;
            write!(f, "#{}: {}", i + 1, connection)?;
            f.write_str(&"".pad_to_width_with_char(60, '*'))?;
            f.write_char('\n')?;
        }

        write!(f, "")
    }
}
