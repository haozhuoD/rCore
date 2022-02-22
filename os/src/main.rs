#![no_std]
#![no_main]
#![feature(panic_info_message)]
#[macro_use]

mod console;
mod lang_items;
mod sbi;


use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

// fn main() {
//     // println!("Hello, world!");
// }

#[no_mangle]
pub fn rust_main() -> ! {
    
    clear_bss();
    println!("Hello, world!");
    error!("Hello, world!");
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn _start();
        fn skernel();
        fn ekernel();
    }
    info!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    error!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    info!("load range : [{:#x}, {:#x}] _start = {:#x} \n", skernel as usize, ekernel as usize, _start as usize);//
    panic!("Shutdown machine!");
    // loop {}
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
