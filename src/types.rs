pub type BOOLEAN = u8;
pub type INTN = isize;
pub type UINTN = usize;
pub type INT8 = i8;
pub type UINT8 = u8;
pub type INT16 = i16;
pub type UINT16 = u16;
pub type INT32 = i32;
pub type UINT32 = u32;
pub type INT64 = i64;
pub type UINT64 = u64;
pub type CHAR8 = u8;
pub type CHAR16 = u16;
pub type VOID = core::ffi::c_void;
pub type PVOID = *mut core::ffi::c_void;

pub const FALSE: BOOLEAN = 0;
pub const TRUE: BOOLEAN = 1;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct EfiGuid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

pub type EfiHandle = PVOID;
pub type EfiEvent = PVOID;
pub type EfiLba = UINT64;
pub type EfiTpl = UINTN;

#[repr(C)]
pub struct EfiMacAddress {
    pub addr: [u8; 32],
}

#[repr(C)]
pub struct EfiIpv4Address {
    pub addr: [u8; 4],
}

#[repr(C)]
pub struct EfiIpv6Address {
    pub addr: [u8; 16],
}

#[repr(C)]
pub struct EfiIpAddress {
    pub addr: [u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct EfiInputKey {
    pub scan_code: UINT16,
    pub unicode_char: CHAR16,
}

#[repr(C)]
pub struct EfiTime {
    pub year: UINT16,
    pub month: UINT8,
    pub day: UINT8,
    pub hour: UINT8,
    pub minute: UINT8,
    pub second: UINT8,
    pub pad1: UINT8,
    pub nanosecond: UINT32,
    pub time_zone: INT16,
    pub daylight: UINT8,
    pub pad2: UINT8,
}

pub const EFI_TIME_ADJUST_DAYLIGHT: UINT8 = 0x01;
pub const EFI_TIME_IN_DAYLIGHT: UINT8 = 0x02;

pub const EFI_UNSPECIFIED_TIMEZONE: INT16 = 0x07FF;

#[repr(C)]
pub struct EfiTimeCapabilities {
    pub resolution: UINT32,
    pub accuracy: UINT32,
    pub sets_to_zero: BOOLEAN,
}

pub type EfiPhysicalAddress = UINT64;
pub type EfiVirtualAddress = UINT64;

#[repr(C)]
pub struct EfiMemoryDescriptor {
    pub mem_type: UINT32,
    pub physical_start: EfiPhysicalAddress,
    pub virtual_start: EfiVirtualAddress,
    pub number_of_pages: UINT64,
    pub attribute: UINT64,
}

#[repr(i32)]
pub enum EfiResetType {
    EfiResetCold,
    EfiResetWarm,
    EfiResetShutdown,
    EfiResetPlatformSpecific,
}

#[repr(C)]
pub struct EfiCapsuleHeader {
    pub capsule_guid: EfiGuid,
    pub header_size: UINT32,
    pub flags: UINT32,
    pub capsule_image_size: UINT32,
}

pub const CAPSULE_FLAGS_PERSIST_ACROSS_RESET: UINT32    = 0x00010000;
pub const CAPSULE_FLAGS_POPULATE_SYSTEM_TABLE: UINT32   = 0x00020000;
pub const CAPSULE_FLAGS_INITIATE_RESET: UINT32          = 0x00040000;

#[repr(i32)]
pub enum EfiMemoryType {
    EfiReservedMemoryType,
    EfiLoaderCode,
    EfiLoaderData,
    EfiBootServicesCode,
    EfiBootServicesData,
    EfiRuntimeServicesCode,
    EfiRuntimeServicesData,
    EfiConventionalMemory,
    EfiUnusableMemory,
    EfiACPIReclaimMemory,
    EfiACPIMemoryNVS,
    EfiMemoryMappedIO,
    EfiMemoryMappedIOPortSpace,
    EfiPalCode,
    EfiPersistentMemory,
    EfiUnacceptedMemoryType,
    EfiMaxMemoryType
}

#[repr(i32)]
pub enum EfiAllocateType {
    AllocateAnyPages,
    AllocateMaxAddress,
    AllocateAddress,
    MaxAllocateType
}

#[repr(usize)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EfiStatus {
    EfiSuccess = 0,
    EfiLoadError = 1,
    EfiInvalidParameter = 2,
    EfiUnsupported = 3,
    EfiBadBufferSize = 4,
    EfiBufferTooSmall = 5,
    EfiAlreadyStarted = 20,
}

impl EfiStatus {
    pub fn is_success(&self) -> bool {
        *self == EfiStatus::EfiSuccess
    }
}

#[repr(C)]
pub struct SimpleTextOutputMode {
    max_mode: INT32,
    mode: INT32,
    attribute: INT32,
    cursor_column: INT32,
    cursor_row: INT32,
    cursor_visible: BOOLEAN,
}
