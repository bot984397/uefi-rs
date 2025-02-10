use crate::types::{UINT32, UINT64};

#[repr(C)]
pub struct EfiTableHeader {
    signature: UINT64,
    revision: UINT32,
    header_size: UINT32,
    crc32: UINT32,
    reserved: UINT32,
}
