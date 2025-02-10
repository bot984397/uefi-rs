#[macro_export]
macro_rules! init_efilib {
    ($system_table:expr) => {{
        unsafe {
            if let Err(status) = $crate::EfiLib::init($system_table) {
                return EfiStatus::EfiLoadError;
            }
        }
    }};
}
