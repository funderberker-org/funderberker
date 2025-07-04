//! Mem related usefull wrappers and utility functions

use crate::collections::fast_lazy_static::FastLazyStatic;
use core::{
    fmt::{self, Debug, Formatter},
    ops::{Add, Sub},
    ptr::{NonNull, read_volatile, write_volatile},
};

pub mod mmio;

/// We set this to 0x0, since in testing we don't want to use HHDM offset
pub static HHDM_OFFSET: FastLazyStatic<usize> = FastLazyStatic::new(0x0);

/// A physical address
#[repr(transparent)]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct PhysAddr(pub usize);

/// A virtual address
#[repr(transparent)]
#[derive(Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Default)]
pub struct VirtAddr(pub usize);

impl VirtAddr {
    /// Get the physical address of a virtual address **that is HHDM mapped**
    ///
    /// NOTE: This function can't be const since we don't know the HHDM offset at compile time
    pub fn subtract_hhdm_offset(self) -> PhysAddr {
        PhysAddr(self.0 - HHDM_OFFSET.get())
    }
}

impl PhysAddr {
    /// Get the virtual address of a physical address. A Virtual address **that is HHDM mapped**
    pub fn add_hhdm_offset(self) -> VirtAddr {
        VirtAddr(self.0 + HHDM_OFFSET.get())
    }
}

impl Debug for VirtAddr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0) // Formats as hex with "0x" prefix
    }
}

impl Debug for PhysAddr {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:#x}", self.0) // Formats as hex with "0x" prefix
    }
}

impl<T> From<*const T> for VirtAddr {
    fn from(value: *const T) -> Self {
        Self(value.addr())
    }
}

impl<T> From<*mut T> for VirtAddr {
    fn from(value: *mut T) -> Self {
        Self(value.addr())
    }
}
impl<T> From<NonNull<T>> for VirtAddr {
    fn from(value: NonNull<T>) -> Self {
        Self(value.as_ptr().addr())
    }
}

impl Add<usize> for VirtAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<usize> for PhysAddr {
    type Output = Self;

    fn add(self, rhs: usize) -> Self::Output {
        Self(self.0 + rhs)
    }
}

impl Add<VirtAddr> for VirtAddr {
    type Output = usize;

    fn add(self, rhs: VirtAddr) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Add<PhysAddr> for PhysAddr {
    type Output = usize;

    fn add(self, rhs: PhysAddr) -> Self::Output {
        self.0 + rhs.0
    }
}

impl Sub<usize> for VirtAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<usize> for PhysAddr {
    type Output = Self;

    fn sub(self, rhs: usize) -> Self::Output {
        Self(self.0 - rhs)
    }
}

impl Sub<VirtAddr> for VirtAddr {
    type Output = usize;

    fn sub(self, rhs: VirtAddr) -> Self::Output {
        self.0 - rhs.0
    }
}

impl Sub<PhysAddr> for PhysAddr {
    type Output = usize;

    fn sub(self, rhs: PhysAddr) -> Self::Output {
        self.0 - rhs.0
    }
}

impl<T> From<*const T> for PhysAddr {
    fn from(value: *const T) -> Self {
        Self(value.addr())
    }
}

impl<T> From<*mut T> for PhysAddr {
    fn from(value: *mut T) -> Self {
        Self(value.addr())
    }
}

impl<T> From<NonNull<T>> for PhysAddr {
    fn from(value: NonNull<T>) -> Self {
        Self(value.as_ptr().addr())
    }
}

/// Wrapper to memset some region of memory to some value
pub unsafe fn memset(ptr: *mut u8, value: u8, len: usize) {
    unsafe {
        for i in 0..len {
            write_volatile(ptr.add(i), value);
        }
    };
}

/// Wrapper to memcpy some region of memory to another
pub unsafe fn memcpy(dst: *mut u8, src: *const u8, len: usize) {
    unsafe {
        for i in 0..len {
            write_volatile(dst.add(i), read_volatile(src.add(i)));
        }
    };
}
