use core::alloc::{GlobalAlloc, Layout};

use core::ptr::null_mut;

use crate::types::*;
use crate::*;

#[allow(dead_code)]
struct EfiAllocator;

trait LayoutExt {
    fn get_uefi_alignment(&self) -> UINTN;
}

impl LayoutExt for Layout {
    fn get_uefi_alignment(&self) -> UINTN {
        self.align() as UINTN
    }
}

unsafe impl GlobalAlloc for EfiAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let efi = match EfiLib::get() {
            Some(efi) => efi,
            None => return null_mut(),
        };

        let size = layout.size().max(layout.get_uefi_alignment()) as UINTN;

        match efi.boot_services.allocate_pool(
            EfiMemoryType::EfiLoaderData,
            size
        ) {
            Ok(non_null_ptr) => non_null_ptr.as_ptr() as *mut u8,
            Err(_) => null_mut(),
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        if let Some(efi) = EfiLib::get() {
            if let Some(non_null) = core::ptr::NonNull::new(ptr as *mut VOID) {
                let _ = efi.boot_services.free_pool(non_null);
            }
        }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        let ptr = unsafe { self.alloc(layout) };
        if !ptr.is_null() {
            unsafe { ptr.write_bytes(0, layout.size()) };
        }
        ptr
    }
}

//#[cfg(feature = "global-alloc")]
#[global_allocator]
static ALLOCATOR: EfiAllocator = EfiAllocator;
