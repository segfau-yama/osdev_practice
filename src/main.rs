#![no_std] // 標準ライブラリをリンクしない
#![no_main] // Rustのエントリポイントを無効化

mod vga_buffer;
use core::panic::PanicInfo;

// エントリポイント
// リンカはデフォルトで_start関数をエントリポイントとして探す
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    loop {}
}

// 自作パニックハンドラ
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
