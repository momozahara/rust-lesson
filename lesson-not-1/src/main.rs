use std::process::Command;

mod debug {
    use std::{ffi::CString, io::Result, os::raw::c_char};

    #[link(name = "debug.dll", kind = "dylib")]
    extern "C" {
        fn _info(ptr: *const c_char);
        fn _error(ptr: *const c_char) -> !;
    }

    pub fn info(v: String) {
        unsafe {
            let c_str = string_to_cstring(v).unwrap();
            _info(c_str.as_ptr() as *const c_char);
        }
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
    let output = Command::new("collection").arg("20").output().unwrap();

    let exit_code = output.status;
    let mut stdout = String::from_utf8_lossy(&output.stdout).to_string();
    stdout = stdout.delete_line(1).trim_end().to_string();

    let mut stderr = String::from_utf8_lossy(&output.stderr).to_string();
    stderr = stderr.trim_end().to_string();

    if !exit_code.success() {
        debug::error(format!(
            "collection exit({}) with \"{}\"",
            exit_code.code().unwrap(),
            stderr
        ))
    }

    debug::info("i like your cut g.".to_string());

    struct Column {
        id: String,
        value: String,
    }

    let cols: Vec<Column> = stdout
        .split('\n')
        .map(|row| {
            let co: Vec<&str> = row.split_ascii_whitespace().collect();
            return Column {
                id: co[0].to_string(),
                value: co[1].to_string(),
            };
        })
        .collect();

    let mut cols_iter = cols.iter();

    while let Some(col) = cols_iter.next() {
        println!("{}\t{}", col.id, col.value);
    }
}

trait DeleteLine {
    fn delete_line(&self, line_number: usize) -> String;
}

impl DeleteLine for String {
    fn delete_line(&self, line_number: usize) -> String {
        let mut result = String::new();
        let mut current_line_number = 0;
        for line in self.lines() {
            current_line_number += 1;
            if current_line_number != line_number {
                result.push_str(line);
                result.push('\n');
            }
        }
        result
    }
}
