use ::libc;
extern "C" {
    fn locale_charset() -> *const libc::c_char;
    fn setlocale(
        __category: libc::c_int,
        __locale: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
}
unsafe fn main_0() -> libc::c_int {
    setlocale(6 as libc::c_int, b"\0" as *const u8 as *const libc::c_char);
    printf(b"%s\n\0" as *const u8 as *const libc::c_char, locale_charset());
    return 0 as libc::c_int;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
