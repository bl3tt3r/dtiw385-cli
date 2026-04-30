# dtiw385-cli

[![License](https://img.shields.io/badge/LICENSE-MIT-blue.svg)](./LICENSE)
![Rust Edition](https://img.shields.io/badge/edition-2024-orange)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?logo=rust)
![CI](https://github.com/bl3tt3r/dtiw385-cli/actions/workflows/rust.yml/badge.svg)

A command-line interface for discovering and querying [Orange](https://www.orange.fr/) DTIW385 decoders on your network.

__This project is not affiliated with, endorsed by, or sponsored by Orange.__

---

## 📦 Installation

### Prebuilt Binaries

Download the latest release for your platform from the [GitHub Releases](https://github.com/bl3tt3r/dtiw385-cli/releases) page and extract the archive.

### From crates.io

```bash
cargo install dtiw385-cli
```

### Building from source

1. Install [Rust](https://rustup.rs/)
2. Clone the repository: `git clone https://github.com/bl3tt3r/dtiw385-cli.git`
3. Build: `cargo build --release`
4. The binary is available at `target/release/dtiw385`

---

## Usage

```bash
dtiw385 <COMMAND>
```

## 🚀 Commands

### 🔍 `search`

Searches a range of IPs and ports to discover reachable decoders. Each result is printed as a JSON line to stdout.

```bash
dtiw385 search --ip-range <start>-<end> --port-range <start>-<end>
```

| Flag           | Format                      | Description                            |
| -------------- | --------------------------- | -------------------------------------- |
| `--ip-range`   | `192.168.1.0-192.168.1.255` | Inclusive IPv4 address range to search |
| `--port-range` | `8080-8090`                 | Inclusive port range to search         |

#### 🧪 Example

```bash
dtiw385 search --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090
```

```json
{"ip":"192.168.1.10","port":8080}
{"ip":"192.168.1.42","port":8085}
```

---

### ℹ️ `infos`

Connects to a specific decoder and retrieves its full information.

```bash
dtiw385 infos --ip <address> --port <port>
```

| Flag     | Format         | Description                 |
| -------- | -------------- | --------------------------- |
| `--ip`   | `192.168.1.10` | IPv4 address of the decoder |
| `--port` | `8080`         | Port of the decoder         |

#### 🧪 Example

```bash
dtiw385 infos --ip 192.168.1.10 --port 8080
```

```json
{"ip":"192.168.1.16","port":8080,"playedMediaType":"NA","playedMediaState":"NA","playedMediaId":"","playedMediaContextId":"","playedMediaPosition":"NA","timeShiftingState":"NA","macAddress":"20:9A:7D:D6:56:B7","wolSupport":"0","friendlyName":"Decodeur TV UHD","activeStandbyState":"0","npvrSupport":"0"}
```

---

### 🎮 `press`

Sends a key press to a specific decoder.

```bash
dtiw385 press --ip <address> --port <port> --key <key>
```

| Flag     | Format         | Description                       |
| -------- | -------------- | --------------------------------- |
| `--ip`   | `192.168.1.10` | IPv4 address of the decoder       |
| `--port` | `8080`         | Port of the decoder               |
| `--key`  | `PowerOnOff`   | Key to press (see available keys) |

#### 📋 Available keys

`PowerOnOff`, `Ok`, `Up`, `Down`, `Left`, `Right`, `Back`, `Menu`, `VolumeUp`, `VolumeDown`, `Mute`, `ChannelUp`, `ChannelDown`, `Play`, `Pause`, `Stop`, `Forward`, `Rewind`, `N0`, `N1`, `N2`, `N3`, `N4`, `N5`, `N6`, `N7`, `N8`, `N9`

#### 🧪 Example

```bash
dtiw385 press --ip 192.168.1.10 --port 8080 --key PowerOnOff
```

```json
{"ip":"192.168.1.10","port":8080,"status":"Ok"}
```

---

## 🔗 NDJSON pipeline

All commands speak [NDJSON](https://github.com/ndjson/ndjson-spec) (Newline Delimited JSON): `search` outputs one JSON object per line, and every command can read JSON objects from stdin to fill in its arguments.

This means commands can be **chained directly**.

### ⛓️ Exemple: Turn off every active decoder on the network

```bash
dtiw385 search --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8080 \
  | dtiw385 infos \
  | jq -c 'select(.activeStandbyState == "0")' \
  | dtiw385 press --key PowerOnOff
```

1. `dtiw385 search --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8080` discovers all decoders on the network in range `192.168.1.0-192.168.1.255` for port `8080`
2. `dtiw385 infos` fetches the full state of each one
3. `jq -c 'select(.activeStandbyState == "0")'` use [`jq`](https://jqlang.org/manual/) to filters only the active (non-standby) decoders
4. `dtiw385 press --key PowerOnOff` sends a toggle power key to each of them

---

## ⚙️ Exit codes

| Code | Meaning                                         |
| ---- | ----------------------------------------------- |
| `0`  | All inputs processed successfully               |
| `1`  | Partial success — at least one input failed     |
| `2`  | Fatal — no input was read, or all inputs failed |

---

License: MIT

---

<p align="center">
  Made with ❤️ and Rust 🦀
</p>