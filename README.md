# etc-passwd

[![maintenance status: passively-maintained](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-badges-section)
[![license](https://img.shields.io/crates/l/etc-passwd.svg)](#license)
[![crates.io](https://img.shields.io/crates/v/etc-passwd.svg)](https://crates.io/crates/etc-passwd)
[![docs.rs](https://docs.rs/etc-passwd/badge.svg)](https://docs.rs/etc-passwd/)
[![rust 1.56.1+ badge](https://img.shields.io/badge/rust-1.56.1+-93450a.svg)](https://doc.rust-lang.org/cargo/reference/manifest.html#the-rust-version-field)
[![Rust CI](https://github.com/gifnksm/etc-passwd/actions/workflows/rust-ci.yml/badge.svg)](https://github.com/gifnksm/etc-passwd/actions/workflows/rust-ci.yml)
[![codecov](https://codecov.io/gh/gifnksm/etc-passwd/branch/master/graph/badge.svg?token=RKB0YYMJKZ)](https://codecov.io/gh/gifnksm/etc-passwd)

Get user information stored in the password file `/etc/passwd`.

This crate provides a safe wrapper for libc functions such as [`getpwnam_r(3)`] and [`getpwuid_r(3)`].

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
etc-passwd = "0.2.1"
```

## Examples

Get a current user information:

```rust
use etc_passwd::Passwd;

if let Some(passwd) = Passwd::current_user()? {
    println!("current user name is: {}", passwd.name.to_str()?);
    println!("your user id is: {}", passwd.uid);
    println!("your group id is: {}", passwd.gid);
    println!("your full name is: {}", passwd.gecos.to_str()?);
    println!("your home directory is: {}", passwd.dir.to_str()?);
    println!("your login shell is: {}", passwd.shell.to_str()?);
} else {
    println!("oops! current user is not found... who are you?");
}
```

[`getpwnam_r(3)`]: https://man7.org/linux/man-pages/man3/getpwnam_r.3.html
[`getpwuid_r(3)`]: https://man7.org/linux/man-pages/man3/getpwuid_r.3.html

## Minimum supported Rust version (MSRV)

The minimum supported Rust version is **Rust 1.56.1**.
At least the last 3 versions of stable Rust are supported at any given time.

While a crate is pre-release status (0.x.x) it may have its MSRV bumped in a patch release.
Once a crate has reached 1.x, any MSRV bump will be accompanied with a new minor version.

## License

Licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
