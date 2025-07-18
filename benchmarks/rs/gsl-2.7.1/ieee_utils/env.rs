use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_ieee_set_mode(
        precision: libc::c_int,
        rounding: libc::c_int,
        exception_mask: libc::c_int,
    ) -> libc::c_int;
    fn gsl_ieee_read_mode_string(
        description: *const libc::c_char,
        precision: *mut libc::c_int,
        rounding: *mut libc::c_int,
        exception_mask: *mut libc::c_int,
    ) -> libc::c_int;
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
pub type C2RustUnnamed = libc::c_uint;
pub const GSL_IEEE_EXTENDED_PRECISION: C2RustUnnamed = 3;
pub const GSL_IEEE_DOUBLE_PRECISION: C2RustUnnamed = 2;
pub const GSL_IEEE_SINGLE_PRECISION: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const GSL_IEEE_ROUND_TO_ZERO: C2RustUnnamed_0 = 4;
pub const GSL_IEEE_ROUND_UP: C2RustUnnamed_0 = 3;
pub const GSL_IEEE_ROUND_DOWN: C2RustUnnamed_0 = 2;
pub const GSL_IEEE_ROUND_TO_NEAREST: C2RustUnnamed_0 = 1;
pub type C2RustUnnamed_1 = libc::c_uint;
pub const GSL_IEEE_TRAP_INEXACT: C2RustUnnamed_1 = 32;
pub const GSL_IEEE_MASK_ALL: C2RustUnnamed_1 = 31;
pub const GSL_IEEE_MASK_UNDERFLOW: C2RustUnnamed_1 = 16;
pub const GSL_IEEE_MASK_OVERFLOW: C2RustUnnamed_1 = 8;
pub const GSL_IEEE_MASK_DIVISION_BY_ZERO: C2RustUnnamed_1 = 4;
pub const GSL_IEEE_MASK_DENORMALIZED: C2RustUnnamed_1 = 2;
pub const GSL_IEEE_MASK_INVALID: C2RustUnnamed_1 = 1;
pub unsafe extern "C" fn gsl_ieee_env_setup() {
    let mut p: *const libc::c_char = getenv(
        b"GSL_IEEE_MODE\0" as *const u8 as *const libc::c_char,
    );
    let mut precision: libc::c_int = 0 as libc::c_int;
    let mut rounding: libc::c_int = 0 as libc::c_int;
    let mut exception_mask: libc::c_int = 0 as libc::c_int;
    let mut comma: libc::c_int = 0 as libc::c_int;
    if p.is_null() {
        return;
    }
    if *p as libc::c_int == '\0' as i32 {
        return;
    }
    gsl_ieee_read_mode_string(p, &mut precision, &mut rounding, &mut exception_mask);
    gsl_ieee_set_mode(precision, rounding, exception_mask);
    fprintf(stderr, b"GSL_IEEE_MODE=\"\0" as *const u8 as *const libc::c_char);
    match precision {
        1 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"single-precision\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        2 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"double-precision\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        3 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"extended-precision\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        _ => {}
    }
    match rounding {
        1 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"round-to-nearest\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        2 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"round-down\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        3 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"round-up\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        4 => {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"round-to-zero\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        _ => {}
    }
    if exception_mask & GSL_IEEE_MASK_ALL as libc::c_int
        == GSL_IEEE_MASK_ALL as libc::c_int
    {
        if comma != 0 {
            fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b"mask-all\0" as *const u8 as *const libc::c_char);
        comma += 1;
        comma;
    } else if exception_mask & GSL_IEEE_MASK_ALL as libc::c_int == 0 as libc::c_int {
        if comma != 0 {
            fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b"trap-common\0" as *const u8 as *const libc::c_char);
        comma += 1;
        comma;
    } else {
        if exception_mask & GSL_IEEE_MASK_INVALID as libc::c_int != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"mask-invalid\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_DENORMALIZED as libc::c_int != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"mask-denormalized\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_DIVISION_BY_ZERO as libc::c_int != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(
                stderr,
                b"mask-division-by-zero\0" as *const u8 as *const libc::c_char,
            );
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_OVERFLOW as libc::c_int != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"mask-overflow\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
        if exception_mask & GSL_IEEE_MASK_UNDERFLOW as libc::c_int != 0 {
            if comma != 0 {
                fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
            }
            fprintf(stderr, b"mask-underflow\0" as *const u8 as *const libc::c_char);
            comma += 1;
            comma;
        }
    }
    if exception_mask & GSL_IEEE_TRAP_INEXACT as libc::c_int != 0 {
        if comma != 0 {
            fprintf(stderr, b",\0" as *const u8 as *const libc::c_char);
        }
        fprintf(stderr, b"trap-inexact\0" as *const u8 as *const libc::c_char);
        comma += 1;
        comma;
    }
    fprintf(stderr, b"\"\n\0" as *const u8 as *const libc::c_char);
}
