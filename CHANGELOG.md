# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

### Added
- Add optional `defmt-03` feature implementing `defmt::Format` on some types.

## [0.3.2] - 2024-03-25

### Added
- Expose `DateTime` type to be used as replacement for some deprecated methods.

## [0.3.1] - 2024-02-01

### Added
- Implemented common traits for `Hours`.

### Changed
- Raised MSRV to 1.62.0.

## [0.3.0] - 2022-02-19

### Changed
- [breaking-change] Extract the `datetime()` and `set_datetime()` methods into a
  separate `DateTimeAccess` trait and recommend using only that trait.
- [breaking-change] Remove `get_` from all method names to comply with the Rust API guidelines.

## [0.2.1] - 2021-06-05

### Changed
- Bumped minimum version of `chrono` to solve issues with `no-std`. Thanks to @Piroro-hs.

## [0.2.0] - 2020-02-09
### Changed
- Changed `get_datetime()` and `set_datetime()` parameter from `DateTime`
  to `chrono::NaiveDateTime`.

### Added
- Methods to set and get date and time using `chrono::NaiveDate` and `chrono::NaiveTime`:
    - `get_time()`
    - `set_time()`
    - `get_date()`
    - `set_date()`
- `chrono` dependency.

### Removed
- `DateTime` data structure was replaced by `chrono::NaiveDateTime`.

## [0.1.0] - 2019-09-15

This is the initial release to crates.io. There may be some API changes in the
future. All changes will be documented in this CHANGELOG.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/rtcc-rs/compare/v0.3.2...HEAD
[0.3.2]: https://github.com/eldruin/rtcc-rs/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/eldruin/rtcc-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/eldruin/rtcc-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/eldruin/rtcc-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/eldruin/rtcc-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/eldruin/rtcc-rs/releases/tag/v0.1.0
