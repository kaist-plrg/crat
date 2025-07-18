use ::libc;
extern "C" {
    pub type internal_state;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct code {
    pub op: libc::c_uchar,
    pub bits: libc::c_uchar,
    pub val: libc::c_ushort,
}
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
pub const COPY_: inflate_mode = 14;
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
pub unsafe extern "C" fn _glp_zlib_inflate_fast(
    mut strm: z_streamp,
    mut start: libc::c_uint,
) {
    let mut state: *mut inflate_state = 0 as *mut inflate_state;
    let mut in_0: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut last: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut out: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut beg: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut wsize: libc::c_uint = 0;
    let mut whave: libc::c_uint = 0;
    let mut wnext: libc::c_uint = 0;
    let mut window: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut hold: libc::c_ulong = 0;
    let mut bits: libc::c_uint = 0;
    let mut lcode: *const code = 0 as *const code;
    let mut dcode: *const code = 0 as *const code;
    let mut lmask: libc::c_uint = 0;
    let mut dmask: libc::c_uint = 0;
    let mut here: code = code { op: 0, bits: 0, val: 0 };
    let mut op: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut dist: libc::c_uint = 0;
    let mut from: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    state = (*strm).state as *mut inflate_state;
    in_0 = ((*strm).next_in).offset(-(1 as libc::c_int as isize));
    last = in_0
        .offset(
            ((*strm).avail_in).wrapping_sub(5 as libc::c_int as libc::c_uint) as isize,
        );
    out = ((*strm).next_out).offset(-(1 as libc::c_int as isize));
    beg = out.offset(-(start.wrapping_sub((*strm).avail_out) as isize));
    end = out
        .offset(
            ((*strm).avail_out).wrapping_sub(257 as libc::c_int as libc::c_uint) as isize,
        );
    wsize = (*state).wsize;
    whave = (*state).whave;
    wnext = (*state).wnext;
    window = (*state).window;
    hold = (*state).hold;
    bits = (*state).bits;
    lcode = (*state).lencode;
    dcode = (*state).distcode;
    lmask = ((1 as libc::c_uint) << (*state).lenbits)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    dmask = ((1 as libc::c_uint) << (*state).distbits)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    let mut current_block_141: u64;
    's_94: loop {
        if bits < 15 as libc::c_int as libc::c_uint {
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
            in_0 = in_0.offset(1);
            hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
        }
        here = *lcode.offset((hold & lmask as libc::c_ulong) as isize);
        loop {
            op = here.bits as libc::c_uint;
            hold >>= op;
            bits = bits.wrapping_sub(op);
            op = here.op as libc::c_uint;
            if op == 0 as libc::c_int as libc::c_uint {
                out = out.offset(1);
                *out = here.val as libc::c_uchar;
                current_block_141 = 5689001924483802034;
                break;
            } else if op & 16 as libc::c_int as libc::c_uint != 0 {
                len = here.val as libc::c_uint;
                op &= 15 as libc::c_int as libc::c_uint;
                if op != 0 {
                    if bits < op {
                        in_0 = in_0.offset(1);
                        hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                        bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    len = len
                        .wrapping_add(
                            hold as libc::c_uint
                                & ((1 as libc::c_uint) << op)
                                    .wrapping_sub(1 as libc::c_int as libc::c_uint),
                        );
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                }
                if bits < 15 as libc::c_int as libc::c_uint {
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                    in_0 = in_0.offset(1);
                    hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                    bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                }
                here = *dcode.offset((hold & dmask as libc::c_ulong) as isize);
                current_block_141 = 6759391920981772247;
                break;
            } else if op & 64 as libc::c_int as libc::c_uint
                == 0 as libc::c_int as libc::c_uint
            {
                here = *lcode
                    .offset(
                        (here.val as libc::c_ulong)
                            .wrapping_add(
                                hold
                                    & ((1 as libc::c_uint) << op)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as libc::c_ulong,
                            ) as isize,
                    );
            } else if op & 32 as libc::c_int as libc::c_uint != 0 {
                current_block_141 = 13505557363059842426;
                break;
            } else {
                current_block_141 = 9180031981464905198;
                break;
            }
        }
        match current_block_141 {
            6759391920981772247 => {
                loop {
                    op = here.bits as libc::c_uint;
                    hold >>= op;
                    bits = bits.wrapping_sub(op);
                    op = here.op as libc::c_uint;
                    if op & 16 as libc::c_int as libc::c_uint != 0 {
                        dist = here.val as libc::c_uint;
                        op &= 15 as libc::c_int as libc::c_uint;
                        if bits < op {
                            in_0 = in_0.offset(1);
                            hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                            bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            if bits < op {
                                in_0 = in_0.offset(1);
                                hold = hold.wrapping_add((*in_0 as libc::c_ulong) << bits);
                                bits = bits.wrapping_add(8 as libc::c_int as libc::c_uint);
                            }
                        }
                        dist = dist
                            .wrapping_add(
                                hold as libc::c_uint
                                    & ((1 as libc::c_uint) << op)
                                        .wrapping_sub(1 as libc::c_int as libc::c_uint),
                            );
                        hold >>= op;
                        bits = bits.wrapping_sub(op);
                        op = out.offset_from(beg) as libc::c_long as libc::c_uint;
                        if dist > op {
                            current_block_141 = 5235537862154438448;
                            break;
                        } else {
                            current_block_141 = 6072622540298447352;
                            break;
                        }
                    } else if op & 64 as libc::c_int as libc::c_uint
                        == 0 as libc::c_int as libc::c_uint
                    {
                        here = *dcode
                            .offset(
                                (here.val as libc::c_ulong)
                                    .wrapping_add(
                                        hold
                                            & ((1 as libc::c_uint) << op)
                                                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                                                as libc::c_ulong,
                                    ) as isize,
                            );
                    } else {
                        (*strm)
                            .msg = b"invalid distance code\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char;
                        (*state).mode = BAD;
                        break 's_94;
                    }
                }
                match current_block_141 {
                    6072622540298447352 => {
                        from = out.offset(-(dist as isize));
                        loop {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            len = len.wrapping_sub(3 as libc::c_int as libc::c_uint);
                            if !(len > 2 as libc::c_int as libc::c_uint) {
                                break;
                            }
                        }
                        if len != 0 {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            if len > 1 as libc::c_int as libc::c_uint {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                            }
                        }
                    }
                    _ => {
                        op = dist.wrapping_sub(op);
                        if op > whave {
                            if (*state).sane != 0 {
                                (*strm)
                                    .msg = b"invalid distance too far back\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char;
                                (*state).mode = BAD;
                                break;
                            }
                        }
                        from = window.offset(-(1 as libc::c_int as isize));
                        if wnext == 0 as libc::c_int as libc::c_uint {
                            from = from.offset(wsize.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        } else if wnext < op {
                            from = from
                                .offset(
                                    wsize.wrapping_add(wnext).wrapping_sub(op) as isize,
                                );
                            op = op.wrapping_sub(wnext);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = window.offset(-(1 as libc::c_int as isize));
                                if wnext < len {
                                    op = wnext;
                                    len = len.wrapping_sub(op);
                                    loop {
                                        from = from.offset(1);
                                        out = out.offset(1);
                                        *out = *from;
                                        op = op.wrapping_sub(1);
                                        if !(op != 0) {
                                            break;
                                        }
                                    }
                                    from = out.offset(-(dist as isize));
                                }
                            }
                        } else {
                            from = from.offset(wnext.wrapping_sub(op) as isize);
                            if op < len {
                                len = len.wrapping_sub(op);
                                loop {
                                    from = from.offset(1);
                                    out = out.offset(1);
                                    *out = *from;
                                    op = op.wrapping_sub(1);
                                    if !(op != 0) {
                                        break;
                                    }
                                }
                                from = out.offset(-(dist as isize));
                            }
                        }
                        while len > 2 as libc::c_int as libc::c_uint {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            len = len.wrapping_sub(3 as libc::c_int as libc::c_uint);
                        }
                        if len != 0 {
                            from = from.offset(1);
                            out = out.offset(1);
                            *out = *from;
                            if len > 1 as libc::c_int as libc::c_uint {
                                from = from.offset(1);
                                out = out.offset(1);
                                *out = *from;
                            }
                        }
                    }
                }
            }
            9180031981464905198 => {
                (*strm)
                    .msg = b"invalid literal/length code\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char;
                (*state).mode = BAD;
                break;
            }
            13505557363059842426 => {
                (*state).mode = TYPE;
                break;
            }
            _ => {}
        }
        if !(in_0 < last && out < end) {
            break;
        }
    }
    len = bits >> 3 as libc::c_int;
    in_0 = in_0.offset(-(len as isize));
    bits = bits.wrapping_sub(len << 3 as libc::c_int);
    hold
        &= ((1 as libc::c_uint) << bits).wrapping_sub(1 as libc::c_int as libc::c_uint)
            as libc::c_ulong;
    (*strm).next_in = in_0.offset(1 as libc::c_int as isize);
    (*strm).next_out = out.offset(1 as libc::c_int as isize);
    (*strm)
        .avail_in = (if in_0 < last {
        5 as libc::c_int as libc::c_long + last.offset_from(in_0) as libc::c_long
    } else {
        5 as libc::c_int as libc::c_long - in_0.offset_from(last) as libc::c_long
    }) as libc::c_uint;
    (*strm)
        .avail_out = (if out < end {
        257 as libc::c_int as libc::c_long + end.offset_from(out) as libc::c_long
    } else {
        257 as libc::c_int as libc::c_long - out.offset_from(end) as libc::c_long
    }) as libc::c_uint;
    (*state).hold = hold;
    (*state).bits = bits;
}
