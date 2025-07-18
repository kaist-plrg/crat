use ::libc;
extern "C" {
    pub type internal_state;
    fn vsprintf(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: ::std::ffi::VaList,
    ) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn _glp_zlib_write(
        fd: libc::c_int,
        buf: *const libc::c_void,
        nbyte: libc::c_ulong,
    ) -> libc::c_long;
    fn _glp_zlib_close(fd: libc::c_int) -> libc::c_int;
    fn _glp_zlib_deflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn _glp_zlib_deflateEnd(strm: z_streamp) -> libc::c_int;
    fn _glp_zlib_deflateReset(strm: z_streamp) -> libc::c_int;
    fn _glp_zlib_deflateParams(
        strm: z_streamp,
        level: libc::c_int,
        strategy: libc::c_int,
    ) -> libc::c_int;
    fn _glp_zlib_gz_error(_: gz_statep, _: libc::c_int, _: *const libc::c_char);
    fn _glp_zlib_deflateInit2_(
        strm: z_streamp,
        level: libc::c_int,
        method: libc::c_int,
        windowBits: libc::c_int,
        memLevel: libc::c_int,
        strategy: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: libc::c_uint,
    pub fp_offset: libc::c_uint,
    pub overflow_arg_area: *mut libc::c_void,
    pub reg_save_area: *mut libc::c_void,
}
pub type va_list = __builtin_va_list;
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
pub type voidpc = *const libc::c_void;
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
pub type z_streamp = *mut z_stream;
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
unsafe extern "C" fn gz_init(mut state: gz_statep) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    (*state).in_0 = malloc((*state).want as libc::c_ulong) as *mut libc::c_uchar;
    (*state).out = malloc((*state).want as libc::c_ulong) as *mut libc::c_uchar;
    if ((*state).in_0).is_null() || ((*state).out).is_null() {
        if !((*state).out).is_null() {
            free((*state).out as *mut libc::c_void);
        }
        if !((*state).in_0).is_null() {
            free((*state).in_0 as *mut libc::c_void);
        }
        _glp_zlib_gz_error(
            state,
            -(4 as libc::c_int),
            b"out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*strm).zalloc = None;
    (*strm).zfree = None;
    (*strm).opaque = 0 as voidpf;
    ret = _glp_zlib_deflateInit2_(
        strm,
        (*state).level,
        8 as libc::c_int,
        15 as libc::c_int + 16 as libc::c_int,
        8 as libc::c_int,
        (*state).strategy,
        b"1.2.5\0" as *const u8 as *const libc::c_char,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
    );
    if ret != 0 as libc::c_int {
        free((*state).in_0 as *mut libc::c_void);
        _glp_zlib_gz_error(
            state,
            -(4 as libc::c_int),
            b"out of memory\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    (*state).size = (*state).want;
    (*strm).avail_out = (*state).size;
    (*strm).next_out = (*state).out;
    (*state).next = (*strm).next_out;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_comp(
    mut state: gz_statep,
    mut flush: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut got: libc::c_int = 0;
    let mut have: libc::c_uint = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).size == 0 as libc::c_int as libc::c_uint
        && gz_init(state) == -(1 as libc::c_int)
    {
        return -(1 as libc::c_int);
    }
    ret = 0 as libc::c_int;
    loop {
        if (*strm).avail_out == 0 as libc::c_int as libc::c_uint
            || flush != 0 as libc::c_int
                && (flush != 4 as libc::c_int || ret == 1 as libc::c_int)
        {
            have = ((*strm).next_out).offset_from((*state).next) as libc::c_long
                as libc::c_uint;
            if have != 0
                && {
                    got = _glp_zlib_write(
                        (*state).fd,
                        (*state).next as *const libc::c_void,
                        have as libc::c_ulong,
                    ) as libc::c_int;
                    got < 0 as libc::c_int || got as libc::c_uint != have
                }
            {
                _glp_zlib_gz_error(
                    state,
                    -(1 as libc::c_int),
                    strerror(*__errno_location()),
                );
                return -(1 as libc::c_int);
            }
            if (*strm).avail_out == 0 as libc::c_int as libc::c_uint {
                (*strm).avail_out = (*state).size;
                (*strm).next_out = (*state).out;
            }
            (*state).next = (*strm).next_out;
        }
        have = (*strm).avail_out;
        ret = _glp_zlib_deflate(strm, flush);
        if ret == -(2 as libc::c_int) {
            _glp_zlib_gz_error(
                state,
                -(2 as libc::c_int),
                b"internal error: deflate stream corrupt\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        have = have.wrapping_sub((*strm).avail_out);
        if !(have != 0) {
            break;
        }
    }
    if flush == 4 as libc::c_int {
        _glp_zlib_deflateReset(strm);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_zero(
    mut state: gz_statep,
    mut len: libc::c_long,
) -> libc::c_int {
    let mut first: libc::c_int = 0;
    let mut n: libc::c_uint = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    if (*strm).avail_in != 0 && gz_comp(state, 0 as libc::c_int) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    first = 1 as libc::c_int;
    while len != 0 {
        n = if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
            && (*state).size > 2147483647 as libc::c_int as libc::c_uint
            || (*state).size as libc::c_long > len
        {
            len as libc::c_uint
        } else {
            (*state).size
        };
        if first != 0 {
            memset(
                (*state).in_0 as *mut libc::c_void,
                0 as libc::c_int,
                n as libc::c_ulong,
            );
            first = 0 as libc::c_int;
        }
        (*strm).avail_in = n;
        (*strm).next_in = (*state).in_0;
        (*state).pos += n as libc::c_long;
        if gz_comp(state, 0 as libc::c_int) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        len -= n as libc::c_long;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzwrite(
    mut file: gzFile,
    mut buf: voidpc,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut put: libc::c_uint = len;
    let mut n: libc::c_uint = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return 0 as libc::c_int;
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as libc::c_int || (*state).err != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (len as libc::c_int) < 0 as libc::c_int {
        _glp_zlib_gz_error(
            state,
            -(5 as libc::c_int),
            b"requested length does not fit in int\0" as *const u8 as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*state).size == 0 as libc::c_int as libc::c_uint
        && gz_init(state) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    if len < (*state).size {
        loop {
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                (*strm).next_in = (*state).in_0;
            }
            n = ((*state).size).wrapping_sub((*strm).avail_in);
            if n > len {
                n = len;
            }
            memcpy(
                ((*strm).next_in).offset((*strm).avail_in as isize) as *mut libc::c_void,
                buf,
                n as libc::c_ulong,
            );
            (*strm)
                .avail_in = ((*strm).avail_in as libc::c_uint).wrapping_add(n) as uInt
                as uInt;
            (*state).pos += n as libc::c_long;
            buf = (buf as *mut libc::c_char).offset(n as isize) as voidpc;
            len = len.wrapping_sub(n);
            if len != 0 && gz_comp(state, 0 as libc::c_int) == -(1 as libc::c_int) {
                return 0 as libc::c_int;
            }
            if !(len != 0) {
                break;
            }
        }
    } else {
        if (*strm).avail_in != 0
            && gz_comp(state, 0 as libc::c_int) == -(1 as libc::c_int)
        {
            return 0 as libc::c_int;
        }
        (*strm).avail_in = len;
        (*strm).next_in = buf as voidp as *mut Bytef;
        (*state).pos += len as libc::c_long;
        if gz_comp(state, 0 as libc::c_int) == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    return put as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzputc(
    mut file: gzFile,
    mut c: libc::c_int,
) -> libc::c_int {
    let mut buf: [libc::c_uchar; 1] = [0; 1];
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if (*strm).avail_in < (*state).size {
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
            (*strm).next_in = (*state).in_0;
        }
        let fresh0 = (*strm).avail_in;
        (*strm).avail_in = ((*strm).avail_in).wrapping_add(1);
        *((*strm).next_in).offset(fresh0 as isize) = c as Bytef;
        (*state).pos += 1;
        (*state).pos;
        return c;
    }
    buf[0 as libc::c_int as usize] = c as libc::c_uchar;
    if _glp_zlib_gzwrite(
        file,
        buf.as_mut_ptr() as voidpc,
        1 as libc::c_int as libc::c_uint,
    ) != 1 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    return c;
}
pub unsafe extern "C" fn _glp_zlib_gzputs(
    mut file: gzFile,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut len: libc::c_uint = 0;
    len = strlen(str) as libc::c_uint;
    ret = _glp_zlib_gzwrite(file, str as voidpc, len);
    return if ret == 0 as libc::c_int && len != 0 as libc::c_int as libc::c_uint {
        -(1 as libc::c_int)
    } else {
        ret
    };
}
pub unsafe extern "C" fn _glp_zlib_gzprintf(
    mut file: gzFile,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    let mut va: ::std::ffi::VaListImpl;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as libc::c_int || (*state).err != 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*state).size == 0 as libc::c_int as libc::c_uint
        && gz_init(state) == -(1 as libc::c_int)
    {
        return 0 as libc::c_int;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return 0 as libc::c_int;
        }
    }
    if (*strm).avail_in != 0 && gz_comp(state, 0 as libc::c_int) == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    size = (*state).size as libc::c_int;
    *((*state).in_0)
        .offset((size - 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_uchar;
    va = args.clone();
    len = vsprintf((*state).in_0 as *mut libc::c_char, format, va.as_va_list());
    if len <= 0 as libc::c_int || len >= size
        || *((*state).in_0).offset((size - 1 as libc::c_int) as isize) as libc::c_int
            != 0 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*strm).avail_in = len as libc::c_uint;
    (*strm).next_in = (*state).in_0;
    (*state).pos += len as libc::c_long;
    return len;
}
pub unsafe extern "C" fn _glp_zlib_gzflush(
    mut file: gzFile,
    mut flush: libc::c_int,
) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 31153 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if flush < 0 as libc::c_int || flush > 4 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    gz_comp(state, flush);
    return (*state).err;
}
pub unsafe extern "C" fn _glp_zlib_gzsetparams(
    mut file: gzFile,
    mut level: libc::c_int,
    mut strategy: libc::c_int,
) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return -(2 as libc::c_int);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 31153 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if level == (*state).level && strategy == (*state).strategy {
        return 0 as libc::c_int;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_zero(state, (*state).skip) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if (*state).size != 0 {
        if (*strm).avail_in != 0
            && gz_comp(state, 1 as libc::c_int) == -(1 as libc::c_int)
        {
            return (*state).err;
        }
        _glp_zlib_deflateParams(strm, level, strategy);
    }
    (*state).level = level;
    (*state).strategy = strategy;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzclose_w(mut file: gzFile) -> libc::c_int {
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(2 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 31153 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        ret += gz_zero(state, (*state).skip);
    }
    ret += gz_comp(state, 4 as libc::c_int);
    _glp_zlib_deflateEnd(&mut (*state).strm);
    free((*state).out as *mut libc::c_void);
    free((*state).in_0 as *mut libc::c_void);
    _glp_zlib_gz_error(state, 0 as libc::c_int, 0 as *const libc::c_char);
    free((*state).path as *mut libc::c_void);
    ret += _glp_zlib_close((*state).fd);
    free(state as *mut libc::c_void);
    return if ret != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
