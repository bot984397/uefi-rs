use crate::types::{
    BOOLEAN, EfiStatus, EfiInputKey, EfiEvent,
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

#[repr(C)]
pub struct EfiSimpleTextOutputProtocol {

}
