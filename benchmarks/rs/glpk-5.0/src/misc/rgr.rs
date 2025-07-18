use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __errno_location() -> *mut libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn glp_error_(file: *const libc::c_char, line: libc::c_int) -> glp_errfunc;
    fn glp_printf(fmt: *const libc::c_char, _: ...);
    fn _glp_xstrerr(_: libc::c_int) -> *mut libc::c_char;
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
pub type glp_errfunc = Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>;
unsafe extern "C" fn put_byte(mut fp: *mut FILE, mut c: libc::c_int) {
    fputc(c, fp);
}
unsafe extern "C" fn put_word(mut fp: *mut FILE, mut w: libc::c_int) {
    put_byte(fp, w);
    put_byte(fp, w >> 8 as libc::c_int);
}
unsafe extern "C" fn put_dword(mut fp: *mut FILE, mut d: libc::c_int) {
    put_word(fp, d);
    put_word(fp, d >> 16 as libc::c_int);
}
pub unsafe extern "C" fn _glp_rgr_write_bmp16(
    mut fname: *const libc::c_char,
    mut m: libc::c_int,
    mut n: libc::c_int,
    mut map: *const libc::c_char,
) -> libc::c_int {
    let mut fp: *mut FILE = 0 as *mut FILE;
    let mut offset: libc::c_int = 0;
    let mut bmsize: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut b: libc::c_int = 0;
    let mut ret: libc::c_int = 0 as libc::c_int;
    if !(1 as libc::c_int <= m && m <= 32767 as libc::c_int) {
        (glp_error_(
            b"misc/rgr.c\0" as *const u8 as *const libc::c_char,
            92 as libc::c_int,
        ))
            .unwrap()(
            b"rgr_write_bmp16: m = %d; invalid height\n\0" as *const u8
                as *const libc::c_char,
            m,
        );
    }
    if !(1 as libc::c_int <= n && n <= 32767 as libc::c_int) {
        (glp_error_(
            b"misc/rgr.c\0" as *const u8 as *const libc::c_char,
            94 as libc::c_int,
        ))
            .unwrap()(
            b"rgr_write_bmp16: n = %d; invalid width\n\0" as *const u8
                as *const libc::c_char,
            n,
        );
    }
    fp = fopen(fname, b"wb\0" as *const u8 as *const libc::c_char);
    if fp.is_null() {
        glp_printf(
            b"rgr_write_bmp16: unable to create '%s' - %s\n\0" as *const u8
                as *const libc::c_char,
            fname,
            _glp_xstrerr(*__errno_location()),
        );
        ret = 1 as libc::c_int;
    } else {
        offset = 14 as libc::c_int + 40 as libc::c_int
            + 16 as libc::c_int * 4 as libc::c_int;
        bmsize = (4 as libc::c_int * n + 31 as libc::c_int) / 32 as libc::c_int;
        put_byte(fp, 'B' as i32);
        put_byte(fp, 'M' as i32);
        put_dword(fp, offset + bmsize * 4 as libc::c_int);
        put_word(fp, 0 as libc::c_int);
        put_word(fp, 0 as libc::c_int);
        put_dword(fp, offset);
        put_dword(fp, 40 as libc::c_int);
        put_dword(fp, n);
        put_dword(fp, m);
        put_word(fp, 1 as libc::c_int);
        put_word(fp, 4 as libc::c_int);
        put_dword(fp, 0 as libc::c_int);
        put_dword(fp, 0 as libc::c_int);
        put_dword(fp, 2953 as libc::c_int);
        put_dword(fp, 2953 as libc::c_int);
        put_dword(fp, 0 as libc::c_int);
        put_dword(fp, 0 as libc::c_int);
        put_dword(fp, 0 as libc::c_int);
        put_dword(fp, 0x80 as libc::c_int);
        put_dword(fp, 0x8000 as libc::c_int);
        put_dword(fp, 0x8080 as libc::c_int);
        put_dword(fp, 0x800000 as libc::c_int);
        put_dword(fp, 0x800080 as libc::c_int);
        put_dword(fp, 0x808000 as libc::c_int);
        put_dword(fp, 0xc0c0c0 as libc::c_int);
        put_dword(fp, 0x808080 as libc::c_int);
        put_dword(fp, 0xff as libc::c_int);
        put_dword(fp, 0xff00 as libc::c_int);
        put_dword(fp, 0xffff as libc::c_int);
        put_dword(fp, 0xff0000 as libc::c_int);
        put_dword(fp, 0xff00ff as libc::c_int);
        put_dword(fp, 0xffff00 as libc::c_int);
        put_dword(fp, 0xffffff as libc::c_int);
        b = 0 as libc::c_int;
        i = m - 1 as libc::c_int;
        while i >= 0 as libc::c_int {
            j = 0 as libc::c_int;
            while j < (n + 7 as libc::c_int) / 8 as libc::c_int * 8 as libc::c_int {
                b <<= 4 as libc::c_int;
                b
                    |= if j < n {
                        *map.offset((i * n + j) as isize) as libc::c_int
                            & 15 as libc::c_int
                    } else {
                        0 as libc::c_int
                    };
                if j & 1 as libc::c_int != 0 {
                    put_byte(fp, b);
                }
                j += 1;
                j;
            }
            i -= 1;
            i;
        }
        fflush(fp);
        if ferror(fp) != 0 {
            glp_printf(
                b"rgr_write_bmp16: write error on '%s' - %s\n\0" as *const u8
                    as *const libc::c_char,
                fname,
                _glp_xstrerr(*__errno_location()),
            );
            ret = 1 as libc::c_int;
        }
    }
    if !fp.is_null() {
        fclose(fp);
    }
    return ret;
}
