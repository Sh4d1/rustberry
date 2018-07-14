pub struct VirtualAddr(usize);

pub struct PhysicalAddr(usize);

impl<T: Sized> From<*mut T> for PhysicalAddr {
    fn from(ptr: *mut T) -> PhysicalAddr {
        PhysicalAddr(ptr as usize)
    }
}

impl PhysicalAddr {
    pub fn as_ptr(&self) -> *const u8 {
        self.0 as *const u8
    }

    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.0 as *mut u8
    }

    pub fn as_usize(&self) -> usize {
        self.0
    }

    #[cfg(target_pointer_width = "64")]
    pub fn as_u64(&self) -> u64 {
        self.0 as u64
    }
}
use core::fmt;
impl fmt::Debug for PhysicalAddr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}({:#x})", stringify!(PhysicalAddr), self.0)
    }
}
