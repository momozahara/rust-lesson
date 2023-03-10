use std::env::args;

mod debug {
    use std::{ffi::CString, io::Result, os::raw::c_char};

    #[link(name = "debug.dll", kind = "dylib")]
    extern "C" {
        fn _info(ptr: *const c_char);
        fn _error(ptr: *const c_char) -> !;
    }

    pub fn error(v: String) -> ! {
        unsafe {
            let c_str = string_to_cstring(v).unwrap();
            _error(c_str.as_ptr() as *const c_char)
        }
    }

    fn string_to_cstring(v: String) -> Result<CString> {
        Ok(CString::new(v).unwrap())
    }
}

fn main() {
    let usage = || -> ! {
        debug::error("usage collection <size>".to_string());
    };

    let arg = args().nth(1).unwrap_or_else(|| usage());
    let size: usize = arg.parse::<usize>().unwrap_or_else(|_| usage());

    if size < 1 {
        debug::error("size must be greater than 0".to_string())
    }

    println!("ID\tVALUE");
    for i in 0..size {
        println!("{i}\t{i}", i = i + 1);
    }
}
