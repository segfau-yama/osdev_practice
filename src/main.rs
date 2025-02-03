#![no_std] // 標準ライブラリをリンクしない
#![no_main] // Rustのエントリポイントを無効化
#![feature(custom_test_frameworks)]
#![test_runner(osdev_practice::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use osdev_practice::println;

// エントリポイント
// リンカはデフォルトで_start関数をエントリポイントとして探す
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}

// 自作パニックハンドラ
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    osdev_practice::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
