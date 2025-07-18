use ::libc;
extern "C" {
    pub type internal_state;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn _glp_zlib_open(
        path: *const libc::c_char,
        oflag: libc::c_int,
        _: ...
    ) -> libc::c_int;
    fn _glp_zlib_lseek(
        fd: libc::c_int,
        offset: libc::c_long,
        whence: libc::c_int,
    ) -> libc::c_long;
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
unsafe extern "C" fn gz_reset(mut state: gz_statep) {
    if (*state).mode == 7247 as libc::c_int {
        (*state).have = 0 as libc::c_int as libc::c_uint;
        (*state).eof = 0 as libc::c_int;
        (*state).how = 0 as libc::c_int;
        (*state).direct = 1 as libc::c_int;
    }
    (*state).seek = 0 as libc::c_int;
    _glp_zlib_gz_error(state, 0 as libc::c_int, 0 as *const libc::c_char);
    (*state).pos = 0 as libc::c_int as libc::c_long;
    (*state).strm.avail_in = 0 as libc::c_int as uInt;
}
unsafe extern "C" fn gz_open(
    mut path: *const libc::c_char,
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
) -> gzFile {
    let mut state: gz_statep = 0 as *mut gz_state;
    state = malloc(::std::mem::size_of::<gz_state>() as libc::c_ulong) as gz_statep;
    if state.is_null() {
        return 0 as *mut libc::c_void;
    }
    (*state).size = 0 as libc::c_int as libc::c_uint;
    (*state).want = 8192 as libc::c_int as libc::c_uint;
    (*state).msg = 0 as *mut libc::c_char;
    (*state).mode = 0 as libc::c_int;
    (*state).level = -(1 as libc::c_int);
    (*state).strategy = 0 as libc::c_int;
    while *mode != 0 {
        if *mode as libc::c_int >= '0' as i32 && *mode as libc::c_int <= '9' as i32 {
            (*state).level = *mode as libc::c_int - '0' as i32;
        } else {
            match *mode as libc::c_int {
                114 => {
                    (*state).mode = 7247 as libc::c_int;
                }
                119 => {
                    (*state).mode = 31153 as libc::c_int;
                }
                97 => {
                    (*state).mode = 1 as libc::c_int;
                }
                43 => {
                    free(state as *mut libc::c_void);
                    return 0 as *mut libc::c_void;
                }
                98 => {}
                102 => {
                    (*state).strategy = 1 as libc::c_int;
                }
                104 => {
                    (*state).strategy = 2 as libc::c_int;
                }
                82 => {
                    (*state).strategy = 3 as libc::c_int;
                }
                70 => {
                    (*state).strategy = 4 as libc::c_int;
                }
                _ => {}
            }
        }
        mode = mode.offset(1);
        mode;
    }
    if (*state).mode == 0 as libc::c_int {
        free(state as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    (*state)
        .path = malloc((strlen(path)).wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    if ((*state).path).is_null() {
        free(state as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    strcpy((*state).path, path);
    (*state)
        .fd = if fd != -(1 as libc::c_int) {
        fd
    } else {
        _glp_zlib_open(
            path,
            if (*state).mode == 7247 as libc::c_int {
                0 as libc::c_int
            } else {
                0x1 as libc::c_int | 0x10 as libc::c_int
                    | (if (*state).mode == 31153 as libc::c_int {
                        0x20 as libc::c_int
                    } else {
                        0x30 as libc::c_int
                    })
            },
            0o666 as libc::c_int,
        )
    };
    if (*state).fd == -(1 as libc::c_int) {
        free((*state).path as *mut libc::c_void);
        free(state as *mut libc::c_void);
        return 0 as *mut libc::c_void;
    }
    if (*state).mode == 1 as libc::c_int {
        (*state).mode = 31153 as libc::c_int;
    }
    if (*state).mode == 7247 as libc::c_int {
        (*state)
            .start = _glp_zlib_lseek(
            (*state).fd,
            0 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        );
        if (*state).start == -(1 as libc::c_int) as libc::c_long {
            (*state).start = 0 as libc::c_int as libc::c_long;
        }
    }
    gz_reset(state);
    return state as gzFile;
}
pub unsafe extern "C" fn _glp_zlib_gzopen(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> gzFile {
    return gz_open(path, -(1 as libc::c_int), mode);
}
pub unsafe extern "C" fn _glp_zlib_gzopen64(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> gzFile {
    return gz_open(path, -(1 as libc::c_int), mode);
}
pub unsafe extern "C" fn _glp_zlib_gzdopen(
    mut fd: libc::c_int,
    mut mode: *const libc::c_char,
) -> gzFile {
    let mut path: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut gz: gzFile = 0 as *mut libc::c_void;
    if fd == -(1 as libc::c_int)
        || {
            path = malloc(
                (7 as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        (3 as libc::c_int as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                            ),
                    ),
            ) as *mut libc::c_char;
            path.is_null()
        }
    {
        return 0 as *mut libc::c_void;
    }
    sprintf(path, b"<fd:%d>\0" as *const u8 as *const libc::c_char, fd);
    gz = gz_open(path, fd, mode);
    free(path as *mut libc::c_void);
    return gz;
}
pub unsafe extern "C" fn _glp_zlib_gzbuffer(
    mut file: gzFile,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*state).size != 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    if size == 0 as libc::c_int as libc::c_uint {
        return -(1 as libc::c_int);
    }
    (*state).want = size;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzrewind(mut file: gzFile) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if _glp_zlib_lseek((*state).fd, (*state).start, 0 as libc::c_int)
        == -(1 as libc::c_int) as libc::c_long
    {
        return -(1 as libc::c_int);
    }
    gz_reset(state);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzseek64(
    mut file: gzFile,
    mut offset: libc::c_long,
    mut whence: libc::c_int,
) -> libc::c_long {
    let mut n: libc::c_uint = 0;
    let mut ret: libc::c_long = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if whence != 0 as libc::c_int && whence != 1 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if whence == 0 as libc::c_int {
        offset -= (*state).pos;
    } else if (*state).seek != 0 {
        offset += (*state).skip;
    }
    (*state).seek = 0 as libc::c_int;
    if (*state).mode == 7247 as libc::c_int && (*state).how == 1 as libc::c_int
        && (*state).pos + offset >= (*state).raw
    {
        ret = _glp_zlib_lseek(
            (*state).fd,
            offset - (*state).have as libc::c_long,
            1 as libc::c_int,
        );
        if ret == -(1 as libc::c_int) as libc::c_long {
            return -(1 as libc::c_int) as libc::c_long;
        }
        (*state).have = 0 as libc::c_int as libc::c_uint;
        (*state).eof = 0 as libc::c_int;
        (*state).seek = 0 as libc::c_int;
        _glp_zlib_gz_error(state, 0 as libc::c_int, 0 as *const libc::c_char);
        (*state).strm.avail_in = 0 as libc::c_int as uInt;
        (*state).pos += offset;
        return (*state).pos;
    }
    if offset < 0 as libc::c_int as libc::c_long {
        if (*state).mode != 7247 as libc::c_int {
            return -(1 as libc::c_int) as libc::c_long;
        }
        offset += (*state).pos;
        if offset < 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int) as libc::c_long;
        }
        if _glp_zlib_gzrewind(file) == -(1 as libc::c_int) {
            return -(1 as libc::c_int) as libc::c_long;
        }
    }
    if (*state).mode == 7247 as libc::c_int {
        n = if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
            == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
            && (*state).have > 2147483647 as libc::c_int as libc::c_uint
            || (*state).have as libc::c_long > offset
        {
            offset as libc::c_uint
        } else {
            (*state).have
        };
        (*state).have = ((*state).have).wrapping_sub(n);
        (*state).next = ((*state).next).offset(n as isize);
        (*state).pos += n as libc::c_long;
        offset -= n as libc::c_long;
    }
    if offset != 0 {
        (*state).seek = 1 as libc::c_int;
        (*state).skip = offset;
    }
    return (*state).pos + offset;
}
pub unsafe extern "C" fn _glp_zlib_gzseek(
    mut file: gzFile,
    mut offset: libc::c_long,
    mut whence: libc::c_int,
) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    ret = _glp_zlib_gzseek64(file, offset, whence);
    return if ret == ret { ret } else { -(1 as libc::c_int) as libc::c_long };
}
pub unsafe extern "C" fn _glp_zlib_gztell64(mut file: gzFile) -> libc::c_long {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    return (*state).pos
        + (if (*state).seek != 0 {
            (*state).skip
        } else {
            0 as libc::c_int as libc::c_long
        });
}
pub unsafe extern "C" fn _glp_zlib_gztell(mut file: gzFile) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    ret = _glp_zlib_gztell64(file);
    return if ret == ret { ret } else { -(1 as libc::c_int) as libc::c_long };
}
pub unsafe extern "C" fn _glp_zlib_gzoffset64(mut file: gzFile) -> libc::c_long {
    let mut offset: libc::c_long = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int) as libc::c_long;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return -(1 as libc::c_int) as libc::c_long;
    }
    offset = _glp_zlib_lseek(
        (*state).fd,
        0 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    );
    if offset == -(1 as libc::c_int) as libc::c_long {
        return -(1 as libc::c_int) as libc::c_long;
    }
    if (*state).mode == 7247 as libc::c_int {
        offset -= (*state).strm.avail_in as libc::c_long;
    }
    return offset;
}
pub unsafe extern "C" fn _glp_zlib_gzoffset(mut file: gzFile) -> libc::c_long {
    let mut ret: libc::c_long = 0;
    ret = _glp_zlib_gzoffset64(file);
    return if ret == ret { ret } else { -(1 as libc::c_int) as libc::c_long };
}
pub unsafe extern "C" fn _glp_zlib_gzeof(mut file: gzFile) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return 0 as libc::c_int;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return 0 as libc::c_int;
    }
    return if (*state).mode == 7247 as libc::c_int {
        ((*state).eof != 0 && (*state).strm.avail_in == 0 as libc::c_int as libc::c_uint
            && (*state).have == 0 as libc::c_int as libc::c_uint) as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub unsafe extern "C" fn _glp_zlib_gzerror(
    mut file: gzFile,
    mut errnum: *mut libc::c_int,
) -> *const libc::c_char {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return 0 as *const libc::c_char;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return 0 as *const libc::c_char;
    }
    if !errnum.is_null() {
        *errnum = (*state).err;
    }
    return if ((*state).msg).is_null() {
        b"\0" as *const u8 as *const libc::c_char
    } else {
        (*state).msg as *const libc::c_char
    };
}
pub unsafe extern "C" fn _glp_zlib_gzclearerr(mut file: gzFile) {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int && (*state).mode != 31153 as libc::c_int {
        return;
    }
    if (*state).mode == 7247 as libc::c_int {
        (*state).eof = 0 as libc::c_int;
    }
    _glp_zlib_gz_error(state, 0 as libc::c_int, 0 as *const libc::c_char);
}
pub unsafe extern "C" fn _glp_zlib_gz_error(
    mut state: gz_statep,
    mut err: libc::c_int,
    mut msg: *const libc::c_char,
) {
    if !((*state).msg).is_null() {
        if (*state).err != -(4 as libc::c_int) {
            free((*state).msg as *mut libc::c_void);
        }
        (*state).msg = 0 as *mut libc::c_char;
    }
    (*state).err = err;
    if msg.is_null() {
        return;
    }
    if err == -(4 as libc::c_int) {
        (*state).msg = msg as *mut libc::c_char;
        return;
    }
    (*state)
        .msg = malloc(
        (strlen((*state).path))
            .wrapping_add(strlen(msg))
            .wrapping_add(3 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if ((*state).msg).is_null() {
        (*state).err = -(4 as libc::c_int);
        (*state)
            .msg = b"out of memory\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char;
        return;
    }
    strcpy((*state).msg, (*state).path);
    strcat((*state).msg, b": \0" as *const u8 as *const libc::c_char);
    strcat((*state).msg, msg);
}
