// Stolen from https://github.com/Peltoche/lsd/blob/master/src/meta/permissions.rs

use serde::{Deserialize, Serialize};
use std::{fmt::Display, fs::Metadata};

pub type RWX = (char, char, char, char, char, char, char, char, char);
pub type Octal = (u8, u8, u8, u8);

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Serialize, Deserialize, Hash)]
pub struct Permissions {
    #[serde(default)]
    pub user_read: bool,

    #[serde(default)]
    pub user_write: bool,

    #[serde(default)]
    pub user_execute: bool,

    #[serde(default)]
    pub group_read: bool,

    #[serde(default)]
    pub group_write: bool,

    #[serde(default)]
    pub group_execute: bool,

    #[serde(default)]
    pub other_read: bool,

    #[serde(default)]
    pub other_write: bool,

    #[serde(default)]
    pub other_execute: bool,

    #[serde(default)]
    pub sticky: bool,

    #[serde(default)]
    pub setgid: bool,

    #[serde(default)]
    pub setuid: bool,
}

impl Permissions {}

impl From<&Metadata> for Permissions {
    #[cfg(unix)]
    fn from(meta: &Metadata) -> Self {
        use std::os::unix::fs::PermissionsExt;

        let bits = meta.permissions().mode();
        let has_bit = |bit| bits & bit == bit;

        Self {
            user_read: has_bit(modes::USER_READ),
            user_write: has_bit(modes::USER_WRITE),
            user_execute: has_bit(modes::USER_EXECUTE),

            group_read: has_bit(modes::GROUP_READ),
            group_write: has_bit(modes::GROUP_WRITE),
            group_execute: has_bit(modes::GROUP_EXECUTE),

            other_read: has_bit(modes::OTHER_READ),
            other_write: has_bit(modes::OTHER_WRITE),
            other_execute: has_bit(modes::OTHER_EXECUTE),

            sticky: has_bit(modes::STICKY),
            setgid: has_bit(modes::SETGID),
            setuid: has_bit(modes::SETUID),
        }
    }

    #[cfg(windows)]
    fn from(_: &Metadata) -> Self {
        panic!("Cannot get permissions from metadata on Windows")
    }
}

impl Display for Permissions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (ur, uw, ux, gr, gw, gx, or, ow, ox) = (*self).into();
        write!(f, "{ur}{uw}{ux}{gr}{gw}{gx}{or}{ow}{ox}")
    }
}

impl Into<RWX> for Permissions {
    fn into(self) -> RWX {
        let bit = |bit: bool, chr: char| {
            if bit {
                chr
            } else {
                '-'
            }
        };

        let ur = bit(self.user_read, 'r');
        let uw = bit(self.user_write, 'w');
        let ux = match (self.user_execute, self.setuid) {
            (true, true) => 's',
            (true, false) => 'x',
            (false, true) => 'S',
            (false, false) => '-',
        };

        let gr = bit(self.group_read, 'r');
        let gw = bit(self.group_write, 'w');
        let gx = match (self.group_execute, self.setgid) {
            (true, true) => 's',
            (true, false) => 'x',
            (false, true) => 'S',
            (false, false) => '-',
        };

        let or = bit(self.other_read, 'r');
        let ow = bit(self.other_write, 'w');
        let ox = match (self.other_execute, self.sticky) {
            (true, true) => 't',
            (true, false) => 'x',
            (false, true) => 'T',
            (false, false) => '-',
        };

        (ur, uw, ux, gr, gw, gx, or, ow, ox)
    }
}

impl Into<Octal> for Permissions {
    fn into(self) -> Octal {
        let bits_to_octal =
            |r: bool, w: bool, x: bool| (r as u8) * 4 + (w as u8) * 2 + (x as u8);

        (
            bits_to_octal(self.setuid, self.setgid, self.sticky),
            bits_to_octal(self.user_read, self.user_write, self.user_execute),
            bits_to_octal(self.group_read, self.group_write, self.group_execute),
            bits_to_octal(self.other_read, self.other_write, self.other_execute),
        )
    }
}

// More readable aliases for the permission bits exposed by libc.
#[allow(trivial_numeric_casts)]
#[cfg(unix)]
mod modes {
    pub type Mode = u32;
    // The `libc::mode_t` type’s actual type varies, but the value returned
    // from `metadata.permissions().mode()` is always `u32`.

    pub const USER_READ: Mode = libc::S_IRUSR as Mode;
    pub const USER_WRITE: Mode = libc::S_IWUSR as Mode;
    pub const USER_EXECUTE: Mode = libc::S_IXUSR as Mode;

    pub const GROUP_READ: Mode = libc::S_IRGRP as Mode;
    pub const GROUP_WRITE: Mode = libc::S_IWGRP as Mode;
    pub const GROUP_EXECUTE: Mode = libc::S_IXGRP as Mode;

    pub const OTHER_READ: Mode = libc::S_IROTH as Mode;
    pub const OTHER_WRITE: Mode = libc::S_IWOTH as Mode;
    pub const OTHER_EXECUTE: Mode = libc::S_IXOTH as Mode;

    pub const STICKY: Mode = libc::S_ISVTX as Mode;
    pub const SETGID: Mode = libc::S_ISGID as Mode;
    pub const SETUID: Mode = libc::S_ISUID as Mode;
}
