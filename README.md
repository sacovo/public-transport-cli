# Swiss Public Transport CLI

This is a simple tool to search public transport connections from one station to another, possible with a via.

It uses the [public transportation data](https://transport.opendata.ch/) from https://opendata.ch and is written in rust.


## Installation

```
cargo install spt_cli
```

## Usage

```
    spt_cli [OPTIONS] <FROM> <TO> [VIA]

ARGS:
    <FROM>    
    <TO>      
    <VIA>     

OPTIONS:
    -c, --color              Always output colors
    -d, --date <DATE>        Date of connection, otherwise the current date is used
    -h, --help               Print help information
    -i, --is-arrival-time    If set the given time is treated as arrival time, otherwise as
                             departure time
    -l, --limit <LIMIT>      Max. number of results, between 1 and 16 [default: 4]
    -n, --no-color           Never output colors
    -t, --time <TIME>        Time of connection, otherwise the current time is used
    -V, --version            Print version information

```

### Examples

```sh
# Simple
spt_cli Zürich Bern

# Specify a time
spt_cli --time 10:00 Basel Chur

# Use a via
spt_cli Basel Genve Biel

spt_cli --help # show all options
```


## Screen Recording

[![asciicast](https://asciinema.org/a/ssnn0cybYBETohWaVOLe03S60.svg)](https://asciinema.org/a/ssnn0cybYBETohWaVOLe03S60)

## Contribution

Issues and merge requests are welcome
