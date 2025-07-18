use ::libc;
extern "C" {
    pub type internal_state;
    fn __errno_location() -> *mut libc::c_int;
    fn _glp_zlib_inflateInit2_(
        strm: z_streamp,
        windowBits: libc::c_int,
        version: *const libc::c_char,
        stream_size: libc::c_int,
    ) -> libc::c_int;
    fn _glp_zlib_crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn _glp_zlib_gz_error(_: gz_statep, _: libc::c_int, _: *const libc::c_char);
    fn _glp_zlib_inflateReset(strm: z_streamp) -> libc::c_int;
    fn _glp_zlib_inflateEnd(strm: z_streamp) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memchr(
        _: *const libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn _glp_zlib_read(
        fd: libc::c_int,
        buf: *mut libc::c_void,
        nbyte: libc::c_ulong,
    ) -> libc::c_long;
    fn _glp_zlib_close(fd: libc::c_int) -> libc::c_int;
    fn _glp_zlib_inflate(strm: z_streamp, flush: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
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
unsafe extern "C" fn gz_load(
    mut state: gz_statep,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
    mut have: *mut libc::c_uint,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    *have = 0 as libc::c_int as libc::c_uint;
    loop {
        ret = _glp_zlib_read(
            (*state).fd,
            buf.offset(*have as isize) as *mut libc::c_void,
            len.wrapping_sub(*have) as libc::c_ulong,
        ) as libc::c_int;
        if ret <= 0 as libc::c_int {
            break;
        }
        *have = (*have).wrapping_add(ret as libc::c_uint);
        if !(*have < len) {
            break;
        }
    }
    if ret < 0 as libc::c_int {
        _glp_zlib_gz_error(state, -(1 as libc::c_int), strerror(*__errno_location()));
        return -(1 as libc::c_int);
    }
    if ret == 0 as libc::c_int {
        (*state).eof = 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_avail(mut state: gz_statep) -> libc::c_int {
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*state).eof == 0 as libc::c_int {
        if gz_load(
            state,
            (*state).in_0,
            (*state).size,
            &mut (*strm).avail_in as *mut uInt as *mut libc::c_uint,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        (*strm).next_in = (*state).in_0;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_next4(
    mut state: gz_statep,
    mut ret: *mut libc::c_ulong,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut val: libc::c_ulong = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    val = (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
        && gz_avail(state) == -(1 as libc::c_int)
    {
        -(1 as libc::c_int)
    } else if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
        -(1 as libc::c_int)
    } else {
        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
        (*strm).avail_in;
        let fresh0 = (*strm).next_in;
        (*strm).next_in = ((*strm).next_in).offset(1);
        *fresh0 as libc::c_int
    }) as libc::c_ulong;
    val = val
        .wrapping_add(
            (((if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int)
            } else {
                (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh1 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh1 as libc::c_int
                })
            }) as libc::c_uint) << 8 as libc::c_int) as libc::c_ulong,
        );
    val = val
        .wrapping_add(
            ((if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int)
            } else {
                (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh2 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh2 as libc::c_int
                })
            }) as libc::c_ulong) << 16 as libc::c_int,
        );
    ch = if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
        && gz_avail(state) == -(1 as libc::c_int)
    {
        -(1 as libc::c_int)
    } else if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
        -(1 as libc::c_int)
    } else {
        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
        (*strm).avail_in;
        let fresh3 = (*strm).next_in;
        (*strm).next_in = ((*strm).next_in).offset(1);
        *fresh3 as libc::c_int
    };
    if ch == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    val = val.wrapping_add((ch as libc::c_ulong) << 24 as libc::c_int);
    *ret = val;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_head(mut state: gz_statep) -> libc::c_int {
    let mut strm: z_streamp = &mut (*state).strm;
    let mut flags: libc::c_int = 0;
    let mut len: libc::c_uint = 0;
    if (*state).size == 0 as libc::c_int as libc::c_uint {
        (*state).in_0 = malloc((*state).want as libc::c_ulong) as *mut libc::c_uchar;
        (*state)
            .out = malloc(((*state).want << 1 as libc::c_int) as libc::c_ulong)
            as *mut libc::c_uchar;
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
        (*state).size = (*state).want;
        (*state).strm.zalloc = None;
        (*state).strm.zfree = None;
        (*state).strm.opaque = 0 as voidpf;
        (*state).strm.avail_in = 0 as libc::c_int as uInt;
        (*state).strm.next_in = 0 as *mut Bytef;
        if _glp_zlib_inflateInit2_(
            &mut (*state).strm,
            -(15 as libc::c_int),
            b"1.2.5\0" as *const u8 as *const libc::c_char,
            ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int,
        ) != 0 as libc::c_int
        {
            free((*state).out as *mut libc::c_void);
            free((*state).in_0 as *mut libc::c_void);
            (*state).size = 0 as libc::c_int as libc::c_uint;
            _glp_zlib_gz_error(
                state,
                -(4 as libc::c_int),
                b"out of memory\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
    }
    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
        if gz_avail(state) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
    }
    if *((*strm).next_in).offset(0 as libc::c_int as isize) as libc::c_int
        == 31 as libc::c_int
    {
        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
        (*strm).avail_in;
        (*strm).next_in = ((*strm).next_in).offset(1);
        (*strm).next_in;
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
            && gz_avail(state) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if (*strm).avail_in != 0
            && *((*strm).next_in).offset(0 as libc::c_int as isize) as libc::c_int
                == 139 as libc::c_int
        {
            (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
            (*strm).avail_in;
            (*strm).next_in = ((*strm).next_in).offset(1);
            (*strm).next_in;
            if (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int)
            } else {
                (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh4 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh4 as libc::c_int
                })
            }) != 8 as libc::c_int
            {
                _glp_zlib_gz_error(
                    state,
                    -(3 as libc::c_int),
                    b"unknown compression method\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            flags = if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int)
            } else if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                -(1 as libc::c_int)
            } else {
                (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                (*strm).avail_in;
                let fresh5 = (*strm).next_in;
                (*strm).next_in = ((*strm).next_in).offset(1);
                *fresh5 as libc::c_int
            };
            if flags & 0xe0 as libc::c_int != 0 {
                _glp_zlib_gz_error(
                    state,
                    -(3 as libc::c_int),
                    b"unknown header flags set\0" as *const u8 as *const libc::c_char,
                );
                return -(1 as libc::c_int);
            }
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh6 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh6;
                };
            };
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh7 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh7;
                };
            };
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh8 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh8;
                };
            };
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh9 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh9;
                };
            };
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh10 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh10;
                };
            };
            if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                && gz_avail(state) == -(1 as libc::c_int)
            {
                -(1 as libc::c_int);
            } else {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int);
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh11 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh11;
                };
            };
            if flags & 4 as libc::c_int != 0 {
                len = (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                    && gz_avail(state) == -(1 as libc::c_int)
                {
                    -(1 as libc::c_int)
                } else if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                    -(1 as libc::c_int)
                } else {
                    (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                    (*strm).avail_in;
                    let fresh12 = (*strm).next_in;
                    (*strm).next_in = ((*strm).next_in).offset(1);
                    *fresh12 as libc::c_int
                }) as libc::c_uint;
                len = len
                    .wrapping_add(
                        ((if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                            && gz_avail(state) == -(1 as libc::c_int)
                        {
                            -(1 as libc::c_int)
                        } else {
                            (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                                -(1 as libc::c_int)
                            } else {
                                (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                                (*strm).avail_in;
                                let fresh13 = (*strm).next_in;
                                (*strm).next_in = ((*strm).next_in).offset(1);
                                *fresh13 as libc::c_int
                            })
                        }) as libc::c_uint) << 8 as libc::c_int,
                    );
                loop {
                    let fresh14 = len;
                    len = len.wrapping_sub(1);
                    if !(fresh14 != 0) {
                        break;
                    }
                    if (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                        && gz_avail(state) == -(1 as libc::c_int)
                    {
                        -(1 as libc::c_int)
                    } else {
                        (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                            -(1 as libc::c_int)
                        } else {
                            (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                            (*strm).avail_in;
                            let fresh15 = (*strm).next_in;
                            (*strm).next_in = ((*strm).next_in).offset(1);
                            *fresh15 as libc::c_int
                        })
                    }) < 0 as libc::c_int
                    {
                        break;
                    }
                }
            }
            if flags & 8 as libc::c_int != 0 {
                while (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                    && gz_avail(state) == -(1 as libc::c_int)
                {
                    -(1 as libc::c_int)
                } else {
                    (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        -(1 as libc::c_int)
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh16 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh16 as libc::c_int
                    })
                }) > 0 as libc::c_int
                {}
            }
            if flags & 16 as libc::c_int != 0 {
                while (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                    && gz_avail(state) == -(1 as libc::c_int)
                {
                    -(1 as libc::c_int)
                } else {
                    (if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        -(1 as libc::c_int)
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh17 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh17 as libc::c_int
                    })
                }) > 0 as libc::c_int
                {}
            }
            if flags & 2 as libc::c_int != 0 {
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                    && gz_avail(state) == -(1 as libc::c_int)
                {
                    -(1 as libc::c_int);
                } else {
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        -(1 as libc::c_int);
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh18 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh18;
                    };
                };
                if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
                    && gz_avail(state) == -(1 as libc::c_int)
                {
                    -(1 as libc::c_int);
                } else {
                    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
                        -(1 as libc::c_int);
                    } else {
                        (*strm).avail_in = ((*strm).avail_in).wrapping_sub(1);
                        (*strm).avail_in;
                        let fresh19 = (*strm).next_in;
                        (*strm).next_in = ((*strm).next_in).offset(1);
                        *fresh19;
                    };
                };
            }
            _glp_zlib_inflateReset(strm);
            (*strm)
                .adler = _glp_zlib_crc32(
                0 as libc::c_long as uLong,
                0 as *const Bytef,
                0 as libc::c_int as uInt,
            );
            (*state).how = 2 as libc::c_int;
            (*state).direct = 0 as libc::c_int;
            return 0 as libc::c_int;
        } else {
            *((*state).out)
                .offset(0 as libc::c_int as isize) = 31 as libc::c_int as libc::c_uchar;
            (*state).have = 1 as libc::c_int as libc::c_uint;
        }
    }
    (*state).raw = (*state).pos;
    (*state).next = (*state).out;
    if (*strm).avail_in != 0 {
        memcpy(
            ((*state).next).offset((*state).have as isize) as *mut libc::c_void,
            (*strm).next_in as *const libc::c_void,
            (*strm).avail_in as libc::c_ulong,
        );
        (*state).have = ((*state).have).wrapping_add((*strm).avail_in);
        (*strm).avail_in = 0 as libc::c_int as uInt;
    }
    (*state).how = 1 as libc::c_int;
    (*state).direct = 1 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_decomp(mut state: gz_statep) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut had: libc::c_uint = 0;
    let mut crc: libc::c_ulong = 0;
    let mut len: libc::c_ulong = 0;
    let mut strm: z_streamp = &mut (*state).strm;
    had = (*strm).avail_out;
    loop {
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
            && gz_avail(state) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        if (*strm).avail_in == 0 as libc::c_int as libc::c_uint {
            _glp_zlib_gz_error(
                state,
                -(3 as libc::c_int),
                b"unexpected end of file\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        ret = _glp_zlib_inflate(strm, 0 as libc::c_int);
        if ret == -(2 as libc::c_int) || ret == 2 as libc::c_int {
            _glp_zlib_gz_error(
                state,
                -(2 as libc::c_int),
                b"internal error: inflate stream corrupt\0" as *const u8
                    as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if ret == -(4 as libc::c_int) {
            _glp_zlib_gz_error(
                state,
                -(4 as libc::c_int),
                b"out of memory\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if ret == -(3 as libc::c_int) {
            _glp_zlib_gz_error(
                state,
                -(3 as libc::c_int),
                if ((*strm).msg).is_null() {
                    b"compressed data error\0" as *const u8 as *const libc::c_char
                } else {
                    (*strm).msg as *const libc::c_char
                },
            );
            return -(1 as libc::c_int);
        }
        if !((*strm).avail_out != 0 && ret != 1 as libc::c_int) {
            break;
        }
    }
    (*state).have = had.wrapping_sub((*strm).avail_out);
    (*state).next = ((*strm).next_out).offset(-((*state).have as isize));
    (*strm).adler = _glp_zlib_crc32((*strm).adler, (*state).next, (*state).have);
    if ret == 1 as libc::c_int {
        if gz_next4(state, &mut crc) == -(1 as libc::c_int)
            || gz_next4(state, &mut len) == -(1 as libc::c_int)
        {
            _glp_zlib_gz_error(
                state,
                -(3 as libc::c_int),
                b"unexpected end of file\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if crc != (*strm).adler {
            _glp_zlib_gz_error(
                state,
                -(3 as libc::c_int),
                b"incorrect data check\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        if len != (*strm).total_out & 0xffffffff as libc::c_long as libc::c_ulong {
            _glp_zlib_gz_error(
                state,
                -(3 as libc::c_int),
                b"incorrect length check\0" as *const u8 as *const libc::c_char,
            );
            return -(1 as libc::c_int);
        }
        (*state).how = 0 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_make(mut state: gz_statep) -> libc::c_int {
    let mut strm: z_streamp = &mut (*state).strm;
    if (*state).how == 0 as libc::c_int {
        if gz_head(state) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
        if (*state).have != 0 {
            return 0 as libc::c_int;
        }
    }
    if (*state).how == 1 as libc::c_int {
        if gz_load(
            state,
            (*state).out,
            (*state).size << 1 as libc::c_int,
            &mut (*state).have,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        (*state).next = (*state).out;
    } else if (*state).how == 2 as libc::c_int {
        (*strm).avail_out = (*state).size << 1 as libc::c_int;
        (*strm).next_out = (*state).out;
        if gz_decomp(state) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn gz_skip(
    mut state: gz_statep,
    mut len: libc::c_long,
) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    while len != 0 {
        if (*state).have != 0 {
            n = if ::std::mem::size_of::<libc::c_int>() as libc::c_ulong
                == ::std::mem::size_of::<libc::c_long>() as libc::c_ulong
                && (*state).have > 2147483647 as libc::c_int as libc::c_uint
                || (*state).have as libc::c_long > len
            {
                len as libc::c_uint
            } else {
                (*state).have
            };
            (*state).have = ((*state).have).wrapping_sub(n);
            (*state).next = ((*state).next).offset(n as isize);
            (*state).pos += n as libc::c_long;
            len -= n as libc::c_long;
        } else {
            if (*state).eof != 0
                && (*state).strm.avail_in == 0 as libc::c_int as libc::c_uint
            {
                break;
            }
            if gz_make(state) == -(1 as libc::c_int) {
                return -(1 as libc::c_int);
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzread(
    mut file: gzFile,
    mut buf: voidp,
    mut len: libc::c_uint,
) -> libc::c_int {
    let mut got: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    let mut strm: z_streamp = 0 as *mut z_stream;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    strm = &mut (*state).strm;
    if (*state).mode != 7247 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (len as libc::c_int) < 0 as libc::c_int {
        _glp_zlib_gz_error(
            state,
            -(5 as libc::c_int),
            b"requested length does not fit in int\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if len == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_skip(state, (*state).skip) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    got = 0 as libc::c_int as libc::c_uint;
    let mut current_block_39: u64;
    loop {
        if (*state).have != 0 {
            n = if (*state).have > len { len } else { (*state).have };
            memcpy(buf, (*state).next as *const libc::c_void, n as libc::c_ulong);
            (*state).next = ((*state).next).offset(n as isize);
            (*state).have = ((*state).have).wrapping_sub(n);
            current_block_39 = 8693738493027456495;
        } else {
            if (*state).eof != 0 && (*strm).avail_in == 0 as libc::c_int as libc::c_uint
            {
                break;
            }
            if (*state).how == 0 as libc::c_int
                || len < (*state).size << 1 as libc::c_int
            {
                if gz_make(state) == -(1 as libc::c_int) {
                    return -(1 as libc::c_int);
                }
                current_block_39 = 4166486009154926805;
            } else {
                if (*state).how == 1 as libc::c_int {
                    if gz_load(state, buf as *mut libc::c_uchar, len, &mut n)
                        == -(1 as libc::c_int)
                    {
                        return -(1 as libc::c_int);
                    }
                } else {
                    (*strm).avail_out = len;
                    (*strm).next_out = buf as *mut Bytef;
                    if gz_decomp(state) == -(1 as libc::c_int) {
                        return -(1 as libc::c_int);
                    }
                    n = (*state).have;
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                }
                current_block_39 = 8693738493027456495;
            }
        }
        match current_block_39 {
            8693738493027456495 => {
                len = len.wrapping_sub(n);
                buf = (buf as *mut libc::c_char).offset(n as isize) as voidp;
                got = got.wrapping_add(n);
                (*state).pos += n as libc::c_long;
            }
            _ => {}
        }
        if !(len != 0) {
            break;
        }
    }
    return got as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_gzgetc(mut file: gzFile) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut buf: [libc::c_uchar; 1] = [0; 1];
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*state).have != 0 {
        (*state).have = ((*state).have).wrapping_sub(1);
        (*state).have;
        (*state).pos += 1;
        (*state).pos;
        let fresh20 = (*state).next;
        (*state).next = ((*state).next).offset(1);
        return *fresh20 as libc::c_int;
    }
    ret = _glp_zlib_gzread(
        file,
        buf.as_mut_ptr() as voidp,
        1 as libc::c_int as libc::c_uint,
    );
    return if ret < 1 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        buf[0 as libc::c_int as usize] as libc::c_int
    };
}
pub unsafe extern "C" fn _glp_zlib_gzungetc(
    mut c: libc::c_int,
    mut file: gzFile,
) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int || (*state).err != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_skip(state, (*state).skip) == -(1 as libc::c_int) {
            return -(1 as libc::c_int);
        }
    }
    if c < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if (*state).have == 0 as libc::c_int as libc::c_uint {
        (*state).have = 1 as libc::c_int as libc::c_uint;
        (*state)
            .next = ((*state).out)
            .offset(((*state).size << 1 as libc::c_int) as isize)
            .offset(-(1 as libc::c_int as isize));
        *((*state).next).offset(0 as libc::c_int as isize) = c as libc::c_uchar;
        (*state).pos -= 1;
        (*state).pos;
        return c;
    }
    if (*state).have == (*state).size << 1 as libc::c_int {
        _glp_zlib_gz_error(
            state,
            -(5 as libc::c_int),
            b"out of room to push characters\0" as *const u8 as *const libc::c_char,
        );
        return -(1 as libc::c_int);
    }
    if (*state).next == (*state).out {
        let mut src: *mut libc::c_uchar = ((*state).out).offset((*state).have as isize);
        let mut dest: *mut libc::c_uchar = ((*state).out)
            .offset(((*state).size << 1 as libc::c_int) as isize);
        while src > (*state).out {
            src = src.offset(-1);
            dest = dest.offset(-1);
            *dest = *src;
        }
        (*state).next = dest;
    }
    (*state).have = ((*state).have).wrapping_add(1);
    (*state).have;
    (*state).next = ((*state).next).offset(-1);
    (*state).next;
    *((*state).next).offset(0 as libc::c_int as isize) = c as libc::c_uchar;
    (*state).pos -= 1;
    (*state).pos;
    return c;
}
pub unsafe extern "C" fn _glp_zlib_gzgets(
    mut file: gzFile,
    mut buf: *mut libc::c_char,
    mut len: libc::c_int,
) -> *mut libc::c_char {
    let mut left: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut str: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut eol: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() || buf.is_null() || len < 1 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int || (*state).err != 0 as libc::c_int {
        return 0 as *mut libc::c_char;
    }
    if (*state).seek != 0 {
        (*state).seek = 0 as libc::c_int;
        if gz_skip(state, (*state).skip) == -(1 as libc::c_int) {
            return 0 as *mut libc::c_char;
        }
    }
    str = buf;
    left = (len as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint);
    if left != 0 {
        loop {
            if (*state).have == 0 as libc::c_int as libc::c_uint {
                if gz_make(state) == -(1 as libc::c_int) {
                    return 0 as *mut libc::c_char;
                }
                if (*state).have == 0 as libc::c_int as libc::c_uint {
                    if buf == str {
                        return 0 as *mut libc::c_char;
                    }
                    break;
                }
            }
            n = if (*state).have > left { left } else { (*state).have };
            eol = memchr(
                (*state).next as *const libc::c_void,
                '\n' as i32,
                n as libc::c_ulong,
            ) as *mut libc::c_uchar;
            if !eol.is_null() {
                n = (eol.offset_from((*state).next) as libc::c_long as libc::c_uint)
                    .wrapping_add(1 as libc::c_int as libc::c_uint);
            }
            memcpy(
                buf as *mut libc::c_void,
                (*state).next as *const libc::c_void,
                n as libc::c_ulong,
            );
            (*state).have = ((*state).have).wrapping_sub(n);
            (*state).next = ((*state).next).offset(n as isize);
            (*state).pos += n as libc::c_long;
            left = left.wrapping_sub(n);
            buf = buf.offset(n as isize);
            if !(left != 0 && eol.is_null()) {
                break;
            }
        }
    }
    *buf.offset(0 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    return str;
}
pub unsafe extern "C" fn _glp_zlib_gzdirect(mut file: gzFile) -> libc::c_int {
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return 0 as libc::c_int;
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*state).how == 0 as libc::c_int
        && (*state).have == 0 as libc::c_int as libc::c_uint
    {
        gz_head(state);
    }
    return (*state).direct;
}
pub unsafe extern "C" fn _glp_zlib_gzclose_r(mut file: gzFile) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut state: gz_statep = 0 as *mut gz_state;
    if file.is_null() {
        return -(2 as libc::c_int);
    }
    state = file as gz_statep;
    if (*state).mode != 7247 as libc::c_int {
        return -(2 as libc::c_int);
    }
    if (*state).size != 0 {
        _glp_zlib_inflateEnd(&mut (*state).strm);
        free((*state).out as *mut libc::c_void);
        free((*state).in_0 as *mut libc::c_void);
    }
    _glp_zlib_gz_error(state, 0 as libc::c_int, 0 as *const libc::c_char);
    free((*state).path as *mut libc::c_void);
    ret = _glp_zlib_close((*state).fd);
    free(state as *mut libc::c_void);
    return if ret != 0 { -(1 as libc::c_int) } else { 0 as libc::c_int };
}
