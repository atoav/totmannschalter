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

You should now be able to run `totmannschalter` directly from the CLI. If you want to run it as a system service (e.g. on system startup) you have to either create something yourself (if you are not on a OS that uses systemd) or install the `totmannschalter.service` file present in the repository like this:

```bash
# Run as root so the config will be created in the right place
sudo totmannschalter

# Setup the config (your path could be different!)
sudo vim /etc/totmannschalter/config.toml

# Download the service file
wget https://raw.githubusercontent.com/atoav/totmannschalter/master/totmannschalter.service

# Find out where your totmannschalter is installed
which totmannschalter

# Use the output from the which command above and replace the path 
# right of ExecStar in the systemd unit file:
vim totmannschalter.service

# Copy the unit file to your services
sudo cp totmannschalter.services /etc/systemd/system/

# Enable the service (so it runs automatically after the next system startup)
sudo systemctl enable totmannschalter

# Start the service (so it starts right away)
sudo systemctl start totmannschalter

# Check what is going on
journalctl -fu totmannschalter
```


### Using Debian package (.deb)

#### Build the package yourself
1. Make sure you have `cargo` installed
2. Install `cargo-deb` using `cargo install cargo-deb`
3. Run `cargo deb` in the root of the repository
4. Go to `target/debian/`
5. Install the package using `sudo dpkg -i totmanschalter<TAB>`
6. Make sure to run `sudo totmannschalter` at least once before enabling the service, so a default config file can be created. Alternatively you could also run totmannschalter as a regular user service without root then you would have to create your own user service file and skip the rest of the steps
7. Enable the service: `sudo systemctl enable totmannschalter`
8. Start the service `sudo systemctl start totmannschalter`
9. Check the status of the service: `systemctl status totmannschalter` or `journalctl -fu totmannschalter`

#### Use a release build

See the github releases page : )