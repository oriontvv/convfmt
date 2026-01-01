# convfmt
[![Actions Status](https://github.com/oriontvv/convfmt/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/oriontvv/convfmt/actions/workflows/ci.yml) [![Coverage badge](https://raw.githubusercontent.com/oriontvv/convfmt/coverage/htmlcov/badges/flat.svg)](https://htmlpreview.github.io/?https://github.com/oriontvv/convfmt/coverage/htmlcov/index.html) [![dependency status](https://deps.rs/repo/github/oriontvv/convfmt/status.svg)](https://deps.rs/repo/github/oriontvv/convfmt) [![Crates.io](https://img.shields.io/crates/v/convfmt.svg)](https://crates.io/crates/convfmt)


[convfmt](https://github.com/oriontvv/convfmt) is a command line tool in rust which can convert between formats:
* [bson](https://en.wikipedia.org/wiki/BSON)
* [csv](https://en.wikipedia.org/wiki/Comma-separated_values)
* [hjson](https://hjson.github.io/)
* [hocon](https://github.com/lightbend/config/blob/main/HOCON.md)
* [json](https://en.wikipedia.org/wiki/JSON)
* [json5](https://en.wikipedia.org/wiki/JSON5)
* [jsonl](https://jsonltools.com/what-is-jsonl)
* [plist](https://en.wikipedia.org/wiki/Property_list)
* [ron](https://github.com/ron-rs/ron)
* [toml](https://en.wikipedia.org/wiki/TOML)
* [toon](https://toonformat.dev/)
* [xml](https://en.wikipedia.org/wiki/XML)
* [yaml](https://en.wikipedia.org/wiki/YAML)

## Usage:

```
$ convfmt --help
cli tool which can convert different formats

Usage: convfmt [OPTIONS] --from <FROM> --to <TO>

Options:
  -f, --from <FROM>  [possible values: bson, csv, hjson, hocon, json, json5, jsonl, plist, ron, toml, toon, xml, yaml]
  -t, --to <TO>      [possible values: bson, csv, hjson, hocon, json, json5, jsonl, plist, ron, toml, toon, xml, yaml]
  -c, --compact
  -h, --help         Print help
  -V, --version      Print version
```

```
$ cat cfg.yml | convfmt -f yaml -t toml > cfg.toml
$ convfmt -f json -t json < compact.json > pretty.json
$ curl https://api.github.com/users/oriontvv | convfmt -f json -t json5 > api.json5
```

By default `convfmt` uses `pretty` format. Can't be compacted with `--compact` option.
Beware of `null`s, some formats don't support them (e.g. toml).

## Installation:
* Download latest [binary](https://github.com/oriontvv/convfmt/releases)

* Install binary using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
```
cargo install cargo-binstall && cargo binstall convfmt
```

* Build from sources with [rust](https://www.rust-lang.org/tools/install)
```
cargo install convfmt
```