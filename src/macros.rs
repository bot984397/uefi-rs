#[macro_export]
macro_rules! system_table {
    () => {
        $crate::SYSTEM_TABLE.get().expect("EfiLib not initialized")
    };
}

#[macro_export]
macro_rules! boot_services {
    () => {
        $crate::BOOT_SERVICES.get().expect("EfiLib not initialized")
    };
}

#[macro_export]
macro_rules! runtime_services {
    () => {
        $crate::RUNTIME_SERVICES.get().expect("EfiLib not initialized")
    };
}

#[macro_export]
macro_rules! efi_try {
    ($status:expr) => {
        if $status.is_success() {
            Ok(())
        } else {
            Err($status)
        }
    };
}
