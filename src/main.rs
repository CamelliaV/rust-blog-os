#![no_std]
#![no_main]

use core::panic::PanicInfo;

// eh_personality lang_item [for handling panic]
// [disable unwind (os-dependency)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// start lang_item [for entry point]
// run with cargo build --target thumbv7em-none-eabihf
// therefore the linker does not try to link the C runtime library

#[no_mangle]
pub extern "C" fn _start() ->! {
    loop {}
}