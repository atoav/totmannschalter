# totmannschalter

A simple Rust-based service that monitors multiple http(s) endpoints with different intervals and notifies administrators if the service goes down. Another notification is sent once it is back up again. _Totmannschalter_ is German for dead man's switch – a contraption which is meant to halt operations or send a notice in the _absence_ of a certain signal. In trains it is a switch that needs to be tapped periodically to make sure person operating the train is still awake and conscious.

## Features
- per-endpoint configuration for values like interval and contact email
- toml based configuration
- sends you the error message and if the service is back up, information on how long it has been down

## Installation

### Using Rust cargo

1. Make sure you have Rust installed
2. Run `cargo install totmannschalter`

You should now be able to run `totmannschalter` directly from the CLI. If you want to run it as a service (e.g. on system startup) you have to either create something yourself (if you are not on a OS that uses systemd) or install the `totmannschalter.service` file present in the repository like this:

```bash

```


### Using Debian package

Still being worked on