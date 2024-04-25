use std::ffi::CStr;

use libc;

fn main() {
    let pw: *mut libc::passwd;
    let uid: libc::uid_t;

    uid = unsafe {
        libc::geteuid()
    };

    pw = unsafe {
        libc::getpwuid(uid)
    };

    println!("{}", unsafe{ CStr::from_ptr((*pw).pw_name).to_str().unwrap() } );
}
