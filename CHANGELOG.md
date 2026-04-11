# Changelog

All notable changes to this project will be documented in this file.

<!-- markdownlint-disable MD024 -->

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->

## [Unreleased] - ReleaseDate

## [0.2.2] - 2025-03-01

### Fixed

- Fix macOS build compatibility by avoiding unpacking `passwd` fields directly. ([#24](https://github.com/gifnksm/etc-passwd/pull/24))

### Changed

- Bump MSRV from 1.56.1 to 1.74.1. ([#7](https://github.com/gifnksm/etc-passwd/pull/7))
- Add `Cargo.lock` for CI testing.
- Update CI and maintenance dependencies. ([#10](https://github.com/gifnksm/etc-passwd/pull/10), [#16](https://github.com/gifnksm/etc-passwd/pull/16), [#22](https://github.com/gifnksm/etc-passwd/pull/22))
- Update `libc` through a series of compatible releases up to `0.2.170`. ([#9](https://github.com/gifnksm/etc-passwd/pull/9), [#11](https://github.com/gifnksm/etc-passwd/pull/11), [#12](https://github.com/gifnksm/etc-passwd/pull/12), [#14](https://github.com/gifnksm/etc-passwd/pull/14), [#19](https://github.com/gifnksm/etc-passwd/pull/19), [#20](https://github.com/gifnksm/etc-passwd/pull/20), [#22](https://github.com/gifnksm/etc-passwd/pull/22), [#23](https://github.com/gifnksm/etc-passwd/pull/23))

## [0.2.1] - 2024-07-09

### Fixed

- Initialize the `getpw_r` buffer before passing it to the C API. ([#6](https://github.com/gifnksm/etc-passwd/pull/6))

### Changed

- Refresh CI and release workflows. ([#4](https://github.com/gifnksm/etc-passwd/pull/4), [#5](https://github.com/gifnksm/etc-passwd/pull/5))
- Update GitHub Actions dependencies. ([#4](https://github.com/gifnksm/etc-passwd/pull/4), [#5](https://github.com/gifnksm/etc-passwd/pull/5))

## [0.2.0] - 2022-07-18

### Changed

- Migrate the crate to Rust 2021 edition.
- Declare MSRV as Rust 1.56.1.
- Add release automation settings.
- Refresh project documentation and CI configuration. ([#2](https://github.com/gifnksm/etc-passwd/pull/2))
- Update dependencies.

## [0.1.1] - 2020-07-28

### Changed

- Improve README documentation and badges.
- Add GitHub Actions testing. ([#1](https://github.com/gifnksm/etc-passwd/pull/1))
- Fix workflow badge links and docs.rs documentation link.
- Add a test to ensure `README.md` stays in sync.

## [0.1.0] - 2020-07-28

### Added

- Initial release of `etc-passwd`.
- Safe wrapper around libc password database lookups such as `getpwnam_r(3)` and `getpwuid_r(3)`.

<!-- next-url -->
[Unreleased]: https://github.com/gifnksm/etc-passwd/compare/v0.2.2...HEAD
[0.2.2]: https://github.com/gifnksm/etc-passwd/compare/v0.2.1...v0.2.2
[0.2.1]: https://github.com/gifnksm/etc-passwd/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/gifnksm/etc-passwd/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/gifnksm/etc-passwd/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/gifnksm/etc-passwd/commits/v0.1.0
