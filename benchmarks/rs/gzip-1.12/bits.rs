use ::libc;
extern "C" {
    fn flush_outbuf();
    fn file_read(buf: *mut libc::c_char, size: libc::c_uint) -> libc::c_int;
    static mut outbuf: [uch; 0];
    static mut outcnt: libc::c_uint;
}
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type file_t = libc::c_int;
static mut zfile: file_t = 0;
static mut bi_buf: libc::c_ushort = 0;
static mut bi_valid: libc::c_int = 0;
pub static mut read_buf: Option::<
    unsafe extern "C" fn(*mut libc::c_char, libc::c_uint) -> libc::c_int,
> = None;
pub unsafe extern "C" fn bi_init(mut zipfile: file_t) {
    zfile = zipfile;
    bi_buf = 0 as libc::c_int as libc::c_ushort;
    bi_valid = 0 as libc::c_int;
    if zfile != -(1 as libc::c_int) {
        read_buf = Some(
            file_read
                as unsafe extern "C" fn(*mut libc::c_char, libc::c_uint) -> libc::c_int,
        );
    }
}
pub unsafe extern "C" fn send_bits(mut value: libc::c_int, mut length: libc::c_int) {
    if bi_valid
        > ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            as libc::c_int - length
    {
        bi_buf = (bi_buf as libc::c_int | value << bi_valid) as libc::c_ushort;
        if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
            let fresh0 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh0 as isize,
                ) = (bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh1 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh1 as isize,
                ) = (bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        } else {
            let fresh2 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh2 as isize,
                ) = (bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
            let fresh3 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh3 as isize,
                ) = (bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
        }
        bi_buf = (value as ush as libc::c_int
            >> ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_sub(bi_valid as libc::c_ulong)) as libc::c_ushort;
        bi_valid = (bi_valid as libc::c_ulong)
            .wrapping_add(
                (length as libc::c_ulong)
                    .wrapping_sub(
                        ((8 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
                            .wrapping_mul(
                                ::std::mem::size_of::<libc::c_char>() as libc::c_ulong,
                            ),
                    ),
            ) as libc::c_int as libc::c_int;
    } else {
        bi_buf = (bi_buf as libc::c_int | value << bi_valid) as libc::c_ushort;
        bi_valid += length;
    };
}
pub unsafe extern "C" fn bi_reverse(
    mut code: libc::c_uint,
    mut len: libc::c_int,
) -> libc::c_uint {
    let mut res: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        res |= code & 1 as libc::c_int as libc::c_uint;
        code >>= 1 as libc::c_int;
        res <<= 1 as libc::c_int;
        len -= 1;
        if !(len > 0 as libc::c_int) {
            break;
        }
    }
    return res >> 1 as libc::c_int;
}
pub unsafe extern "C" fn bi_windup() {
    if bi_valid > 8 as libc::c_int {
        if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
            let fresh4 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh4 as isize,
                ) = (bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh5 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh5 as isize,
                ) = (bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
        } else {
            let fresh6 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh6 as isize,
                ) = (bi_buf as libc::c_int & 0xff as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
            let fresh7 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh7 as isize,
                ) = (bi_buf as libc::c_int >> 8 as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
        }
    } else if bi_valid > 0 as libc::c_int {
        let fresh8 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh8 as isize) = bi_buf as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    bi_buf = 0 as libc::c_int as libc::c_ushort;
    bi_valid = 0 as libc::c_int;
}
pub unsafe extern "C" fn copy_block(
    mut buf: *mut libc::c_char,
    mut len: libc::c_uint,
    mut header: libc::c_int,
) {
    bi_windup();
    if header != 0 {
        if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
            let fresh9 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh9 as isize,
                ) = (len as ush as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh10 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh10 as isize,
                ) = (len as ush as libc::c_int >> 8 as libc::c_int) as uch;
        } else {
            let fresh11 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh11 as isize,
                ) = (len as ush as libc::c_int & 0xff as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
            let fresh12 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh12 as isize,
                ) = (len as ush as libc::c_int >> 8 as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
        }
        if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
            let fresh13 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh13 as isize,
                ) = (!len as ush as libc::c_int & 0xff as libc::c_int) as uch;
            let fresh14 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh14 as isize,
                ) = (!len as ush as libc::c_int >> 8 as libc::c_int) as uch;
        } else {
            let fresh15 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh15 as isize,
                ) = (!len as ush as libc::c_int & 0xff as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
            let fresh16 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf
                .as_mut_ptr()
                .offset(
                    fresh16 as isize,
                ) = (!len as ush as libc::c_int >> 8 as libc::c_int) as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
        }
    }
    loop {
        let fresh17 = len;
        len = len.wrapping_sub(1);
        if !(fresh17 != 0) {
            break;
        }
        let fresh18 = buf;
        buf = buf.offset(1);
        let fresh19 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf.as_mut_ptr().offset(fresh19 as isize) = *fresh18 as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    };
}
