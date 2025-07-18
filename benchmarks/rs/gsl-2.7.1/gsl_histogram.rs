use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fscanf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn exit(_: libc::c_int) -> !;
    fn gsl_histogram_alloc(n: size_t) -> *mut gsl_histogram;
    fn gsl_histogram_free(h: *mut gsl_histogram);
    fn gsl_histogram_increment(h: *mut gsl_histogram, x: libc::c_double) -> libc::c_int;
    fn gsl_histogram_set_ranges_uniform(
        h: *mut gsl_histogram,
        xmin: libc::c_double,
        xmax: libc::c_double,
    ) -> libc::c_int;
    fn gsl_histogram_fprintf(
        stream: *mut FILE,
        h: *const gsl_histogram,
        range_format: *const libc::c_char,
        bin_format: *const libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_histogram {
    pub n: size_t,
    pub range: *mut libc::c_double,
    pub bin: *mut libc::c_double,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut a: libc::c_double = 0.0f64;
    let mut b: libc::c_double = 1.0f64;
    let mut n: size_t = 10 as libc::c_int as size_t;
    if argc != 3 as libc::c_int && argc != 4 as libc::c_int {
        printf(
            b"Usage: gsl-histogram xmin xmax [n]\n\0" as *const u8 as *const libc::c_char,
        );
        printf(
            b"Computes a histogram of the data on stdin using n bins from xmin to xmax.\nIf n is unspecified then bins of integer width are used.\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(0 as libc::c_int);
    }
    a = atof(*argv.offset(1 as libc::c_int as isize));
    b = atof(*argv.offset(2 as libc::c_int as isize));
    if argc == 4 as libc::c_int {
        n = atoi(*argv.offset(3 as libc::c_int as isize)) as size_t;
    } else {
        n = (b - a) as libc::c_int as size_t;
    }
    let mut x: libc::c_double = 0.;
    let mut h: *mut gsl_histogram = gsl_histogram_alloc(n);
    gsl_histogram_set_ranges_uniform(h, a, b);
    while fscanf(
        stdin,
        b"%lg\0" as *const u8 as *const libc::c_char,
        &mut x as *mut libc::c_double,
    ) == 1 as libc::c_int
    {
        gsl_histogram_increment(h, x);
    }
    gsl_histogram_fprintf(
        stdout,
        h,
        b"%g\0" as *const u8 as *const libc::c_char,
        b"%g\0" as *const u8 as *const libc::c_char,
    );
    gsl_histogram_free(h);
    return 0 as libc::c_int;
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
