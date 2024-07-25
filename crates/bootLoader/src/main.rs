#![no_std]
#![no_main]

use core::panic::PanicInfo;

use utf16_literal::utf16;

use uefi::{EFIHandle, EFIStatus, EFISystemTable};

#[no_mangle]
pub extern "C" fn efi_main(
    image_handle: EFIHandle,
    system_table: EFISystemTable,
) -> EFIStatus {
    let con_out = system_table.get_con_out();
    con_out.reset(false);
    con_out.output_string(utf16!("Hello, UEFI!\n").as_ptr());

    loop {}

    EFIStatus::Success
}

#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}