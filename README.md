<p align="center">
 <img src="static/onions.svg">
</p>

<p align="center">
 <img alt="GitHub Last Commit" src="https://img.shields.io/github/last-commit/tuxicorn/onions.watch" />
 <img alt="PR friendly" src="https://img.shields.io/badge/PRs-welcome-brightgreen.svg?style=flat" />
 <img alt="Rust Version" src="https://img.shields.io/badge/Rust-latest-blue" />
 <img alt="Actix Version" src="https://img.shields.io/badge/Actix-4.x-brightgreen" />
</p>
</br>

This project is inspired by [dark.fail](https://dark.fail) and aims to provide a codebase to allow anyone to create their own website that assesses the uptime of popular [onion services](https://community.torproject.org/onion-services/).


It serves content both on the clearnet (default at `http://127.0.0.1:8080`) and over Tor as a hidden service.  
Ensure that your Tor service is running and configured properly before starting the application.  
It uses `mkp224o` for generating `.onion` addresses with custom prefixes.

# Prerequisites 

## Install Tor

**Debian or Ubuntu**
```sh
sudo apt install tor -y
```

**RHEL, CentOS or Fedora**
```sh
sudo dnf install tor -y
```

**Arch Linux and Other Arch-Based Distributions**
```sh
sudo pacman -S tor
```

## Install `mkp224o`

Clone and build `mkp224o` following its README instructions
```sh
git clone https://github.com/cathugger/mkp224o.git
```

Generate a `.onion` address with a custom prefix
```sh
./mkp224o onionswatch
```

Copy the directory for the `.onion` service

```sh
sudo cp -r <generated_onion_dir> /var/lib/tor/onionswatch
```

**Debian or Ubuntu**
```sh
sudo chown -R debian-tor:debian-tor /var/lib/tor/onionswatch
sudo chmod -R u+rwX,go-rwx /var/lib/tor/onionswatch

```

**RHEL, CentOS or Fedora**
```sh
sudo chown -R toranon:toranon /var/lib/tor/onionswatch
sudo chmod -R u+rwX,go-rwx /var/lib/tor/onionswatch

```

**Arch Linux and Other Arch-Based Distributions**
```sh
sudo chown -R tor:tor /var/lib/tor/onionswatch
sudo chmod -R u+rwX,go-rwx /var/lib/tor/onionswatch
```

## Tor Configuration

Update your `/etc/tor/torrc` file
```plaintext
HiddenServiceDir /var/lib/tor/onionswatch/
HiddenServicePort 80 127.0.0.1:8080
```

Restart the Tor service
```sh
sudo systemctl restart tor
```

## Setup and Run

Clone this Repository
```sh
git clone https://github.com/yourusername/onions.watch.git
cd onions.watch
```

Run the Application
```sh
cargo run
```

Access the Onion Service

The `.onion` hostname will be available in:
```sh
cat /var/lib/tor/onionswatch/hostname
```

