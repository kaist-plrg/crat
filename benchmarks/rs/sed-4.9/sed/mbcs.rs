use ::libc;
extern "C" {
    fn mbsinit(__ps: *const mbstate_t) -> libc::c_int;
    fn rpl_mbrtowc(
        pwc: *mut wchar_t,
        s: *const libc::c_char,
        n: size_t,
        ps: *mut mbstate_t,
    ) -> size_t;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __ctype_get_mb_cur_max() -> size_t;
    fn panic(str: *const libc::c_char, _: ...);
    fn locale_charset() -> *const libc::c_char;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
pub type __mbstate_t = mbstate_t;
pub static mut mb_cur_max: libc::c_int = 0;
pub static mut is_utf8: bool = false;
pub unsafe extern "C" fn is_mb_char(
    mut ch: libc::c_int,
    mut cur_stat: *mut mbstate_t,
) -> libc::c_int {
    let c: libc::c_char = ch as libc::c_char;
    let mb_pending: libc::c_int = (mbsinit(cur_stat) == 0) as libc::c_int;
    let result: libc::c_int = rpl_mbrtowc(
        0 as *mut wchar_t,
        &c,
        1 as libc::c_int as size_t,
        cur_stat,
    ) as libc::c_int;
    match result {
        -2 => return 1 as libc::c_int,
        -1 => {
            memset(
                cur_stat as *mut libc::c_void,
                0 as libc::c_int,
                ::std::mem::size_of::<mbstate_t>() as libc::c_ulong,
            );
            return 0 as libc::c_int;
        }
        1 => return mb_pending,
        0 => return 1 as libc::c_int,
        _ => {
            panic(
                b"is_mb_char: mbrtowc (0x%x) returned %d\0" as *const u8
                    as *const libc::c_char,
                ch as libc::c_uint,
                result,
            );
        }
    }
    panic!();
}
pub unsafe extern "C" fn initialize_mbcs() {
    let mut codeset_name: *const libc::c_char = 0 as *const libc::c_char;
    codeset_name = locale_charset();
    is_utf8 = strcmp(codeset_name, b"UTF-8\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int;
    mb_cur_max = __ctype_get_mb_cur_max() as libc::c_int;
}
