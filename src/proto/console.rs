use crate::safeptr::*;
use crate::types::*;
use crate::*;

#[cfg(feature = "global-alloc")]
extern crate alloc;
#[cfg(feature = "global-alloc")]
use alloc::vec::Vec;

pub const EFI_SIMPLE_TEXT_INPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x387477c1,
    data2: 0x69c7,
    data3: 0x11d2,
    data4: [0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b],
};

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

    pub fn reset(&self, extended: BOOLEAN) -> Result<(), EfiStatus> {
        let status = unsafe {
            ((*self.protocol.as_ptr()).reset)(
                self.protocol.as_ptr(),
                extended,
            )
        };
        efi_try!(status)
    }

    pub fn read_key_stroke(&self) -> Result<EfiInputKey, EfiStatus> {
        let mut key: EfiInputKey = EfiInputKey {
            scan_code: 0,
            unicode_char: 0,
        };
        let status = unsafe {
            ((*self.protocol.as_ptr()).read_key_stroke)(
                self.protocol.as_ptr(),
                &mut key,
            )
        };
        if status.is_success() {
            Ok(key)
        } else {
            Err(status)
        }
    }
}

pub const EFI_SIMPLE_TEXT_OUTPUT_PROTOCOL_GUID: EfiGuid = EfiGuid {
    data1: 0x387477c2,
    data2: 0x69c7,
    data3: 0x11d2,
    data4: [0x8e,0x39,0x00,0xa0,0xc9,0x69,0x72,0x3b],
};

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

    query_mode: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        mode_number: UINTN,
        columns: *mut UINTN,
        rows: *mut UINTN,
    ) -> EfiStatus,

    set_mode: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        mode_number: UINTN,
    ) -> EfiStatus,

    set_attribute: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        attribute: UINTN,
    ) -> EfiStatus,

    clear_screen: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
    ) -> EfiStatus,

    set_cursor_position: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        column: UINTN,
        row: UINTN,
    ) -> EfiStatus,

    enable_cursor: unsafe extern "efiapi" fn(
        this: *mut EfiSimpleTextOutputProtocol,
        visible: BOOLEAN,
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
