use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn secfopen(_: *mut libc::c_char, _: *mut libc::c_char) -> *mut FILE;
}
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
static mut loadav: [libc::c_double; 3] = [0.; 3];
static mut loadok: libc::c_int = 0;
pub unsafe extern "C" fn InitLoadav() {
    loadok = 1 as libc::c_int;
}
unsafe extern "C" fn GetLoadav() -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut buf: [libc::c_char; 128] = [0; 128];
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut d: libc::c_double = 0.;
    let mut e: libc::c_double = 0.;
    fp = secfopen(
        b"/proc/loadavg\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    *buf.as_mut_ptr() = 0 as libc::c_int as libc::c_char;
    fgets(
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
        fp,
    );
    fclose(fp);
    s = buf.as_mut_ptr();
    i = 0 as libc::c_int;
    while i
        < (if 3 as libc::c_int > 3 as libc::c_int {
            3 as libc::c_int
        } else {
            3 as libc::c_int
        })
    {
        e = 0 as libc::c_int as libc::c_double;
        d = e;
        while *s as libc::c_int == ' ' as i32 {
            s = s.offset(1);
            s;
        }
        if *s as libc::c_int == 0 as libc::c_int {
            break;
        }
        loop {
            if *s as libc::c_int == '.' as i32 {
                e = 1 as libc::c_int as libc::c_double;
            } else {
                if !(*s as libc::c_int >= '0' as i32 && *s as libc::c_int <= '9' as i32)
                {
                    break;
                }
                d = d * 10 as libc::c_int as libc::c_double
                    + (*s as libc::c_int - '0' as i32) as libc::c_double;
                if e != 0. {
                    e *= 10 as libc::c_int as libc::c_double;
                }
            }
            s = s.offset(1);
            s;
        }
        loadav[i as usize] = if e != 0. { d / e } else { d };
        i += 1;
        i;
    }
    return i;
}
pub unsafe extern "C" fn AddLoadav(mut p: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    if loadok == 0 as libc::c_int {
        return;
    }
    j = GetLoadav();
    i = 0 as libc::c_int;
    while i < j {
        sprintf(
            p,
            (b" %2.2f\0" as *const u8 as *const libc::c_char)
                .offset((i == 0) as libc::c_int as isize),
            loadav[i as usize] / 1 as libc::c_int as libc::c_double,
        );
        p = p.offset(strlen(p) as isize);
        i += 1;
        i;
    }
}
