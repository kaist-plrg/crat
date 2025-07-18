use ::libc;
extern "C" {
    pub type internal_state;
    fn _glp_zlib_adler32(adler: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn _glp_zlib_crc32(crc: uLong, buf: *const Bytef, len: uInt) -> uLong;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn _glp_zlib_zcfree(opaque: voidpf, ptr: voidpf);
    fn _glp_zlib_zcalloc(
        opaque: voidpf,
        items: libc::c_uint,
        size: libc::c_uint,
    ) -> voidpf;
    fn _glp_zlib_inflate_table(
        type_0: codetype,
        lens: *mut libc::c_ushort,
        codes: libc::c_uint,
        table: *mut *mut code,
        bits: *mut libc::c_uint,
        work: *mut libc::c_ushort,
    ) -> libc::c_int;
    fn _glp_zlib_inflate_fast(strm: z_streamp, start: libc::c_uint);
}
pub type Byte = libc::c_uchar;
pub type uInt = libc::c_uint;
pub type uLong = libc::c_ulong;
pub type Bytef = Byte;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct gz_header_s {
    pub text: libc::c_int,
    pub time: uLong,
    pub xflags: libc::c_int,
    pub os: libc::c_int,
    pub extra: *mut Bytef,
    pub extra_len: uInt,
    pub extra_max: uInt,
    pub name: *mut Bytef,
    pub name_max: uInt,
    pub comment: *mut Bytef,
    pub comm_max: uInt,
    pub hcrc: libc::c_int,
    pub done: libc::c_int,
}
pub type gz_header = gz_header_s;
pub type gz_headerp = *mut gz_header;
pub const COPY_: inflate_mode = 14;
pub type inflate_mode = libc::c_uint;
pub const SYNC: inflate_mode = 31;
pub const MEM: inflate_mode = 30;
pub const BAD: inflate_mode = 29;
pub const DONE: inflate_mode = 28;
pub const LENGTH: inflate_mode = 27;
pub const CHECK: inflate_mode = 26;
pub const LIT: inflate_mode = 25;
pub const MATCH: inflate_mode = 24;
pub const DISTEXT: inflate_mode = 23;
pub const DIST: inflate_mode = 22;
pub const LENEXT: inflate_mode = 21;
pub const LEN: inflate_mode = 20;
pub const LEN_: inflate_mode = 19;
pub const CODELENS: inflate_mode = 18;
pub const LENLENS: inflate_mode = 17;
pub const TABLE: inflate_mode = 16;
pub const COPY: inflate_mode = 15;
pub const STORED: inflate_mode = 13;
pub const TYPEDO: inflate_mode = 12;
pub const TYPE: inflate_mode = 11;
pub const DICT: inflate_mode = 10;
pub const DICTID: inflate_mode = 9;
pub const HCRC: inflate_mode = 8;
pub const COMMENT: inflate_mode = 7;
pub const NAME: inflate_mode = 6;
pub const EXTRA: inflate_mode = 5;
pub const EXLEN: inflate_mode = 4;
pub const OS: inflate_mode = 3;
pub const TIME: inflate_mode = 2;
pub const FLAGS: inflate_mode = 1;
pub const HEAD: inflate_mode = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct inflate_state {
    pub mode: inflate_mode,
    pub last: libc::c_int,
    pub wrap: libc::c_int,
    pub havedict: libc::c_int,
    pub flags: libc::c_int,
    pub dmax: libc::c_uint,
    pub check: libc::c_ulong,
    pub total: libc::c_ulong,
    pub head: gz_headerp,
    pub wbits: libc::c_uint,
    pub wsize: libc::c_uint,
    pub whave: libc::c_uint,
    pub wnext: libc::c_uint,
    pub window: *mut libc::c_uchar,
    pub hold: libc::c_ulong,
    pub bits: libc::c_uint,
    pub length: libc::c_uint,
    pub offset: libc::c_uint,
    pub extra: libc::c_uint,
    pub lencode: *const code,
    pub distcode: *const code,
    pub lenbits: libc::c_uint,
    pub distbits: libc::c_uint,
    pub ncode: libc::c_uint,
    pub nlen: libc::c_uint,
    pub ndist: libc::c_uint,
    pub have: libc::c_uint,
    pub next: *mut code,
    pub lens: [libc::c_ushort; 320],
    pub work: [libc::c_ushort; 288],
    pub codes: [code; 1444],
    pub sane: libc::c_int,
    pub back: libc::c_int,
    pub was: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}
pub type codetype = libc::c_uint;
pub const DISTS: codetype = 2;
pub const LENS: codetype = 1;
pub const CODES: codetype = 0;
pub unsafe extern "C" fn _glp_zlib_inflateReset(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    (*state).total = 0 as libc::c_int as libc::c_ulong;
    (*strm).total_out = (*state).total;
    (*strm).total_in = (*strm).total_out;
    (*strm).msg = 0 as *mut libc::c_char;
    (*strm).adler = 1 as libc::c_int as uLong;
    (*state).mode = HEAD;
    (*state).last = 0 as libc::c_int;
    (*state).havedict = 0 as libc::c_int;
    (*state).dmax = 32768 as libc::c_uint;
    (*state).head = 0 as gz_headerp;
    (*state).wsize = 0 as libc::c_int as libc::c_uint;
    (*state).whave = 0 as libc::c_int as libc::c_uint;
    (*state).wnext = 0 as libc::c_int as libc::c_uint;
    (*state).hold = 0 as libc::c_int as libc::c_ulong;
    (*state).bits = 0 as libc::c_int as libc::c_uint;
    (*state).next = ((*state).codes).as_mut_ptr();
    (*state).distcode = (*state).next;
    (*state).lencode = (*state).distcode;
    (*state).sane = 1 as libc::c_int;
    (*state).back = -(1 as libc::c_int);
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflateReset2(
    mut strm: z_streamp,
    mut windowBits: libc::c_int,
) -> libc::c_int {
    let mut wrap: libc::c_int = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if windowBits < 0 as libc::c_int {
        wrap = 0 as libc::c_int;
        windowBits = -windowBits;
    } else {
        wrap = (windowBits >> 4 as libc::c_int) + 1 as libc::c_int;
        if windowBits < 48 as libc::c_int {
            windowBits &= 15 as libc::c_int;
        }
    }
    if windowBits != 0
        && (windowBits < 8 as libc::c_int || windowBits > 15 as libc::c_int)
    {
        return -(2 as libc::c_int);
    }
    if !((*state).window).is_null() && (*state).wbits != windowBits as libc::c_uint {
        (Some(((*strm).zfree).unwrap()))
            .unwrap()((*strm).opaque, (*state).window as voidpf);
        (*state).window = 0 as *mut libc::c_uchar;
    }
    (*state).wrap = wrap;
    (*state).wbits = windowBits as libc::c_uint;
    return _glp_zlib_inflateReset(strm);
}
pub unsafe extern "C" fn _glp_zlib_inflateInit2_(
    mut strm: z_streamp,
    mut windowBits: libc::c_int,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if version.is_null()
        || *version.offset(0 as libc::c_int as isize) as libc::c_int
            != (*::std::mem::transmute::<
                &[u8; 6],
                &[libc::c_char; 6],
            >(b"1.2.5\0"))[0 as libc::c_int as usize] as libc::c_int
        || stream_size
            != ::std::mem::size_of::<z_stream>() as libc::c_ulong as libc::c_int
    {
        return -(6 as libc::c_int);
    }
    if strm.is_null() {
        return -(2 as libc::c_int);
    }
    (*strm).msg = 0 as *mut libc::c_char;
    if ((*strm).zalloc).is_none() {
        (*strm)
            .zalloc = Some(
            _glp_zlib_zcalloc
                as unsafe extern "C" fn(voidpf, libc::c_uint, libc::c_uint) -> voidpf,
        );
        (*strm).opaque = 0 as voidpf;
    }
    if ((*strm).zfree).is_none() {
        (*strm)
            .zfree = Some(
            _glp_zlib_zcfree as unsafe extern "C" fn(voidpf, voidpf) -> (),
        );
    }
    state = (Some(((*strm).zalloc).unwrap()))
        .unwrap()(
        (*strm).opaque,
        1 as libc::c_int as uInt,
        ::std::mem::size_of::<inflate_state>() as libc::c_ulong as uInt,
    ) as *mut inflate_state;
    if state.is_null() {
        return -(4 as libc::c_int);
    }
    (*strm).state = state as *mut internal_state;
    (*state).window = 0 as *mut libc::c_uchar;
    ret = _glp_zlib_inflateReset2(strm, windowBits);
    if ret != 0 as libc::c_int {
        (Some(((*strm).zfree).unwrap())).unwrap()((*strm).opaque, state as voidpf);
        (*strm).state = 0 as *mut internal_state;
    }
    return ret;
}
pub unsafe extern "C" fn _glp_zlib_inflateInit_(
    mut strm: z_streamp,
    mut version: *const libc::c_char,
    mut stream_size: libc::c_int,
) -> libc::c_int {
    return _glp_zlib_inflateInit2_(strm, 15 as libc::c_int, version, stream_size);
}
pub unsafe extern "C" fn _glp_zlib_inflatePrime(
    mut strm: z_streamp,
    mut bits: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if bits < 0 as libc::c_int {
        (*state).hold = 0 as libc::c_int as libc::c_ulong;
        (*state).bits = 0 as libc::c_int as libc::c_uint;
        return 0 as libc::c_int;
    }
    if bits > 16 as libc::c_int
        || ((*state).bits).wrapping_add(bits as libc::c_uint)
            > 32 as libc::c_int as libc::c_uint
    {
        return -(2 as libc::c_int);
    }
    value = (value as libc::c_long
        & ((1 as libc::c_long) << bits) - 1 as libc::c_int as libc::c_long)
        as libc::c_int;
    (*state)
        .hold = ((*state).hold).wrapping_add((value << (*state).bits) as libc::c_ulong);
    (*state).bits = ((*state).bits).wrapping_add(bits as libc::c_uint);
    return 0 as libc::c_int;
}
unsafe extern "C" fn fixedtables(mut state: *mut inflate_state) {
    static mut lenfix: [code; 512] = [
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 80 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 16 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 112 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 48 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 192 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 96 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 32 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 160 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 128 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 64 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 224 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 88 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 24 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 144 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 120 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 56 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 208 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 104 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 40 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 176 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 136 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 72 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 240 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 84 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 20 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 116 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 52 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 200 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 100 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 36 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 168 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 132 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 68 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 232 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 92 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 28 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 152 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 124 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 60 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 216 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 108 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 44 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 184 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 12 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 140 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 76 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 248 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 82 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 18 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 114 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 50 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 196 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 98 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 34 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 164 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 130 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 66 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 228 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 90 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 26 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 148 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 122 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 58 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 212 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 106 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 42 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 180 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 138 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 74 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 244 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 86 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 22 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 118 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 54 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 204 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 102 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 38 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 172 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 134 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 70 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 236 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 94 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 30 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 156 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 126 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 62 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 220 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 110 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 46 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 188 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 14 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 142 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 78 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 252 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 81 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 113 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 194 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 162 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 226 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 89 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 146 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 121 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 57 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 210 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 105 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 41 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 178 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 137 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 73 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 242 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 85 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 21 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 258 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 117 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 53 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 202 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 101 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 37 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 170 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 133 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 69 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 234 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 93 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 29 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 154 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 125 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 61 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 218 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 109 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 45 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 186 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 141 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 77 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 250 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 198 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 166 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 230 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 91 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 150 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 123 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 214 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 107 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 182 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 139 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 75 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 246 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 87 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 119 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 55 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 206 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 103 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 39 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 174 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 135 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 71 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 238 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 95 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 158 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 127 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 63 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 222 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 111 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 47 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 190 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 143 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 79 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 254 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 80 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 16 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 112 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 48 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 96 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 32 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 161 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 128 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 64 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 225 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 88 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 24 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 145 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 120 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 56 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 209 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 104 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 40 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 177 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 136 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 72 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 241 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 84 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 20 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 116 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 52 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 201 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 100 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 36 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 169 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 132 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 68 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 233 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 92 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 28 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 153 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 124 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 60 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 217 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 108 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 44 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 185 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 12 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 140 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 76 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 249 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 82 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 18 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 114 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 50 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 197 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 98 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 34 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 165 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 130 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 66 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 229 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 90 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 26 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 149 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 122 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 58 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 213 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 106 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 42 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 181 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 138 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 74 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 245 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 86 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 22 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 118 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 54 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 205 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 102 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 38 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 173 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 134 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 70 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 237 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 94 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 30 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 157 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 126 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 62 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 221 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 110 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 46 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 189 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 14 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 142 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 78 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 253 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 96 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 81 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 113 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 10 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 163 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 227 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 6 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 89 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 147 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 121 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 57 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 211 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 105 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 41 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 179 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 137 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 73 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 243 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 85 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 21 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 258 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 117 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 53 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 203 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 101 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 37 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 171 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 133 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 69 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 235 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 8 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 93 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 29 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 155 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 125 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 61 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 219 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 109 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 45 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 187 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 141 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 77 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 251 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 83 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 195 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 115 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 199 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 35 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 167 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 131 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 231 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 91 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 151 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 67 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 123 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 59 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 215 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 19 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 107 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 43 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 183 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 11 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 139 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 75 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 247 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 87 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 23 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 51 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 119 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 55 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 207 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 103 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 39 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 175 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 135 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 71 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 239 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 95 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 31 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 159 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 99 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 127 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 63 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 223 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 7 as libc::c_int as libc::c_uchar,
                val: 27 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 111 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 47 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 191 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 15 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 143 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 8 as libc::c_int as libc::c_uchar,
                val: 79 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 0 as libc::c_int as libc::c_uchar,
                bits: 9 as libc::c_int as libc::c_uchar,
                val: 255 as libc::c_int as libc::c_ushort,
            };
            init
        },
    ];
    static mut distfix: [code; 32] = [
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 257 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 17 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 4097 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 5 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1025 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 65 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 16385 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 3 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 513 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 33 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 8193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 9 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 2049 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 129 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 2 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 23 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 385 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 19 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 25 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 27 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 6145 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 17 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 7 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 25 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 1537 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 21 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 97 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 29 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 24577 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 16 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 4 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 24 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 769 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 20 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 49 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 28 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 12289 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 18 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 13 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 26 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 3073 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 22 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 193 as libc::c_int as libc::c_ushort,
            };
            init
        },
        {
            let mut init = code {
                op: 64 as libc::c_int as libc::c_uchar,
                bits: 5 as libc::c_int as libc::c_uchar,
                val: 0 as libc::c_int as libc::c_ushort,
            };
            init
        },
    ];
    (*state).lencode = lenfix.as_ptr();
    (*state).lenbits = 9 as libc::c_int as libc::c_uint;
    (*state).distcode = distfix.as_ptr();
    (*state).distbits = 5 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn updatewindow(
    mut strm: z_streamp,
    mut out: libc::c_uint,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut copy: libc::c_uint = 0;
    let mut dist: libc::c_uint = 0;
    state = (*strm).state as *mut inflate_state;
    if ((*state).window).is_null() {
        (*state)
            .window = (Some(((*strm).zalloc).unwrap()))
            .unwrap()(
            (*strm).opaque,
            (1 as libc::c_uint) << (*state).wbits,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as uInt,
        ) as *mut libc::c_uchar;
        if ((*state).window).is_null() {
            return 1 as libc::c_int;
        }
    }
    if (*state).wsize == 0 as libc::c_int as libc::c_uint {
        (*state).wsize = (1 as libc::c_uint) << (*state).wbits;
        (*state).wnext = 0 as libc::c_int as libc::c_uint;
        (*state).whave = 0 as libc::c_int as libc::c_uint;
    }
    copy = out.wrapping_sub((*strm).avail_out);
    if copy >= (*state).wsize {
        memcpy(
            (*state).window as *mut libc::c_void,
            ((*strm).next_out).offset(-((*state).wsize as isize)) as *const libc::c_void,
            (*state).wsize as libc::c_ulong,
        );
        (*state).wnext = 0 as libc::c_int as libc::c_uint;
        (*state).whave = (*state).wsize;
    } else {
        dist = ((*state).wsize).wrapping_sub((*state).wnext);
        if dist > copy {
            dist = copy;
        }
        memcpy(
            ((*state).window).offset((*state).wnext as isize) as *mut libc::c_void,
            ((*strm).next_out).offset(-(copy as isize)) as *const libc::c_void,
            dist as libc::c_ulong,
        );
        copy = copy.wrapping_sub(dist);
        if copy != 0 {
            memcpy(
                (*state).window as *mut libc::c_void,
                ((*strm).next_out).offset(-(copy as isize)) as *const libc::c_void,
                copy as libc::c_ulong,
            );
            (*state).wnext = copy;
            (*state).whave = (*state).wsize;
        } else {
            (*state).wnext = ((*state).wnext).wrapping_add(dist);
            if (*state).wnext == (*state).wsize {
                (*state).wnext = 0 as libc::c_int as libc::c_uint;
            }
            if (*state).whave < (*state).wsize {
                (*state).whave = ((*state).whave).wrapping_add(dist);
            }
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflate(
    mut strm: z_streamp,
    mut flush: libc::c_int,
) -> libc::c_int {
    let mut current_block: u64;
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut next: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut put: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut have: libc::c_uint = 0;
    let mut left: libc::c_uint = 0;
    let mut hold: libc::c_ulong = 0;
    let mut bits: libc::c_uint = 0;
    let mut in_0: libc::c_uint = 0;
    let mut out: libc::c_uint = 0;
    let mut copy: libc::c_uint = 0;
    let mut from: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut here: code = code { op: 0, bits: 0, val: 0 };
    let mut last: code = code { op: 0, bits: 0, val: 0 };
    let mut len: libc::c_uint = 0;
    let mut ret: libc::c_int = 0;
    let mut hbuf: [libc::c_uchar; 4] = [0; 4];
    static mut order: [libc::c_ushort; 19] = [
        16 as libc::c_int as libc::c_ushort,
        17 as libc::c_int as libc::c_ushort,
        18 as libc::c_int as libc::c_ushort,
        0 as libc::c_int as libc::c_ushort,
        8 as libc::c_int as libc::c_ushort,
        7 as libc::c_int as libc::c_ushort,
        9 as libc::c_int as libc::c_ushort,
        6 as libc::c_int as libc::c_ushort,
        10 as libc::c_int as libc::c_ushort,
        5 as libc::c_int as libc::c_ushort,
        11 as libc::c_int as libc::c_ushort,
        4 as libc::c_int as libc::c_ushort,
        12 as libc::c_int as libc::c_ushort,
        3 as libc::c_int as libc::c_ushort,
        13 as libc::c_int as libc::c_ushort,
        2 as libc::c_int as libc::c_ushort,
        14 as libc::c_int as libc::c_ushort,
        1 as libc::c_int as libc::c_ushort,
        15 as libc::c_int as libc::c_ushort,
    ];
    if strm.is_null() || ((*strm).state).is_null() || ((*strm).next_out).is_null()
        || ((*strm).next_in).is_null()
            && (*strm).avail_in != 0 as libc::c_int as libc::c_uint
    {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).mode as libc::c_uint == TYPE as libc::c_int as libc::c_uint {
        (*state).mode = TYPEDO;
    }
    put = (*strm).next_out;
    left = (*strm).avail_out;
    next = (*strm).next_in;
    have = (*strm).avail_in;
    hold = (*state).hold;
    bits = (*state).bits;
    in_0 = have;
    out = left;
    ret = 0 as libc::c_int;
    's_88: loop {
        match (*state).mode as libc::c_uint {
            0 => {
                if (*state).wrap == 0 as libc::c_int {
                    (*state).mode = TYPEDO;
                    continue;
                } else {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh0 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh0 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    if (*state).wrap & 2 as libc::c_int != 0
                        && hold == 0x8b1f as libc::c_int as libc::c_ulong
                    {
                        (*state)
                            .check = _glp_zlib_crc32(
                            0 as libc::c_long as uLong,
                            0 as *const Bytef,
                            0 as libc::c_int as uInt,
                        );
                        hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                        hbuf[1 as libc::c_int
                            as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                        (*state)
                            .check = _glp_zlib_crc32(
                            (*state).check,
                            hbuf.as_mut_ptr(),
                            2 as libc::c_int as uInt,
                        );
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint;
                        (*state).mode = FLAGS;
                        continue;
                    } else {
                        (*state).flags = 0 as libc::c_int;
                        if !((*state).head).is_null() {
                            (*(*state).head).done = -(1 as libc::c_int);
                        }
                        if (*state).wrap & 1 as libc::c_int == 0
                            || (((hold as libc::c_uint
                                & ((1 as libc::c_uint) << 8 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                << 8 as libc::c_int) as libc::c_ulong)
                                .wrapping_add(hold >> 8 as libc::c_int)
                                .wrapping_rem(31 as libc::c_int as libc::c_ulong) != 0
                        {
                            (*strm)
                                .msg = b"incorrect header check\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else if hold as libc::c_uint
                            & ((1 as libc::c_uint) << 4 as libc::c_int)
                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                            != 8 as libc::c_int as libc::c_uint
                        {
                            (*strm)
                                .msg = b"unknown compression method\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            hold >>= 4 as libc::c_int;
                            bits = bits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                            len = (hold as libc::c_uint
                                & ((1 as libc::c_uint) << 4 as libc::c_int)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                .wrapping_add(8 as libc::c_int as libc::c_uint);
                            if (*state).wbits == 0 as libc::c_int as libc::c_uint {
                                (*state).wbits = len;
                            } else if len > (*state).wbits {
                                (*strm)
                                    .msg = b"invalid window size\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                                (*state).mode = BAD;
                                continue;
                            }
                            (*state).dmax = (1 as libc::c_uint) << len;
                            (*state)
                                .check = _glp_zlib_adler32(
                                0 as libc::c_long as uLong,
                                0 as *const Bytef,
                                0 as libc::c_int as uInt,
                            );
                            (*strm).adler = (*state).check;
                            (*state)
                                .mode = (if hold & 0x200 as libc::c_int as libc::c_ulong
                                != 0
                            {
                                DICTID as libc::c_int
                            } else {
                                TYPE as libc::c_int
                            }) as inflate_mode;
                            hold = 0 as libc::c_int as libc::c_ulong;
                            bits = 0 as libc::c_int as libc::c_uint;
                            continue;
                        }
                    }
                }
            }
            1 => {
                while bits < 16 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh1 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh1 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                (*state).flags = hold as libc::c_int;
                if (*state).flags & 0xff as libc::c_int != 8 as libc::c_int {
                    (*strm)
                        .msg = b"unknown compression method\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else if (*state).flags & 0xe000 as libc::c_int != 0 {
                    (*strm)
                        .msg = b"unknown header flags set\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    if !((*state).head).is_null() {
                        (*(*state).head)
                            .text = (hold >> 8 as libc::c_int
                            & 1 as libc::c_int as libc::c_ulong) as libc::c_int;
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0 {
                        hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                        hbuf[1 as libc::c_int
                            as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                        (*state)
                            .check = _glp_zlib_crc32(
                            (*state).check,
                            hbuf.as_mut_ptr(),
                            2 as libc::c_int as uInt,
                        );
                    }
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = TIME;
                }
                current_block = 14648606000749551097;
            }
            2 => {
                current_block = 14648606000749551097;
            }
            3 => {
                current_block = 4691324637564808323;
            }
            4 => {
                current_block = 9972291475702099212;
            }
            5 => {
                current_block = 4968302471276406058;
            }
            6 => {
                current_block = 10561208245340211210;
            }
            7 => {
                current_block = 1449162932215994610;
            }
            8 => {
                current_block = 6087309661972766253;
            }
            9 => {
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh10 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh10 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                (*state)
                    .check = (hold >> 24 as libc::c_int
                    & 0xff as libc::c_int as libc::c_ulong)
                    .wrapping_add(
                        hold >> 8 as libc::c_int & 0xff00 as libc::c_int as libc::c_ulong,
                    )
                    .wrapping_add(
                        (hold & 0xff00 as libc::c_int as libc::c_ulong)
                            << 8 as libc::c_int,
                    )
                    .wrapping_add(
                        (hold & 0xff as libc::c_int as libc::c_ulong)
                            << 24 as libc::c_int,
                    );
                (*strm).adler = (*state).check;
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = DICT;
                current_block = 16168035904432745724;
            }
            10 => {
                current_block = 16168035904432745724;
            }
            11 => {
                current_block = 3609958504996442327;
            }
            12 => {
                current_block = 7343662559263759439;
            }
            13 => {
                hold >>= bits & 7 as libc::c_int as libc::c_uint;
                bits = bits.wrapping_sub(bits & 7 as libc::c_int as libc::c_uint);
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh12 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh12 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                if hold & 0xffff as libc::c_int as libc::c_ulong
                    != hold >> 16 as libc::c_int ^ 0xffff as libc::c_int as libc::c_ulong
                {
                    (*strm)
                        .msg = b"invalid stored block lengths\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state)
                        .length = hold as libc::c_uint
                        & 0xffff as libc::c_int as libc::c_uint;
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = COPY_;
                    if flush == 6 as libc::c_int {
                        break;
                    }
                }
                current_block = 5033545425852514390;
            }
            14 => {
                current_block = 5033545425852514390;
            }
            15 => {
                current_block = 2999491485055334776;
            }
            16 => {
                while bits < 14 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh13 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh13 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                (*state)
                    .nlen = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 5 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                    .wrapping_add(257 as libc::c_int as libc::c_uint);
                hold >>= 5 as libc::c_int;
                bits = bits.wrapping_sub(5 as libc::c_int as libc::c_uint);
                (*state)
                    .ndist = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 5 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                    .wrapping_add(1 as libc::c_int as libc::c_uint);
                hold >>= 5 as libc::c_int;
                bits = bits.wrapping_sub(5 as libc::c_int as libc::c_uint);
                (*state)
                    .ncode = (hold as libc::c_uint
                    & ((1 as libc::c_uint) << 4 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint))
                    .wrapping_add(4 as libc::c_int as libc::c_uint);
                hold >>= 4 as libc::c_int;
                bits = bits.wrapping_sub(4 as libc::c_int as libc::c_uint);
                if (*state).nlen > 286 as libc::c_int as libc::c_uint
                    || (*state).ndist > 30 as libc::c_int as libc::c_uint
                {
                    (*strm)
                        .msg = b"too many length or distance symbols\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = LENLENS;
                }
                current_block = 13572305558904641610;
            }
            17 => {
                current_block = 13572305558904641610;
            }
            18 => {
                current_block = 16280550096250383041;
            }
            19 => {
                current_block = 16152771998566204523;
            }
            20 => {
                current_block = 16818959577490016211;
            }
            21 => {
                current_block = 6589414636452177323;
            }
            22 => {
                current_block = 18161743480530436584;
            }
            23 => {
                current_block = 16762320103137014335;
            }
            24 => {
                current_block = 12341693363195858590;
            }
            25 => {
                if left == 0 as libc::c_int as libc::c_uint {
                    break;
                }
                let fresh33 = put;
                put = put.offset(1);
                *fresh33 = (*state).length as libc::c_uchar;
                left = left.wrapping_sub(1);
                left;
                (*state).mode = LEN;
                continue;
            }
            26 => {
                if (*state).wrap != 0 {
                    while bits < 32 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh34 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh34 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    out = out.wrapping_sub(left);
                    (*strm)
                        .total_out = ((*strm).total_out as libc::c_ulong)
                        .wrapping_add(out as libc::c_ulong) as uLong as uLong;
                    (*state).total = ((*state).total).wrapping_add(out as libc::c_ulong);
                    if out != 0 {
                        (*state)
                            .check = if (*state).flags != 0 {
                            _glp_zlib_crc32(
                                (*state).check,
                                put.offset(-(out as isize)),
                                out,
                            )
                        } else {
                            _glp_zlib_adler32(
                                (*state).check,
                                put.offset(-(out as isize)),
                                out,
                            )
                        };
                        (*strm).adler = (*state).check;
                    }
                    out = left;
                    if (if (*state).flags != 0 {
                        hold
                    } else {
                        (hold >> 24 as libc::c_int
                            & 0xff as libc::c_int as libc::c_ulong)
                            .wrapping_add(
                                hold >> 8 as libc::c_int
                                    & 0xff00 as libc::c_int as libc::c_ulong,
                            )
                            .wrapping_add(
                                (hold & 0xff00 as libc::c_int as libc::c_ulong)
                                    << 8 as libc::c_int,
                            )
                            .wrapping_add(
                                (hold & 0xff as libc::c_int as libc::c_ulong)
                                    << 24 as libc::c_int,
                            )
                    }) != (*state).check
                    {
                        (*strm)
                            .msg = b"incorrect data check\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint;
                    }
                }
                (*state).mode = LENGTH;
                current_block = 2334784937562835604;
            }
            27 => {
                current_block = 2334784937562835604;
            }
            28 => {
                current_block = 4046302689674688614;
            }
            29 => {
                ret = -(3 as libc::c_int);
                break;
            }
            30 => return -(4 as libc::c_int),
            31 | _ => return -(2 as libc::c_int),
        }
        match current_block {
            2334784937562835604 => {
                if (*state).wrap != 0 && (*state).flags != 0 {
                    while bits < 32 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh35 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh35 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    if hold != (*state).total & 0xffffffff as libc::c_ulong {
                        (*strm)
                            .msg = b"incorrect length check\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint;
                    }
                }
                (*state).mode = DONE;
                current_block = 4046302689674688614;
            }
            13572305558904641610 => {
                while (*state).have < (*state).ncode {
                    while bits < 3 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh14 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh14 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    let fresh15 = (*state).have;
                    (*state).have = ((*state).have).wrapping_add(1);
                    (*state)
                        .lens[order[fresh15 as usize]
                        as usize] = (hold as libc::c_uint
                        & ((1 as libc::c_uint) << 3 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_ushort;
                    hold >>= 3 as libc::c_int;
                    bits = bits.wrapping_sub(3 as libc::c_int as libc::c_uint);
                }
                while (*state).have < 19 as libc::c_int as libc::c_uint {
                    let fresh16 = (*state).have;
                    (*state).have = ((*state).have).wrapping_add(1);
                    (*state)
                        .lens[order[fresh16 as usize]
                        as usize] = 0 as libc::c_int as libc::c_ushort;
                }
                (*state).next = ((*state).codes).as_mut_ptr();
                (*state).lencode = (*state).next as *const code;
                (*state).lenbits = 7 as libc::c_int as libc::c_uint;
                ret = _glp_zlib_inflate_table(
                    CODES,
                    ((*state).lens).as_mut_ptr(),
                    19 as libc::c_int as libc::c_uint,
                    &mut (*state).next,
                    &mut (*state).lenbits,
                    ((*state).work).as_mut_ptr(),
                );
                if ret != 0 {
                    (*strm)
                        .msg = b"invalid code lengths set\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).have = 0 as libc::c_int as libc::c_uint;
                    (*state).mode = CODELENS;
                }
                current_block = 16280550096250383041;
            }
            16168035904432745724 => {
                if (*state).havedict == 0 as libc::c_int {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    return 2 as libc::c_int;
                }
                (*state)
                    .check = _glp_zlib_adler32(
                    0 as libc::c_long as uLong,
                    0 as *const Bytef,
                    0 as libc::c_int as uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = TYPE;
                current_block = 3609958504996442327;
            }
            14648606000749551097 => {
                while bits < 32 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh2 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh2 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                if !((*state).head).is_null() {
                    (*(*state).head).time = hold;
                }
                if (*state).flags & 0x200 as libc::c_int != 0 {
                    hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                    hbuf[1 as libc::c_int
                        as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                    hbuf[2 as libc::c_int
                        as usize] = (hold >> 16 as libc::c_int) as libc::c_uchar;
                    hbuf[3 as libc::c_int
                        as usize] = (hold >> 24 as libc::c_int) as libc::c_uchar;
                    (*state)
                        .check = _glp_zlib_crc32(
                        (*state).check,
                        hbuf.as_mut_ptr(),
                        4 as libc::c_int as uInt,
                    );
                }
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = OS;
                current_block = 4691324637564808323;
            }
            5033545425852514390 => {
                (*state).mode = COPY;
                current_block = 2999491485055334776;
            }
            _ => {}
        }
        match current_block {
            16280550096250383041 => {
                while (*state).have < ((*state).nlen).wrapping_add((*state).ndist) {
                    loop {
                        here = *((*state).lencode)
                            .offset(
                                (hold as libc::c_uint
                                    & ((1 as libc::c_uint) << (*state).lenbits)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                            );
                        if here.bits as libc::c_uint <= bits {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh17 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh17 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    if (here.val as libc::c_int) < 16 as libc::c_int {
                        while bits < here.bits as libc::c_uint {
                            if have == 0 as libc::c_int as libc::c_uint {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            have;
                            let fresh18 = next;
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*fresh18 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                        }
                        hold >>= here.bits as libc::c_int;
                        bits = bits.wrapping_sub(here.bits as libc::c_uint);
                        let fresh19 = (*state).have;
                        (*state).have = ((*state).have).wrapping_add(1);
                        (*state).lens[fresh19 as usize] = here.val;
                    } else {
                        if here.val as libc::c_int == 16 as libc::c_int {
                            while bits
                                < (here.bits as libc::c_int + 2 as libc::c_int)
                                    as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                have;
                                let fresh20 = next;
                                next = next.offset(1);
                                hold = hold
                                    .wrapping_add((*fresh20 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            }
                            hold >>= here.bits as libc::c_int;
                            bits = bits.wrapping_sub(here.bits as libc::c_uint);
                            if (*state).have == 0 as libc::c_int as libc::c_uint {
                                (*strm)
                                    .msg = b"invalid bit length repeat\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                                (*state).mode = BAD;
                                break;
                            } else {
                                len = (*state)
                                    .lens[((*state).have)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                                    as libc::c_uint;
                                copy = (3 as libc::c_int as libc::c_uint)
                                    .wrapping_add(
                                        hold as libc::c_uint
                                            & ((1 as libc::c_uint) << 2 as libc::c_int)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                    );
                                hold >>= 2 as libc::c_int;
                                bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint);
                            }
                        } else if here.val as libc::c_int == 17 as libc::c_int {
                            while bits
                                < (here.bits as libc::c_int + 3 as libc::c_int)
                                    as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                have;
                                let fresh21 = next;
                                next = next.offset(1);
                                hold = hold
                                    .wrapping_add((*fresh21 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            }
                            hold >>= here.bits as libc::c_int;
                            bits = bits.wrapping_sub(here.bits as libc::c_uint);
                            len = 0 as libc::c_int as libc::c_uint;
                            copy = (3 as libc::c_int as libc::c_uint)
                                .wrapping_add(
                                    hold as libc::c_uint
                                        & ((1 as libc::c_uint) << 3 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                );
                            hold >>= 3 as libc::c_int;
                            bits = bits.wrapping_sub(3 as libc::c_int as libc::c_uint);
                        } else {
                            while bits
                                < (here.bits as libc::c_int + 7 as libc::c_int)
                                    as libc::c_uint
                            {
                                if have == 0 as libc::c_int as libc::c_uint {
                                    break 's_88;
                                }
                                have = have.wrapping_sub(1);
                                have;
                                let fresh22 = next;
                                next = next.offset(1);
                                hold = hold
                                    .wrapping_add((*fresh22 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            }
                            hold >>= here.bits as libc::c_int;
                            bits = bits.wrapping_sub(here.bits as libc::c_uint);
                            len = 0 as libc::c_int as libc::c_uint;
                            copy = (11 as libc::c_int as libc::c_uint)
                                .wrapping_add(
                                    hold as libc::c_uint
                                        & ((1 as libc::c_uint) << 7 as libc::c_int)
                                            .wrapping_sub(1 as libc::c_int as libc::c_uint),
                                );
                            hold >>= 7 as libc::c_int;
                            bits = bits.wrapping_sub(7 as libc::c_int as libc::c_uint);
                        }
                        if ((*state).have).wrapping_add(copy)
                            > ((*state).nlen).wrapping_add((*state).ndist)
                        {
                            (*strm)
                                .msg = b"invalid bit length repeat\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            (*state).mode = BAD;
                            break;
                        } else {
                            loop {
                                let fresh23 = copy;
                                copy = copy.wrapping_sub(1);
                                if !(fresh23 != 0) {
                                    break;
                                }
                                let fresh24 = (*state).have;
                                (*state).have = ((*state).have).wrapping_add(1);
                                (*state).lens[fresh24 as usize] = len as libc::c_ushort;
                            }
                        }
                    }
                }
                if (*state).mode as libc::c_uint == BAD as libc::c_int as libc::c_uint {
                    continue;
                }
                if (*state).lens[256 as libc::c_int as usize] as libc::c_int
                    == 0 as libc::c_int
                {
                    (*strm)
                        .msg = b"invalid code -- missing end-of-block\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).next = ((*state).codes).as_mut_ptr();
                    (*state).lencode = (*state).next as *const code;
                    (*state).lenbits = 9 as libc::c_int as libc::c_uint;
                    ret = _glp_zlib_inflate_table(
                        LENS,
                        ((*state).lens).as_mut_ptr(),
                        (*state).nlen,
                        &mut (*state).next,
                        &mut (*state).lenbits,
                        ((*state).work).as_mut_ptr(),
                    );
                    if ret != 0 {
                        (*strm)
                            .msg = b"invalid literal/lengths set\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state).distcode = (*state).next as *const code;
                        (*state).distbits = 6 as libc::c_int as libc::c_uint;
                        ret = _glp_zlib_inflate_table(
                            DISTS,
                            ((*state).lens).as_mut_ptr().offset((*state).nlen as isize),
                            (*state).ndist,
                            &mut (*state).next,
                            &mut (*state).distbits,
                            ((*state).work).as_mut_ptr(),
                        );
                        if ret != 0 {
                            (*strm)
                                .msg = b"invalid distances set\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            (*state).mode = BAD;
                            continue;
                        } else {
                            (*state).mode = LEN_;
                            if flush == 6 as libc::c_int {
                                break;
                            }
                        }
                    }
                }
                current_block = 16152771998566204523;
            }
            2999491485055334776 => {
                copy = (*state).length;
                if copy != 0 {
                    if copy > have {
                        copy = have;
                    }
                    if copy > left {
                        copy = left;
                    }
                    if copy == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    memcpy(
                        put as *mut libc::c_void,
                        next as *const libc::c_void,
                        copy as libc::c_ulong,
                    );
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    left = left.wrapping_sub(copy);
                    put = put.offset(copy as isize);
                    (*state).length = ((*state).length).wrapping_sub(copy);
                    continue;
                } else {
                    (*state).mode = TYPE;
                    continue;
                }
            }
            4691324637564808323 => {
                while bits < 16 as libc::c_int as libc::c_uint {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh3 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh3 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                if !((*state).head).is_null() {
                    (*(*state).head)
                        .xflags = (hold & 0xff as libc::c_int as libc::c_ulong)
                        as libc::c_int;
                    (*(*state).head).os = (hold >> 8 as libc::c_int) as libc::c_int;
                }
                if (*state).flags & 0x200 as libc::c_int != 0 {
                    hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                    hbuf[1 as libc::c_int
                        as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                    (*state)
                        .check = _glp_zlib_crc32(
                        (*state).check,
                        hbuf.as_mut_ptr(),
                        2 as libc::c_int as uInt,
                    );
                }
                hold = 0 as libc::c_int as libc::c_ulong;
                bits = 0 as libc::c_int as libc::c_uint;
                (*state).mode = EXLEN;
                current_block = 9972291475702099212;
            }
            3609958504996442327 => {
                if flush == 5 as libc::c_int || flush == 6 as libc::c_int {
                    break;
                }
                current_block = 7343662559263759439;
            }
            4046302689674688614 => {
                ret = 1 as libc::c_int;
                break;
            }
            _ => {}
        }
        match current_block {
            7343662559263759439 => {
                if (*state).last != 0 {
                    hold >>= bits & 7 as libc::c_int as libc::c_uint;
                    bits = bits.wrapping_sub(bits & 7 as libc::c_int as libc::c_uint);
                    (*state).mode = CHECK;
                    continue;
                } else {
                    while bits < 3 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh11 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh11 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    (*state)
                        .last = (hold as libc::c_uint
                        & ((1 as libc::c_uint) << 1 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint))
                        as libc::c_int;
                    hold >>= 1 as libc::c_int;
                    bits = bits.wrapping_sub(1 as libc::c_int as libc::c_uint);
                    match hold as libc::c_uint
                        & ((1 as libc::c_uint) << 2 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint)
                    {
                        0 => {
                            (*state).mode = STORED;
                        }
                        1 => {
                            fixedtables(state);
                            (*state).mode = LEN_;
                            if flush == 6 as libc::c_int {
                                hold >>= 2 as libc::c_int;
                                bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint);
                                break;
                            }
                        }
                        2 => {
                            (*state).mode = TABLE;
                        }
                        3 => {
                            (*strm)
                                .msg = b"invalid block type\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char;
                            (*state).mode = BAD;
                        }
                        _ => {}
                    }
                    hold >>= 2 as libc::c_int;
                    bits = bits.wrapping_sub(2 as libc::c_int as libc::c_uint);
                    continue;
                }
            }
            9972291475702099212 => {
                if (*state).flags & 0x400 as libc::c_int != 0 {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh4 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh4 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    (*state).length = hold as libc::c_uint;
                    if !((*state).head).is_null() {
                        (*(*state).head).extra_len = hold as libc::c_uint;
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0 {
                        hbuf[0 as libc::c_int as usize] = hold as libc::c_uchar;
                        hbuf[1 as libc::c_int
                            as usize] = (hold >> 8 as libc::c_int) as libc::c_uchar;
                        (*state)
                            .check = _glp_zlib_crc32(
                            (*state).check,
                            hbuf.as_mut_ptr(),
                            2 as libc::c_int as uInt,
                        );
                    }
                    hold = 0 as libc::c_int as libc::c_ulong;
                    bits = 0 as libc::c_int as libc::c_uint;
                } else if !((*state).head).is_null() {
                    (*(*state).head).extra = 0 as *mut Bytef;
                }
                (*state).mode = EXTRA;
                current_block = 4968302471276406058;
            }
            16152771998566204523 => {
                (*state).mode = LEN;
                current_block = 16818959577490016211;
            }
            _ => {}
        }
        match current_block {
            16818959577490016211 => {
                if have >= 6 as libc::c_int as libc::c_uint
                    && left >= 258 as libc::c_int as libc::c_uint
                {
                    (*strm).next_out = put;
                    (*strm).avail_out = left;
                    (*strm).next_in = next;
                    (*strm).avail_in = have;
                    (*state).hold = hold;
                    (*state).bits = bits;
                    _glp_zlib_inflate_fast(strm, out);
                    put = (*strm).next_out;
                    left = (*strm).avail_out;
                    next = (*strm).next_in;
                    have = (*strm).avail_in;
                    hold = (*state).hold;
                    bits = (*state).bits;
                    if (*state).mode as libc::c_uint
                        == TYPE as libc::c_int as libc::c_uint
                    {
                        (*state).back = -(1 as libc::c_int);
                    }
                    continue;
                } else {
                    (*state).back = 0 as libc::c_int;
                    loop {
                        here = *((*state).lencode)
                            .offset(
                                (hold as libc::c_uint
                                    & ((1 as libc::c_uint) << (*state).lenbits)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                            );
                        if here.bits as libc::c_uint <= bits {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh25 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh25 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    if here.op as libc::c_int != 0
                        && here.op as libc::c_int & 0xf0 as libc::c_int
                            == 0 as libc::c_int
                    {
                        last = here;
                        loop {
                            here = *((*state).lencode)
                                .offset(
                                    (last.val as libc::c_uint)
                                        .wrapping_add(
                                            (hold as libc::c_uint
                                                & ((1 as libc::c_uint)
                                                    << last.bits as libc::c_int + last.op as libc::c_int)
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                                >> last.bits as libc::c_int,
                                        ) as isize,
                                );
                            if (last.bits as libc::c_int + here.bits as libc::c_int)
                                as libc::c_uint <= bits
                            {
                                break;
                            }
                            if have == 0 as libc::c_int as libc::c_uint {
                                break 's_88;
                            }
                            have = have.wrapping_sub(1);
                            have;
                            let fresh26 = next;
                            next = next.offset(1);
                            hold = hold
                                .wrapping_add((*fresh26 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                        }
                        hold >>= last.bits as libc::c_int;
                        bits = bits.wrapping_sub(last.bits as libc::c_uint);
                        (*state).back += last.bits as libc::c_int;
                    }
                    hold >>= here.bits as libc::c_int;
                    bits = bits.wrapping_sub(here.bits as libc::c_uint);
                    (*state).back += here.bits as libc::c_int;
                    (*state).length = here.val as libc::c_uint;
                    if here.op as libc::c_int == 0 as libc::c_int {
                        (*state).mode = LIT;
                        continue;
                    } else if here.op as libc::c_int & 32 as libc::c_int != 0 {
                        (*state).back = -(1 as libc::c_int);
                        (*state).mode = TYPE;
                        continue;
                    } else if here.op as libc::c_int & 64 as libc::c_int != 0 {
                        (*strm)
                            .msg = b"invalid literal/length code\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        (*state)
                            .extra = here.op as libc::c_uint
                            & 15 as libc::c_int as libc::c_uint;
                        (*state).mode = LENEXT;
                    }
                }
                current_block = 6589414636452177323;
            }
            4968302471276406058 => {
                if (*state).flags & 0x400 as libc::c_int != 0 {
                    copy = (*state).length;
                    if copy > have {
                        copy = have;
                    }
                    if copy != 0 {
                        if !((*state).head).is_null()
                            && !((*(*state).head).extra).is_null()
                        {
                            len = ((*(*state).head).extra_len)
                                .wrapping_sub((*state).length);
                            memcpy(
                                ((*(*state).head).extra).offset(len as isize)
                                    as *mut libc::c_void,
                                next as *const libc::c_void,
                                (if len.wrapping_add(copy) > (*(*state).head).extra_max {
                                    ((*(*state).head).extra_max).wrapping_sub(len)
                                } else {
                                    copy
                                }) as libc::c_ulong,
                            );
                        }
                        if (*state).flags & 0x200 as libc::c_int != 0 {
                            (*state).check = _glp_zlib_crc32((*state).check, next, copy);
                        }
                        have = have.wrapping_sub(copy);
                        next = next.offset(copy as isize);
                        (*state).length = ((*state).length).wrapping_sub(copy);
                    }
                    if (*state).length != 0 {
                        break;
                    }
                }
                (*state).length = 0 as libc::c_int as libc::c_uint;
                (*state).mode = NAME;
                current_block = 10561208245340211210;
            }
            _ => {}
        }
        match current_block {
            6589414636452177323 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh27 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh27 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    (*state)
                        .length = ((*state).length)
                        .wrapping_add(
                            hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).extra)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state)
                        .back = ((*state).back as libc::c_uint)
                        .wrapping_add((*state).extra) as libc::c_int as libc::c_int;
                }
                (*state).was = (*state).length;
                (*state).mode = DIST;
                current_block = 18161743480530436584;
            }
            10561208245340211210 => {
                if (*state).flags & 0x800 as libc::c_int != 0 {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    copy = 0 as libc::c_int as libc::c_uint;
                    loop {
                        let fresh5 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(fresh5 as isize) as libc::c_uint;
                        if !((*state).head).is_null()
                            && !((*(*state).head).name).is_null()
                            && (*state).length < (*(*state).head).name_max
                        {
                            let fresh6 = (*state).length;
                            (*state).length = ((*state).length).wrapping_add(1);
                            *((*(*state).head).name)
                                .offset(fresh6 as isize) = len as Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0 {
                        (*state).check = _glp_zlib_crc32((*state).check, next, copy);
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
                        break;
                    }
                } else if !((*state).head).is_null() {
                    (*(*state).head).name = 0 as *mut Bytef;
                }
                (*state).length = 0 as libc::c_int as libc::c_uint;
                (*state).mode = COMMENT;
                current_block = 1449162932215994610;
            }
            _ => {}
        }
        match current_block {
            18161743480530436584 => {
                loop {
                    here = *((*state).distcode)
                        .offset(
                            (hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).distbits)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint)) as isize,
                        );
                    if here.bits as libc::c_uint <= bits {
                        break;
                    }
                    if have == 0 as libc::c_int as libc::c_uint {
                        break 's_88;
                    }
                    have = have.wrapping_sub(1);
                    have;
                    let fresh28 = next;
                    next = next.offset(1);
                    hold = hold.wrapping_add((*fresh28 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                if here.op as libc::c_int & 0xf0 as libc::c_int == 0 as libc::c_int {
                    last = here;
                    loop {
                        here = *((*state).distcode)
                            .offset(
                                (last.val as libc::c_uint)
                                    .wrapping_add(
                                        (hold as libc::c_uint
                                            & ((1 as libc::c_uint)
                                                << last.bits as libc::c_int + last.op as libc::c_int)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint))
                                            >> last.bits as libc::c_int,
                                    ) as isize,
                            );
                        if (last.bits as libc::c_int + here.bits as libc::c_int)
                            as libc::c_uint <= bits
                        {
                            break;
                        }
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh29 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh29 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    hold >>= last.bits as libc::c_int;
                    bits = bits.wrapping_sub(last.bits as libc::c_uint);
                    (*state).back += last.bits as libc::c_int;
                }
                hold >>= here.bits as libc::c_int;
                bits = bits.wrapping_sub(here.bits as libc::c_uint);
                (*state).back += here.bits as libc::c_int;
                if here.op as libc::c_int & 64 as libc::c_int != 0 {
                    (*strm)
                        .msg = b"invalid distance code\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                } else {
                    (*state).offset = here.val as libc::c_uint;
                    (*state)
                        .extra = here.op as libc::c_uint
                        & 15 as libc::c_int as libc::c_uint;
                    (*state).mode = DISTEXT;
                }
                current_block = 16762320103137014335;
            }
            1449162932215994610 => {
                if (*state).flags & 0x1000 as libc::c_int != 0 {
                    if have == 0 as libc::c_int as libc::c_uint {
                        break;
                    }
                    copy = 0 as libc::c_int as libc::c_uint;
                    loop {
                        let fresh7 = copy;
                        copy = copy.wrapping_add(1);
                        len = *next.offset(fresh7 as isize) as libc::c_uint;
                        if !((*state).head).is_null()
                            && !((*(*state).head).comment).is_null()
                            && (*state).length < (*(*state).head).comm_max
                        {
                            let fresh8 = (*state).length;
                            (*state).length = ((*state).length).wrapping_add(1);
                            *((*(*state).head).comment)
                                .offset(fresh8 as isize) = len as Bytef;
                        }
                        if !(len != 0 && copy < have) {
                            break;
                        }
                    }
                    if (*state).flags & 0x200 as libc::c_int != 0 {
                        (*state).check = _glp_zlib_crc32((*state).check, next, copy);
                    }
                    have = have.wrapping_sub(copy);
                    next = next.offset(copy as isize);
                    if len != 0 {
                        break;
                    }
                } else if !((*state).head).is_null() {
                    (*(*state).head).comment = 0 as *mut Bytef;
                }
                (*state).mode = HCRC;
                current_block = 6087309661972766253;
            }
            _ => {}
        }
        match current_block {
            16762320103137014335 => {
                if (*state).extra != 0 {
                    while bits < (*state).extra {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh30 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh30 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    (*state)
                        .offset = ((*state).offset)
                        .wrapping_add(
                            hold as libc::c_uint
                                & ((1 as libc::c_uint) << (*state).extra)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                    hold >>= (*state).extra;
                    bits = bits.wrapping_sub((*state).extra);
                    (*state)
                        .back = ((*state).back as libc::c_uint)
                        .wrapping_add((*state).extra) as libc::c_int as libc::c_int;
                }
                (*state).mode = MATCH;
            }
            6087309661972766253 => {
                if (*state).flags & 0x200 as libc::c_int != 0 {
                    while bits < 16 as libc::c_int as libc::c_uint {
                        if have == 0 as libc::c_int as libc::c_uint {
                            break 's_88;
                        }
                        have = have.wrapping_sub(1);
                        have;
                        let fresh9 = next;
                        next = next.offset(1);
                        hold = hold.wrapping_add((*fresh9 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    if hold != (*state).check & 0xffff as libc::c_int as libc::c_ulong {
                        (*strm)
                            .msg = b"header crc mismatch\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        (*state).mode = BAD;
                        continue;
                    } else {
                        hold = 0 as libc::c_int as libc::c_ulong;
                        bits = 0 as libc::c_int as libc::c_uint;
                    }
                }
                if !((*state).head).is_null() {
                    (*(*state).head)
                        .hcrc = (*state).flags >> 9 as libc::c_int & 1 as libc::c_int;
                    (*(*state).head).done = 1 as libc::c_int;
                }
                (*state)
                    .check = _glp_zlib_crc32(
                    0 as libc::c_long as uLong,
                    0 as *const Bytef,
                    0 as libc::c_int as uInt,
                );
                (*strm).adler = (*state).check;
                (*state).mode = TYPE;
                continue;
            }
            _ => {}
        }
        if left == 0 as libc::c_int as libc::c_uint {
            break;
        }
        copy = out.wrapping_sub(left);
        if (*state).offset > copy {
            copy = ((*state).offset).wrapping_sub(copy);
            if copy > (*state).whave {
                if (*state).sane != 0 {
                    (*strm)
                        .msg = b"invalid distance too far back\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char;
                    (*state).mode = BAD;
                    continue;
                }
            }
            if copy > (*state).wnext {
                copy = copy.wrapping_sub((*state).wnext);
                from = ((*state).window)
                    .offset(((*state).wsize).wrapping_sub(copy) as isize);
            } else {
                from = ((*state).window)
                    .offset(((*state).wnext).wrapping_sub(copy) as isize);
            }
            if copy > (*state).length {
                copy = (*state).length;
            }
        } else {
            from = put.offset(-((*state).offset as isize));
            copy = (*state).length;
        }
        if copy > left {
            copy = left;
        }
        left = left.wrapping_sub(copy);
        (*state).length = ((*state).length).wrapping_sub(copy);
        loop {
            let fresh31 = from;
            from = from.offset(1);
            let fresh32 = put;
            put = put.offset(1);
            *fresh32 = *fresh31;
            copy = copy.wrapping_sub(1);
            if !(copy != 0) {
                break;
            }
        }
        if (*state).length == 0 as libc::c_int as libc::c_uint {
            (*state).mode = LEN;
        }
    }
    (*strm).next_out = put;
    (*strm).avail_out = left;
    (*strm).next_in = next;
    (*strm).avail_in = have;
    (*state).hold = hold;
    (*state).bits = bits;
    if (*state).wsize != 0
        || ((*state).mode as libc::c_uint) < CHECK as libc::c_int as libc::c_uint
            && out != (*strm).avail_out
    {
        if updatewindow(strm, out) != 0 {
            (*state).mode = MEM;
            return -(4 as libc::c_int);
        }
    }
    in_0 = in_0.wrapping_sub((*strm).avail_in);
    out = out.wrapping_sub((*strm).avail_out);
    (*strm)
        .total_in = ((*strm).total_in as libc::c_ulong)
        .wrapping_add(in_0 as libc::c_ulong) as uLong as uLong;
    (*strm)
        .total_out = ((*strm).total_out as libc::c_ulong)
        .wrapping_add(out as libc::c_ulong) as uLong as uLong;
    (*state).total = ((*state).total).wrapping_add(out as libc::c_ulong);
    if (*state).wrap != 0 && out != 0 {
        (*state)
            .check = if (*state).flags != 0 {
            _glp_zlib_crc32(
                (*state).check,
                ((*strm).next_out).offset(-(out as isize)),
                out,
            )
        } else {
            _glp_zlib_adler32(
                (*state).check,
                ((*strm).next_out).offset(-(out as isize)),
                out,
            )
        };
        (*strm).adler = (*state).check;
    }
    (*strm)
        .data_type = ((*state).bits)
        .wrapping_add(
            (if (*state).last != 0 { 64 as libc::c_int } else { 0 as libc::c_int })
                as libc::c_uint,
        )
        .wrapping_add(
            (if (*state).mode as libc::c_uint == TYPE as libc::c_int as libc::c_uint {
                128 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        )
        .wrapping_add(
            (if (*state).mode as libc::c_uint == LEN_ as libc::c_int as libc::c_uint
                || (*state).mode as libc::c_uint == COPY_ as libc::c_int as libc::c_uint
            {
                256 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint,
        ) as libc::c_int;
    if (in_0 == 0 as libc::c_int as libc::c_uint
        && out == 0 as libc::c_int as libc::c_uint || flush == 4 as libc::c_int)
        && ret == 0 as libc::c_int
    {
        ret = -(5 as libc::c_int);
    }
    return ret;
}
pub unsafe extern "C" fn _glp_zlib_inflateEnd(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() || ((*strm).zfree).is_none() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if !((*state).window).is_null() {
        (Some(((*strm).zfree).unwrap()))
            .unwrap()((*strm).opaque, (*state).window as voidpf);
    }
    (Some(((*strm).zfree).unwrap())).unwrap()((*strm).opaque, (*strm).state as voidpf);
    (*strm).state = 0 as *mut internal_state;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflateSetDictionary(
    mut strm: z_streamp,
    mut dictionary: *const Bytef,
    mut dictLength: uInt,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut id: libc::c_ulong = 0;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap != 0 as libc::c_int
        && (*state).mode as libc::c_uint != DICT as libc::c_int as libc::c_uint
    {
        return -(2 as libc::c_int);
    }
    if (*state).mode as libc::c_uint == DICT as libc::c_int as libc::c_uint {
        id = _glp_zlib_adler32(
            0 as libc::c_long as uLong,
            0 as *const Bytef,
            0 as libc::c_int as uInt,
        );
        id = _glp_zlib_adler32(id, dictionary, dictLength);
        if id != (*state).check {
            return -(3 as libc::c_int);
        }
    }
    if updatewindow(strm, (*strm).avail_out) != 0 {
        (*state).mode = MEM;
        return -(4 as libc::c_int);
    }
    if dictLength > (*state).wsize {
        memcpy(
            (*state).window as *mut libc::c_void,
            dictionary.offset(dictLength as isize).offset(-((*state).wsize as isize))
                as *const libc::c_void,
            (*state).wsize as libc::c_ulong,
        );
        (*state).whave = (*state).wsize;
    } else {
        memcpy(
            ((*state).window)
                .offset((*state).wsize as isize)
                .offset(-(dictLength as isize)) as *mut libc::c_void,
            dictionary as *const libc::c_void,
            dictLength as libc::c_ulong,
        );
        (*state).whave = dictLength;
    }
    (*state).havedict = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflateGetHeader(
    mut strm: z_streamp,
    mut head: gz_headerp,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if (*state).wrap & 2 as libc::c_int == 0 as libc::c_int {
        return -(2 as libc::c_int);
    }
    (*state).head = head;
    (*head).done = 0 as libc::c_int;
    return 0 as libc::c_int;
}
unsafe extern "C" fn syncsearch(
    mut have: *mut libc::c_uint,
    mut buf: *mut libc::c_uchar,
    mut len: libc::c_uint,
) -> libc::c_uint {
    let mut got: libc::c_uint = 0;
    let mut next: libc::c_uint = 0;
    got = *have;
    next = 0 as libc::c_int as libc::c_uint;
    while next < len && got < 4 as libc::c_int as libc::c_uint {
        if *buf.offset(next as isize) as libc::c_int
            == (if got < 2 as libc::c_int as libc::c_uint {
                0 as libc::c_int
            } else {
                0xff as libc::c_int
            })
        {
            got = got.wrapping_add(1);
            got;
        } else if *buf.offset(next as isize) != 0 {
            got = 0 as libc::c_int as libc::c_uint;
        } else {
            got = (4 as libc::c_int as libc::c_uint).wrapping_sub(got);
        }
        next = next.wrapping_add(1);
        next;
    }
    *have = got;
    return next;
}
pub unsafe extern "C" fn _glp_zlib_inflateSync(mut strm: z_streamp) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    let mut in_0: libc::c_ulong = 0;
    let mut out: libc::c_ulong = 0;
    let mut buf: [libc::c_uchar; 4] = [0; 4];
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    if (*strm).avail_in == 0 as libc::c_int as libc::c_uint
        && (*state).bits < 8 as libc::c_int as libc::c_uint
    {
        return -(5 as libc::c_int);
    }
    if (*state).mode as libc::c_uint != SYNC as libc::c_int as libc::c_uint {
        (*state).mode = SYNC;
        (*state).hold <<= (*state).bits & 7 as libc::c_int as libc::c_uint;
        (*state)
            .bits = ((*state).bits)
            .wrapping_sub((*state).bits & 7 as libc::c_int as libc::c_uint);
        len = 0 as libc::c_int as libc::c_uint;
        while (*state).bits >= 8 as libc::c_int as libc::c_uint {
            let fresh36 = len;
            len = len.wrapping_add(1);
            buf[fresh36 as usize] = (*state).hold as libc::c_uchar;
            (*state).hold >>= 8 as libc::c_int;
            (*state)
                .bits = ((*state).bits).wrapping_sub(8 as libc::c_int as libc::c_uint);
        }
        (*state).have = 0 as libc::c_int as libc::c_uint;
        syncsearch(&mut (*state).have, buf.as_mut_ptr(), len);
    }
    len = syncsearch(&mut (*state).have, (*strm).next_in, (*strm).avail_in);
    (*strm)
        .avail_in = ((*strm).avail_in as libc::c_uint).wrapping_sub(len) as uInt as uInt;
    (*strm).next_in = ((*strm).next_in).offset(len as isize);
    (*strm)
        .total_in = ((*strm).total_in as libc::c_ulong)
        .wrapping_add(len as libc::c_ulong) as uLong as uLong;
    if (*state).have != 4 as libc::c_int as libc::c_uint {
        return -(3 as libc::c_int);
    }
    in_0 = (*strm).total_in;
    out = (*strm).total_out;
    _glp_zlib_inflateReset(strm);
    (*strm).total_in = in_0;
    (*strm).total_out = out;
    (*state).mode = TYPE;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflateSyncPoint(mut strm: z_streamp) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    return ((*state).mode as libc::c_uint == STORED as libc::c_int as libc::c_uint
        && (*state).bits == 0 as libc::c_int as libc::c_uint) as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflateCopy(
    mut dest: z_streamp,
    mut source: z_streamp,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut copy: *mut inflate_state = 0 as *mut inflate_state;
    let mut window: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wsize: libc::c_uint = 0;
    if dest.is_null() || source.is_null() || ((*source).state).is_null()
        || ((*source).zalloc).is_none() || ((*source).zfree).is_none()
    {
        return -(2 as libc::c_int);
    }
    state = (*source).state as *mut inflate_state;
    copy = (Some(((*source).zalloc).unwrap()))
        .unwrap()(
        (*source).opaque,
        1 as libc::c_int as uInt,
        ::std::mem::size_of::<inflate_state>() as libc::c_ulong as uInt,
    ) as *mut inflate_state;
    if copy.is_null() {
        return -(4 as libc::c_int);
    }
    window = 0 as *mut libc::c_uchar;
    if !((*state).window).is_null() {
        window = (Some(((*source).zalloc).unwrap()))
            .unwrap()(
            (*source).opaque,
            (1 as libc::c_uint) << (*state).wbits,
            ::std::mem::size_of::<libc::c_uchar>() as libc::c_ulong as uInt,
        ) as *mut libc::c_uchar;
        if window.is_null() {
            (Some(((*source).zfree).unwrap()))
                .unwrap()((*source).opaque, copy as voidpf);
            return -(4 as libc::c_int);
        }
    }
    memcpy(
        dest as *mut libc::c_void,
        source as *const libc::c_void,
        ::std::mem::size_of::<z_stream>() as libc::c_ulong,
    );
    memcpy(
        copy as *mut libc::c_void,
        state as *const libc::c_void,
        ::std::mem::size_of::<inflate_state>() as libc::c_ulong,
    );
    if (*state).lencode >= ((*state).codes).as_mut_ptr() as *const code
        && (*state).lencode
            <= ((*state).codes)
                .as_mut_ptr()
                .offset((852 as libc::c_int + 592 as libc::c_int) as isize)
                .offset(-(1 as libc::c_int as isize)) as *const code
    {
        (*copy)
            .lencode = ((*copy).codes)
            .as_mut_ptr()
            .offset(
                ((*state).lencode).offset_from(((*state).codes).as_mut_ptr())
                    as libc::c_long as isize,
            );
        (*copy)
            .distcode = ((*copy).codes)
            .as_mut_ptr()
            .offset(
                ((*state).distcode).offset_from(((*state).codes).as_mut_ptr())
                    as libc::c_long as isize,
            );
    }
    (*copy)
        .next = ((*copy).codes)
        .as_mut_ptr()
        .offset(
            ((*state).next).offset_from(((*state).codes).as_mut_ptr()) as libc::c_long
                as isize,
        );
    if !window.is_null() {
        wsize = (1 as libc::c_uint) << (*state).wbits;
        memcpy(
            window as *mut libc::c_void,
            (*state).window as *const libc::c_void,
            wsize as libc::c_ulong,
        );
    }
    (*copy).window = window;
    (*dest).state = copy as *mut internal_state;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn _glp_zlib_inflateUndermine(
    mut strm: z_streamp,
    mut subvert: libc::c_int,
) -> libc::c_int {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(2 as libc::c_int);
    }
    state = (*strm).state as *mut inflate_state;
    (*state).sane = (subvert == 0) as libc::c_int;
    (*state).sane = 1 as libc::c_int;
    return -(3 as libc::c_int);
}
pub unsafe extern "C" fn _glp_zlib_inflateMark(mut strm: z_streamp) -> libc::c_long {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    if strm.is_null() || ((*strm).state).is_null() {
        return -(1 as libc::c_long) << 16 as libc::c_int;
    }
    state = (*strm).state as *mut inflate_state;
    return (((*state).back as libc::c_long) << 16 as libc::c_int)
        + (if (*state).mode as libc::c_uint == COPY as libc::c_int as libc::c_uint {
            (*state).length
        } else {
            (if (*state).mode as libc::c_uint == MATCH as libc::c_int as libc::c_uint {
                ((*state).was).wrapping_sub((*state).length)
            } else {
                0 as libc::c_int as libc::c_uint
            })
        }) as libc::c_long;
}
