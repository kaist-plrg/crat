use ::libc;
extern "C" {
    pub type internal_state;
    fn _glp_zlib_gzclose_w(file: gzFile) -> libc::c_int;
    fn _glp_zlib_gzclose_r(file: gzFile) -> libc::c_int;
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpf = *mut libc::c_void;
pub type voidp = *mut libc::c_void;
pub type alloc_func = Option::<unsafe extern "C" fn(voidpf, uInt, uInt) -> voidpf>;
pub type free_func = Option::<unsafe extern "C" fn(voidpf, voidpf) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct z_stream_s {
    pub next_in: *mut Bytef,
    pub avail_in: uInt,
    pub total_in: uLong,
    pub next_out: *mut Bytef,
    pub avail_out: uInt,
    pub total_out: uLong,
    pub msg: *mut libc::c_char,
    pub state: *mut internal_state,
    pub zalloc: alloc_func,
    pub zfree: free_func,
    pub opaque: voidpf,
    pub data_type: libc::c_int,
    pub adler: uLong,
    pub reserved: uLong,
}
pub type z_stream = z_stream_s;
pub type gzFile = voidp;
pub type gz_statep = *mut gz_state;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_state {
    pub mode: libc::c_int,
    pub fd: libc::c_int,
    pub path: *mut libc::c_char,
    pub pos: libc::c_long,
    pub size: libc::c_uint,
    pub want: libc::c_uint,
    pub in_0: *mut libc::c_uchar,
    pub out: *mut libc::c_uchar,
    pub next: *mut libc::c_uchar,
    pub have: libc::c_uint,
    pub eof: libc::c_int,
    pub start: libc::c_long,
    pub raw: libc::c_long,
    pub how: libc::c_int,
    pub direct: libc::c_int,
    pub level: libc::c_int,
    pub strategy: libc::c_int,
    pub skip: libc::c_long,
    pub seek: libc::c_int,
    pub err: libc::c_int,
    pub msg: *mut libc::c_char,
    pub strm: z_stream,
}
pub unsafe extern "C" fn _glp_zlib_gzclose(mut file: gzFile) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(2 as libc::c_int);
    }
    state = file as gz_statep;
    return if (*state).mode == 7247 as libc::c_int {
        _glp_zlib_gzclose_r(file)
    } else {
        _glp_zlib_gzclose_w(file)
    };
}
