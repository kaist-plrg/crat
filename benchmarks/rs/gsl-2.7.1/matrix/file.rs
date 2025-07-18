use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
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
    fn gsl_block_char_raw_fread(
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
    fn gsl_block_char_raw_fscanf(
        stream: *mut FILE,
        b: *mut libc::c_char,
        n: size_t,
        stride: size_t,
    ) -> libc::c_int;
    fn gsl_block_char_raw_fprintf(
        stream: *mut FILE,
        b: *const libc::c_char,
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
pub struct gsl_block_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_long_double = gsl_block_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_long_double_struct {
    pub size: size_t,
    pub data: *mut f128::f128,
}
pub type gsl_block_complex_long_double = gsl_block_complex_long_double_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_complex_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block = gsl_block_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_struct {
    pub size: size_t,
    pub data: *mut libc::c_double,
}
pub type gsl_block_complex = gsl_block_complex_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block_complex,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_float = gsl_block_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_complex_float_struct {
    pub size: size_t,
    pub data: *mut libc::c_float,
}
pub type gsl_block_complex_float = gsl_block_complex_float_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_complex_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_complex_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long_double {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut f128::f128,
    pub block: *mut gsl_block_long_double,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_double,
    pub block: *mut gsl_block,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_float {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_float,
    pub block: *mut gsl_block_float,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ulong_struct {
    pub size: size_t,
    pub data: *mut libc::c_ulong,
}
pub type gsl_block_ulong = gsl_block_ulong_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ulong {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ulong,
    pub block: *mut gsl_block_ulong,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_long_struct {
    pub size: size_t,
    pub data: *mut libc::c_long,
}
pub type gsl_block_long = gsl_block_long_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_long {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_long,
    pub block: *mut gsl_block_long,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uint_struct {
    pub size: size_t,
    pub data: *mut libc::c_uint,
}
pub type gsl_block_uint = gsl_block_uint_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uint {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_uint,
    pub block: *mut gsl_block_uint,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_int_struct {
    pub size: size_t,
    pub data: *mut libc::c_int,
}
pub type gsl_block_int = gsl_block_int_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_int {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_int,
    pub block: *mut gsl_block_int,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_ushort_struct {
    pub size: size_t,
    pub data: *mut libc::c_ushort,
}
pub type gsl_block_ushort = gsl_block_ushort_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_ushort {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_ushort,
    pub block: *mut gsl_block_ushort,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_short_struct {
    pub size: size_t,
    pub data: *mut libc::c_short,
}
pub type gsl_block_short = gsl_block_short_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_short {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_short,
    pub block: *mut gsl_block_short,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_uchar_struct {
    pub size: size_t,
    pub data: *mut libc::c_uchar,
}
pub type gsl_block_uchar = gsl_block_uchar_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_uchar {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_uchar,
    pub block: *mut gsl_block_uchar,
    pub owner: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_block_char_struct {
    pub size: size_t,
    pub data: *mut libc::c_char,
}
pub type gsl_block_char = gsl_block_char_struct;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gsl_matrix_char {
    pub size1: size_t,
    pub size2: size_t,
    pub tda: size_t,
    pub data: *mut libc::c_char,
    pub block: *mut gsl_block_char,
    pub owner: libc::c_int,
}
pub unsafe extern "C" fn gsl_matrix_ulong_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ulong_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ulong_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_float_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_float_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_int_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_int_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_int_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uchar_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uchar_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uchar_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_long_double_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_long_double_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_double_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_double_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_double_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_char_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_char_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_char_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_short_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_short,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_short_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_short_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_long,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_complex,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_float_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_float,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_float_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_float_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ushort_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ushort_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ushort_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uint_fread(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_uint,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uint_raw_fread(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uint_raw_fread(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uint_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_uint,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uint_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uint_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ushort_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ushort_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ushort_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_float_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_float,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_float_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_float_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_int_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_int_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_int_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_complex,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uchar_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uchar_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uchar_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ulong_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ulong_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ulong_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_double_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_double_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_double_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_long_double_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_long_double_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_long,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_float_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_float_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_char_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_char_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_char_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_short_fwrite(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_short,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_short_raw_fwrite(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_short_raw_fwrite(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_int_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_int,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_int_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_int_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ulong_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_ulong,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ulong_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ulong_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uchar_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_uchar,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uchar_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uchar_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_short_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_short,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_short_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_short_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ushort_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_ushort,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ushort_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ushort_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_complex,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_complex_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_float_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_float_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_double_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_double_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_double_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_char_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_char,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_char_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_char_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_float_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_float,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_float_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_float_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_complex_long_double,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_long_double_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_long_double_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uint_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_uint,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uint_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uint_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_fprintf(
    mut stream: *mut FILE,
    mut m: *const gsl_matrix_long,
    mut format: *const libc::c_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_raw_fprintf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
            format,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_raw_fprintf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
                format,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_complex,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_long,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_float_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_complex_float,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_float_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_float_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uint_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_uint,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uint_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uint_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ulong_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_ulong,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ulong_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ulong_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_complex_long_double_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_complex_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_complex_long_double_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_complex_long_double_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_int_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_int,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_int_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_int_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_float_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_float,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_float_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_float_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_ushort_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_ushort,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_ushort_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_ushort_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_char_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_char,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_char_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_char_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_short_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_short,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_short_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_short_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_uchar_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_uchar,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_uchar_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_uchar_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
pub unsafe extern "C" fn gsl_matrix_long_double_fscanf(
    mut stream: *mut FILE,
    mut m: *mut gsl_matrix_long_double,
) -> libc::c_int {
    let mut status: libc::c_int = 0 as libc::c_int;
    let size1: size_t = (*m).size1;
    let size2: size_t = (*m).size2;
    let tda: size_t = (*m).tda;
    if tda == size2 {
        status = gsl_block_long_double_raw_fscanf(
            stream,
            (*m).data,
            size1.wrapping_mul(size2),
            1 as libc::c_int as size_t,
        );
    } else {
        let mut i: size_t = 0;
        i = 0 as libc::c_int as size_t;
        while i < size1 {
            status = gsl_block_long_double_raw_fscanf(
                stream,
                ((*m).data)
                    .offset(
                        i
                            .wrapping_mul(1 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(tda) as isize,
                    ),
                size2,
                1 as libc::c_int as size_t,
            );
            if status != 0 {
                break;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    return status;
}
