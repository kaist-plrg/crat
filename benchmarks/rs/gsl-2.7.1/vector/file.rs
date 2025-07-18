use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn gsl_block_char_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_char,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_char_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_char,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_char_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_char,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_char_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_char,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_double,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_complex_float_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_float,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_float_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_float,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_float_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_float,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_float_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_float,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_long_double_raw_fread(
        stream: *mut FILE,
        b: *mut f128::f128,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_long_double_raw_fwrite(
        stream: *mut FILE,
        b: *const f128::f128,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_long_double_raw_fscanf(
        stream: *mut FILE,
        b: *mut f128::f128,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_long_double_raw_fprintf(
        stream: *mut FILE,
        b: *const f128::f128,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_double,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_double,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_float_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_float,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_float_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_float,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_float_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_float,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_float_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_float,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_ulong_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_ulong,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_ulong_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_ulong,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_ulong_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_ulong,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_ulong_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_ulong,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_long_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_long,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_long_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_long,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_long_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_long,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_long_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_long,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_uint_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_uint,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_uint_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_uint,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_uint_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_uint,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_uint_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_uint,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_int_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_int,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_int_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_int,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_int_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_int,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_int_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_int,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_ushort_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_ushort,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_ushort_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_ushort,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_ushort_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_ushort,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_ushort_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_ushort,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_short_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_short,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_short_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_short,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_short_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_short,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_short_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_short,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_uchar_raw_fread(
        stream: *mut FILE,
        b: *mut libc::c_uchar,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_uchar_raw_fwrite(
        stream: *mut FILE,
        b: *const libc::c_uchar,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_uchar_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_uchar,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_uchar_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_uchar,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
    ) -> libc::c_int;
    fn gsl_block_complex_long_double_raw_fread(
        stream: *mut FILE,
        b: *mut f128::f128,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_long_double_raw_fwrite(
        stream: *mut FILE,
        b: *const f128::f128,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_long_double_raw_fscanf(
        stream: *mut FILE,
        b: *mut f128::f128,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_complex_long_double_raw_fprintf(
        stream: *mut FILE,
        b: *const f128::f128,
        n: size_t,
        stride: size_t,
        format: *const libc::c_char,
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
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_long_double {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_complex_float {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ulong {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_long {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uint {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_int {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_ushort {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_short {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_uchar {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_vector_char {
    pub size: size_t,
    pub stride: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
pub unsafe extern "C" fn gsl_vector_short_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_short,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_short_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uchar_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uchar_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_char_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_char_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_long_double_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_double_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_double_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uint_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_uint,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uint_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_float_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_float_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_float_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_float_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ulong_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ulong_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ushort_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ushort_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_long,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_int_fread(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_int,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_int_raw_fread(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uint_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_uint,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uint_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_long_double_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_int_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_int,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_int_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_char_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_char_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_double_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_double_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_complex,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uchar_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uchar_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_long,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_float_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_float_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ushort_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ushort_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_short_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_short,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_short_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ulong_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ulong_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_float_fwrite(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_float_raw_fwrite(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_int_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_int,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_int_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uint_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_uint,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uint_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_float_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_complex_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_float_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_char_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_char,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_char_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_float_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_float_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_complex_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_long_double_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ulong_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_ulong,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ulong_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_short_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_short,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_short_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_complex,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_double_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_double_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ushort_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_ushort,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ushort_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_long,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uchar_fprintf(
    mut stream: *mut FILE,
    mut v: *const gsl_vector_uchar,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uchar_raw_fprintf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
        format,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ulong_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ulong_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_char_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_char,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_char_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_complex,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uint_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_uint,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uint_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_uchar_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_uchar_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_ushort_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_ushort_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_long_double_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_long_double_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_float_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_float_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_complex_float_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_complex_float_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_int_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_int,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_int_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_double_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_double_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_long_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_long,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_long_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
pub unsafe extern "C" fn gsl_vector_short_fscanf(
    mut stream: *mut FILE,
    mut v: *mut gsl_vector_short,
) -> libc::c_int {
    let mut status: libc::c_int = gsl_block_short_raw_fscanf(
        stream,
        (*v).data,
        (*v).size,
        (*v).stride,
    );
    return status;
}
