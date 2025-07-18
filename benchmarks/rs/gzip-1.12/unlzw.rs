use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn rpl_fprintf(fp: *mut FILE, format: *const libc::c_char, _: ...) -> libc::c_int;
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
    static mut inbuf: [uch; 0];
    static mut outbuf: [uch; 0];
    static mut d_buf: [ush; 0];
    static mut window: [uch; 0];
    static mut prev: [ush; 0];
    static mut insize: libc::c_uint;
    static mut inptr: libc::c_uint;
    static mut bytes_in: off_t;
    static mut ifname: [libc::c_char; 0];
    static mut program_name: *mut libc::c_char;
    static mut exit_code: libc::c_int;
    static mut quiet: libc::c_int;
    static mut to_stdout: libc::c_int;
    fn fill_inbuf(eof_ok: libc::c_int) -> libc::c_int;
    fn write_buf(fd: libc::c_int, buf: voidp, cnt: libc::c_uint);
    fn read_buffer(fd: libc::c_int, buf: voidp, cnt: libc::c_uint) -> libc::c_int;
    fn gzip_error(m: *const libc::c_char);
    fn read_error();
    static mut maxbits: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type off_t = __off_t;
pub type voidp = *mut libc::c_void;
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
pub type uch = libc::c_uchar;
pub type ush = libc::c_ushort;
pub type code_int = libc::c_long;
pub type char_type = libc::c_uchar;
pub type cmp_code_int = libc::c_ulong;
static mut block_mode: libc::c_int = 0x80 as libc::c_int;
pub unsafe extern "C" fn unlzw(
    mut in_0: libc::c_int,
    mut out: libc::c_int,
) -> libc::c_int {
    let mut stackp: *mut char_type = 0 as *mut char_type;
    let mut code: code_int = 0;
    let mut finchar: libc::c_int = 0;
    let mut oldcode: code_int = 0;
    let mut incode: code_int = 0;
    let mut inbits: libc::c_long = 0;
    let mut posbits: libc::c_long = 0;
    let mut outpos: libc::c_int = 0;
    let mut bitmask: libc::c_uint = 0;
    let mut free_ent: code_int = 0;
    let mut maxcode: code_int = 0;
    let mut maxmaxcode: code_int = 0;
    let mut n_bits: libc::c_int = 0;
    let mut rsize: libc::c_int = 0;
    maxbits = if inptr < insize {
        let fresh0 = inptr;
        inptr = inptr.wrapping_add(1);
        *inbuf.as_mut_ptr().offset(fresh0 as isize) as libc::c_int
    } else {
        fill_inbuf(0 as libc::c_int)
    };
    block_mode = maxbits & 0x80 as libc::c_int;
    if maxbits & 0x60 as libc::c_int != 0 as libc::c_int {
        if quiet == 0 {
            rpl_fprintf(
                stderr,
                b"\n%s: %s: warning, unknown flags 0x%x\n\0" as *const u8
                    as *const libc::c_char,
                program_name,
                ifname.as_mut_ptr(),
                maxbits as libc::c_uint & 0x60 as libc::c_int as libc::c_uint,
            );
        }
        if exit_code == 0 as libc::c_int {
            exit_code = 2 as libc::c_int;
        }
    }
    maxbits &= 0x1f as libc::c_int;
    maxmaxcode = (1 as libc::c_long) << maxbits;
    if maxbits > 16 as libc::c_int {
        rpl_fprintf(
            stderr,
            b"\n%s: %s: compressed with %d bits, can only handle %d bits\n\0"
                as *const u8 as *const libc::c_char,
            program_name,
            ifname.as_mut_ptr(),
            maxbits,
            16 as libc::c_int,
        );
        exit_code = 1 as libc::c_int;
        return 1 as libc::c_int;
    }
    rsize = insize as libc::c_int;
    n_bits = 9 as libc::c_int;
    maxcode = ((1 as libc::c_long) << n_bits) - 1 as libc::c_int as libc::c_long;
    bitmask = (((1 as libc::c_int) << n_bits) - 1 as libc::c_int) as libc::c_uint;
    oldcode = -(1 as libc::c_int) as code_int;
    finchar = 0 as libc::c_int;
    outpos = 0 as libc::c_int;
    posbits = (inptr << 3 as libc::c_int) as libc::c_long;
    free_ent = (if block_mode != 0 {
        256 as libc::c_int + 1 as libc::c_int
    } else {
        256 as libc::c_int
    }) as code_int;
    memset(
        prev.as_mut_ptr() as voidp,
        0 as libc::c_int,
        256 as libc::c_int as libc::c_ulong,
    );
    code = 255 as libc::c_int as code_int;
    while code >= 0 as libc::c_int as libc::c_long {
        *window.as_mut_ptr().offset(code as isize) = code as char_type;
        code -= 1;
        code;
    }
    loop {
        let mut i: libc::c_int = 0;
        let mut e: libc::c_int = 0;
        let mut o: libc::c_int = 0;
        '_resetbuf: loop {
            o = (posbits >> 3 as libc::c_int) as libc::c_int;
            e = (if o as libc::c_uint <= insize {
                insize.wrapping_sub(o as libc::c_uint)
            } else {
                0 as libc::c_int as libc::c_uint
            }) as libc::c_int;
            i = 0 as libc::c_int;
            while i < e {
                *inbuf
                    .as_mut_ptr()
                    .offset(i as isize) = *inbuf.as_mut_ptr().offset((i + o) as isize);
                i += 1;
                i;
            }
            insize = e as libc::c_uint;
            posbits = 0 as libc::c_int as libc::c_long;
            if insize < 64 as libc::c_int as libc::c_uint {
                rsize = read_buffer(
                    in_0,
                    (inbuf.as_mut_ptr() as *mut libc::c_char).offset(insize as isize)
                        as voidp,
                    0x40000 as libc::c_int as libc::c_uint,
                );
                if rsize == -(1 as libc::c_int) {
                    read_error();
                }
                insize = insize.wrapping_add(rsize as libc::c_uint);
                bytes_in += rsize as off_t;
            }
            inbits = if rsize != 0 as libc::c_int {
                (insize as libc::c_long
                    - insize.wrapping_rem(n_bits as libc::c_uint) as libc::c_long)
                    << 3 as libc::c_int
            } else {
                ((insize as libc::c_long) << 3 as libc::c_int)
                    - (n_bits - 1 as libc::c_int) as libc::c_long
            };
            loop {
                if !(inbits > posbits) {
                    break '_resetbuf;
                }
                if free_ent > maxcode {
                    posbits = posbits - 1 as libc::c_int as libc::c_long
                        + ((n_bits << 3 as libc::c_int) as libc::c_long
                            - (posbits - 1 as libc::c_int as libc::c_long
                                + (n_bits << 3 as libc::c_int) as libc::c_long)
                                % (n_bits << 3 as libc::c_int) as libc::c_long);
                    n_bits += 1;
                    n_bits;
                    if n_bits == maxbits {
                        maxcode = maxmaxcode;
                    } else {
                        maxcode = ((1 as libc::c_long) << n_bits)
                            - 1 as libc::c_int as libc::c_long;
                    }
                    bitmask = (((1 as libc::c_int) << n_bits) - 1 as libc::c_int)
                        as libc::c_uint;
                    break;
                } else {
                    let mut p: *mut char_type = &mut *inbuf
                        .as_mut_ptr()
                        .offset((posbits >> 3 as libc::c_int) as isize) as *mut uch;
                    code = (*p.offset(0 as libc::c_int as isize) as libc::c_long
                        | (*p.offset(1 as libc::c_int as isize) as libc::c_long)
                            << 8 as libc::c_int
                        | (*p.offset(2 as libc::c_int as isize) as libc::c_long)
                            << 16 as libc::c_int)
                        >> (posbits & 0x7 as libc::c_int as libc::c_long)
                        & bitmask as libc::c_long;
                    posbits += n_bits as libc::c_long;
                    if oldcode == -(1 as libc::c_int) as libc::c_long {
                        if 256 as libc::c_int as libc::c_long <= code {
                            gzip_error(
                                b"corrupt input.\0" as *const u8 as *const libc::c_char,
                            );
                        }
                        oldcode = code;
                        finchar = oldcode as libc::c_int;
                        let fresh1 = outpos;
                        outpos = outpos + 1;
                        *outbuf
                            .as_mut_ptr()
                            .offset(fresh1 as isize) = finchar as char_type;
                    } else if code == 256 as libc::c_int as libc::c_long
                        && block_mode != 0
                    {
                        memset(
                            prev.as_mut_ptr() as voidp,
                            0 as libc::c_int,
                            256 as libc::c_int as libc::c_ulong,
                        );
                        free_ent = (256 as libc::c_int + 1 as libc::c_int
                            - 1 as libc::c_int) as code_int;
                        posbits = posbits - 1 as libc::c_int as libc::c_long
                            + ((n_bits << 3 as libc::c_int) as libc::c_long
                                - (posbits - 1 as libc::c_int as libc::c_long
                                    + (n_bits << 3 as libc::c_int) as libc::c_long)
                                    % (n_bits << 3 as libc::c_int) as libc::c_long);
                        n_bits = 9 as libc::c_int;
                        maxcode = ((1 as libc::c_long) << n_bits)
                            - 1 as libc::c_int as libc::c_long;
                        bitmask = (((1 as libc::c_int) << n_bits) - 1 as libc::c_int)
                            as libc::c_uint;
                        break;
                    } else {
                        incode = code;
                        stackp = &mut *d_buf
                            .as_mut_ptr()
                            .offset((0x8000 as libc::c_int - 1 as libc::c_int) as isize)
                            as *mut ush as *mut char_type;
                        if code >= free_ent {
                            if code > free_ent {
                                if outpos > 0 as libc::c_int {
                                    write_buf(
                                        out,
                                        outbuf.as_mut_ptr() as voidp,
                                        outpos as libc::c_uint,
                                    );
                                }
                                gzip_error(
                                    if to_stdout != 0 {
                                        b"corrupt input.\0" as *const u8 as *const libc::c_char
                                    } else {
                                        b"corrupt input. Use zcat to recover some data.\0"
                                            as *const u8 as *const libc::c_char
                                    },
                                );
                            }
                            stackp = stackp.offset(-1);
                            *stackp = finchar as char_type;
                            code = oldcode;
                        }
                        while code as cmp_code_int >= 256 as libc::c_int as cmp_code_int
                        {
                            stackp = stackp.offset(-1);
                            *stackp = *window.as_mut_ptr().offset(code as isize);
                            code = *prev.as_mut_ptr().offset(code as isize) as code_int;
                        }
                        finchar = *window.as_mut_ptr().offset(code as isize)
                            as libc::c_int;
                        stackp = stackp.offset(-1);
                        *stackp = finchar as char_type;
                        let mut i_0: libc::c_int = 0;
                        i_0 = (&mut *d_buf
                            .as_mut_ptr()
                            .offset((0x8000 as libc::c_int - 1 as libc::c_int) as isize)
                            as *mut ush as *mut char_type)
                            .offset_from(stackp) as libc::c_long as libc::c_int;
                        if outpos + i_0 >= 0x40000 as libc::c_int {
                            loop {
                                if i_0 > 0x40000 as libc::c_int - outpos {
                                    i_0 = 0x40000 as libc::c_int - outpos;
                                }
                                if i_0 > 0 as libc::c_int {
                                    memcpy(
                                        outbuf.as_mut_ptr().offset(outpos as isize)
                                            as *mut libc::c_void,
                                        stackp as *const libc::c_void,
                                        i_0 as libc::c_ulong,
                                    );
                                    outpos += i_0;
                                }
                                if outpos >= 0x40000 as libc::c_int {
                                    write_buf(
                                        out,
                                        outbuf.as_mut_ptr() as voidp,
                                        outpos as libc::c_uint,
                                    );
                                    outpos = 0 as libc::c_int;
                                }
                                stackp = stackp.offset(i_0 as isize);
                                i_0 = (&mut *d_buf
                                    .as_mut_ptr()
                                    .offset((0x8000 as libc::c_int - 1 as libc::c_int) as isize)
                                    as *mut ush as *mut char_type)
                                    .offset_from(stackp) as libc::c_long as libc::c_int;
                                if !(i_0 > 0 as libc::c_int) {
                                    break;
                                }
                            }
                        } else {
                            memcpy(
                                outbuf.as_mut_ptr().offset(outpos as isize)
                                    as *mut libc::c_void,
                                stackp as *const libc::c_void,
                                i_0 as libc::c_ulong,
                            );
                            outpos += i_0;
                        }
                        code = free_ent;
                        if code < maxmaxcode {
                            *prev
                                .as_mut_ptr()
                                .offset(code as isize) = oldcode as libc::c_ushort;
                            *window
                                .as_mut_ptr()
                                .offset(code as isize) = finchar as char_type;
                            free_ent = code + 1 as libc::c_int as libc::c_long;
                        }
                        oldcode = incode;
                    }
                }
            }
        }
        if !(rsize != 0 as libc::c_int) {
            break;
        }
    }
    if outpos > 0 as libc::c_int {
        write_buf(out, outbuf.as_mut_ptr() as voidp, outpos as libc::c_uint);
    }
    return 0 as libc::c_int;
}
