# convfmt
[![Actions Status](https://github.com/oriontvv/convfmt/actions/workflows/ci.yml/badge.svg?branch=master)](https://github.com/oriontvv/convfmt/actions/workflows/ci.yml) [![Coverage badge](https://raw.githubusercontent.com/oriontvv/convfmt/coverage/htmlcov/badges/flat.svg)](https://htmlpreview.github.io/?https://github.com/oriontvv/convfmt/coverage/htmlcov/index.html) [![dependency status](https://deps.rs/repo/github/oriontvv/convfmt/status.svg)](https://deps.rs/repo/github/oriontvv/convfmt) [![Crates.io](https://img.shields.io/crates/v/convfmt.svg)](https://crates.io/crates/convfmt)


[convfmt](https://github.com/oriontvv/convfmt) is a command line tool in rust which can convert between formats:
* [bson](https://en.wikipedia.org/wiki/BSON)
* [csv](https://en.wikipedia.org/wiki/Comma-separated_values)
* [dotenv](https://www.dotenv.org/docs/security/env.html)
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
  -f, --from <FROM>         [possible values: bson, csv, dotenv, hjson, hocon, json, json5, jsonl, plist, ron, toml, toon, xml, yaml]
  -t, --to <TO>             [possible values: bson, csv, dotenv, hjson, hocon, json, json5, jsonl, plist, ron, toml, toon, xml, yaml]
  -c, --compact             Compress output if possible (default = false)
  -s, --sort-keys           Sort keys of objects (default = false)
  -i, --ignore-unsupported  Drop values the target format can't represent, e.g. `null` for toml (default = false)
  -h, --help                Print help
  -V, --version             Print version
```

```
$ cat cfg.yml | convfmt -f yaml -t toml > cfg.toml
$ convfmt -f json -t json < compact.json > pretty.json
$ convfmt -f json -t json --sort-keys < unordered.json > sorted.json
$ convfmt -f json -t toml --ignore-unsupported < with-nulls.json > cfg.toml
$ curl https://api.github.com/users/oriontvv | convfmt -f json -t json5 > api.json5
```

By default `convfmt` tries to use `pretty` format. Enable `--compact` option for compression.

By default the original order of keys is preserved. Enable `--sort-keys` option to sort keys of all objects (recursively) in alphabetical order.

**Beware of `null`s, some formats don't support them**: `toml` fails to serialize a `null` and `plist` silently turns it into an empty string. Enable `--ignore-unsupported` option to drop such values.

## Installation:
There are few ways:
* Use the [web version](https://oriontvv.github.io/convfmt/) without installing anything, convert locally in your browser

* Macos [homebrew tap](https://github.com/oriontvv/homebrew-tap)
```
brew install oriontvv/tap/convfmt
```

* Download latest [binary](https://github.com/oriontvv/convfmt/releases)

* Run it from [docker](https://hub.docker.com/r/oriontvv/convfmt) without installing anything (`-i` is required, `convfmt` reads stdin)
```
$ cat cfg.yml | docker run --rm -i oriontvv/convfmt -f yaml -t toml > cfg.toml
```

* Install binary using [cargo-binstall](https://github.com/cargo-bins/cargo-binstall)
```
cargo install cargo-binstall && cargo binstall convfmt
```

* Build local [crate](https://crates.io/crates/convfmt) with [rust](https://www.rust-lang.org/tools/install)
```
cargo install --git https://github.com/oriontvv/convfmt
```