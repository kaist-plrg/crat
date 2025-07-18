use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn exit(_: libc::c_int) -> !;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
}
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type mt_off_t = off_t;
pub static mut max_off_t_31: mt_off_t = 0;
static mut max_off_t_32: mt_off_t = 0;
pub static mut max_off_t_41: mt_off_t = 0;
pub static mut max_off_t_seek: mt_off_t = 0;
pub unsafe extern "C" fn fileTooBig(mut off: mt_off_t) -> libc::c_int {
    return (off & !max_off_t_32 != 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn truncBytes32(mut off: mt_off_t) -> off_t {
    if fileTooBig(off) != 0 {
        fprintf(
            stderr,
            b"Internal error, offset too big\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return off;
}
pub unsafe extern "C" fn truncMtOffTo32u(mut off: mt_off_t) -> uint32_t {
    if fileTooBig(off) != 0 {
        fprintf(
            stderr,
            b"Internal error, offset too big\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return off as uint32_t;
}
pub unsafe extern "C" fn truncSizeTo32u(mut siz: size_t) -> uint32_t {
    if siz > 4294967295 as libc::c_uint as libc::c_ulong {
        fprintf(
            stderr,
            b"Internal error, size too big\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    return siz as uint32_t;
}
pub unsafe extern "C" fn mt_lseek(
    mut fd: libc::c_int,
    mut where_0: mt_off_t,
    mut whence: libc::c_int,
) -> libc::c_int {
    if lseek(fd, where_0, whence) >= 0 as libc::c_int as libc::c_long {
        return 0 as libc::c_int
    } else {
        return 1 as libc::c_int
    };
}
pub unsafe extern "C" fn log_2(mut size: libc::c_uint) -> libc::c_uint {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < 24 as libc::c_int as libc::c_uint {
        if (1 as libc::c_uint) << i == size {
            return i;
        }
        i = i.wrapping_add(1);
        i;
    }
    return 24 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn run_static_initializers() {
    max_off_t_31 = (((1 as libc::c_int as mt_off_t)
        << (if ((31 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            < (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
            (31 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        } else {
            (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        })) - 1 as libc::c_int as libc::c_long) << 1 as libc::c_int
        | 1 as libc::c_int as libc::c_long;
    max_off_t_32 = (((1 as libc::c_int as mt_off_t)
        << (if ((32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            < (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
            (32 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        } else {
            (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        })) - 1 as libc::c_int as libc::c_long) << 1 as libc::c_int
        | 1 as libc::c_int as libc::c_long;
    max_off_t_41 = (((1 as libc::c_int as mt_off_t)
        << (if ((41 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
            < (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
            (41 as libc::c_int - 1 as libc::c_int) as libc::c_ulong
        } else {
            (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        })) - 1 as libc::c_int as libc::c_long) << 1 as libc::c_int
        | 1 as libc::c_int as libc::c_long;
    max_off_t_seek = (((1 as libc::c_int as mt_off_t)
        << (if (::std::mem::size_of::<off_t>() as libc::c_ulong)
            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
            < (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        {
            (::std::mem::size_of::<off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        } else {
            (::std::mem::size_of::<mt_off_t>() as libc::c_ulong)
                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                .wrapping_sub(2 as libc::c_int as libc::c_ulong)
        })) - 1 as libc::c_int as libc::c_long) << 1 as libc::c_int
        | 1 as libc::c_int as libc::c_long;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
