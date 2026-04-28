# dtiw385

A fast command-line interface for discovering and querying DTIW385 decoders on a network, built on the [`dtiw385`](https://crates.io/crates/dtiw385) library.

## Features

- **Scan** a range of IPs and ports to discover decoders on your network
- **Query** a specific decoder for detailed information
- **NDJSON pipeline** — chain commands together using newline-delimited JSON streams
- **Flexible input** — arguments can be passed via CLI flags or piped from stdin as JSON

## Installation

```bash
cargo install dtiw385-cli
```

## Commands

### `scan`

Scans an IP range and a port range to discover reachable decoders. Each discovered decoder is printed as a JSON line to stdout.

```bash
dtiw385 scan --ip-range <start>-<end> --port-range <start>-<end>
```

**Arguments**

| Flag           | Format                      | Description                          |
| -------------- | --------------------------- | ------------------------------------ |
| `--ip-range`   | `192.168.1.0-192.168.1.255` | Inclusive IPv4 address range to scan |
| `--port-range` | `8080-8090`                 | Inclusive port range to scan         |

**Example**

```bash
dtiw385 scan --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090
```

```json
{"ip":"192.168.1.10","port":8080,"id":"abc123"}
{"ip":"192.168.1.42","port":8085,"id":"def456"}
```

---

### `infos`

Connects to a specific decoder and retrieves its full information.

```bash
dtiw385 infos --ip <address> --port <port>
```

**Arguments**

| Flag     | Format         | Description                 |
| -------- | -------------- | --------------------------- |
| `--ip`   | `192.168.1.10` | IPv4 address of the decoder |
| `--port` | `8080`         | Port of the decoder         |

**Example**

```bash
dtiw385 infos --ip 192.168.1.10 --port 8080
```

```json
{"ip":"192.168.1.10","port":8080,"id":"abc123","firmware":"1.4.2","uptime":3600}
```

---

## NDJSON pipeline

Both commands speak [NDJSON](https://github.com/ndjson/ndjson-spec) (Newline Delimited JSON): `scan` outputs one JSON object per line, and every command can read JSON objects from stdin to fill in its arguments.

This means commands can be **chained directly** — the output of `scan` feeds naturally into `infos`.

### How stdin merging works

When stdin is not a terminal, the CLI reads one JSON line at a time and merges it with the arguments already provided on the command line. **CLI flags always take priority** over values coming from stdin.

For each input line, the command is executed independently, which makes it straightforward to process multiple decoders in a single pipeline.

### Chaining scan → infos

```bash
dtiw385 scan --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090 \
  | dtiw385 infos
```

`scan` discovers decoders and streams their `ip` and `port` as JSON. `infos` reads each line from stdin, extracts the fields it needs, and queries each decoder in turn.

### Overriding stdin values with CLI flags

CLI flags always win over values coming from stdin. This lets you force a specific port for all decoders found by a scan, regardless of what the scan reported:

```bash
dtiw385 scan --ip-range 192.168.1.0-192.168.1.255 --port-range 8080-8090 \
  | dtiw385 infos --port 9000
```

### Feeding JSON manually

You can also feed JSON directly without using `scan`:

```bash
echo '{"ip":"192.168.1.10","port":8080}' | dtiw385 infos
```

Or process a list of known decoders from a file:

```bash
cat decoders.ndjson | dtiw385 infos
```

---

## Exit codes

| Code | Meaning                                         |
| ---- | ----------------------------------------------- |
| `0`  | All inputs processed successfully               |
| `1`  | Partial success — at least one input failed     |
| `2`  | Fatal — no input was read, or all inputs failed |

The distinction between `1` and `2` makes it easy to detect partial failures in scripts.

---

## License

MIT