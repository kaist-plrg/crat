use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdout: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn gsl_ieee_float_to_rep(x: *const libc::c_float, r: *mut gsl_ieee_float_rep);
    fn gsl_ieee_double_to_rep(x: *const libc::c_double, r: *mut gsl_ieee_double_rep);
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
pub const GSL_IEEE_TYPE_ZERO: C2RustUnnamed = 5;
pub const GSL_IEEE_TYPE_DENORMAL: C2RustUnnamed = 4;
pub const GSL_IEEE_TYPE_NORMAL: C2RustUnnamed = 3;
pub const GSL_IEEE_TYPE_INF: C2RustUnnamed = 2;
pub const GSL_IEEE_TYPE_NAN: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_float_rep {
    pub sign: libc::c_int,
    pub mantissa: [libc::c_char; 24],
    pub exponent: libc::c_int,
    pub type_0: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_ieee_double_rep {
    pub sign: libc::c_int,
    pub mantissa: [libc::c_char; 53],
    pub exponent: libc::c_int,
    pub type_0: libc::c_int,
}
static mut signs: [libc::c_char; 2] = [
    ' ' as i32 as libc::c_char,
    '-' as i32 as libc::c_char,
];
pub unsafe extern "C" fn gsl_ieee_fprintf_float(
    mut stream: *mut FILE,
    mut x: *const libc::c_float,
) {
    let mut r: gsl_ieee_float_rep = gsl_ieee_float_rep {
        sign: 0,
        mantissa: [0; 24],
        exponent: 0,
        type_0: 0,
    };
    gsl_ieee_float_to_rep(x, &mut r);
    match r.type_0 {
        1 => {
            fprintf(stream, b"NaN\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(
                stream,
                b"%cInf\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
            );
        }
        3 => {
            fprintf(
                stream,
                b"%c1.%s*2^%d\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
                (r.mantissa).as_mut_ptr(),
                r.exponent,
            );
        }
        4 => {
            fprintf(
                stream,
                b"%c0.%s*2^%d\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
                (r.mantissa).as_mut_ptr(),
                r.exponent + 1 as libc::c_int,
            );
        }
        5 => {
            fprintf(
                stream,
                b"%c0\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
            );
        }
        _ => {
            fprintf(
                stream,
                b"[non-standard IEEE float]\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
pub unsafe extern "C" fn gsl_ieee_printf_float(mut x: *const libc::c_float) {
    gsl_ieee_fprintf_float(stdout, x);
}
pub unsafe extern "C" fn gsl_ieee_fprintf_double(
    mut stream: *mut FILE,
    mut x: *const libc::c_double,
) {
    let mut r: gsl_ieee_double_rep = gsl_ieee_double_rep {
        sign: 0,
        mantissa: [0; 53],
        exponent: 0,
        type_0: 0,
    };
    gsl_ieee_double_to_rep(x, &mut r);
    match r.type_0 {
        1 => {
            fprintf(stream, b"NaN\0" as *const u8 as *const libc::c_char);
        }
        2 => {
            fprintf(
                stream,
                b"%cInf\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
            );
        }
        3 => {
            fprintf(
                stream,
                b"%c1.%s*2^%d\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
                (r.mantissa).as_mut_ptr(),
                r.exponent,
            );
        }
        4 => {
            fprintf(
                stream,
                b"%c0.%s*2^%d\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
                (r.mantissa).as_mut_ptr(),
                r.exponent + 1 as libc::c_int,
            );
        }
        5 => {
            fprintf(
                stream,
                b"%c0\0" as *const u8 as *const libc::c_char,
                signs[r.sign as usize] as libc::c_int,
            );
        }
        _ => {
            fprintf(
                stream,
                b"[non-standard IEEE double]\0" as *const u8 as *const libc::c_char,
            );
        }
    };
}
pub unsafe extern "C" fn gsl_ieee_printf_double(mut x: *const libc::c_double) {
    gsl_ieee_fprintf_double(stdout, x);
}
