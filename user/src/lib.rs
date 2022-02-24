#![no_std]
#![feature(linkage)]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod syscall;
mod lang_items;

//引入syscall.rs中两个pub的函数
use syscall::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit");
}

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!")
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|addr| unsafe {
        (addr as *mut u8).write_volatile(0);
    });
}

pub fn write(fd:usize, buf: &[u8]) -> isize{ 
    sys_write(fd, buf) 
}

// dhz???
// pub fn exit(exit_code:isize) -> isize { sys_exit(exit_code) }
pub fn exit(exit_code:i32) -> isize { 
    sys_exit(exit_code)  
}