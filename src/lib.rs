#![no_std]

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "system" fn driver_entry() -> u32 {
    0 /* NT_STATUS_SUCCESS */
}
