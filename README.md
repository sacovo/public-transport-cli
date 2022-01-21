# Swiss Public Transport CLI

This is a simple tool to search public transport connections from one station to another, possible with a via.

It uses the [public transportation data](https://transport.opendata.ch/) from https://opendata.ch and is written in rust.

## How to use:

```sh
# Simple
spt_cli Zürich Bern
# Specify a time
spt_cli --time 10:00 Basel Chur
# Use a via
spt_cli Basel Genv Biel
spt_cli --help # show all options
```

## Contribution

Issues and merge requests are welcome
