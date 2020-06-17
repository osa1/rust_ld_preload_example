//! Loading this library with LD_PRELOAD overrides the libc free function. The new free function
//! prints a line and then calls the libc free.

#![no_std]
#![feature(lang_items, core_intrinsics, link_args)]

use core::panic::PanicInfo;
use libc::*;

static FREE_STR: &[u8] = b"free\0";

static mut FREE: Option<fn(*mut c_void)> = None;

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut c_void) {
    puts(b"free called\0".as_ptr() as *const _);
    match FREE {
        None => {
            let free =
                ::core::mem::transmute(libc::dlsym(libc::RTLD_NEXT, FREE_STR.as_ptr() as *const _));
            FREE = Some(free);
            free(ptr)
        }
        Some(free) => free(ptr),
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    ::core::intrinsics::abort()
}

#[lang = "eh_personality"]
extern "C" fn eh_personality() {}
