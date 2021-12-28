#![no_std]
use winapi::km::wdm::{DbgPrint, DRIVER_OBJECT};
use winapi::shared::ntdef::{NTSTATUS, NT_SUCCESS, UNICODE_STRING};
use winapi::shared::ntstatus::STATUS_SUCCESS;

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry(driver: &mut DRIVER_OBJECT, _: *const UNICODE_STRING)
    -> NTSTATUS
{
    unsafe {
        DbgPrint("Hello, from Rust!\0".as_ptr());
    }

    driver.DriverUnload = Some(driver_exit);

    STATUS_SUCCESS
}

pub extern "system" fn driver_exit(driver: &mut DRIVER_OBJECT) {
    unsafe {
        DbgPrint("Bye, unloading Rust!\0".as_ptr());
    }
}
