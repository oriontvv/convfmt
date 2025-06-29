# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0] - 2025-06-29
### Added
- `hjson` format #132
- upgrade to 2024 edition

## [0.1.9] - 2025-06-11
### Added
- `xml` format #106

## [0.1.8] - 2025-06-07
### Added
- `hocon` format (as input format only) #130

## [0.1.7] - 2025-01-15
### Added
- `bson` format
- binstall support for installation
### Fixed
- Update dependencies

## [0.1.6] - 2023-09-28
### Fixed
- Issue with toml
- Update dependencies

## [0.1.5] - 2022-09-18
### Added
- `json5` format
- Using keep changelog format
- Building windows-installer in release job
### Fixed
- Fix indentation for ron on windows [24](https://github.com/oriontvv/convfmt/pull/24)
### Removed
- `cbor` format
- homebrew packaging

## [0.1.3]  - 2022-07-17
### Added
- homebrew packaging

## [0.1.2] - 2022-07-15
### Added
- support of `compact` option for toml
- `ron` format
- `cbor` format

## [0.1.1] - 2022-06-28
### Added
- more tests

## [0.1.0] - 2022-06-28
### Added
- initial version with support of `json`, `yaml` and `toml`
- json supports `compact` option
