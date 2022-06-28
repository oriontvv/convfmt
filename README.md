# convfmt
A command line tool which can convert formats:
* json
* yaml
* toml

## Usage:

```bash
$ convfmt < cfg.json > cfg.yml --from json --to yaml
$ cat cfg.toml | convfmt -f toml -t json --compact > cfg.json
```

Some formats allow to use `compact` and `pretty`(default) options

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
