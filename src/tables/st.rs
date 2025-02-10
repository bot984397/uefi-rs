use core::ptr::NonNull;

use super::hdr::EfiTableHeader;
use crate::types::*;
use crate::proto::*;
use crate::tables::*;
use crate::tables::rs::*;

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
    hdr: EfiTableHeader,
    firmware_vendor: *mut CHAR16,
    firmware_revision: UINT32,
    console_in_handle: EfiHandle,
    con_in: *mut EfiSimpleTextInputProtocol,
    console_out_handle: EfiHandle,
    con_out: *mut EfiSimpleTextOutputProtocol,
    standard_error_handle: EfiHandle,
    std_err: *mut EfiSimpleTextOutputProtocol,
    runtime_services: *mut EfiRuntimeServices,
    boot_services: *mut EfiBootServices,
    number_of_table_entries: UINTN,
    configuration_table: *mut EfiConfigurationTable,
}

#[repr(C)]
pub struct EfiConfigurationTable {
    vendor_guid: EfiGuid,
    vendor_table: *mut VOID,
}

pub struct SystemTable {
    table: ThreadSafePtr<EfiSystemTable>,
}

impl SystemTable {
    pub unsafe fn new(table: *mut EfiSystemTable) -> Option<Self> {
        Some(SystemTable {
            table: unsafe { ThreadSafePtr::new(table) }
        })
    }

    pub fn boot_services(&self) -> Option<NonNull<EfiBootServices>> {
        unsafe { NonNull::new((*self.table.as_ptr()).boot_services) }
    }

    pub fn runtime_services(&self) -> Option<NonNull<EfiRuntimeServices>> {
        unsafe { NonNull::new((*self.table.as_ptr()).runtime_services) }
    }
}
