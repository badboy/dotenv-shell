#[macro_use]
extern crate log;
extern crate env_logger;
extern crate dotenv;
extern crate libc;

use std::{env, ptr, process};
use std::ffi::CString;
use libc::c_char;

/// Literally taken from the libstd
/// (sys/unix/process.rs that is)
fn make_argv(prog: &CString, args: &[CString])
             -> *const *const libc::c_char
{
    let mut ptrs: Vec<*const libc::c_char> = Vec::with_capacity(args.len()+1);

    // Convert the CStrings into an array of pointers. Also return the
    // vector that owns the raw pointers, to ensure they live long
    // enough.
    ptrs.push(prog.as_ptr());
    ptrs.extend(args.iter().map(|tmp| {
        tmp.as_ptr()
    }));

    // Add a terminating null pointer (required by libc).
    ptrs.push(ptr::null());

    ptrs.as_ptr()
}

fn exec(prog: &str, args: &[String]) -> ! {
    let prog = CString::new(prog).expect("Convertable to CString");
    let args = args.iter()
        .map(|tmp| CString::new(&tmp[..]).expect("Convertable to CString"))
        .collect::<Vec<_>>();

    let argv = make_argv(&prog, &args);

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

    let args = env::args().skip(1).collect::<Vec<String>>();
    info!("args: {:?}", args);

    match args.len() {
        0 =>  {
            info!("Starting '{}' as shell", shell);
            exec(&shell, &[]);
        },
        _ => {
            let program = &args[0];
            info!("Starting '{}' with arguments: {:?}", program, &args[1..]);
            exec(program, &args[1..]);
        }
    }
}
