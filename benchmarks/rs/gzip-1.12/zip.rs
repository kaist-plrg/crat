use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn rpl_fprintf(fp: *mut FILE, format: *const libc::c_char, _: ...) -> libc::c_int;
    static mut method: libc::c_int;
    static mut outbuf: [uch; 0];
    static mut outcnt: libc::c_uint;
    static mut bytes_in: off_t;
    static mut ifd: libc::c_int;
    static mut ofd: libc::c_int;
    static mut ifname: [libc::c_char; 0];
    static mut program_name: *mut libc::c_char;
    static mut time_stamp: timespec;
    static mut ifile_size: off_t;
    static mut level: libc::c_int;
    static mut save_orig_name: libc::c_int;
    fn flush_outbuf();
    fn getcrc() -> ulg;
    fn deflate(pack_level: libc::c_int) -> off_t;
    fn gzip_base_name(fname: *mut libc::c_char) -> *mut libc::c_char;
    fn ct_init(attr: *mut ush, method_0: *mut libc::c_int);
    fn bi_init(zipfile: file_t);
    fn updcrc(s: *const uch, n: libc::c_uint) -> ulg;
    fn warning(m: *const libc::c_char);
    fn read_error();
    fn read_buffer(fd: libc::c_int, buf: voidp, cnt: libc::c_uint) -> libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type voidp = *mut libc::c_void;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type off_t = __off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type ulg = libc::c_ulong;
pub type file_t = libc::c_int;
pub const SLOW: C2RustUnnamed = 2;
pub const FAST: C2RustUnnamed = 4;
pub type C2RustUnnamed = libc::c_uint;
pub static mut header_bytes: off_t = 0;
pub unsafe extern "C" fn zip(
    mut in_0: libc::c_int,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut flags: uch = 0 as libc::c_int as uch;
    let mut attr: ush = 0 as libc::c_int as ush;
    let mut deflate_flags: ush = 0 as libc::c_int as ush;
    let mut stamp: ulg = 0;
    ifd = in_0;
    ofd = out;
    outcnt = 0 as libc::c_int as libc::c_uint;
    method = 8 as libc::c_int;
    let fresh0 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf
        .as_mut_ptr()
        .offset(
            fresh0 as isize,
        ) = (*::std::mem::transmute::<
        &[u8; 3],
        &[libc::c_char; 3],
    >(b"\x1F\x8B\0"))[0 as libc::c_int as usize] as uch;
    if outcnt == 0x40000 as libc::c_int as libc::c_uint {
        flush_outbuf();
    }
    let fresh1 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf
        .as_mut_ptr()
        .offset(
            fresh1 as isize,
        ) = (*::std::mem::transmute::<
        &[u8; 3],
        &[libc::c_char; 3],
    >(b"\x1F\x8B\0"))[1 as libc::c_int as usize] as uch;
    if outcnt == 0x40000 as libc::c_int as libc::c_uint {
        flush_outbuf();
    }
    let fresh2 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh2 as isize) = 8 as libc::c_int as uch;
    if outcnt == 0x40000 as libc::c_int as libc::c_uint {
        flush_outbuf();
    }
    if save_orig_name != 0 {
        flags = (flags as libc::c_int | 0x8 as libc::c_int) as uch;
    }
    let fresh3 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh3 as isize) = flags;
    if outcnt == 0x40000 as libc::c_int as libc::c_uint {
        flush_outbuf();
    }
    if time_stamp.tv_nsec < 0 as libc::c_int as libc::c_long {
        stamp = 0 as libc::c_int as ulg;
    } else if (0 as libc::c_int as libc::c_long) < time_stamp.tv_sec
        && time_stamp.tv_sec <= 0xffffffff as libc::c_uint as libc::c_long
    {
        stamp = time_stamp.tv_sec as ulg;
    } else {
        warning(
            b"file timestamp out of range for gzip format\0" as *const u8
                as *const libc::c_char,
        );
        stamp = 0 as libc::c_int as ulg;
    }
    if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        let fresh4 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh4 as isize,
            ) = (stamp & 0xffff as libc::c_int as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        let fresh5 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh5 as isize,
            ) = ((stamp & 0xffff as libc::c_int as libc::c_ulong) as ush as libc::c_int
            >> 8 as libc::c_int) as uch;
    } else {
        let fresh6 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh6 as isize,
            ) = (stamp & 0xffff as libc::c_int as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
        let fresh7 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh7 as isize,
            ) = ((stamp & 0xffff as libc::c_int as libc::c_ulong) as ush as libc::c_int
            >> 8 as libc::c_int) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        let fresh8 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh8 as isize,
            ) = (stamp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uch;
        let fresh9 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh9 as isize,
            ) = ((stamp >> 16 as libc::c_int) as ush as libc::c_int >> 8 as libc::c_int)
            as uch;
    } else {
        let fresh10 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh10 as isize,
            ) = (stamp >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
        let fresh11 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh11 as isize,
            ) = ((stamp >> 16 as libc::c_int) as ush as libc::c_int >> 8 as libc::c_int)
            as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    updcrc(0 as *const uch, 0 as libc::c_int as libc::c_uint);
    bi_init(out);
    ct_init(&mut attr, &mut method);
    if level == 1 as libc::c_int {
        deflate_flags = (deflate_flags as libc::c_int | FAST as libc::c_int) as ush;
    } else if level == 9 as libc::c_int {
        deflate_flags = (deflate_flags as libc::c_int | SLOW as libc::c_int) as ush;
    }
    let fresh12 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh12 as isize) = deflate_flags as uch;
    if outcnt == 0x40000 as libc::c_int as libc::c_uint {
        flush_outbuf();
    }
    let fresh13 = outcnt;
    outcnt = outcnt.wrapping_add(1);
    *outbuf.as_mut_ptr().offset(fresh13 as isize) = 0x3 as libc::c_int as uch;
    if outcnt == 0x40000 as libc::c_int as libc::c_uint {
        flush_outbuf();
    }
    if save_orig_name != 0 {
        let mut p: *mut libc::c_char = gzip_base_name(ifname.as_mut_ptr());
        loop {
            let fresh14 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *outbuf.as_mut_ptr().offset(fresh14 as isize) = *p as uch;
            if outcnt == 0x40000 as libc::c_int as libc::c_uint {
                flush_outbuf();
            }
            let fresh15 = p;
            p = p.offset(1);
            if !(*fresh15 != 0) {
                break;
            }
        }
    }
    header_bytes = outcnt as off_t;
    deflate(level);
    if ifile_size != -(1 as libc::c_long) && bytes_in != ifile_size {
        rpl_fprintf(
            stderr,
            b"%s: %s: file size changed while zipping\n\0" as *const u8
                as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
    }
    if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        let fresh16 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh16 as isize,
            ) = (getcrc() & 0xffff as libc::c_int as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        let fresh17 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh17 as isize,
            ) = ((getcrc() & 0xffff as libc::c_int as libc::c_ulong) as ush
            as libc::c_int >> 8 as libc::c_int) as uch;
    } else {
        let fresh18 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh18 as isize,
            ) = (getcrc() & 0xffff as libc::c_int as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
        let fresh19 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh19 as isize,
            ) = ((getcrc() & 0xffff as libc::c_int as libc::c_ulong) as ush
            as libc::c_int >> 8 as libc::c_int) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        let fresh20 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh20 as isize,
            ) = (getcrc() >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uch;
        let fresh21 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh21 as isize,
            ) = ((getcrc() >> 16 as libc::c_int) as ush as libc::c_int
            >> 8 as libc::c_int) as uch;
    } else {
        let fresh22 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh22 as isize,
            ) = (getcrc() >> 16 as libc::c_int & 0xff as libc::c_int as libc::c_ulong)
            as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
        let fresh23 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh23 as isize,
            ) = ((getcrc() >> 16 as libc::c_int) as ush as libc::c_int
            >> 8 as libc::c_int) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        let fresh24 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh24 as isize,
            ) = (bytes_in as ulg & 0xffff as libc::c_int as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        let fresh25 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh25 as isize,
            ) = ((bytes_in as ulg & 0xffff as libc::c_int as libc::c_ulong) as ush
            as libc::c_int >> 8 as libc::c_int) as uch;
    } else {
        let fresh26 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh26 as isize,
            ) = (bytes_in as ulg & 0xffff as libc::c_int as libc::c_ulong
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
        let fresh27 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh27 as isize,
            ) = ((bytes_in as ulg & 0xffff as libc::c_int as libc::c_ulong) as ush
            as libc::c_int >> 8 as libc::c_int) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    if outcnt < (0x40000 as libc::c_int - 2 as libc::c_int) as libc::c_uint {
        let fresh28 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh28 as isize,
            ) = (bytes_in as ulg >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        let fresh29 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh29 as isize,
            ) = ((bytes_in as ulg >> 16 as libc::c_int) as ush as libc::c_int
            >> 8 as libc::c_int) as uch;
    } else {
        let fresh30 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh30 as isize,
            ) = (bytes_in as ulg >> 16 as libc::c_int
            & 0xff as libc::c_int as libc::c_ulong) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
        let fresh31 = outcnt;
        outcnt = outcnt.wrapping_add(1);
        *outbuf
            .as_mut_ptr()
            .offset(
                fresh31 as isize,
            ) = ((bytes_in as ulg >> 16 as libc::c_int) as ush as libc::c_int
            >> 8 as libc::c_int) as uch;
        if outcnt == 0x40000 as libc::c_int as libc::c_uint {
            flush_outbuf();
        }
    }
    header_bytes += (2 as libc::c_int * 4 as libc::c_int) as libc::c_long;
    flush_outbuf();
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn file_read(
    mut buf: *mut libc::c_char,
    mut size: libc::c_uint,
) -> libc::c_int {
    let mut len: libc::c_uint = 0;
    len = read_buffer(ifd, buf as voidp, size) as libc::c_uint;
    if len == 0 as libc::c_int as libc::c_uint {
        return len as libc::c_int;
    }
    if len == -(1 as libc::c_int) as libc::c_uint {
        read_error();
    }
    updcrc(buf as *mut uch, len);
    bytes_in += len as off_t;
    return len as libc::c_int;
}
