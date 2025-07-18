use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn _basename(filename: *const libc::c_char) -> *const libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn __ctype_tolower_loc() -> *mut *const __int32_t;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut mversion: *const libc::c_char;
    static mut mdate: *const libc::c_char;
}
pub type __int32_t = libc::c_int;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn tolower(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_tolower_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[inline]
unsafe extern "C" fn ch_tolower(mut ch: libc::c_char) -> libc::c_char {
    return ({
        let mut __res: libc::c_int = 0;
        if ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong
            > 1 as libc::c_int as libc::c_ulong
        {
            if 0 != 0 {
                let mut __c: libc::c_int = ch as libc::c_uchar as libc::c_int;
                __res = if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                    __c
                } else {
                    *(*__ctype_tolower_loc()).offset(__c as isize)
                };
            } else {
                __res = tolower(ch as libc::c_uchar as libc::c_int);
            }
        } else {
            __res = *(*__ctype_tolower_loc())
                .offset(ch as libc::c_uchar as libc::c_int as isize);
        }
        __res
    }) as libc::c_char;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut name: *const libc::c_char = 0 as *const libc::c_char;
    let mut new_name: *mut libc::c_char = 0 as *mut libc::c_char;
    if argc >= 2 as libc::c_int
        && strcmp(
            *argv.offset(1 as libc::c_int as isize),
            b"-V\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        printf(
            b"Mtools version %s, dated %s\n\0" as *const u8 as *const libc::c_char,
            mversion,
            mdate,
        );
        return 0 as libc::c_int;
    }
    if argc == 1 as libc::c_int {
        fprintf(
            stderr,
            b"Usage: mkmanifest [-V] <list-of-files>\n\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    i = 1 as libc::c_int;
    while i < argc {
        name = _basename(*argv.offset(i as isize));
        new_name = dos_name2(name);
        if strcasecmp(new_name, name) != 0 {
            printf(b"mv %s %s\n\0" as *const u8 as *const libc::c_char, new_name, name);
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn dos_name2(mut name: *const libc::c_char) -> *mut libc::c_char {
    static mut dev: [*const libc::c_char; 9] = [
        b"con\0" as *const u8 as *const libc::c_char,
        b"aux\0" as *const u8 as *const libc::c_char,
        b"com1\0" as *const u8 as *const libc::c_char,
        b"com2\0" as *const u8 as *const libc::c_char,
        b"lpt1\0" as *const u8 as *const libc::c_char,
        b"prn\0" as *const u8 as *const libc::c_char,
        b"lpt2\0" as *const u8 as *const libc::c_char,
        b"lpt3\0" as *const u8 as *const libc::c_char,
        b"nul\0" as *const u8 as *const libc::c_char,
    ];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut temp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut i: libc::c_int = 0;
    let mut dot: libc::c_int = 0;
    static mut ans: [libc::c_char; 13] = [0; 13];
    strncpy(
        buf.as_mut_ptr(),
        name,
        (128 as libc::c_int - 1 as libc::c_int) as libc::c_ulong,
    );
    temp = buf.as_mut_ptr();
    ext = 0 as *mut libc::c_char;
    dot = 0 as libc::c_int;
    i = strlen(buf.as_mut_ptr()) as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if buf[i as usize] as libc::c_int == '.' as i32 && dot == 0 {
            dot = 1 as libc::c_int;
            buf[i as usize] = '\0' as i32 as libc::c_char;
            ext = &mut *buf.as_mut_ptr().offset((i + 1 as libc::c_int) as isize)
                as *mut libc::c_char;
        }
        if *(*__ctype_b_loc())
            .offset(buf[i as usize] as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISupper as libc::c_int as libc::c_ushort as libc::c_int
            != 0
        {
            buf[i as usize] = ch_tolower(buf[i as usize]);
        }
        i -= 1;
        i;
    }
    if *temp as libc::c_int == '\0' as i32 {
        strcpy(ans.as_mut_ptr(), b"x\0" as *const u8 as *const libc::c_char);
    } else {
        i = 0 as libc::c_int;
        while i < 9 as libc::c_int {
            if strcasecmp(temp, dev[i as usize]) == 0 {
                *temp = 'x' as i32 as libc::c_char;
            }
            i += 1;
            i;
        }
        if strlen(temp) > 8 as libc::c_int as libc::c_ulong {
            *temp.offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        if !ext.is_null() && strlen(ext) > 3 as libc::c_int as libc::c_ulong {
            *ext.offset(3 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        }
        loop {
            s = strpbrk(
                temp,
                b"^+=/[]:',?*\\<>|\". \0" as *const u8 as *const libc::c_char,
            );
            if s.is_null() {
                break;
            }
            *s = 'x' as i32 as libc::c_char;
        }
        while !ext.is_null()
            && {
                s = strpbrk(
                    ext,
                    b"^+=/[]:',?*\\<>|\". \0" as *const u8 as *const libc::c_char,
                );
                !s.is_null()
            }
        {
            *s = 'x' as i32 as libc::c_char;
        }
        strncpy(ans.as_mut_ptr(), temp, 12 as libc::c_int as libc::c_ulong);
        ans[12 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    }
    if !ext.is_null() && *ext as libc::c_int != 0 {
        strcat(ans.as_mut_ptr(), b".\0" as *const u8 as *const libc::c_char);
        strcat(ans.as_mut_ptr(), ext);
    }
    return ans.as_mut_ptr();
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
