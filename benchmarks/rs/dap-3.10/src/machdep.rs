use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn dap_putc(c: libc::c_int, fp: *mut DFILE);
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct RFILE {
    pub rfile_str: *mut libc::c_char,
    pub rfile_pos: *mut libc::c_char,
    pub rfile_end: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DFILE {
    pub dfile_name: *mut libc::c_char,
    pub dfile_disk: *mut FILE,
    pub dfile_ram: *mut RFILE,
}
pub static mut dap_double: libc::c_double = 0.;
unsafe extern "C" fn putnd(
    mut h: libc::c_uint,
    mut n: libc::c_int,
    mut dstr: *mut libc::c_char,
) -> libc::c_int {
    let mut d: libc::c_int = 0;
    d = 0 as libc::c_int;
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        let fresh0 = d;
        d = d + 1;
        *dstr
            .offset(
                fresh0 as isize,
            ) = (h >> 6 as libc::c_int * n & 0x3f as libc::c_int as libc::c_uint)
            .wrapping_add('!' as i32 as libc::c_uint) as libc::c_char;
    }
    return d;
}
pub static mut dap_dblhigh: libc::c_int = 0;
pub static mut dap_dbllow: libc::c_int = 0;
pub unsafe extern "C" fn dap_putdouble(mut dfp: *mut DFILE) {
    let mut ix: [libc::c_uint; 2] = [0; 2];
    let mut sign: libc::c_uint = 0;
    let mut e: libc::c_uint = 0;
    let mut dstr: [libc::c_char; 13] = [0; 13];
    let mut d: libc::c_int = 0;
    d = 0 as libc::c_int;
    ix[0 as libc::c_int
        as usize] = *(&mut dap_double as *mut libc::c_double as *mut libc::c_uint)
        .offset(dap_dbllow as isize);
    ix[1 as libc::c_int
        as usize] = *(&mut dap_double as *mut libc::c_double as *mut libc::c_uint)
        .offset(dap_dblhigh as isize);
    if ix[0 as libc::c_int as usize] & 0x7fffffff as libc::c_int as libc::c_uint == 0
        && ix[1 as libc::c_int as usize] == 0
    {
        let fresh1 = d;
        d = d + 1;
        dstr[fresh1 as usize] = 'A' as i32 as libc::c_char;
        e = 0 as libc::c_int as libc::c_uint;
        while e < 11 as libc::c_int as libc::c_uint {
            let fresh2 = d;
            d = d + 1;
            dstr[fresh2 as usize] = '!' as i32 as libc::c_char;
            e = e.wrapping_add(1);
            e;
        }
    } else {
        sign = ix[1 as libc::c_int as usize] >> 20 as libc::c_int
            & 0x800 as libc::c_int as libc::c_uint;
        e = ix[1 as libc::c_int as usize] >> 20 as libc::c_int
            & 0x7ff as libc::c_int as libc::c_uint;
        if e == 0 {
            let fresh3 = d;
            d = d + 1;
            dstr[fresh3 as usize] = 'A' as i32 as libc::c_char;
            e = 0 as libc::c_int as libc::c_uint;
            while e < 11 as libc::c_int as libc::c_uint {
                let fresh4 = d;
                d = d + 1;
                dstr[fresh4 as usize] = '!' as i32 as libc::c_char;
                e = e.wrapping_add(1);
                e;
            }
        } else if e == 0x7ff as libc::c_int as libc::c_uint {
            e = 0 as libc::c_int as libc::c_uint;
            while e < 12 as libc::c_int as libc::c_uint {
                let fresh5 = d;
                d = d + 1;
                dstr[fresh5 as usize] = 'a' as i32 as libc::c_char;
                e = e.wrapping_add(1);
                e;
            }
        } else {
            if sign != 0 {
                e = (0x800 as libc::c_int as libc::c_uint).wrapping_sub(e);
                ix[1 as libc::c_int
                    as usize] = !ix[1 as libc::c_int as usize]
                    & 0xfffff as libc::c_int as libc::c_uint;
                ix[0 as libc::c_int as usize] = !ix[0 as libc::c_int as usize];
            } else {
                e = e.wrapping_add(0x800 as libc::c_int as libc::c_uint);
            }
            d += putnd(e, 2 as libc::c_int, dstr.as_mut_ptr().offset(d as isize));
            d
                += putnd(
                    ix[1 as libc::c_int as usize],
                    4 as libc::c_int,
                    dstr.as_mut_ptr().offset(d as isize),
                );
            d
                += putnd(
                    ix[0 as libc::c_int as usize],
                    6 as libc::c_int,
                    dstr.as_mut_ptr().offset(d as isize),
                );
        }
    }
    dstr[d as usize] = '\0' as i32 as libc::c_char;
    d = 0 as libc::c_int;
    while dstr[d as usize] != 0 {
        dap_putc(dstr[d as usize] as libc::c_int, dfp);
        d += 1;
        d;
    }
}
unsafe extern "C" fn getnh(
    mut s: *mut libc::c_char,
    mut n: libc::c_int,
) -> libc::c_uint {
    let mut h: libc::c_uint = 0;
    let mut i: libc::c_int = 0;
    h = (*s.offset(0 as libc::c_int as isize) as libc::c_int - '!' as i32
        & 0x3f as libc::c_int) as libc::c_uint;
    i = 1 as libc::c_int;
    while i < n {
        h = h << 6 as libc::c_int
            | (*s.offset(i as isize) as libc::c_int - '!' as i32 & 0x3f as libc::c_int)
                as libc::c_uint;
        i += 1;
        i;
    }
    return h;
}
pub unsafe extern "C" fn dap_getdouble(mut code: *mut libc::c_char) {
    let mut ix: [libc::c_uint; 2] = [0; 2];
    let mut sign: libc::c_uint = 0;
    let mut e: libc::c_uint = 0;
    if strncmp(
        code as *const libc::c_char,
        b"A!!!!!!!!!!!\0" as *const u8 as *const libc::c_char,
        12 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        dap_double = 0.0f64;
        return;
    } else if strncmp(
        code as *const libc::c_char,
        b"aaaaaaaaaaaa\0" as *const u8 as *const libc::c_char,
        12 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        dap_double = 0.0f64 / 0.0f64;
        return;
    }
    sign = ('!' as i32 <= *code.offset(0 as libc::c_int as isize) as libc::c_int
        && *code.offset(0 as libc::c_int as isize) as libc::c_int <= '@' as i32)
        as libc::c_int as libc::c_uint;
    e = getnh(code, 2 as libc::c_int);
    if sign != 0 {
        e = (0x800 as libc::c_int as libc::c_uint).wrapping_sub(e)
            | 0x800 as libc::c_int as libc::c_uint;
    } else {
        e = e.wrapping_sub(0x800 as libc::c_int as libc::c_uint);
    }
    ix[1 as libc::c_int
        as usize] = getnh(code.offset(2 as libc::c_int as isize), 4 as libc::c_int);
    ix[0 as libc::c_int
        as usize] = getnh(code.offset(6 as libc::c_int as isize), 6 as libc::c_int);
    if sign != 0 {
        ix[1 as libc::c_int
            as usize] = !ix[1 as libc::c_int as usize]
            & 0xfffff as libc::c_int as libc::c_uint | e << 20 as libc::c_int;
        ix[1 as libc::c_int
            as usize] = ix[1 as libc::c_int as usize] | 0x80000000 as libc::c_uint;
        ix[0 as libc::c_int as usize] = !ix[0 as libc::c_int as usize];
    } else {
        ix[1 as libc::c_int
            as usize] = ix[1 as libc::c_int as usize] | e << 20 as libc::c_int;
    }
    *(&mut dap_double as *mut libc::c_double as *mut libc::c_uint)
        .offset(dap_dbllow as isize) = ix[0 as libc::c_int as usize];
    *(&mut dap_double as *mut libc::c_double as *mut libc::c_uint)
        .offset(dap_dblhigh as isize) = ix[1 as libc::c_int as usize];
}
pub unsafe extern "C" fn dap_putint(mut i: libc::c_int, mut dfp: *mut DFILE) {
    let mut j: libc::c_int = 0;
    let mut ndig: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    if i == 0 {
        dap_putc('0' as i32, dfp);
    }
    sign = 1 as libc::c_int;
    if i < 0 as libc::c_int {
        sign = -(1 as libc::c_int);
        i = -i;
    }
    ndig = 0 as libc::c_int;
    j = i;
    while j != 0 {
        j = j >> 6 as libc::c_int;
        ndig += 1;
        ndig;
    }
    dap_putc('0' as i32 + sign * ndig, dfp);
    if sign > 0 as libc::c_int {
        loop {
            ndig -= 1;
            if !(ndig >= 0 as libc::c_int) {
                break;
            }
            dap_putc(
                (i >> 6 as libc::c_int * ndig & 0x3f as libc::c_int) + '!' as i32,
                dfp,
            );
        }
    } else {
        loop {
            ndig -= 1;
            if !(ndig >= 0 as libc::c_int) {
                break;
            }
            dap_putc(
                (0x40 as libc::c_int - (i >> 6 as libc::c_int * ndig)
                    & 0x3f as libc::c_int) + '!' as i32,
                dfp,
            );
        }
    };
}
pub unsafe extern "C" fn dap_getint(mut code: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ndig: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    if strncmp(
        code as *const libc::c_char,
        b"0\0" as *const u8 as *const libc::c_char,
        1 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    sign = ((*code.offset(0 as libc::c_int as isize) as libc::c_int) < '0' as i32)
        as libc::c_int;
    if sign != 0 {
        ndig = '0' as i32 - *code.offset(0 as libc::c_int as isize) as libc::c_int;
    } else {
        ndig = *code.offset(0 as libc::c_int as isize) as libc::c_int - '0' as i32;
    }
    if sign != 0 {
        j = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while j <= ndig {
            i = i << 6 as libc::c_int
                | 0x40 as libc::c_int + '!' as i32
                    - *code.offset(j as isize) as libc::c_int;
            j += 1;
            j;
        }
    } else {
        j = 1 as libc::c_int;
        i = 0 as libc::c_int;
        while j <= ndig {
            i = i << 6 as libc::c_int
                | *code.offset(j as isize) as libc::c_int - '!' as i32;
            j += 1;
            j;
        }
    }
    return if sign != 0 { -i } else { i };
}
