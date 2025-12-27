#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use blog_os::println;

#[unsafe(no_mangle)] // don't mangle (mix up) the name of the function,
// keeping it just `_start` instead of `_start_234kdjflsjejrsfj` or whatever
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn _test_runner(_tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

/// ----
/// TESTS
/// -----

#[test_case]
fn test_println() {
    println!("test_println output");
}
