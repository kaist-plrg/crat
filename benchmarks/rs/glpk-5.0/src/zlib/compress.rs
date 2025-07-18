use ::libc;
extern "C" {
    pub type internal_state;
    fn _glp_zlib_deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn _glp_zlib_deflateEnd(strm: z_streamp) -> libc::c_int;
    fn _glp_zlib_deflateInit_(
        strm: z_streamp,
        level: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type uLongf = uLong;
pub type voidpf = *mut libc::c_void;
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
pub type z_streamp = *mut z_stream;
pub unsafe extern "C" fn _glp_zlib_compress2(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut stream: z_stream = z_stream {
        next_in: 0 as *mut Bytef,
        avail_in: 0,
        total_in: 0,
        next_out: 0 as *mut Bytef,
        avail_out: 0,
        total_out: 0,
        msg: 0 as *mut libc::c_char,
        state: 0 as *mut internal_state,
        zalloc: None,
        zfree: None,
        opaque: 0 as *mut libc::c_void,
        data_type: 0,
        adler: 0,
        reserved: 0,
    };
    let mut err: libc::c_int = 0;
    stream.next_in = source as *mut Bytef;
    stream.avail_in = sourceLen as uInt;
    stream.next_out = dest;
    stream.avail_out = *destLen as uInt;
    if stream.avail_out as uLong != *destLen {
        return -(5 as libc::c_int);
    }
    stream.zalloc = None;
    stream.zfree = None;
    stream.opaque = 0 as voidpf;
    err = _glp_zlib_deflateInit_(
        &mut stream,
        level,
        b"1.2.5\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if err != 0 as libc::c_int {
        return err;
    }
    err = _glp_zlib_deflate(&mut stream, 4 as libc::c_int);
    if err != 1 as libc::c_int {
        _glp_zlib_deflateEnd(&mut stream);
        return if err == 0 as libc::c_int { -(5 as libc::c_int) } else { err };
    }
    *destLen = stream.total_out;
    err = _glp_zlib_deflateEnd(&mut stream);
    return err;
}
pub unsafe extern "C" fn _glp_zlib_compress(
    mut dest: *mut Bytef,
    mut destLen: *mut uLongf,
    mut source: *const Bytef,
    mut sourceLen: uLong,
) -> libc::c_int {
    return _glp_zlib_compress2(dest, destLen, source, sourceLen, -(1 as libc::c_int));
}
pub unsafe extern "C" fn _glp_zlib_compressBound(mut sourceLen: uLong) -> uLong {
    return sourceLen
        .wrapping_add(sourceLen >> 12 as libc::c_int)
        .wrapping_add(sourceLen >> 14 as libc::c_int)
        .wrapping_add(sourceLen >> 25 as libc::c_int)
        .wrapping_add(13 as libc::c_int as libc::c_ulong);
}
