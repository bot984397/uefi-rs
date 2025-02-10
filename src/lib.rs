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

use crate::tables::st::*;
use crate::tables::rs::*;
use crate::tables::bs::*;

use core::panic::PanicInfo;

#[derive(Debug)]
pub enum EfiInitError {
    AlreadyInitialized,
    InvalidArgument,
}

pub struct EfiLib {
    system_table: SystemTable,
    boot_services: BootServices,
    runtime_services: RuntimeServices,
}

static EFI_LIB: Once<EfiLib> = Once::new();
static INITIALIZED: AtomicBool = AtomicBool::new(false);

#[cfg(feature = "panic-handler")]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

impl EfiLib {
    pub unsafe fn init(system_table: *mut EfiSystemTable) 
        -> Result<(), EfiInitError> {
        if INITIALIZED.load(Ordering::SeqCst) {
            return Err(EfiInitError::AlreadyInitialized);
        }

        let st = unsafe { SystemTable::new(system_table) }
            .ok_or(EfiInitError::InvalidArgument)?;

        let bs_ptr = st.boot_services()
            .ok_or(EfiInitError::InvalidArgument)?;
        let bs = unsafe { BootServices::new(bs_ptr.as_ptr()) }
            .ok_or(EfiInitError::InvalidArgument)?;

        let rs_ptr = st.runtime_services()
            .ok_or(EfiInitError::InvalidArgument)?;
        let rs = unsafe { RuntimeServices::new(rs_ptr.as_ptr()) }
            .ok_or(EfiInitError::InvalidArgument)?;

        EFI_LIB.call_once(|| EfiLib {
            system_table: st,
            boot_services: bs,
            runtime_services: rs,
        });

        INITIALIZED.store(true, Ordering::SeqCst);
        Ok(())
    }

    pub fn get() -> Option<&'static EfiLib> {
        EFI_LIB.get()
    }
}
