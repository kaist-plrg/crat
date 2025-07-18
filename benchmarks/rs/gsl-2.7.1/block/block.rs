use ::libc;
pub type size_t = libc::c_ulong;
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
pub unsafe extern "C" fn gsl_block_size(mut b: *const gsl_block) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_uchar_size(mut b: *const gsl_block_uchar) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_char_size(mut b: *const gsl_block_char) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_short_size(mut b: *const gsl_block_short) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_ushort_size(
    mut b: *const gsl_block_ushort,
) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_size(
    mut b: *const gsl_block_complex_long_double,
) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_int_size(mut b: *const gsl_block_int) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_complex_size(
    mut b: *const gsl_block_complex,
) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_uint_size(mut b: *const gsl_block_uint) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_complex_float_size(
    mut b: *const gsl_block_complex_float,
) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_long_size(mut b: *const gsl_block_long) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_long_double_size(
    mut b: *const gsl_block_long_double,
) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_ulong_size(mut b: *const gsl_block_ulong) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_float_size(mut b: *const gsl_block_float) -> size_t {
    return (*b).size;
}
pub unsafe extern "C" fn gsl_block_ulong_data(
    mut b: *const gsl_block_ulong,
) -> *mut libc::c_ulong {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_data(mut b: *const gsl_block) -> *mut libc::c_double {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_float_data(
    mut b: *const gsl_block_float,
) -> *mut libc::c_float {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_long_double_data(
    mut b: *const gsl_block_long_double,
) -> *mut f128::f128 {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_short_data(
    mut b: *const gsl_block_short,
) -> *mut libc::c_short {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_complex_float_data(
    mut b: *const gsl_block_complex_float,
) -> *mut libc::c_float {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_uchar_data(
    mut b: *const gsl_block_uchar,
) -> *mut libc::c_uchar {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_complex_data(
    mut b: *const gsl_block_complex,
) -> *mut libc::c_double {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_long_data(
    mut b: *const gsl_block_long,
) -> *mut libc::c_long {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_complex_long_double_data(
    mut b: *const gsl_block_complex_long_double,
) -> *mut f128::f128 {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_uint_data(
    mut b: *const gsl_block_uint,
) -> *mut libc::c_uint {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_int_data(
    mut b: *const gsl_block_int,
) -> *mut libc::c_int {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_ushort_data(
    mut b: *const gsl_block_ushort,
) -> *mut libc::c_ushort {
    return (*b).data;
}
pub unsafe extern "C" fn gsl_block_char_data(
    mut b: *const gsl_block_char,
) -> *mut libc::c_char {
    return (*b).data;
}
