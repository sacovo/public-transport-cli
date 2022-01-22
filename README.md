# Swiss Public Transport CLI

This is a simple tool to search public transport connections from one station to another, possible with a via.

It uses the [public transportation data](https://transport.opendata.ch/) from https://opendata.ch and is written in rust.


## Installation

```
cargo install spt_cli
```

## How to use:

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
