# convfmt
[![Actions Status](https://github.com/oriontvv/convfmt/workflows/CI/badge.svg)](https://github.com/oriontvv/convfmt/actions) [![dependency status](https://deps.rs/repo/github/oriontvv/convfmt/status.svg)](https://deps.rs/repo/github/oriontvv/convfmt) [![Crates.io](https://img.shields.io/crates/v/convfmt.svg)](https://crates.io/crates/convfmt)

[convfmt](https://github.com/oriontvv/convfmt) is a command line tool which can convert between formats:
* [json](https://en.wikipedia.org/wiki/JSON)
* [yaml](https://en.wikipedia.org/wiki/YAML)
* [toml](https://en.wikipedia.org/wiki/TOML)
* [ron](https://github.com/ron-rs/ron)
* [cbor](https://en.wikipedia.org/wiki/CBOR)

## Usage:

```bash
$ cat cfg.toml | convfmt -f toml -t yaml > cfg.yml
$ convfmt -f json -t json < compact.json > pretty.json
$ curl https://api.github.com/users/oriontvv | convfmt -f json -t cbor > api.cbor
```

By default `convfmt` uses `pretty` format, but it can be disabled with `--compact` option.
Beware `null`s, some formats dont support them (e.g. toml).

## Installation:
```
cargo install convfmt
```

## Many thanks to:
This tool stands on the shoulders of such giants:
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [serde_yaml](https://crates.io/crates/serde_yaml)
* [toml-rs](https://crates.io/crates/toml)
* [ron](https://crates.io/crates/ron)
* [serde_cbor](https://crates.io/crates/serde_cbor)
