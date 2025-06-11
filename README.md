# convfmt
[![Actions Status](https://github.com/oriontvv/convfmt/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/oriontvv/convfmt/actions/workflows/ci.yml) [![Coverage badge](https://raw.githubusercontent.com/oriontvv/convfmt/coverage/htmlcov/badges/flat.svg)](https://htmlpreview.github.io/?https://github.com/oriontvv/convfmt/coverage/htmlcov/index.html) [![dependency status](https://deps.rs/repo/github/oriontvv/convfmt/status.svg)](https://deps.rs/repo/github/oriontvv/convfmt) [![Crates.io](https://img.shields.io/crates/v/convfmt.svg)](https://crates.io/crates/convfmt)


[convfmt](https://github.com/oriontvv/convfmt) is a command line tool which can convert between formats:
* [json](https://en.wikipedia.org/wiki/JSON)
* [yaml](https://en.wikipedia.org/wiki/YAML)
* [toml](https://en.wikipedia.org/wiki/TOML)
* [ron](https://github.com/ron-rs/ron)
* [json5](https://en.wikipedia.org/wiki/JSON5)
* [bson](https://en.wikipedia.org/wiki/BSON)
* [xml](https://en.wikipedia.org/wiki/XML)
* [hocon](https://github.com/lightbend/config/blob/main/HOCON.md) (from only)

## Usage:

```bash
$ cat cfg.yml | convfmt -f yaml -t toml > cfg.toml
$ convfmt -f json -t json < compact.json > pretty.json
$ curl https://api.github.com/users/oriontvv | convfmt -f json -t json5 > api.json5
```

By default `convfmt` uses `pretty` format(can be disabled with `--compact` option).
Beware of `null`s, some formats don't support them (e.g. toml).

## Installation:
* Download built [binary](https://github.com/oriontvv/convfmt/releases)

* Install binary using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
```bash
cargo install cargo-binstall && cargo binstall convfmt
```

* Build from sources with [rust](https://www.rust-lang.org/tools/install)
```bash
cargo install convfmt
```

## Many thanks to:
This tool stands on the shoulders of such giants:
* [serde](https://crates.io/crates/serde)
* [serde_json](https://crates.io/crates/serde_json)
* [serde_yaml](https://crates.io/crates/serde_yaml)
* [toml-rs](https://crates.io/crates/toml)
* [ron](https://crates.io/crates/ron)
* [json5](https://crates.io/crates/json5)
* [bson](https://crates.io/crates/bson)
* [xml](https://crates.io/crates/quick-xml)
* [hocon](https://crates.io/crates/hocon)
