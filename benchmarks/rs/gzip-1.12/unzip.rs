use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn rpl_fprintf(fp: *mut FILE, format: *const libc::c_char, _: ...) -> libc::c_int;
    static mut method: libc::c_int;
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut window: [uch; 0];
    static mut insize: libc::c_uint;
    static mut inptr: libc::c_uint;
    static mut outcnt: libc::c_uint;
    static mut bytes_out: off_t;
    static mut ifd: libc::c_int;
    static mut ofd: libc::c_int;
    static mut ifname: [libc::c_char; 0];
    static mut program_name: *mut libc::c_char;
    static mut exit_code: libc::c_int;
    static mut quiet: libc::c_int;
    static mut test: libc::c_int;
    static mut to_stdout: libc::c_int;
    fn abort_gzip();
    fn updcrc(s: *const uch, n: libc::c_uint) -> ulg;
    fn fill_inbuf(eof_ok: libc::c_int) -> libc::c_int;
    fn gzip_error(m: *const libc::c_char);
    fn flush_window();
    fn inflate() -> libc::c_int;
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type ulg = libc::c_ulong;
pub static mut unzip_crc: ulg = 0;
static mut decrypt: libc::c_int = 0;
static mut pkzip: libc::c_int = 0 as libc::c_int;
static mut ext_header: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn check_zipfile(mut in_0: libc::c_int) -> libc::c_int {
    let mut h: *mut uch = inbuf.as_mut_ptr().offset(inptr as isize);
    ifd = in_0;
    inptr = inptr
        .wrapping_add(
            (30 as libc::c_int
                + (*h
                    .offset(26 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as ush as libc::c_int
                    | (*h
                        .offset(26 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                        << 8 as libc::c_int)
                + (*h
                    .offset(28 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as ush as libc::c_int
                    | (*h
                        .offset(28 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                        << 8 as libc::c_int)) as libc::c_uint,
        );
    if inptr > insize
        || (*h.offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*h.offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*h.offset(2 as libc::c_int as isize).offset(0 as libc::c_int as isize)
                as ush as libc::c_int
                | (*h.offset(2 as libc::c_int as isize).offset(1 as libc::c_int as isize)
                    as ush as libc::c_int) << 8 as libc::c_int) as ulg)
                << 16 as libc::c_int != 0x4034b50 as libc::c_long as libc::c_ulong
    {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: not a valid zip file\n\0" as *const u8 as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    method = *h.offset(8 as libc::c_int as isize) as libc::c_int;
    if method != 0 as libc::c_int && method != 8 as libc::c_int {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: first entry not deflated or stored -- use unzip\n\0" as *const u8
                as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    decrypt = *h.offset(6 as libc::c_int as isize) as libc::c_int & 1 as libc::c_int;
    if decrypt != 0 as libc::c_int {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: encrypted file -- use unzip\n\0" as *const u8
                as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
        exit_code = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    ext_header = (*h.offset(6 as libc::c_int as isize) as libc::c_int & 8 as libc::c_int
        != 0 as libc::c_int) as libc::c_int;
    pkzip = 1 as libc::c_int;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn unzip(
    mut in_0: libc::c_int,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut orig_crc: ulg = 0 as libc::c_int as ulg;
    let mut orig_len: ulg = 0 as libc::c_int as ulg;
    let mut n: libc::c_int = 0;
    let mut buf: [uch; 16] = [0; 16];
    let mut err: libc::c_int = 0 as libc::c_int;
    ifd = in_0;
    ofd = out;
    updcrc(0 as *const uch, 0 as libc::c_int as libc::c_uint);
    if pkzip != 0 && ext_header == 0 {
        orig_crc = (*inbuf
            .as_mut_ptr()
            .offset(14 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*inbuf
                .as_mut_ptr()
                .offset(14 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(14 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*inbuf
                    .as_mut_ptr()
                    .offset(14 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
        orig_len = (*inbuf
            .as_mut_ptr()
            .offset(22 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*inbuf
                .as_mut_ptr()
                .offset(22 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(22 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*inbuf
                    .as_mut_ptr()
                    .offset(22 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
    }
    if method == 8 as libc::c_int {
        let mut res: libc::c_int = inflate();
        if res == 3 as libc::c_int {
            xalloc_die();
        } else if res != 0 as libc::c_int {
            gzip_error(
                b"invalid compressed data--format violated\0" as *const u8
                    as *const libc::c_char,
            );
        }
    } else if pkzip != 0 && method == 0 as libc::c_int {
        let mut n_0: ulg = (*inbuf
            .as_mut_ptr()
            .offset(22 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*inbuf
                .as_mut_ptr()
                .offset(22 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(22 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*inbuf
                    .as_mut_ptr()
                    .offset(22 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
        if n_0
            != ((*inbuf
                .as_mut_ptr()
                .offset(18 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*inbuf
                    .as_mut_ptr()
                    .offset(18 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg
                | ((*inbuf
                    .as_mut_ptr()
                    .offset(18 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as ush as libc::c_int
                    | (*inbuf
                        .as_mut_ptr()
                        .offset(18 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                        << 8 as libc::c_int) as ulg) << 16 as libc::c_int)
                .wrapping_sub(
                    (if decrypt != 0 { 12 as libc::c_int } else { 0 as libc::c_int })
                        as libc::c_ulong,
                )
        {
            rpl_fprintf(
                stderr,
                b"len %lu, siz %lu\n\0" as *const u8 as *const libc::c_char,
                n_0,
                (*inbuf
                    .as_mut_ptr()
                    .offset(18 as libc::c_int as isize)
                    .offset(0 as libc::c_int as isize) as ush as libc::c_int
                    | (*inbuf
                        .as_mut_ptr()
                        .offset(18 as libc::c_int as isize)
                        .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                        << 8 as libc::c_int) as ulg
                    | ((*inbuf
                        .as_mut_ptr()
                        .offset(18 as libc::c_int as isize)
                        .offset(2 as libc::c_int as isize)
                        .offset(0 as libc::c_int as isize) as ush as libc::c_int
                        | (*inbuf
                            .as_mut_ptr()
                            .offset(18 as libc::c_int as isize)
                            .offset(2 as libc::c_int as isize)
                            .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                            << 8 as libc::c_int) as ulg) << 16 as libc::c_int,
            );
            gzip_error(
                b"invalid compressed data--length mismatch\0" as *const u8
                    as *const libc::c_char,
            );
        }
        loop {
            let fresh0 = n_0;
            n_0 = n_0.wrapping_sub(1);
            if !(fresh0 != 0) {
                break;
            }
            let mut c: uch = (if inptr < insize {
                let fresh1 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh1 as isize) as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as uch;
            let fresh2 = outcnt;
            outcnt = outcnt.wrapping_add(1);
            *window.as_mut_ptr().offset(fresh2 as isize) = c;
            if outcnt == 0x8000 as libc::c_int as libc::c_uint {
                flush_window();
            }
        }
        flush_window();
    } else {
        gzip_error(
            b"internal error, invalid method\0" as *const u8 as *const libc::c_char,
        );
    }
    if pkzip == 0 {
        n = 0 as libc::c_int;
        while n < 8 as libc::c_int {
            buf[n
                as usize] = (if inptr < insize {
                let fresh3 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh3 as isize) as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as uch;
            n += 1;
            n;
        }
        orig_crc = (buf[0 as libc::c_int as usize] as ush as libc::c_int
            | (buf[1 as libc::c_int as usize] as ush as libc::c_int) << 8 as libc::c_int)
            as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*buf
                    .as_mut_ptr()
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
        orig_len = (*buf
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*buf
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*buf
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
    } else if ext_header != 0 {
        n = 0 as libc::c_int;
        while n < 16 as libc::c_int {
            buf[n
                as usize] = (if inptr < insize {
                let fresh4 = inptr;
                inptr = inptr.wrapping_add(1);
                *inbuf.as_mut_ptr().offset(fresh4 as isize) as libc::c_int
            } else {
                fill_inbuf(0 as libc::c_int)
            }) as uch;
            n += 1;
            n;
        }
        orig_crc = (*buf
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*buf
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*buf
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
        orig_len = (*buf
            .as_mut_ptr()
            .offset(12 as libc::c_int as isize)
            .offset(0 as libc::c_int as isize) as ush as libc::c_int
            | (*buf
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*buf
                .as_mut_ptr()
                .offset(12 as libc::c_int as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*buf
                    .as_mut_ptr()
                    .offset(12 as libc::c_int as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int;
    }
    if orig_crc != updcrc(outbuf.as_mut_ptr(), 0 as libc::c_int as libc::c_uint) {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: invalid compressed data--crc error\n\0" as *const u8
                as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
        err = 1 as libc::c_int;
    }
    if orig_len != (bytes_out & 0xffffffff as libc::c_uint as libc::c_long) as ulg {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: invalid compressed data--length error\n\0" as *const u8
                as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
        );
        err = 1 as libc::c_int;
    }
    if pkzip != 0 && inptr.wrapping_add(4 as libc::c_int as libc::c_uint) < insize
        && (*inbuf.as_mut_ptr().offset(inptr as isize).offset(0 as libc::c_int as isize)
            as ush as libc::c_int
            | (*inbuf
                .as_mut_ptr()
                .offset(inptr as isize)
                .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                << 8 as libc::c_int) as ulg
            | ((*inbuf
                .as_mut_ptr()
                .offset(inptr as isize)
                .offset(2 as libc::c_int as isize)
                .offset(0 as libc::c_int as isize) as ush as libc::c_int
                | (*inbuf
                    .as_mut_ptr()
                    .offset(inptr as isize)
                    .offset(2 as libc::c_int as isize)
                    .offset(1 as libc::c_int as isize) as ush as libc::c_int)
                    << 8 as libc::c_int) as ulg) << 16 as libc::c_int
            == 0x4034b50 as libc::c_long as libc::c_ulong
    {
        if to_stdout != 0 {
            if quiet == 0 {
                rpl_fprintf(
                    stderr,
                    b"%s: %s has more than one entry--rest ignored\n\0" as *const u8
                        as *const libc::c_char,
                    program_name,
                    ifname.as_mut_ptr(),
                );
            }
            if exit_code == 0 as libc::c_int {
                exit_code = 2 as libc::c_int;
            }
        } else {
            rpl_fprintf(
                stderr,
                b"%s: %s has more than one entry -- unchanged\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
            );
            err = 1 as libc::c_int;
        }
    }
    pkzip = 0 as libc::c_int;
    ext_header = pkzip;
    unzip_crc = orig_crc;
    if err == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    exit_code = 1 as libc::c_int;
    if test == 0 {
        abort_gzip();
    }
    return err;
}
