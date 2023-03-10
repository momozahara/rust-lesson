#![cfg_attr(debug_assertions, allow(dead_code))]

use colored::Colorize;

use std::{ffi::CStr, os::raw::c_char, process::exit};

#[no_mangle]
pub extern "C" fn _info(ptr: *const c_char) {
    let s = ptr_to_string(ptr).unwrap();
    println!("{}: {s}", "Info".bright_blue());
}

#[no_mangle]
pub extern "C" fn _error(ptr: *const c_char) -> ! {
    let s = ptr_to_string(ptr).unwrap();
    eprintln!("{}: {s}", "Error".bright_red());
    exit(1)
}

fn ptr_to_string(ptr: *const c_char) -> Option<String> {
    let s = unsafe { CStr::from_ptr(ptr) }.to_str().unwrap();
    Some(String::from(s))
}
