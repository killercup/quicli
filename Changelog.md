# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.2] - 2018-01-28

### Added

- [A website with guides!](https://killercup.github.io/quicli/)
- `glob`
- `create_dir`
- Re-export Rayon traits
- Export `Result` type alias using failure's Error

## Removed

- All the examples are now guides

## Changed

- `main!` now sets up logging in all cases
- Use buffered reading/writing in fs functions

## [0.1.1] - 2018-01-28

### Added

- Re-export log macros
- Automatically set up env_logger in main!
- `main!` parameter for Cli struct and its logging level field
- Readme fixes
- Expose fs module

## [0.1.0] - 2018-01-28

### Added

- `main!` macro
- Re-exports of failure, serde, structopt
- Commit Message generator example
- read/write file functions

[Unreleased]: https://github.com/killercup/quicli/compare/v1.0.0...HEAD
[0.1.2]: https://github.com/killercup/quicli/compare/v0.1.0...v0.1.2
[0.1.1]: https://github.com/killercup/quicli/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/killercup/quicli/compare/cb747195866d2a240ab8154d00facfead3e55a9e...v0.1.0
