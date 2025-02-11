use crate::safeptr::*;
use crate::types::*;
use crate::*;

#[cfg(feature = "global-alloc")]
extern crate alloc;
#[cfg(feature = "global-alloc")]
use alloc::vec::Vec;


#[repr(C)]
pub struct EfiSimpleTextInputProtocol {
    reset: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextInputProtocol,
        extended_verification: BOOLEAN,
    ) -> EfiStatus,

    read_key_stroke: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextInputProtocol,
        key: *mut EfiInputKey,
    ) -> EfiStatus,

    wait_for_key: EfiEvent,
}

pub struct SimpleTextInputProtocol {
    protocol: ThreadSafePtr<EfiSimpleTextInputProtocol>,
}

impl SimpleTextInputProtocol {
    pub unsafe fn new(ptr: *mut EfiSimpleTextInputProtocol) -> Self {
        SimpleTextInputProtocol {
            protocol: unsafe { ThreadSafePtr::new(ptr) },
        }
    } 
}

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {
    pub reset: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        extended_verification: BOOLEAN,
    ) -> EfiStatus,

    output_string: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        string: *mut CHAR16,
    ) -> EfiStatus,

    test_string: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        string: *mut CHAR16,
    ) -> EfiStatus,
}

pub struct SimpleTextOutputProtocol {
    protocol: ThreadSafePtr<EfiSimpleTextOutputProtocol>,
}

impl SimpleTextOutputProtocol {
    pub unsafe fn new(ptr: *mut EfiSimpleTextOutputProtocol) -> Self {
        SimpleTextOutputProtocol {
            protocol: unsafe { ThreadSafePtr::new(ptr) },
        }
    }

    pub fn reset(&self, extended: BOOLEAN) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.protocol.as_ptr()).reset)(
                self.protocol.as_ptr(),
                extended,
            )
        };
        efi_try!(status)
    }

    pub fn output_string(&self, string: &str) -> Result<(), EfiStatus> {
        #[cfg(feature = "global-alloc")]
        let mut buf: Vec<u16> = string.encode_utf16()
                                      .chain(core::iter::once(0))
                                      .collect();
        
        #[cfg(not(feature = "global-alloc"))]
        const STACK_BUF_SIZE: usize = 256;
        #[cfg(not(feature = "global-alloc"))]
        let mut stack_buf = [0u16; STACK_BUF_SIZE];

        #[cfg(not(feature = "global-alloc"))]
        {
            let mut idx = 0;
            for c in string.encode_utf16() {
                if idx > STACK_BUF_SIZE - 1 {
                    return Err(EfiStatus::EfiInvalidParameter);
                }
                stack_buf[idx] = c;
                idx += 1;
            }
            stack_buf[idx] = 0;
        }

        let status = unsafe {
            ((*self.protocol.as_ptr()).output_string)(
                self.protocol.as_ptr(),
                #[cfg(feature = "global-alloc")]
                buf.as_mut_ptr(),
                #[cfg(not(feature = "global-alloc"))]
                stack_buf.as_mut_ptr(),
            )
        };
        efi_try!(status)
    }

    pub fn test_string(&self, string: &str) -> Result<(), EfiStatus> {
        #[cfg(feature = "global-alloc")]
        let mut buf: Vec<u16> = string.encode_utf16()
                                      .chain(core::iter::once(0))
                                      .collect();

        #[cfg(not(feature = "global-alloc"))]
        const STACK_BUF_SIZE: usize = 256;
        #[cfg(not(feature = "global-alloc"))]
        let mut stack_buf = [0u16; STACK_BUF_SIZE];

        #[cfg(not(feature = "global-alloc"))]
        {
            let mut idx = 0;
            for c in string.encode_utf16() {
                if idx > STACK_BUF_SIZE - 1 {
                    return Err(EfiStatus::EfiInvalidParameter);
                }
                stack_buf[idx] = c;
                idx += 1;
            }
            stack_buf[idx] = 0;

        }

        let status = unsafe {
            ((*self.protocol.as_ptr()).test_string)(
                self.protocol.as_ptr(),
                #[cfg(feature = "global-alloc")]
                buf.as_mut_ptr(),
                #[cfg(not(feature = "global-alloc"))]
                stack_buf.as_mut_ptr(),
            )
        };
        efi_try!(status)
    }
}
