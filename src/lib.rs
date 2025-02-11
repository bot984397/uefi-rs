#![no_std]
#![feature(allocator_api)]

pub mod types;
pub mod proto;
pub mod tables;
pub mod allocator;
pub mod safeptr;
pub mod macros;

use spin::Once;

use core::sync::atomic::{AtomicBool, Ordering};
use core::ptr::NonNull;

use crate::tables::st::*;
use crate::tables::rs::*;
use crate::tables::bs::*;
use crate::safeptr::*;

use core::panic::PanicInfo;

#[derive(Debug)]
pub enum EfiInitError {
    AlreadyInitialized,
    InvalidArgument,
}

pub struct EfiGlobal<T> {
    inner: Once<T>,
}

impl<T> EfiGlobal<T> {
    pub const fn new() -> Self {
        Self {
            inner: Once::new()
        }
    }

    fn init(&self, val: T) {
        self.inner.call_once(|| val);
    }
}

impl<T> core::ops::Deref for EfiGlobal<T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.get().expect("EfiLib not initialized")
    }
}

pub unsafe fn init_efilib(system_table: *mut EfiSystemTable
) -> Result<(), EfiInitError> {
    if INITIALIZED.load(Ordering::SeqCst) {
        return Err(EfiInitError::AlreadyInitialized);
    }

    let st = NonNull::new(system_table).ok_or(EfiInitError::InvalidArgument)?;
    let st_ref = unsafe { st.as_ref() };

    let system_table = SystemTable {
        table: unsafe { ThreadSafePtr::new(st.as_ptr()) },
        con_in: Once::new(),
        con_out: Once::new(),
    };

    let boot_services = BootServices {
        services: unsafe { ThreadSafePtr::new(st_ref.boot_services) },
    };

    let runtime_services = RuntimeServices {
        services: unsafe { ThreadSafePtr::new(st_ref.runtime_services) },
    };

    SYSTEM_TABLE.init(system_table);
    BOOT_SERVICES.init(boot_services);
    RUNTIME_SERVICES.init(runtime_services);

    INITIALIZED.store(true, Ordering::SeqCst);
    Ok(())
}

pub static SYSTEM_TABLE: EfiGlobal<SystemTable> = EfiGlobal::new();
pub static BOOT_SERVICES: EfiGlobal<BootServices> = EfiGlobal::new();
pub static RUNTIME_SERVICES: EfiGlobal<RuntimeServices> = EfiGlobal::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
