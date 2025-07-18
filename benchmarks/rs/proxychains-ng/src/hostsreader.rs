use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn pc_isnumericipv4(ipstring: *const libc::c_char) -> libc::c_int;
    fn inet_aton(__cp: *const libc::c_char, __inp: *mut in_addr) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
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
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hostsreader {
    pub f: *mut FILE,
    pub ip: *mut libc::c_char,
    pub name: *mut libc::c_char,
}
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union ip_type4 {
    pub octet: [libc::c_uchar; 4],
    pub as_int: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
pub unsafe extern "C" fn hostsreader_open(mut ctx: *mut hostsreader) -> libc::c_int {
    (*ctx)
        .f = fopen(
        b"/etc/hosts\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if ((*ctx).f).is_null() {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn hostsreader_close(mut ctx: *mut hostsreader) {
    fclose((*ctx).f);
}
pub unsafe extern "C" fn hostsreader_get(
    mut ctx: *mut hostsreader,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> libc::c_int {
    loop {
        if (fgets(buf, bufsize as libc::c_int, (*ctx).f)).is_null() {
            return 0 as libc::c_int;
        }
        if *buf as libc::c_int == '#' as i32 {
            continue;
        }
        let mut p: *mut libc::c_char = buf;
        let mut l: size_t = bufsize;
        (*ctx).ip = p;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0 && l != 0
        {
            p = p.offset(1);
            p;
            l = l.wrapping_sub(1);
            l;
        }
        if l == 0 || *p == 0 || p == (*ctx).ip {
            continue;
        }
        *p = 0 as libc::c_int as libc::c_char;
        p = p.offset(1);
        p;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0 && l != 0
        {
            p = p.offset(1);
            p;
            l = l.wrapping_sub(1);
            l;
        }
        if l == 0 || *p == 0 {
            continue;
        }
        (*ctx).name = p;
        while *p as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*p as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int == 0 && l != 0
        {
            p = p.offset(1);
            p;
            l = l.wrapping_sub(1);
            l;
        }
        if l == 0 || *p == 0 {
            continue;
        }
        *p = 0 as libc::c_int as libc::c_char;
        if pc_isnumericipv4((*ctx).ip) != 0 {
            return 1 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn hostsreader_get_ip_for_name(
    mut name: *const libc::c_char,
    mut buf: *mut libc::c_char,
    mut bufsize: size_t,
) -> *mut libc::c_char {
    let mut ctx: hostsreader = hostsreader {
        f: 0 as *mut FILE,
        ip: 0 as *mut libc::c_char,
        name: 0 as *mut libc::c_char,
    };
    let mut res: *mut libc::c_char = 0 as *mut libc::c_char;
    if hostsreader_open(&mut ctx) == 0 {
        return 0 as *mut libc::c_char;
    }
    while hostsreader_get(&mut ctx, buf, bufsize) != 0 {
        if !(strcmp(ctx.name, name) == 0) {
            continue;
        }
        res = ctx.ip;
        break;
    }
    hostsreader_close(&mut ctx);
    return res;
}
pub unsafe extern "C" fn hostsreader_get_numeric_ip_for_name(
    mut name: *const libc::c_char,
) -> ip_type4 {
    let mut hres: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut buf: [libc::c_char; 320] = [0; 320];
    hres = hostsreader_get_ip_for_name(
        name,
        buf.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 320]>() as libc::c_ulong,
    );
    if !hres.is_null() {
        let mut c: in_addr = in_addr { s_addr: 0 };
        inet_aton(hres, &mut c);
        let mut res: ip_type4 = ip_type4 { octet: [0; 4] };
        memcpy(
            (res.octet).as_mut_ptr() as *mut libc::c_void,
            &mut c.s_addr as *mut in_addr_t as *const libc::c_void,
            4 as libc::c_int as libc::c_ulong,
        );
        return res;
    } else {
        return ip_type4 {
            as_int: -(1 as libc::c_int) as uint32_t,
        }
    };
}
