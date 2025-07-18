use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn exp(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub unsafe extern "C" fn WriteArray(
    mut pfile: *mut FILE,
    mut cElems: libc::c_long,
    mut rg: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    let mut cElems_minus_1: libc::c_long = cElems - 1 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < cElems {
        fprintf(
            pfile,
            b"%g\0" as *const u8 as *const libc::c_char,
            *rg.offset(i as isize),
        );
        if i < cElems_minus_1 {
            fputc('\t' as i32, pfile);
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn WriteArrayExp(
    mut pfile: *mut FILE,
    mut cElems: libc::c_long,
    mut rg: *mut libc::c_double,
) {
    let mut i: libc::c_long = 0;
    let mut cElems_minus_1: libc::c_long = cElems - 1 as libc::c_int as libc::c_long;
    i = 0 as libc::c_int as libc::c_long;
    while i < cElems {
        fprintf(
            pfile,
            b"%g\0" as *const u8 as *const libc::c_char,
            exp(*rg.offset(i as isize)),
        );
        if i < cElems_minus_1 {
            fputc('\t' as i32, pfile);
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn _walog(mut cElems: libc::c_long, mut rg: *mut libc::c_double) {
    let mut i: libc::c_int = 0;
    let mut dSum: libc::c_double = 0.0f64;
    printf(b"{\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while (i as libc::c_long) < cElems {
        dSum += exp(*rg.offset(i as isize));
        printf(
            b"%s%g\0" as *const u8 as *const libc::c_char,
            if i != 0 {
                b", \0" as *const u8 as *const libc::c_char
            } else {
                b"\0" as *const u8 as *const libc::c_char
            },
            exp(*rg.offset(i as isize)),
        );
        i += 1;
        i;
    }
    printf(b"} => %g [%g]\n\0" as *const u8 as *const libc::c_char, dSum, 1.0f64 - dSum);
}
