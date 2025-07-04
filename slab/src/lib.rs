//! A simple slab allocator implementation

#![cfg_attr(not(test), no_std)]
#![feature(sync_unsafe_cell)]
#![feature(allocator_api)]
#![feature(pointer_is_aligned_to)]
#![feature(box_vec_non_null)]
// TODO: Remove this once you fix the `as` conversion warnings
#![allow(clippy::cast_possible_truncation)]

use core::{alloc::Layout, cell::SyncUnsafeCell, marker::PhantomData, ptr::NonNull};

use alloc::alloc::{AllocError, Allocator};
use internal::InternalSlabAllocator;
use utils::sync::spinlock::SpinLockable;

extern crate alloc;

// TODO: Actually call 'initializer' of the SlabAllocatable trait

pub mod heap;
mod internal;

/// A trait for every type that can be allocated using a custom slab allocator.
pub trait SlabAllocatable {}

pub struct SlabAllocator<T>
where
    T: SlabAllocatable,
{
    allocator: SyncUnsafeCell<InternalSlabAllocator>,
    phantom_data: PhantomData<T>,
}

impl<T> SlabAllocator<T>
where
    T: SlabAllocatable,
{
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        let allocator = InternalSlabAllocator::new(Layout::new::<T>());

        Self {
            allocator: SyncUnsafeCell::new(allocator),
            phantom_data: PhantomData,
        }
    }
}

// XXX: We need to make sure only values T are allocated using this allocator. Checking the layout
// isn't enough if we're gonna use the 'initalizer()' fn
unsafe impl<T> Allocator for SlabAllocator<T>
where
    T: SlabAllocatable,
{
    fn allocate(&self, layout: Layout) -> Result<NonNull<[u8]>, AllocError> {
        assert!(
            layout == Layout::new::<T>(),
            "Tried to allocate incompatible type 'A' with a slab allocator designated for type 'B'"
        );

        // Try getting the allocator
        // Then also try allocating an object
        if let Some(allocator) = unsafe { self.allocator.get().as_mut() } {
            let object = allocator.allocate().unwrap();
            // if we were successfull, return the object
            Ok(NonNull::slice_from_raw_parts(
                object.cast::<u8>(),
                layout.size(),
            ))
        } else {
            Err(AllocError)
        }
    }

    unsafe fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        assert!(
            layout == Layout::new::<T>(),
            "Tried to deallocate incompatible type 'A' with a slab allocator designated for type 'B'"
        );

        // Try getting the allocator
        if let Some(allocator) = unsafe { self.allocator.get().as_mut() } {
            assert!(
                !unsafe { allocator.free(ptr.cast()).is_err() },
                "Tried to deallocate a pointer that was not allocated by this allocator"
            );
        }
    }
}

unsafe impl<T> Sync for SlabAllocator<T> where T: SlabAllocatable + Send {}

impl<T> SpinLockable for SlabAllocator<T> where T: SlabAllocatable + Send {}
