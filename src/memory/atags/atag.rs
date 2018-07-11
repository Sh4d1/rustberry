use memory::atags::raw;

pub use memory::atags::raw::{Core, Mem};

/// An ATAG
#[derive(Debug, Copy, Clone)]
pub enum Atag {
    Core(raw::Core),
    Mem(raw::Mem),
    Cmd(&'static str),
    Unknown(u32),
    None,
}

impl Atag {
    /// Returns `Some` if it is a `Core` ATAG, `None` otherwise
    pub fn core(&self) -> Option<Core> {
        match self {
            Atag::Core(core) => return Some(*core),
            _ => return None,
        }
    }

    /// Returns `Some` if it is a `Mem` ATAG, `None` otherwise
    pub fn mem(&self) -> Option<Mem> {
        match self {
            Atag::Mem(mem) => return Some(*mem),
            _ => return None,
        }
    }

    /// Returns `Some` if it is a `Cmd` ATAG, with the command string, `None` otherwise
    pub fn cmd(&self) -> Option<&'static str> {
        match self {
            Atag::Cmd(cmd) => Some(cmd),
            _ => return None,
        }
    }
}

impl<'a> From<&'a raw::RawAtag> for Atag {
    fn from(atag: &raw::RawAtag) -> Atag {
        unsafe {
            match (atag.tag, &atag.kind) {
                (raw::RawAtag::CORE, &raw::Kind { core }) => Atag::from(core),
                (raw::RawAtag::MEM, &raw::Kind { mem }) => Atag::from(mem),
                (raw::RawAtag::CMDLINE, &raw::Kind { ref cmd }) => Atag::from(cmd),
                (raw::RawAtag::NONE, _) => Atag::None,
                (id, _) => Atag::Unknown(id),
            }
        }
    }
}

impl<'a> From<raw::Core> for Atag {
    fn from(atag: raw::Core) -> Atag {
        Atag::Core(atag)
    }
}

impl<'a> From<raw::Mem> for Atag {
    fn from(atag: raw::Mem) -> Atag {
        Atag::Mem(atag)
    }
}

impl<'a> From<&'a raw::Cmd> for Atag {
    fn from(atag: &'a raw::Cmd) -> Atag {
        use core::slice;
        use core::str;
        let mut i: usize = 0;
        loop {
            unsafe {
                if ((atag as *const raw::Cmd as *const u8).offset(i as isize) as u8) == b'\0' {
                    break;
                }
                i += 1;
            }
        }

        unsafe {
            Atag::Cmd(str::from_utf8_unchecked(slice::from_raw_parts(
                atag as *const raw::Cmd as *const u8,
                i,
            )))
        }
    }
}
