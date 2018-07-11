/// A raw ATAG
#[repr(C)]
pub struct RawAtag {
    pub dwords: u32,
    pub tag: u32,
    pub kind: Kind,
}

impl RawAtag {
    pub const NONE: u32 = 0x00000000;
    pub const CORE: u32 = 0x54410001;
    pub const MEM: u32 = 0x54410002;
    pub const VIDEOTEXT: u32 = 0x54410003;
    pub const RAMDISK: u32 = 0x54410004;
    pub const INITRD2: u32 = 0x54420005;
    pub const SERIAL: u32 = 0x54410006;
    pub const REVISION: u32 = 0x54410007;
    pub const VIDEOLFB: u32 = 0x54410008;
    pub const CMDLINE: u32 = 0x54410009;

    /// Returns the next `RawAtag` or `None`
    pub fn next(&self) -> Option<&RawAtag> {
        if self.tag == RawAtag::NONE {
            return None;
        }
        let atag = unsafe {
            (&(self as *const RawAtag as *const u32)).offset(self.dwords as isize) as *const RawAtag
        };
        unsafe { Some(&*atag) }
    }
}

/// Different kind of ATAG
#[repr(C)]
pub union Kind {
    pub core: Core,
    pub mem: Mem,
    pub cmd: Cmd,
}

/// A core ATAG
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Core {
    pub flags: u32,
    pub page_size: u32,
    pub root_dev: u32,
}

/// A mem ATAG
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Mem {
    pub size: u32,
    pub start: u32,
}

/// A cmd ATAG
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Cmd {
    pub cmd: u8,
}
