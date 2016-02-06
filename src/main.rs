#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate libc;

use std::{env, ptr};
use std::ffi::CString;
use libc::{c_char, c_int};

/// Literally taken from the libstd
/// (sys/unix/process.rs that is)
fn make_argv(prog: &CString)
             -> *const *const libc::c_char
{
    let mut ptrs: Vec<*const libc::c_char> = Vec::with_capacity(2);

    // Convert the CStrings into an array of pointers. Also return the
    // vector that owns the raw pointers, to ensure they live long
    // enough.
    ptrs.push(prog.as_ptr());

    // Add a terminating null pointer (required by libc).
    ptrs.push(ptr::null());

    ptrs.as_ptr()
}

fn exec(program: &str) -> c_int {
    let program = CString::new(program).expect("Convertable to CString");
    let argv = make_argv(&program);

    unsafe {
        let exit_code = libc::execv(*argv, argv);
        process::exit(exit_code);
    }
}

fn main() {
    env_logger::init().expect("Setup env_logger failed");

    info!("Loading dotenv");
    dotenv::dotenv().expect("Can't load dotenv");

    let shell = env::var("SHELL").unwrap_or("/bin/sh".into());

    info!("Starting '{}' as shell", shell);
    exec(&shell);
}
