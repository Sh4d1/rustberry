// Thanks to https://web.stanford.edu/class/cs140e (a lot of the code is from there)

mod raw;

mod atag;

pub use self::atag::*;

/// Address of ATAGs
const ATAG_BASE: usize = 0x100;

pub struct Atags {
    ptr: &'static raw::RawAtag,
}

impl Atags {
    pub fn get() -> Atags {
        Atags {
            ptr: unsafe { &*(ATAG_BASE as *const raw::RawAtag) }
        }
    }
}

impl Iterator for Atags {
    type Item = Atag;

    fn next(&mut self) -> Option<Atag> {
        let atag = self.ptr;
        self.ptr = self.ptr.next()?;
        Some(Atag::from(atag))
    }
}
