use ::libc;
extern "C" {
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut d_buf: [ush; 0];
    static mut window: [uch; 0];
    static mut prev: [ush; 0];
    static mut insize: libc::c_uint;
    static mut inptr: libc::c_uint;
    static mut ifd: libc::c_int;
    static mut ofd: libc::c_int;
    fn write_buf(fd: libc::c_int, buf: voidp, cnt: libc::c_uint);
    fn fill_inbuf(eof_ok: libc::c_int) -> libc::c_int;
    fn gzip_error(m: *const libc::c_char);
}
pub type voidp = *mut libc::c_void;
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
static mut pt_len: [uch; 32] = [0; 32];
static mut blocksize: libc::c_uint = 0;
static mut pt_table: [ush; 256] = [0; 256];
static mut bitbuf: ush = 0;
static mut subbitbuf: libc::c_uint = 0;
static mut bitcount: libc::c_int = 0;
unsafe extern "C" fn fillbuf(mut n: libc::c_int) {
    bitbuf = ((bitbuf as libc::c_int) << n) as ush;
    while n > bitcount {
        n -= bitcount;
        bitbuf = (bitbuf as libc::c_uint | subbitbuf << n) as ush;
        subbitbuf = (if inptr < insize {
            let fresh0 = inptr;
            inptr = inptr.wrapping_add(1);
            *inbuf.as_mut_ptr().offset(fresh0 as isize) as libc::c_int
        } else {
            fill_inbuf(1 as libc::c_int)
        }) as libc::c_uint;
        if subbitbuf as libc::c_int == -(1 as libc::c_int) {
            subbitbuf = 0 as libc::c_int as libc::c_uint;
        }
        bitcount = 8 as libc::c_int;
    }
    bitcount -= n;
    bitbuf = (bitbuf as libc::c_uint | subbitbuf >> bitcount) as ush;
}
unsafe extern "C" fn getbits(mut n: libc::c_int) -> libc::c_uint {
    let mut x: libc::c_uint = 0;
    x = (bitbuf as libc::c_int
        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(n as libc::c_ulong)) as libc::c_uint;
    fillbuf(n);
    return x;
}
unsafe extern "C" fn init_getbits() {
    bitbuf = 0 as libc::c_int as ush;
    subbitbuf = 0 as libc::c_int as libc::c_uint;
    bitcount = 0 as libc::c_int;
    fillbuf(
        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int,
    );
}
unsafe extern "C" fn make_table(
    mut nchar: libc::c_int,
    mut bitlen: *mut uch,
    mut tablebits: libc::c_int,
    mut table: *mut ush,
) {
    let mut count: [ush; 17] = [0; 17];
    let mut weight: [ush; 17] = [0; 17];
    let mut start: [ush; 18] = [0; 18];
    let mut p: *mut ush = 0 as *mut ush;
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut ch: libc::c_uint = 0;
    let mut jutbits: libc::c_uint = 0;
    let mut avail: libc::c_uint = 0;
    let mut nextcode: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= 16 as libc::c_int as libc::c_uint {
        count[i as usize] = 0 as libc::c_int as ush;
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < nchar as libc::c_uint {
        count[*bitlen.offset(i as isize)
            as usize] = (count[*bitlen.offset(i as isize) as usize]).wrapping_add(1);
        count[*bitlen.offset(i as isize) as usize];
        i = i.wrapping_add(1);
        i;
    }
    start[1 as libc::c_int as usize] = 0 as libc::c_int as ush;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= 16 as libc::c_int as libc::c_uint {
        start[i.wrapping_add(1 as libc::c_int as libc::c_uint)
            as usize] = (start[i as usize] as libc::c_int
            + ((count[i as usize] as libc::c_int)
                << (16 as libc::c_int as libc::c_uint).wrapping_sub(i))) as ush;
        i = i.wrapping_add(1);
        i;
    }
    if start[17 as libc::c_int as usize] as libc::c_int & 0xffff as libc::c_int
        != 0 as libc::c_int
    {
        gzip_error(b"Bad table\n\0" as *const u8 as *const libc::c_char);
    }
    jutbits = (16 as libc::c_int - tablebits) as libc::c_uint;
    i = 1 as libc::c_int as libc::c_uint;
    while i <= tablebits as libc::c_uint {
        start[i as usize] = (start[i as usize] as libc::c_int >> jutbits) as ush;
        weight[i
            as usize] = ((1 as libc::c_int as libc::c_uint)
            << (tablebits as libc::c_uint).wrapping_sub(i)) as ush;
        i = i.wrapping_add(1);
        i;
    }
    while i <= 16 as libc::c_int as libc::c_uint {
        weight[i
            as usize] = ((1 as libc::c_int as libc::c_uint)
            << (16 as libc::c_int as libc::c_uint).wrapping_sub(i)) as ush;
        i = i.wrapping_add(1);
        i;
    }
    i = (start[(tablebits + 1 as libc::c_int) as usize] as libc::c_int >> jutbits)
        as libc::c_uint;
    if i != 0 as libc::c_int as libc::c_uint {
        k = ((1 as libc::c_int) << tablebits) as libc::c_uint;
        while i != k {
            let fresh1 = i;
            i = i.wrapping_add(1);
            *table.offset(fresh1 as isize) = 0 as libc::c_int as ush;
        }
    }
    avail = nchar as libc::c_uint;
    mask = (1 as libc::c_int as libc::c_uint) << 15 as libc::c_int - tablebits;
    ch = 0 as libc::c_int as libc::c_uint;
    while ch < nchar as libc::c_uint {
        len = *bitlen.offset(ch as isize) as libc::c_uint;
        if !(len == 0 as libc::c_int as libc::c_uint) {
            nextcode = (start[len as usize] as libc::c_int
                + weight[len as usize] as libc::c_int) as libc::c_uint;
            if len <= tablebits as libc::c_uint {
                if (1 as libc::c_int as libc::c_uint) << tablebits < nextcode {
                    gzip_error(b"Bad table\n\0" as *const u8 as *const libc::c_char);
                }
                i = start[len as usize] as libc::c_uint;
                while i < nextcode {
                    *table.offset(i as isize) = ch as ush;
                    i = i.wrapping_add(1);
                    i;
                }
            } else {
                k = start[len as usize] as libc::c_uint;
                p = &mut *table.offset((k >> jutbits) as isize) as *mut ush;
                i = len.wrapping_sub(tablebits as libc::c_uint);
                while i != 0 as libc::c_int as libc::c_uint {
                    if *p as libc::c_int == 0 as libc::c_int {
                        let ref mut fresh2 = *prev.as_mut_ptr().offset(avail as isize);
                        *fresh2 = 0 as libc::c_int as ush;
                        *prev
                            .as_mut_ptr()
                            .offset(0x8000 as libc::c_int as isize)
                            .offset(avail as isize) = *fresh2;
                        let fresh3 = avail;
                        avail = avail.wrapping_add(1);
                        *p = fresh3 as ush;
                    }
                    if k & mask != 0 {
                        p = &mut *prev
                            .as_mut_ptr()
                            .offset(0x8000 as libc::c_int as isize)
                            .offset(*p as isize) as *mut ush;
                    } else {
                        p = &mut *prev.as_mut_ptr().offset(*p as isize) as *mut ush;
                    }
                    k <<= 1 as libc::c_int;
                    i = i.wrapping_sub(1);
                    i;
                }
                *p = ch as ush;
            }
            start[len as usize] = nextcode as ush;
        }
        ch = ch.wrapping_add(1);
        ch;
    }
}
unsafe extern "C" fn read_pt_len(
    mut nn: libc::c_int,
    mut nbit: libc::c_int,
    mut i_special: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    n = getbits(nbit) as libc::c_int;
    if n == 0 as libc::c_int {
        c = getbits(nbit) as libc::c_int;
        i = 0 as libc::c_int;
        while i < nn {
            pt_len[i as usize] = 0 as libc::c_int as uch;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 256 as libc::c_int {
            pt_table[i as usize] = c as ush;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            c = bitbuf as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(3 as libc::c_int as libc::c_ulong);
            if c == 7 as libc::c_int {
                mask = (1 as libc::c_int as libc::c_uint)
                    << ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(3 as libc::c_int as libc::c_ulong);
                while mask & bitbuf as libc::c_uint != 0 {
                    mask >>= 1 as libc::c_int;
                    c += 1;
                    c;
                }
                if (16 as libc::c_int) < c {
                    gzip_error(b"Bad table\n\0" as *const u8 as *const libc::c_char);
                }
            }
            fillbuf(
                if c < 7 as libc::c_int {
                    3 as libc::c_int
                } else {
                    c - 3 as libc::c_int
                },
            );
            let fresh4 = i;
            i = i + 1;
            pt_len[fresh4 as usize] = c as uch;
            if i == i_special {
                c = getbits(2 as libc::c_int) as libc::c_int;
                loop {
                    c -= 1;
                    if !(c >= 0 as libc::c_int) {
                        break;
                    }
                    let fresh5 = i;
                    i = i + 1;
                    pt_len[fresh5 as usize] = 0 as libc::c_int as uch;
                }
            }
        }
        while i < nn {
            let fresh6 = i;
            i = i + 1;
            pt_len[fresh6 as usize] = 0 as libc::c_int as uch;
        }
        make_table(nn, pt_len.as_mut_ptr(), 8 as libc::c_int, pt_table.as_mut_ptr());
    };
}
unsafe extern "C" fn read_c_len() {
    let mut i: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut mask: libc::c_uint = 0;
    n = getbits(9 as libc::c_int) as libc::c_int;
    if n == 0 as libc::c_int {
        c = getbits(9 as libc::c_int) as libc::c_int;
        i = 0 as libc::c_int;
        while i
            < 255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
                - 3 as libc::c_int
        {
            *outbuf.as_mut_ptr().offset(i as isize) = 0 as libc::c_int as uch;
            i += 1;
            i;
        }
        i = 0 as libc::c_int;
        while i < 4096 as libc::c_int {
            *d_buf.as_mut_ptr().offset(i as isize) = c as ush;
            i += 1;
            i;
        }
    } else {
        i = 0 as libc::c_int;
        while i < n {
            c = pt_table[(bitbuf as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(8 as libc::c_int as libc::c_ulong)) as usize]
                as libc::c_int;
            if c >= 16 as libc::c_int + 3 as libc::c_int {
                mask = (1 as libc::c_int as libc::c_uint)
                    << ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                        .wrapping_mul(
                            ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                        )
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(8 as libc::c_int as libc::c_ulong);
                loop {
                    if bitbuf as libc::c_uint & mask != 0 {
                        c = *prev
                            .as_mut_ptr()
                            .offset(0x8000 as libc::c_int as isize)
                            .offset(c as isize) as libc::c_int;
                    } else {
                        c = *prev.as_mut_ptr().offset(c as isize) as libc::c_int;
                    }
                    mask >>= 1 as libc::c_int;
                    if !(c >= 16 as libc::c_int + 3 as libc::c_int) {
                        break;
                    }
                }
            }
            fillbuf(pt_len[c as usize] as libc::c_int);
            if c <= 2 as libc::c_int {
                if c == 0 as libc::c_int {
                    c = 1 as libc::c_int;
                } else if c == 1 as libc::c_int {
                    c = (getbits(4 as libc::c_int))
                        .wrapping_add(3 as libc::c_int as libc::c_uint) as libc::c_int;
                } else {
                    c = (getbits(9 as libc::c_int))
                        .wrapping_add(20 as libc::c_int as libc::c_uint) as libc::c_int;
                }
                loop {
                    c -= 1;
                    if !(c >= 0 as libc::c_int) {
                        break;
                    }
                    let fresh7 = i;
                    i = i + 1;
                    *outbuf
                        .as_mut_ptr()
                        .offset(fresh7 as isize) = 0 as libc::c_int as uch;
                }
            } else {
                let fresh8 = i;
                i = i + 1;
                *outbuf
                    .as_mut_ptr()
                    .offset(fresh8 as isize) = (c - 2 as libc::c_int) as uch;
            }
        }
        while i
            < 255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
                - 3 as libc::c_int
        {
            let fresh9 = i;
            i = i + 1;
            *outbuf.as_mut_ptr().offset(fresh9 as isize) = 0 as libc::c_int as uch;
        }
        make_table(
            255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
                - 3 as libc::c_int,
            outbuf.as_mut_ptr(),
            12 as libc::c_int,
            d_buf.as_mut_ptr(),
        );
    };
}
unsafe extern "C" fn decode_c() -> libc::c_uint {
    let mut j_0: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    if blocksize == 0 as libc::c_int as libc::c_uint {
        blocksize = getbits(16 as libc::c_int);
        if blocksize == 0 as libc::c_int as libc::c_uint {
            return (255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
                - 3 as libc::c_int) as libc::c_uint;
        }
        read_pt_len(
            16 as libc::c_int + 3 as libc::c_int,
            5 as libc::c_int,
            3 as libc::c_int,
        );
        read_c_len();
        read_pt_len(
            13 as libc::c_int + 1 as libc::c_int,
            4 as libc::c_int,
            -(1 as libc::c_int),
        );
    }
    blocksize = blocksize.wrapping_sub(1);
    blocksize;
    j_0 = *d_buf
        .as_mut_ptr()
        .offset(
            (bitbuf as libc::c_int
                >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                    .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_sub(12 as libc::c_int as libc::c_ulong)) as isize,
        ) as libc::c_uint;
    if j_0
        >= (255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
            - 3 as libc::c_int) as libc::c_uint
    {
        mask = (1 as libc::c_int as libc::c_uint)
            << ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(12 as libc::c_int as libc::c_ulong);
        loop {
            if bitbuf as libc::c_uint & mask != 0 {
                j_0 = *prev
                    .as_mut_ptr()
                    .offset(0x8000 as libc::c_int as isize)
                    .offset(j_0 as isize) as libc::c_uint;
            } else {
                j_0 = *prev.as_mut_ptr().offset(j_0 as isize) as libc::c_uint;
            }
            mask >>= 1 as libc::c_int;
            if !(j_0
                >= (255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
                    - 3 as libc::c_int) as libc::c_uint)
            {
                break;
            }
        }
    }
    fillbuf(*outbuf.as_mut_ptr().offset(j_0 as isize) as libc::c_int);
    return j_0;
}
unsafe extern "C" fn decode_p() -> libc::c_uint {
    let mut j_0: libc::c_uint = 0;
    let mut mask: libc::c_uint = 0;
    j_0 = pt_table[(bitbuf as libc::c_int
        >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_sub(8 as libc::c_int as libc::c_ulong)) as usize] as libc::c_uint;
    if j_0 >= (13 as libc::c_int + 1 as libc::c_int) as libc::c_uint {
        mask = (1 as libc::c_int as libc::c_uint)
            << ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_sub(8 as libc::c_int as libc::c_ulong);
        loop {
            if bitbuf as libc::c_uint & mask != 0 {
                j_0 = *prev
                    .as_mut_ptr()
                    .offset(0x8000 as libc::c_int as isize)
                    .offset(j_0 as isize) as libc::c_uint;
            } else {
                j_0 = *prev.as_mut_ptr().offset(j_0 as isize) as libc::c_uint;
            }
            mask >>= 1 as libc::c_int;
            if !(j_0 >= (13 as libc::c_int + 1 as libc::c_int) as libc::c_uint) {
                break;
            }
        }
    }
    fillbuf(pt_len[j_0 as usize] as libc::c_int);
    if j_0 != 0 as libc::c_int as libc::c_uint {
        j_0 = ((1 as libc::c_int as libc::c_uint)
            << j_0.wrapping_sub(1 as libc::c_int as libc::c_uint))
            .wrapping_add(
                getbits(
                    j_0.wrapping_sub(1 as libc::c_int as libc::c_uint) as libc::c_int,
                ),
            );
    }
    return j_0;
}
unsafe extern "C" fn huf_decode_start() {
    init_getbits();
    blocksize = 0 as libc::c_int as libc::c_uint;
}
static mut j: libc::c_int = 0;
static mut done: libc::c_int = 0;
unsafe extern "C" fn decode_start() {
    huf_decode_start();
    j = 0 as libc::c_int;
    done = 0 as libc::c_int;
}
unsafe extern "C" fn decode(
    mut count: libc::c_uint,
    mut buffer: *mut uch,
) -> libc::c_uint {
    static mut i: libc::c_uint = 0;
    let mut r: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    r = 0 as libc::c_int as libc::c_uint;
    loop {
        j -= 1;
        if !(j >= 0 as libc::c_int) {
            break;
        }
        *buffer.offset(r as isize) = *buffer.offset(i as isize);
        i = i.wrapping_add(1 as libc::c_int as libc::c_uint)
            & ((1 as libc::c_int as libc::c_uint) << 13 as libc::c_int)
                .wrapping_sub(1 as libc::c_int as libc::c_uint);
        r = r.wrapping_add(1);
        if r == count {
            return r;
        }
    }
    loop {
        c = decode_c();
        if c
            == (255 as libc::c_int + 256 as libc::c_int + 2 as libc::c_int
                - 3 as libc::c_int) as libc::c_uint
        {
            done = 1 as libc::c_int;
            return r;
        }
        if c <= 255 as libc::c_int as libc::c_uint {
            *buffer.offset(r as isize) = c as uch;
            r = r.wrapping_add(1);
            if r == count {
                return r;
            }
        } else {
            j = c
                .wrapping_sub(
                    (255 as libc::c_int + 1 as libc::c_int - 3 as libc::c_int)
                        as libc::c_uint,
                ) as libc::c_int;
            i = r.wrapping_sub(decode_p()).wrapping_sub(1 as libc::c_int as libc::c_uint)
                & ((1 as libc::c_int as libc::c_uint) << 13 as libc::c_int)
                    .wrapping_sub(1 as libc::c_int as libc::c_uint);
            loop {
                j -= 1;
                if !(j >= 0 as libc::c_int) {
                    break;
                }
                *buffer.offset(r as isize) = *buffer.offset(i as isize);
                i = i.wrapping_add(1 as libc::c_int as libc::c_uint)
                    & ((1 as libc::c_int as libc::c_uint) << 13 as libc::c_int)
                        .wrapping_sub(1 as libc::c_int as libc::c_uint);
                r = r.wrapping_add(1);
                if r == count {
                    return r;
                }
            }
        }
    };
}
pub unsafe extern "C" fn unlzh(
    mut in_0: libc::c_int,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut n: libc::c_uint = 0;
    ifd = in_0;
    ofd = out;
    decode_start();
    while done == 0 {
        n = decode(
            (1 as libc::c_int as libc::c_uint) << 13 as libc::c_int,
            window.as_mut_ptr(),
        );
        if n > 0 as libc::c_int as libc::c_uint {
            write_buf(out, window.as_mut_ptr() as voidp, n);
        }
    }
    return 0 as libc::c_int;
}
