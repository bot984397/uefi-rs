use spin::Once;

use super::hdr::EfiTableHeader;
use crate::types::*;
use crate::proto::*;
use crate::tables::*;
use crate::tables::rs::*;

use crate::proto::console::*;

use crate::safeptr::ThreadSafePtr;

pub const EFI_SYSTEM_TABLE_SIGNATURE: UINT64 = 0x5453_5953_2049_4249;

pub const EFI_2_100_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (100);
pub const EFI_2_90_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (90);
pub const EFI_2_80_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (80);
pub const EFI_2_70_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (70);
pub const EFI_2_60_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (60);
pub const EFI_2_50_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (50);
pub const EFI_2_40_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (40);
pub const EFI_2_31_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (31);
pub const EFI_2_30_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (30);
pub const EFI_2_20_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (20);
pub const EFI_2_10_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (10);
pub const EFI_2_00_SYSTEM_TABLE_REVISION: UINT32 = (2<<16) | (00);
pub const EFI_1_10_SYSTEM_TABLE_REVISION: UINT32 = (1<<16) | (10);
pub const EFI_1_02_SYSTEM_TABLE_REVISION: UINT32 = (1<<16) | (02);

pub const EFI_SPECIFICATION_VERSION: UINT32 = EFI_SYSTEM_TABLE_REVISION;
pub const EFI_SYSTEM_TABLE_REVISION: UINT32 = EFI_2_100_SYSTEM_TABLE_REVISION;

#[repr(C)]
pub struct EfiSystemTable {
    pub hdr: EfiTableHeader,
    pub firmware_vendor: *mut CHAR16,
    pub firmware_revision: UINT32,
    pub console_in_handle: EfiHandle,
    pub con_in: *mut EfiSimpleTextInputProtocol,
    pub console_out_handle: EfiHandle,
    pub con_out: *mut EfiSimpleTextOutputProtocol,
    pub standard_error_handle: EfiHandle,
    pub std_err: *mut EfiSimpleTextOutputProtocol,
    pub runtime_services: *mut EfiRuntimeServices,
    pub boot_services: *mut EfiBootServices,
    pub number_of_table_entries: UINTN,
    pub configuration_table: *mut EfiConfigurationTable,
}

#[repr(C)]
pub struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: *mut VOID,
}

pub struct SystemTable {
    pub table: ThreadSafePtr<EfiSystemTable>,
    pub con_in: Once<SimpleTextInputProtocol>,
    pub con_out: Once<SimpleTextOutputProtocol>,
}

impl SystemTable {
    pub fn boot_services(&self) -> &EfiBootServices {
        unsafe { &*(*self.table.as_ptr()).boot_services }
    }

    pub fn runtime_services(&self) -> &EfiRuntimeServices {
        unsafe { &*(*self.table.as_ptr()).runtime_services }
    }

    pub fn con_in(&self) -> &SimpleTextInputProtocol {
        self.con_in.call_once(|| {
        // SAFETY: table pointer and con_in are guaranteed valid by UEFI spec
        let raw_ptr = unsafe { &mut *(*self.table.as_ptr()).con_in };
        unsafe { SimpleTextInputProtocol::new(raw_ptr) }
        });
        // unwrap() is guaranteed to be safe as we just called call_once()
        self.con_in.get().unwrap()
    }

    pub fn con_out(&self) -> &SimpleTextOutputProtocol {
        self.con_out.call_once(|| {
        // SAFETY: table pointer and con_out are guaranteed valid by UEFI spec
        let raw_ptr = unsafe { &mut *(*self.table.as_ptr()).con_out };
        unsafe { SimpleTextOutputProtocol::new(raw_ptr) }
        });
        // unwrap() is guaranteed to be safe as we just called call_once()
        self.con_out.get().unwrap()
    }
}
