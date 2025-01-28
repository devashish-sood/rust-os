#![no_std]
#![no_main]
use core::panic::PanicInfo;
mod vga_buffer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello!").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", here are some numbers {} {}",
        42,
        22.0 / 7.0
    )
    .unwrap();
    loop {}
}
