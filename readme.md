# dtiw385-cli

[![License](https://img.shields.io/badge/LICENSE-MIT-blue.svg)](./LICENSE)
![Rust Edition](https://img.shields.io/badge/edition-2024-orange)
![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-000000?logo=rust)
![CI](https://github.com/bl3tt3r/dtiw385-cli/actions/workflows/rust.yml/badge.svg)

A command-line interface for discovering and querying [Orange](https://www.orange.fr/) DTIW385 decoders on your network, built on the [`dtiw385`](https://crates.io/crates/dtiw385) library.

__This project is not affiliated with, endorsed by, or sponsored by Orange.__

---

## 📦 Installation

```bash
cargo install dtiw385-cli
```

---

## 🚀 Commands

### 🔍 `Search`

Searchs a range of IPs and ports to discover reachable decoders. Each result is printed as a JSON line to stdout.

```bash
dtiw385 Search --ip-range <start>-<end> --port-range <start>-<end>
```

| Flag           | Format                      | Description                            |
| -------------- | --------------------------- | -------------------------------------- |
| `--ip-range`   | `192.168.1.0-192.168.1.255` | Inclusive IPv4 address range to Search |
| `--port-range` | `8080-8090`                 | Inclusive port range to Search         |

#### 🧪 Example

```bash
dtiw385 Search --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090
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
{"ip":"192.168.1.10","port":8080,"firmware":"1.4.2","uptime":3600}
```

---

## 🔗 NDJSON pipeline

Both commands speak [NDJSON](https://github.com/ndjson/ndjson-spec) (Newline Delimited JSON): `Search` outputs one JSON object per line, and every command can read JSON objects from stdin to fill in its arguments.

This means commands can be **chained directly**.

### ⛓️ Chain `Search` → `infos`

```bash
dtiw385 Search --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090 \
  | dtiw385 infos
```

`Search` discovers decoders and streams their `ip` and `port` as JSON. `infos` reads each line and queries each decoder in turn.

### 🏳️ CLI flags override stdin

CLI flags always take priority over values coming from stdin. For example, force a specific port regardless of what the Search reported:

```bash
dtiw385 Search --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090 \
  | dtiw385 infos --port 9000
```

### 📄 Feed JSON manually

```bash
# single decoder
echo '{"ip":"192.168.1.10","port":8080}' | dtiw385 infos

# list of known decoders
cat decoders.ndjson | dtiw385 infos
```

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