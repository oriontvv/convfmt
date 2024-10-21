# convfmt
[![Actions Status](https://github.com/oriontvv/convfmt/workflows/CI/badge.svg)](https://github.com/oriontvv/convfmt/actions) [![Coverage badge](https://raw.githubusercontent.com/oriontvv/convfmt/coverage/htmlcov/badges/flat.svg)](https://htmlpreview.github.io/?https://github.com/oriontvv/convfmt/coverage/htmlcov/index.html) [![dependency status](https://deps.rs/repo/github/oriontvv/convfmt/status.svg)](https://deps.rs/repo/github/oriontvv/convfmt) [![Crates.io](https://img.shields.io/crates/v/convfmt.svg)](https://crates.io/crates/convfmt)


[convfmt](https://github.com/oriontvv/convfmt) is a command line tool which can convert between formats:
* [json](https://en.wikipedia.org/wiki/JSON)
* [yaml](https://en.wikipedia.org/wiki/YAML)
* [toml](https://en.wikipedia.org/wiki/TOML)
* [ron](https://github.com/ron-rs/ron)
* [json5](https://en.wikipedia.org/wiki/JSON5)
* [bson](https://en.wikipedia.org/wiki/BSON)

## Usage:

```bash
$ cat cfg.toml | convfmt -f toml -t yaml > cfg.yml
$ convfmt -f json -t json < compact.json > pretty.json
$ curl https://api.github.com/users/oriontvv | convfmt -f json -t json5 > api.json5
```

By default `convfmt` uses `pretty` format(can be disabled with `--compact` option).
Beware of `null`s, some formats don't support them (e.g. toml).

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
* [json5](https://crates.io/crates/json5)
* [bson](https://crates.io/crates/bson)
