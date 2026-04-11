//! Get user information stored in the password file `/etc/passwd`.
//!
//! This crate provides a safe wrapper for libc functions such as [`getpwnam_r(3)`] and [`getpwuid_r(3)`].
//!
//! # Usage
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! etc-passwd = "0.2.2"
//! ```
//!
//! # Examples
//!
//! Get a current user information:
//!
//! ```
//! use etc_passwd::Passwd;
//!
//! # fn main() -> Result<(), Box<dyn std::error::Error>> {
//! if let Some(passwd) = Passwd::current_user()? {
//!     println!("current user name is: {}", passwd.name.to_str()?);
//!     println!("your user id is: {}", passwd.uid);
//!     println!("your group id is: {}", passwd.gid);
//!     println!("your full name is: {}", passwd.gecos.to_str()?);
//!     println!("your home directory is: {}", passwd.dir.to_str()?);
//!     println!("your login shell is: {}", passwd.shell.to_str()?);
//! } else {
//!     println!("oops! current user is not found... who are you?");
//! }
//! # Ok(())
//! # }
//! ```
//!
//! [`getpwnam_r(3)`]: https://man7.org/linux/man-pages/man3/getpwnam_r.3.html
//! [`getpwuid_r(3)`]: https://man7.org/linux/man-pages/man3/getpwuid_r.3.html

#![doc(html_root_url = "https://docs.rs/etc-passwd/0.2.2")]
#![warn(missing_docs)]
#![warn(unused)]
#![warn(unused_crate_dependencies)]

use std::{
    ffi::{CStr, CString},
    io::{Error, Result},
    mem, ptr,
};

/// Representation of a user information stored in the password file `/etc/passwd`.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Passwd {
    /// A username.
    pub name: CString,
    /// A user password.
    pub passwd: CString,
    /// A user ID.
    pub uid: libc::uid_t,
    /// A group ID.
    pub gid: libc::gid_t,
    /// A user full name or a comment.
    pub gecos: CString,
    /// A home directory.
    pub dir: CString,
    /// A shell program.
    pub shell: CString,
}

impl Passwd {
    /// Looks up the username in the password file and returns a `Passwd` with user information, if the user is found.
    pub fn from_name(name: impl AsRef<CStr>) -> Result<Option<Self>> {
        let name = name.as_ref();
        getpw_r(name.as_ptr(), libc::getpwnam_r)
    }

    /// Looks up the user ID and returns a `Passwd` with user information, if the user is found.
    pub fn from_uid(uid: libc::uid_t) -> Result<Option<Self>> {
        getpw_r(uid, libc::getpwuid_r)
    }

    /// Looks up current user's information in the password file and return a `Passwd` with user information, if the user is found.
    ///
    /// This is a shortcut for `Passwd::from_uid(libc::getuid())`.
    pub fn current_user() -> Result<Option<Self>> {
        Self::from_uid(unsafe { libc::getuid() })
    }

    unsafe fn from_c_struct(passwd: &libc::passwd) -> Self {
        Self {
            name: CStr::from_ptr(passwd.pw_name).to_owned(),
            passwd: CStr::from_ptr(passwd.pw_passwd).to_owned(),
            uid: passwd.pw_uid,
            gid: passwd.pw_gid,
            gecos: CStr::from_ptr(passwd.pw_gecos).to_owned(),
            dir: CStr::from_ptr(passwd.pw_dir).to_owned(),
            shell: CStr::from_ptr(passwd.pw_shell).to_owned(),
        }
    }
}

fn getpw_r<T>(
    key: T,
    f: unsafe extern "C" fn(
        key: T,
        pwd: *mut libc::passwd,
        buf: *mut libc::c_char,
        buflen: libc::size_t,
        result: *mut *mut libc::passwd,
    ) -> libc::c_int,
) -> Result<Option<Passwd>>
where
    T: Copy,
{
    let mut passwd = unsafe { mem::zeroed() };
    let amt = unsafe { libc::sysconf(libc::_SC_GETPW_R_SIZE_MAX) };
    let mut amt = libc::c_long::max(amt, 512) as usize;
    let mut buf = vec![0; amt];

    loop {
        let mut result = ptr::null_mut();
        unsafe {
            f(key, &mut passwd, buf.as_mut_ptr(), buf.len(), &mut result);
        }

        if !result.is_null() {
            // Success
            return Ok(Some(unsafe { Passwd::from_c_struct(&passwd) }));
        }

        let e = Error::last_os_error();
        let errno = e.raw_os_error().unwrap();
        match errno {
            // A signal was caught
            libc::EINTR => continue,

            // Insufficient buffer space
            libc::ERANGE => {
                amt *= 2;
                buf.resize(amt, 0);
                continue;
            }

            // The given name or uid was not found.
            // see https://man7.org/linux/man-pages/man3/getpwnam_r.3.html
            0 | libc::ENOENT | libc::ESRCH | libc::EBADF | libc::EPERM => return Ok(None),

            // Other errors
            _ => return Err(e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

    #[test]
    fn root() -> Result<()> {
        let by_name = Passwd::from_name(CString::new("root")?)?.unwrap();
        let by_uid = Passwd::from_uid(0)?.unwrap();

        assert_eq!(by_name.uid, 0);
        assert_eq!(by_name.gid, 0);
        assert_eq!(by_name.name.to_str()?, "root");
        #[cfg(not(target_os = "macos"))]
        assert_eq!(by_name.dir.to_str()?, "/root");
        #[cfg(target_os = "macos")]
        assert_eq!(by_name.dir.to_str()?, "/var/root");

        // `getpwnam_r` and `getpwuid_r` may return slightly different
        // non-identity fields for the same account depending on the platform's
        // user database backend. This was observed in macOS GitHub Actions CI,
        // where the `root` account resolved to different `shell` values, so
        // avoid asserting full struct equality here.
        assert_eq!(by_uid.uid, by_name.uid);
        assert_eq!(by_uid.gid, by_name.gid);
        assert_eq!(by_uid.name, by_name.name);
        assert_eq!(by_uid.dir, by_name.dir);

        Ok(())
    }

    #[test]
    fn current_user() -> Result<()> {
        let uid = unsafe { libc::getuid() };
        let by_cu = Passwd::current_user()?.unwrap();
        let by_name = Passwd::from_name(&by_cu.name)?.unwrap();

        assert_eq!(by_cu.uid, uid);
        // Assume $HOME is not modified
        assert_eq!(by_cu.dir.to_str()?, std::env::var("HOME")?);

        assert_eq!(by_cu, by_name);

        Ok(())
    }

    #[test]
    fn user_not_exist() -> Result<()> {
        assert!(Passwd::from_uid(u32::MAX)?.is_none());
        assert!(Passwd::from_name(CString::new("")?)?.is_none());
        Ok(())
    }
}
