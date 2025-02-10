use super::hdr::*;
use crate::types::*;
use crate::safeptr::ThreadSafePtr;

#[repr(C)]
pub struct EfiRuntimeServices {
    pub hdr: EfiTableHeader,

    // time services

    pub get_time: unsafe extern "efiapi" fn(
        time: *mut EfiTime,
        capabilities: *mut EfiTimeCapabilities,
    ) -> EfiStatus,

    pub set_time: unsafe extern "efiapi" fn(
        time: *mut EfiTime,
    ) -> EfiStatus,

    // virtual memory services

    pub set_virtual_address_map: unsafe extern "efiapi" fn(
        memory_map_size: UINTN,
        descriptor_size: UINTN,
        descriptor_version: UINT32,
        virtual_map: *mut EfiMemoryDescriptor,
    ) -> EfiStatus,

    pub convert_pointer: unsafe extern "efiapi" fn(
        debug_disposition: UINTN,
        address: *mut VOID,
    ) -> EfiStatus,

    // variable services

    pub get_variable: unsafe extern "efiapi" fn(
        variable_name: *mut CHAR16,
        vendor_guid: *mut EfiGuid,
        attributes: *mut UINT32,
        data_size: *mut UINTN,
        data: *mut VOID,
    ) -> EfiStatus,

    pub get_next_variable_name: unsafe extern "efiapi" fn(
        variable_name_size: *mut UINTN,
        variable_name: *mut CHAR16,
        vendor_guid: *mut EfiGuid,
    ) -> EfiStatus,

    pub set_variable: unsafe extern "efiapi" fn(
        variable_name: *mut CHAR16,
        vendor_guid: *mut EfiGuid,
        attributes: UINT32,
        data_size: UINTN,
        data: *mut VOID,
    ) -> EfiStatus,

    // miscellaneous services

    pub get_next_high_monotonic_count: unsafe extern "efiapi" fn(
        high_count: *mut UINT32,
    ) -> EfiStatus,

    pub reset_system: unsafe extern "efiapi" fn(
        reset_type: EfiResetType,
        reset_status: EfiStatus,
        data_size: UINTN,
        reset_data: *mut VOID,
    ) -> EfiStatus,

    // uefi 2.0 capsule services

    pub update_capsule: unsafe extern "efiapi" fn(
        capsule_header_array: *mut *mut EfiCapsuleHeader,
        capsule_count: UINTN,
        scatter_gather_list: EfiPhysicalAddress,
    ) -> EfiStatus,

    pub query_capsule_capabilities: unsafe extern "efiapi" fn(
        capsule_header_array: *mut *mut EfiCapsuleHeader,
        capsule_count: UINTN,
        maximum_capsule_size: *mut UINT64,
        reset_type: *mut EfiResetType,
    ) -> EfiStatus,

    // miscellaneous uefi 2.0 services

    pub query_variable_info: unsafe extern "efiapi" fn(
        attributes: UINT32,
        maximum_variable_storage_size: *mut UINT64,
        remaining_variable_storage_size: *mut UINT64,
        maximum_variable_size: *mut UINT64,
    ) -> EfiStatus,
}

pub struct RuntimeServices {
    services: ThreadSafePtr<EfiRuntimeServices>,
}

impl RuntimeServices {
    pub unsafe fn new(services: *mut EfiRuntimeServices) -> Option<Self> {
        Some(RuntimeServices {
            services: unsafe { ThreadSafePtr::new(services) }
        })
    }
}
