use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
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
pub type pointer = *mut libc::c_void;
pub type boolean = libc::c_int;
pub type UUCONF_SIZE_T = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sglobal {
    pub qprocess: *mut sprocess,
    pub pblock: pointer,
    pub ierrno: libc::c_int,
    pub zfilename: *const libc::c_char,
    pub ilineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sprocess {
    pub zlocalname: *const libc::c_char,
    pub zspooldir: *const libc::c_char,
    pub zpubdir: *const libc::c_char,
    pub zlockdir: *const libc::c_char,
    pub zlogfile: *const libc::c_char,
    pub zstatsfile: *const libc::c_char,
    pub zdebugfile: *const libc::c_char,
    pub zdebug: *const libc::c_char,
    pub fstrip_login: boolean,
    pub fstrip_proto: boolean,
    pub cmaxuuxqts: libc::c_int,
    pub zrunuuxqt: *const libc::c_char,
    pub fv2: boolean,
    pub fhdb: boolean,
    pub pzdialcodefiles: *mut *mut libc::c_char,
    pub pztimetables: *mut *mut libc::c_char,
    pub zconfigfile: *mut libc::c_char,
    pub pzsysfiles: *mut *mut libc::c_char,
    pub pzportfiles: *mut *mut libc::c_char,
    pub pzdialfiles: *mut *mut libc::c_char,
    pub pzpwdfiles: *mut *mut libc::c_char,
    pub pzcallfiles: *mut *mut libc::c_char,
    pub qunknown: *mut sunknown,
    pub fread_syslocs: boolean,
    pub qsyslocs: *mut stsysloc,
    pub qvalidate: *mut svalidate,
    pub fuses_myname: boolean,
    pub zv2systems: *mut libc::c_char,
    pub zv2devices: *mut libc::c_char,
    pub zv2userfile: *mut libc::c_char,
    pub zv2cmds: *mut libc::c_char,
    pub pzhdb_systems: *mut *mut libc::c_char,
    pub pzhdb_devices: *mut *mut libc::c_char,
    pub pzhdb_dialers: *mut *mut libc::c_char,
    pub fhdb_read_permissions: boolean,
    pub qhdb_permissions: *mut shpermissions,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shpermissions {
    pub qnext: *mut shpermissions,
    pub pzlogname: *mut *mut libc::c_char,
    pub pzmachine: *mut *mut libc::c_char,
    pub frequest: libc::c_int,
    pub fsendfiles: libc::c_int,
    pub pzread: *mut *mut libc::c_char,
    pub pzwrite: *mut *mut libc::c_char,
    pub fcallback: libc::c_int,
    pub pzcommands: *mut *mut libc::c_char,
    pub pzvalidate: *mut *mut libc::c_char,
    pub zmyname: *mut libc::c_char,
    pub zpubdir: *const libc::c_char,
    pub pzalias: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct svalidate {
    pub qnext: *mut svalidate,
    pub zlogname: *const libc::c_char,
    pub pzmachines: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stsysloc {
    pub qnext: *mut stsysloc,
    pub zname: *const libc::c_char,
    pub falias: boolean,
    pub zfile: *const libc::c_char,
    pub e: *mut FILE,
    pub iloc: libc::c_long,
    pub ilineno: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sunknown {
    pub qnext: *mut sunknown,
    pub ilineno: libc::c_int,
    pub cargs: libc::c_int,
    pub pzargs: *mut *mut libc::c_char,
}
pub static mut _uuconf_errstr_rcsid: [libc::c_char; 50] = unsafe {
    *::std::mem::transmute::<
        &[u8; 50],
        &[libc::c_char; 50],
    >(b"$Id: errstr.c,v 1.6 2002/03/05 19:10:42 ian Rel $\0")
};
pub unsafe extern "C" fn uuconf_error_string(
    mut pglobal: pointer,
    mut ierr: libc::c_int,
    mut zbuf: *mut libc::c_char,
    mut cbuf: size_t,
) -> libc::c_int {
    let mut qglobal: *mut sglobal = pglobal as *mut sglobal;
    let mut zfile: *const libc::c_char = 0 as *const libc::c_char;
    let mut cfile: size_t = 0;
    let mut zlineno: *const libc::c_char = 0 as *const libc::c_char;
    let mut ablineno: [libc::c_char; 100] = [0; 100];
    let mut clineno: size_t = 0;
    let mut zmsg: *const libc::c_char = 0 as *const libc::c_char;
    let mut abmsg: [libc::c_char; 100] = [0; 100];
    let mut cmsg: size_t = 0;
    let mut zerrno: *const libc::c_char = 0 as *const libc::c_char;
    let mut cerrno: size_t = 0;
    let mut cret: size_t = 0;
    let mut ccopy: size_t = 0;
    if ierr & 0x200 as libc::c_int == 0 as libc::c_int || qglobal.is_null()
        || ((*qglobal).zfilename).is_null()
    {
        zfile = b"\0" as *const u8 as *const libc::c_char;
        cfile = 0 as libc::c_int as size_t;
    } else {
        zfile = (*qglobal).zfilename;
        cfile = (strlen(zfile)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    if cfile == 0 as libc::c_int as libc::c_ulong
        || ierr & 0x400 as libc::c_int == 0 as libc::c_int || qglobal.is_null()
        || (*qglobal).ilineno <= 0 as libc::c_int
    {
        zlineno = b"\0" as *const u8 as *const libc::c_char;
        clineno = 0 as libc::c_int as size_t;
    } else {
        zlineno = zeprint_num(
            ablineno.as_mut_ptr(),
            ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            (*qglobal).ilineno,
        );
        clineno = (strlen(zlineno)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    }
    match ierr & 0xff as libc::c_int {
        0 => {
            zmsg = b"no error\0" as *const u8 as *const libc::c_char;
        }
        1 => {
            zmsg = b"not found\0" as *const u8 as *const libc::c_char;
        }
        2 => {
            zmsg = b"fopen\0" as *const u8 as *const libc::c_char;
        }
        3 => {
            zmsg = b"fseek\0" as *const u8 as *const libc::c_char;
        }
        4 => {
            zmsg = b"malloc\0" as *const u8 as *const libc::c_char;
        }
        5 => {
            zmsg = b"syntax error\0" as *const u8 as *const libc::c_char;
        }
        _ => {
            zmsg = zeprint_num(
                abmsg.as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                ierr & 0xff as libc::c_int,
            );
            zmsg = zmsg
                .offset(
                    -((::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize),
                );
            memcpy(
                zmsg as pointer,
                b"error \0" as *const u8 as *const libc::c_char as pointer
                    as *const libc::c_void,
                (::std::mem::size_of::<[libc::c_char; 7]>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
            );
        }
    }
    cmsg = strlen(zmsg);
    if cfile > 0 as libc::c_int as libc::c_ulong {
        cmsg = cmsg.wrapping_add(1);
        cmsg;
    }
    if ierr & 0x100 as libc::c_int == 0 as libc::c_int || qglobal.is_null() {
        zerrno = b"\0" as *const u8 as *const libc::c_char;
        cerrno = 0 as libc::c_int as size_t;
    } else {
        zerrno = strerror((*qglobal).ierrno);
        cerrno = (strlen(zerrno)).wrapping_add(2 as libc::c_int as libc::c_ulong);
    }
    cret = cfile
        .wrapping_add(clineno)
        .wrapping_add(cmsg)
        .wrapping_add(cerrno)
        .wrapping_add(1 as libc::c_int as libc::c_ulong);
    if cbuf == 0 as libc::c_int as libc::c_ulong {
        return cret as libc::c_int;
    }
    cbuf = cbuf.wrapping_sub(1);
    cbuf;
    if cfile > 0 as libc::c_int as libc::c_ulong {
        ccopy = cfile.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if ccopy > cbuf {
            ccopy = cbuf;
        }
        memcpy(zbuf as pointer, zfile as pointer as *const libc::c_void, ccopy);
        zbuf = zbuf.offset(ccopy as isize);
        cbuf = (cbuf as libc::c_ulong).wrapping_sub(ccopy) as size_t as size_t;
        if cbuf > 0 as libc::c_int as libc::c_ulong {
            let fresh0 = zbuf;
            zbuf = zbuf.offset(1);
            *fresh0 = ':' as i32 as libc::c_char;
            cbuf = cbuf.wrapping_sub(1);
            cbuf;
        }
    }
    if clineno > 0 as libc::c_int as libc::c_ulong {
        ccopy = clineno.wrapping_sub(1 as libc::c_int as libc::c_ulong);
        if ccopy > cbuf {
            ccopy = cbuf;
        }
        memcpy(zbuf as pointer, zlineno as pointer as *const libc::c_void, ccopy);
        zbuf = zbuf.offset(ccopy as isize);
        cbuf = (cbuf as libc::c_ulong).wrapping_sub(ccopy) as size_t as size_t;
        if cbuf > 0 as libc::c_int as libc::c_ulong {
            let fresh1 = zbuf;
            zbuf = zbuf.offset(1);
            *fresh1 = ':' as i32 as libc::c_char;
            cbuf = cbuf.wrapping_sub(1);
            cbuf;
        }
    }
    if cbuf > 0 as libc::c_int as libc::c_ulong
        && cfile > 0 as libc::c_int as libc::c_ulong
    {
        let fresh2 = zbuf;
        zbuf = zbuf.offset(1);
        *fresh2 = ' ' as i32 as libc::c_char;
        cbuf = cbuf.wrapping_sub(1);
        cbuf;
        cmsg = cmsg.wrapping_sub(1);
        cmsg;
    }
    ccopy = cmsg;
    if ccopy > cbuf {
        ccopy = cbuf;
    }
    memcpy(zbuf as pointer, zmsg as pointer as *const libc::c_void, ccopy);
    zbuf = zbuf.offset(ccopy as isize);
    cbuf = (cbuf as libc::c_ulong).wrapping_sub(ccopy) as size_t as size_t;
    if cerrno > 0 as libc::c_int as libc::c_ulong {
        if cbuf > 0 as libc::c_int as libc::c_ulong {
            let fresh3 = zbuf;
            zbuf = zbuf.offset(1);
            *fresh3 = ':' as i32 as libc::c_char;
            cbuf = cbuf.wrapping_sub(1);
            cbuf;
        }
        if cbuf > 0 as libc::c_int as libc::c_ulong {
            let fresh4 = zbuf;
            zbuf = zbuf.offset(1);
            *fresh4 = ' ' as i32 as libc::c_char;
            cbuf = cbuf.wrapping_sub(1);
            cbuf;
        }
        ccopy = cerrno.wrapping_sub(2 as libc::c_int as libc::c_ulong);
        if ccopy > cbuf {
            ccopy = cbuf;
        }
        memcpy(zbuf as pointer, zerrno as pointer as *const libc::c_void, ccopy);
        zbuf = zbuf.offset(ccopy as isize);
        cbuf = (cbuf as libc::c_ulong).wrapping_sub(ccopy) as size_t as size_t;
    }
    *zbuf = '\0' as i32 as libc::c_char;
    return cret as libc::c_int;
}
unsafe extern "C" fn zeprint_num(
    mut ab: *mut libc::c_char,
    mut c: size_t,
    mut i: libc::c_int,
) -> *mut libc::c_char {
    let mut z: *mut libc::c_char = 0 as *mut libc::c_char;
    z = ab.offset(c as isize);
    z = z.offset(-1);
    *z = '\0' as i32 as libc::c_char;
    loop {
        z = z.offset(-1);
        *z = (i % 10 as libc::c_int + '0' as i32) as libc::c_char;
        i /= 10 as libc::c_int;
        if !(i != 0 as libc::c_int) {
            break;
        }
    }
    return z;
}
