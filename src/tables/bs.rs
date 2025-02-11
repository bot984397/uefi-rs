use core::ptr::NonNull;

use super::hdr::*;
use crate::types::*;

use crate::safeptr::ThreadSafePtr;

#[repr(C)]
pub struct EfiBootServices {
    pub hdr: EfiTableHeader,

    // task priority services

    pub raise_tpl: unsafe extern "efiapi" fn(
        new_tpl: EfiTpl,
    ) -> EfiTpl,

    pub restore_tpl: unsafe extern "efiapi" fn(
        old_tpl: EfiTpl,
    ) -> VOID,

    // memory services
    
    pub allocate_pages: unsafe extern "efiapi" fn(
        alloc_type: EfiAllocateType,
        memory_type: EfiMemoryType,
        pages: UINTN,
        memory: *mut EfiPhysicalAddress,
    ) -> EfiStatus,

    pub free_pages: unsafe extern "efiapi" fn(
        memory: *mut EfiPhysicalAddress,
        pages: UINTN,
    ) -> EfiStatus,

    pub get_memory_map: unsafe extern "efiapi" fn(
        memory_map_size: *mut UINTN,
        memory_map: *mut EfiMemoryDescriptor,
        map_key: *mut UINTN,
        descriptor_size: *mut UINTN,
        descriptor_version: *mut UINT32,
    ) -> EfiStatus,

    pub allocate_pool: unsafe extern "efiapi" fn(
        pool_type: EfiMemoryType,
        size: UINTN,
        buffer: *mut *mut VOID,
    ) -> EfiStatus,

    pub free_pool: unsafe extern "efiapi" fn(
        buffer: *mut VOID,
    ) -> EfiStatus,
}

pub struct BootServices {
    pub services: ThreadSafePtr<EfiBootServices>,
}

impl BootServices {
    pub fn allocate_pool(&self, 
                         pool_type: EfiMemoryType,
                         size: UINTN
    ) -> Result<NonNull<VOID>, EfiStatus> {
        let mut buffer: *mut VOID = core::ptr::null_mut();

        unsafe {
            let status = ((*self.services.as_ptr()).allocate_pool)(
                pool_type,
                size,
                &mut buffer
            );
            if status.is_success() {
                NonNull::new(buffer).ok_or(EfiStatus::EfiBufferTooSmall)
            } else {
                Err(status)
            }
        }
    } 

    pub fn free_pool(&self, buffer: NonNull<VOID>) -> Result<(), EfiStatus> {
        unsafe {
            let bs = self.services.as_ptr();
            
            let status = ((*bs).free_pool)(buffer.as_ptr());
            if status.is_success() {
                Ok(())
            } else {
                Err(status)
            }
        }
    }
}
