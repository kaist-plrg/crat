use ::libc;
extern "C" {
    static mut program_invocation_short_name: *mut libc::c_char;
}
pub unsafe extern "C" fn getprogname() -> *const libc::c_char {
    return program_invocation_short_name;
}
