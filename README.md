[![Crates.io](https://img.shields.io/crates/v/etc-passwd.svg)](https://crates.io/crates/etc-passwd)
[![Docs.rs](https://docs.rs/etc-passwd/badge.svg)](https://docs.rs/etc-passwd)
![LICENSE](https://img.shields.io/crates/l/etc-passwd.svg)
[![Workflow Status](https://github.com/gifnksm/etc-passwd/workflows/CI/badge.svg)](https://github.com/gifnksm/etc-passwd/actions?query=workflow%3A%22CI%22)
![Maintenance](https://img.shields.io/badge/maintenance-passively--maintained-yellowgreen.svg)

# etc-passwd

Get user information stored in the password file `/etc/passwd`.

This crate provides a safe wrapper for libc functions such as [`getpwnam_r(3)`] and [`getpwuid_r(3)`].

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
etc-passwd = "0.1.0"
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

[`getpwnam_r(3)`]: ../libc/fn.getpwnam_r.html
[`getpwuid_r(3)`]: ../libc/fn.getpwuid_r.html

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
