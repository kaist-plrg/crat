use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn index(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn perror(__s: *const libc::c_char);
    fn rewind(__stream: *mut FILE);
    fn ftell(__stream: *mut FILE) -> libc::c_long;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ungetc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn dap_initpict();
    fn dap_list(
        varlist: *mut libc::c_char,
        varv: *mut libc::c_int,
        maxvars: libc::c_int,
    ) -> libc::c_int;
    fn dap_putint(i: libc::c_int, dfp: *mut DFILE);
    fn dap_putdouble(dfp: *mut DFILE);
    fn dap_getdouble(code: *mut libc::c_char);
    fn dap_getint(code: *mut libc::c_char) -> libc::c_int;
    fn skip(nlines: libc::c_int) -> libc::c_int;
    static mut dap_dblhigh: libc::c_int;
    static mut dap_dbllow: libc::c_int;
    static mut dap_double: libc::c_double;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct dataobs {
    pub do_int: *mut libc::c_int,
    pub do_il: *mut *mut libc::c_int,
    pub do_dbl: *mut libc::c_double,
    pub do_dl: *mut *mut libc::c_double,
    pub do_str: *mut *mut libc::c_char,
    pub do_sl: *mut libc::c_int,
    pub do_nam: *mut *mut libc::c_char,
    pub do_len: *mut libc::c_int,
    pub do_in: *mut libc::c_int,
    pub do_out: *mut libc::c_int,
    pub do_ivar: libc::c_int,
    pub do_ovar: libc::c_int,
    pub do_nvar: libc::c_int,
    pub do_valid: libc::c_int,
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct CharList {
    pub word: *mut libc::c_char,
    pub next: *mut CharList,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct AttributeList {
    pub word: *mut libc::c_char,
    pub size: libc::c_int,
    pub type_0: libc::c_int,
    pub next: *mut AttributeList,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
pub static mut dap_toolong: libc::c_int = 10 as libc::c_int;
pub static mut dap_maxlev: libc::c_int = 96 as libc::c_int;
pub static mut dap_intlen: libc::c_int = 20 as libc::c_int;
pub static mut dap_mabort: libc::c_int = 0 as libc::c_int;
pub static mut dap_listlen: libc::c_int = 256 as libc::c_int
    * (15 as libc::c_int + 7 as libc::c_int);
pub static mut dap_strlen: libc::c_int = 63 as libc::c_int;
pub static mut dap_maxval: libc::c_int = 32768 as libc::c_int;
pub static mut dap_maxcell: libc::c_int = 512 as libc::c_int;
pub static mut dap_maxtreat: libc::c_int = 9 as libc::c_int;
pub static mut dap_maxbars: libc::c_int = 128 as libc::c_int;
pub static mut dap_maxpts: libc::c_int = 16384 as libc::c_int;
pub static mut dap_maxchar: libc::c_int = 65536 as libc::c_int;
pub static mut dap_maxntxt: libc::c_int = 128 as libc::c_int;
pub static mut dap_maxtxt: libc::c_int = 127 as libc::c_int;
pub static mut dap_maxfont: libc::c_int = 63 as libc::c_int;
pub static mut dap_redtol: libc::c_double = 1e-9f64;
pub static mut dap_orthtol: libc::c_double = 1e-9f64;
pub static mut dap_zerotol: libc::c_double = 1e-6f64;
pub static mut dap_tol: libc::c_double = 1e-8f64;
pub static mut dap_ctol: libc::c_double = 1e-6f64;
pub static mut dap_ktol: libc::c_double = 1e-6f64;
pub static mut dap_prtol: libc::c_double = 1e-6f64;
pub static mut dap_addtozero: libc::c_double = 1e-8f64;
pub static mut dap_maxiter: libc::c_int = 500 as libc::c_int;
pub static mut dap_maxex1: libc::c_int = 20 as libc::c_int;
pub static mut dap_maxex2: libc::c_int = 20 as libc::c_int;
pub static mut dap_cattol: libc::c_double = 0.0000005f64;
pub static mut dap_maxlines: libc::c_int = 2048 as libc::c_int;
pub static mut dap_maxmem: libc::c_int = 1048576 as libc::c_int;
pub static mut dap_tmpdir: *mut libc::c_char = b"dap_tmp\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut dap_maxrows: libc::c_int = 1024 as libc::c_int;
pub static mut dap_maxcols: libc::c_int = 64 as libc::c_int;
pub static mut dap_maxclab: libc::c_int = 128 as libc::c_int;
pub static mut dap_maxrowv: libc::c_int = 8 as libc::c_int;
pub static mut dap_maxcolv: libc::c_int = 8 as libc::c_int;
pub static mut dap_lablen: libc::c_int = 63 as libc::c_int;
pub static mut dap_outreport: libc::c_int = 100000 as libc::c_int;
pub static mut dap_namelen: libc::c_int = 15 as libc::c_int;
pub static mut dap_linelen: libc::c_int = 2047 as libc::c_int;
pub static mut dap_nrfiles: libc::c_int = 128 as libc::c_int;
pub static mut dap_setdir: *mut libc::c_char = b"dap_sets\0" as *const u8
    as *const libc::c_char as *mut libc::c_char;
pub static mut dap_fabort: libc::c_int = 0 as libc::c_int;
pub static mut dap_memtrace: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut dap_rfilesize: libc::c_int = 16384 as libc::c_int;
pub static mut dap_maxvar: libc::c_int = 256 as libc::c_int;
pub static mut dap_obs: [dataobs; 3] = [dataobs {
    do_int: 0 as *const libc::c_int as *mut libc::c_int,
    do_il: 0 as *const *mut libc::c_int as *mut *mut libc::c_int,
    do_dbl: 0 as *const libc::c_double as *mut libc::c_double,
    do_dl: 0 as *const *mut libc::c_double as *mut *mut libc::c_double,
    do_str: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    do_sl: 0 as *const libc::c_int as *mut libc::c_int,
    do_nam: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    do_len: 0 as *const libc::c_int as *mut libc::c_int,
    do_in: 0 as *const libc::c_int as *mut libc::c_int,
    do_out: 0 as *const libc::c_int as *mut libc::c_int,
    do_ivar: 0,
    do_ovar: 0,
    do_nvar: 0,
    do_valid: 0,
}; 3];
pub static mut dap_prev: [dataobs; 2] = [dataobs {
    do_int: 0 as *const libc::c_int as *mut libc::c_int,
    do_il: 0 as *const *mut libc::c_int as *mut *mut libc::c_int,
    do_dbl: 0 as *const libc::c_double as *mut libc::c_double,
    do_dl: 0 as *const *mut libc::c_double as *mut *mut libc::c_double,
    do_str: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    do_sl: 0 as *const libc::c_int as *mut libc::c_int,
    do_nam: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    do_len: 0 as *const libc::c_int as *mut libc::c_int,
    do_in: 0 as *const libc::c_int as *mut libc::c_int,
    do_out: 0 as *const libc::c_int as *mut libc::c_int,
    do_ivar: 0,
    do_ovar: 0,
    do_nvar: 0,
    do_valid: 0,
}; 2];
static mut dosave: dataobs = dataobs {
    do_int: 0 as *const libc::c_int as *mut libc::c_int,
    do_il: 0 as *const *mut libc::c_int as *mut *mut libc::c_int,
    do_dbl: 0 as *const libc::c_double as *mut libc::c_double,
    do_dl: 0 as *const *mut libc::c_double as *mut *mut libc::c_double,
    do_str: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    do_sl: 0 as *const libc::c_int as *mut libc::c_int,
    do_nam: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
    do_len: 0 as *const libc::c_int as *mut libc::c_int,
    do_in: 0 as *const libc::c_int as *mut libc::c_int,
    do_out: 0 as *const libc::c_int as *mut libc::c_int,
    do_ivar: 0,
    do_ovar: 0,
    do_nvar: 0,
    do_valid: 0,
};
static mut rfile: *mut RFILE = 0 as *const RFILE as *mut RFILE;
static mut dfile: *mut DFILE = 0 as *const DFILE as *mut DFILE;
pub static mut dap_in: [*mut DFILE; 3] = [0 as *const DFILE as *mut DFILE; 3];
pub static mut dap_out: [*mut DFILE; 3] = [0 as *const DFILE as *mut DFILE; 3];
pub static mut dap_lst: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut dap_log: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut dap_err: *mut FILE = 0 as *const FILE as *mut FILE;
pub static mut dap_ono: libc::c_int = 0;
pub static mut dap_delim: libc::c_int = 0;
static mut fieldwd: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut nfields: libc::c_int = 0;
static mut lineno: [libc::c_int; 2] = [0; 2];
static mut outline: libc::c_int = 0;
static mut pageno: libc::c_int = 0;
static mut eof: [libc::c_int; 2] = [0; 2];
static mut filepos: [libc::c_long; 2] = [0; 2];
static mut intype: libc::c_int = 0;
static mut inlen: libc::c_int = 0;
static mut toolong: libc::c_int = 0;
pub static mut dap_title: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut dap_dapname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
pub static mut dap_psname: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut nmallocs: libc::c_int = 0 as libc::c_int;
static mut nfrees: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn dap_malloc(
    mut nbytes: libc::c_int,
    mut mesg: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut m: *mut libc::c_char = 0 as *mut libc::c_char;
    nmallocs += 1;
    nmallocs;
    m = malloc(nbytes as libc::c_ulong) as *mut libc::c_char;
    if m.is_null() {
        perror(dap_dapname);
        exit(1 as libc::c_int);
    }
    if !dap_memtrace.is_null() {
        fprintf(
            dap_log,
            b"malloc %x %s\n\0" as *const u8 as *const libc::c_char,
            m as libc::c_uint,
            mesg,
        );
        fflush(dap_log);
        if dap_mabort != 0 && m == dap_memtrace {
            abort();
        }
    }
    return m;
}
pub unsafe extern "C" fn dap_free(
    mut ptr: *mut libc::c_void,
    mut mesg: *mut libc::c_char,
) {
    nfrees += 1;
    nfrees;
    if !dap_memtrace.is_null() {
        fprintf(
            dap_log,
            b"free %x %s\n\0" as *const u8 as *const libc::c_char,
            ptr as libc::c_uint,
            mesg,
        );
        fflush(dap_log);
        if dap_fabort != 0 && ptr == dap_memtrace as *mut libc::c_void {
            abort();
        }
    }
    free(ptr);
}
unsafe extern "C" fn initdo(mut dato: *mut dataobs) {
    let mut d: libc::c_int = 0;
    (*dato)
        .do_int = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_int\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut libc::c_int;
    (*dato)
        .do_il = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_il\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_int;
    (*dato)
        .do_dbl = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_dbl\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut libc::c_double;
    (*dato)
        .do_dl = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_dl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_double;
    (*dato)
        .do_str = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_str\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    (*dato)
        .do_sl = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_sl\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    (*dato)
        .do_nam = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_nam\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    (*dato)
        .do_len = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_len\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut libc::c_int;
    (*dato)
        .do_in = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_in\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    (*dato)
        .do_out = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"initdo: dato->do_out\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
    ) as *mut libc::c_int;
    d = 0 as libc::c_int;
    while d < dap_maxvar {
        let ref mut fresh0 = *((*dato).do_str).offset(d as isize);
        *fresh0 = 0 as *mut libc::c_char;
        let ref mut fresh1 = *((*dato).do_nam).offset(d as isize);
        *fresh1 = 0 as *mut libc::c_char;
        d += 1;
        d;
    }
}
static mut testd: libc::c_double = 0.;
static mut ptesti: *mut libc::c_uint = 0 as *const libc::c_uint as *mut libc::c_uint;
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut lstname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut logname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut errname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    dap_dapname = *argv.offset(0 as libc::c_int as isize);
    len = strlen(*argv.offset(0 as libc::c_int as isize)) as libc::c_int;
    lstname = malloc((len + 5 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    strcpy(lstname, *argv.offset(0 as libc::c_int as isize));
    if len >= 4 as libc::c_int
        && strcmp(
            lstname.offset(len as isize).offset(-(4 as libc::c_int as isize)),
            b".dap\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        strcpy(
            lstname.offset(len as isize).offset(-(3 as libc::c_int as isize)),
            b"lst\0" as *const u8 as *const libc::c_char,
        );
    } else {
        strcat(lstname, b".lst\0" as *const u8 as *const libc::c_char);
        len += 4 as libc::c_int;
    }
    dap_lst = fopen(lstname, b"a\0" as *const u8 as *const libc::c_char);
    if dap_lst.is_null() {
        perror(dap_dapname);
        exit(1 as libc::c_int);
    }
    logname = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    strcpy(logname, lstname);
    strcpy(
        logname.offset(len as isize).offset(-(3 as libc::c_int as isize)),
        b"log\0" as *const u8 as *const libc::c_char,
    );
    dap_log = fopen(logname, b"w\0" as *const u8 as *const libc::c_char);
    if dap_log.is_null() {
        perror(dap_dapname);
        exit(1 as libc::c_int);
    }
    errname = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut libc::c_char;
    strcpy(errname, lstname);
    strcpy(
        errname.offset(len as isize).offset(-(3 as libc::c_int as isize)),
        b"err\0" as *const u8 as *const libc::c_char,
    );
    dap_err = fopen(errname, b"w\0" as *const u8 as *const libc::c_char);
    if dap_err.is_null() {
        perror(dap_dapname);
        exit(1 as libc::c_int);
    }
    initdo(dap_obs.as_mut_ptr());
    initdo(dap_obs.as_mut_ptr().offset(1 as libc::c_int as isize));
    initdo(dap_obs.as_mut_ptr().offset(2 as libc::c_int as isize));
    initdo(dap_prev.as_mut_ptr());
    initdo(dap_prev.as_mut_ptr().offset(1 as libc::c_int as isize));
    initdo(&mut dosave);
    rfile = dap_malloc(
        (::std::mem::size_of::<RFILE>() as libc::c_ulong)
            .wrapping_mul(dap_nrfiles as libc::c_ulong) as libc::c_int,
        b"main: rfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut RFILE;
    dfile = dap_malloc(
        (::std::mem::size_of::<DFILE>() as libc::c_ulong)
            .wrapping_mul((dap_nrfiles + 4 as libc::c_int) as libc::c_ulong)
            as libc::c_int,
        b"main: dfile\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut DFILE;
    pageno = 1 as libc::c_int;
    dap_psname = dap_malloc(
        len + 1 as libc::c_int,
        b"main: dap_psname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(dap_psname, lstname);
    strcpy(
        dap_psname.offset(len as isize).offset(-(3 as libc::c_int as isize)),
        b"ps\0" as *const u8 as *const libc::c_char,
    );
    dap_initpict();
    v = 0 as libc::c_int;
    while v < dap_maxvar {
        dap_ono = 0 as libc::c_int;
        while dap_ono < 3 as libc::c_int {
            let ref mut fresh2 = *(dap_obs[dap_ono as usize].do_nam).offset(v as isize);
            *fresh2 = 0 as *mut libc::c_char;
            let ref mut fresh3 = *(dap_obs[dap_ono as usize].do_dl).offset(v as isize);
            *fresh3 = 0 as *mut libc::c_double;
            let ref mut fresh4 = *(dap_obs[dap_ono as usize].do_il).offset(v as isize);
            *fresh4 = 0 as *mut libc::c_int;
            let ref mut fresh5 = *(dap_obs[dap_ono as usize].do_str).offset(v as isize);
            *fresh5 = 0 as *mut libc::c_char;
            *(dap_obs[dap_ono as usize].do_sl).offset(v as isize) = 0 as libc::c_int;
            if dap_ono < 2 as libc::c_int {
                let ref mut fresh6 = *(dap_prev[dap_ono as usize].do_str)
                    .offset(v as isize);
                *fresh6 = 0 as *mut libc::c_char;
            }
            dap_ono += 1;
            dap_ono;
        }
        let ref mut fresh7 = *(dosave.do_str).offset(v as isize);
        *fresh7 = 0 as *mut libc::c_char;
        v += 1;
        v;
    }
    dap_ono = 0 as libc::c_int;
    testd = -2.0f64;
    ptesti = &mut testd as *mut libc::c_double as *mut libc::c_int as *mut libc::c_uint;
    if *ptesti == 0 {
        dap_dbllow = 0 as libc::c_int;
        dap_dblhigh = 1 as libc::c_int;
    } else {
        dap_dbllow = 1 as libc::c_int;
        dap_dblhigh = 0 as libc::c_int;
    }
    v = 0 as libc::c_int;
    while v < 3 as libc::c_int {
        dap_in[v as usize] = 0 as *mut libc::c_void as *mut DFILE;
        dap_out[v as usize] = 0 as *mut libc::c_void as *mut DFILE;
        v += 1;
        v;
    }

    return 0 as libc::c_int;
}
unsafe extern "C" fn dopen(
    mut fname: *mut libc::c_char,
    mut mode: *mut libc::c_char,
) -> *mut FILE {
    let mut dname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    dname = dap_malloc(
        (strlen(fname as *const libc::c_char))
            .wrapping_add(strlen(dap_setdir))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"dopen: dname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_name(dname, fname);
    f = fopen(dname, mode as *const libc::c_char);
    dap_free(
        dname as *mut libc::c_void,
        b"dopen: dname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return f;
}
unsafe extern "C" fn dfopen(
    mut fname: *mut libc::c_char,
    mut mode: *mut libc::c_char,
) -> *mut DFILE {
    static mut rfileinit: libc::c_int = 0 as libc::c_int;
    let mut f: libc::c_int = 0;
    let mut truemode: [libc::c_char; 2] = [0; 2];
    if rfileinit == 0 {
        rfileinit = 1 as libc::c_int;
        f = 0 as libc::c_int;
        while f < dap_nrfiles {
            let ref mut fresh8 = (*dfile.offset((4 as libc::c_int + f) as isize))
                .dfile_name;
            *fresh8 = 0 as *mut libc::c_char;
            let ref mut fresh9 = (*dfile.offset((4 as libc::c_int + f) as isize))
                .dfile_ram;
            *fresh9 = rfile.offset(f as isize);
            let ref mut fresh10 = (*dfile.offset((4 as libc::c_int + f) as isize))
                .dfile_disk;
            *fresh10 = 0 as *mut FILE;
            let ref mut fresh11 = (*rfile.offset(f as isize)).rfile_str;
            *fresh11 = 0 as *mut libc::c_char;
            f += 1;
            f;
        }
        f = 0 as libc::c_int;
        while f < 4 as libc::c_int {
            let ref mut fresh12 = (*dfile.offset(f as isize)).dfile_disk;
            *fresh12 = 0 as *mut FILE;
            let ref mut fresh13 = (*dfile.offset(f as isize)).dfile_ram;
            *fresh13 = 0 as *mut RFILE;
            f += 1;
            f;
        }
    }
    if fname.is_null() {
        fputs(
            b"(dfopen) no file name given\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if *fname.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        if strcmp(mode, b"r\0" as *const u8 as *const libc::c_char) == 0 {
            f = 0 as libc::c_int;
            while f < dap_nrfiles {
                if !((*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name)
                    .is_null()
                    && strcmp(
                        (*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name,
                        fname.offset(1 as libc::c_int as isize),
                    ) == 0
                {
                    let ref mut fresh14 = (*rfile.offset(f as isize)).rfile_pos;
                    *fresh14 = (*rfile.offset(f as isize)).rfile_str;
                    return dfile.offset(4 as libc::c_int as isize).offset(f as isize);
                }
                f += 1;
                f;
            }
            return 0 as *mut DFILE;
        } else if strcmp(mode, b"w\0" as *const u8 as *const libc::c_char) == 0 {
            f = 0 as libc::c_int;
            while f < dap_nrfiles {
                if !((*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name)
                    .is_null()
                    && strcmp(
                        (*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name,
                        fname.offset(1 as libc::c_int as isize),
                    ) == 0
                {
                    let ref mut fresh15 = (*rfile.offset(f as isize)).rfile_end;
                    *fresh15 = (*rfile.offset(f as isize)).rfile_str;
                    let ref mut fresh16 = (*rfile.offset(f as isize)).rfile_pos;
                    *fresh16 = (*rfile.offset(f as isize)).rfile_str;
                    return dfile.offset(4 as libc::c_int as isize).offset(f as isize);
                }
                f += 1;
                f;
            }
            f = 0 as libc::c_int;
            while f < dap_nrfiles {
                if ((*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name)
                    .is_null()
                {
                    let ref mut fresh17 = (*rfile.offset(f as isize)).rfile_str;
                    *fresh17 = dap_malloc(
                        dap_rfilesize,
                        b"dfopen: rfile[f].rfile_str\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    let ref mut fresh18 = (*dfile
                        .offset((4 as libc::c_int + f) as isize))
                        .dfile_name;
                    *fresh18 = dap_malloc(
                        strlen(fname) as libc::c_int,
                        b"dfopen: dfile[NDFILES + f].dfile_name\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    strcpy(
                        (*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name,
                        fname.offset(1 as libc::c_int as isize),
                    );
                    let ref mut fresh19 = (*rfile.offset(f as isize)).rfile_end;
                    *fresh19 = (*rfile.offset(f as isize)).rfile_str;
                    let ref mut fresh20 = (*rfile.offset(f as isize)).rfile_pos;
                    *fresh20 = (*rfile.offset(f as isize)).rfile_str;
                    return dfile.offset(4 as libc::c_int as isize).offset(f as isize);
                }
                f += 1;
                f;
            }
            return 0 as *mut DFILE;
        } else if strcmp(mode, b"a\0" as *const u8 as *const libc::c_char) == 0 {
            f = 0 as libc::c_int;
            while f < dap_nrfiles {
                if strcmp(
                    (*dfile.offset((4 as libc::c_int + f) as isize)).dfile_name,
                    fname.offset(1 as libc::c_int as isize),
                ) == 0
                {
                    let ref mut fresh21 = (*rfile.offset(f as isize)).rfile_pos;
                    *fresh21 = (*rfile.offset(f as isize)).rfile_end;
                    return dfile.offset(4 as libc::c_int as isize).offset(f as isize);
                }
                f += 1;
                f;
            }
            return 0 as *mut DFILE;
        } else {
            fprintf(
                dap_err,
                b"(dfopen) bad mode: %s\n\0" as *const u8 as *const libc::c_char,
                mode,
            );
            exit(1 as libc::c_int);
        }
    }
    f = 0 as libc::c_int;
    while f < 4 as libc::c_int {
        if ((*dfile.offset(f as isize)).dfile_disk).is_null() {
            if *mode.offset(1 as libc::c_int as isize) as libc::c_int == 'f' as i32 {
                truemode[0 as libc::c_int
                    as usize] = *mode.offset(0 as libc::c_int as isize);
                truemode[1 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                let ref mut fresh22 = (*dfile.offset(f as isize)).dfile_disk;
                *fresh22 = fopen(fname, truemode.as_mut_ptr());
            } else {
                let ref mut fresh23 = (*dfile.offset(f as isize)).dfile_disk;
                *fresh23 = dopen(fname, mode);
            }
            if !((*dfile.offset(f as isize)).dfile_disk).is_null() {
                let ref mut fresh24 = (*dfile.offset(f as isize)).dfile_name;
                *fresh24 = dap_malloc(
                    (strlen(fname)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                        as libc::c_int,
                    b"dfopen: dfile[f].dfile_name\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                strcpy((*dfile.offset(f as isize)).dfile_name, fname);
                return dfile.offset(f as isize);
            } else {
                return 0 as *mut DFILE
            }
        }
        f += 1;
        f;
    }
    return 0 as *mut DFILE;
}
unsafe extern "C" fn dfclose(mut fp: *mut DFILE) {
    if !((*fp).dfile_disk).is_null() {
        fclose((*fp).dfile_disk);
        (*fp).dfile_disk = 0 as *mut FILE;
    } else {
        (*(*fp).dfile_ram).rfile_pos = (*(*fp).dfile_ram).rfile_str;
    };
}
unsafe extern "C" fn dgetc(mut fp: *mut DFILE) -> libc::c_int {
    let mut c: libc::c_int = 0;
    if !((*fp).dfile_disk).is_null() {
        c = getc((*fp).dfile_disk);
    } else if (*(*fp).dfile_ram).rfile_pos < (*(*fp).dfile_ram).rfile_end {
        let fresh25 = (*(*fp).dfile_ram).rfile_pos;
        (*(*fp).dfile_ram).rfile_pos = ((*(*fp).dfile_ram).rfile_pos).offset(1);
        c = *fresh25 as libc::c_int;
    } else {
        c = -(1 as libc::c_int);
    }
    return c;
}
unsafe extern "C" fn undgetc(mut c: libc::c_int, mut fp: *mut DFILE) {
    if !((*fp).dfile_disk).is_null() {
        ungetc(c, (*fp).dfile_disk);
    } else if (*(*fp).dfile_ram).rfile_pos > (*(*fp).dfile_ram).rfile_str {
        (*(*fp).dfile_ram).rfile_pos = ((*(*fp).dfile_ram).rfile_pos).offset(-1);
        (*(*fp).dfile_ram).rfile_pos;
        *(*(*fp).dfile_ram).rfile_pos = c as libc::c_char;
    } else {
        fprintf(
            dap_err,
            b"(undgetc) can't unget past beginning of file %s\n\0" as *const u8
                as *const libc::c_char,
            (*fp).dfile_name,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn dap_putc(mut c: libc::c_int, mut fp: *mut DFILE) {
    if !((*fp).dfile_disk).is_null() {
        putc(c, (*fp).dfile_disk);
    } else if (*(*fp).dfile_ram).rfile_pos
        < ((*(*fp).dfile_ram).rfile_str).offset(dap_rfilesize as isize)
    {
        let fresh26 = (*(*fp).dfile_ram).rfile_pos;
        (*(*fp).dfile_ram).rfile_pos = ((*(*fp).dfile_ram).rfile_pos).offset(1);
        *fresh26 = c as libc::c_char;
        (*(*fp).dfile_ram).rfile_end = ((*(*fp).dfile_ram).rfile_end).offset(1);
        (*(*fp).dfile_ram).rfile_end;
    } else {
        fprintf(
            dap_err,
            b"(dap_putc) too many characters: %s\n\0" as *const u8
                as *const libc::c_char,
            (*fp).dfile_name,
        );
        exit(1 as libc::c_int);
    };
}
unsafe extern "C" fn dputs(
    mut s: *mut libc::c_char,
    mut suff: *mut libc::c_char,
    mut fp: *mut DFILE,
) {
    if !((*fp).dfile_disk).is_null() {
        fputs(s, (*fp).dfile_disk);
        fputs(suff, (*fp).dfile_disk);
    } else {
        while *s != 0 {
            let fresh27 = s;
            s = s.offset(1);
            dap_putc(*fresh27 as libc::c_int, fp);
        }
        while *suff != 0 {
            let fresh28 = suff;
            suff = suff.offset(1);
            dap_putc(*fresh28 as libc::c_int, fp);
        }
    };
}
unsafe extern "C" fn dputi(mut i: libc::c_int, mut fp: *mut DFILE) {
    static mut istr: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    if istr.is_null() {
        istr = dap_malloc(
            dap_intlen + 1 as libc::c_int,
            b"dputi: istr\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !((*fp).dfile_disk).is_null() {
        fprintf((*fp).dfile_disk, b"%d\0" as *const u8 as *const libc::c_char, i);
    } else {
        sprintf(istr, b"%d\0" as *const u8 as *const libc::c_char, i);
        s = 0 as libc::c_int;
        while (s as libc::c_ulong) < strlen(istr) {
            dap_putc(*istr.offset(s as isize) as libc::c_int, fp);
            s += 1;
            s;
        }
    };
}
unsafe extern "C" fn dflush(mut fp: *mut DFILE) {
    fflush((*fp).dfile_disk);
}
pub unsafe extern "C" fn dap_suffix(
    mut dst: *mut libc::c_char,
    mut src: *mut libc::c_char,
    mut suff: *mut libc::c_char,
) {
    let mut n: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    n = 0 as libc::c_int;
    while *src.offset(n as isize) != 0 {
        *dst.offset(n as isize) = *src.offset(n as isize);
        n += 1;
        n;
    }
    if *suff.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        loop {
            n -= 1;
            if !(n >= 0 as libc::c_int) {
                break;
            }
            if *dst.offset(n as isize) as libc::c_int == '.' as i32 {
                break;
            }
        }
        if n < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(dap_suffix) source name has no '.': %s\n\0" as *const u8
                    as *const libc::c_char,
                src,
            );
            exit(1 as libc::c_int);
        }
        n += 1;
        n;
        s = 1 as libc::c_int;
    } else {
        s = 0 as libc::c_int;
    }
    while *suff.offset(s as isize) != 0 {
        let fresh29 = n;
        n = n + 1;
        *dst.offset(fresh29 as isize) = *suff.offset(s as isize);
        s += 1;
        s;
    }
    *dst.offset(n as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn dap_varnum(mut vname: *mut libc::c_char) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut nonblank: libc::c_int = 0;
    while *vname as libc::c_int == ' ' as i32 {
        vname = vname.offset(1);
        vname;
    }
    nonblank = 0 as libc::c_int;
    while *vname.offset(nonblank as isize) as libc::c_int != 0
        && *vname.offset(nonblank as isize) as libc::c_int != ' ' as i32
    {
        nonblank += 1;
        nonblank;
    }
    if !vname.is_null() {
        v = 0 as libc::c_int;
        while v < dap_obs[dap_ono as usize].do_nvar {
            if strncmp(
                vname,
                *(dap_obs[dap_ono as usize].do_nam).offset(v as isize),
                nonblank as libc::c_ulong,
            ) == 0
                && *(*(dap_obs[dap_ono as usize].do_nam).offset(v as isize))
                    .offset(nonblank as isize) == 0
            {
                return v;
            }
            v += 1;
            v;
        }
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn dap_arrnum(
    mut vname: *mut libc::c_char,
    mut dim: *mut libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    v = 0 as libc::c_int;
    while v < dap_obs[dap_ono as usize].do_nvar {
        n = 0 as libc::c_int;
        while *vname.offset(n as isize) as libc::c_int != 0
            && *vname.offset(n as isize) as libc::c_int
                == *(*(dap_obs[dap_ono as usize].do_nam).offset(v as isize))
                    .offset(n as isize) as libc::c_int
        {
            n += 1;
            n;
        }
        if *vname.offset(n as isize) == 0
            && *(*(dap_obs[dap_ono as usize].do_nam).offset(v as isize))
                .offset(n as isize) as libc::c_int == '[' as i32
        {
            d = 1 as libc::c_int;
            while v + d < dap_obs[dap_ono as usize].do_nvar {
                n = 0 as libc::c_int;
                while *vname.offset(n as isize) as libc::c_int != 0
                    && *vname.offset(n as isize) as libc::c_int
                        == *(*(dap_obs[dap_ono as usize].do_nam)
                            .offset((v + d) as isize))
                            .offset(n as isize) as libc::c_int
                {
                    n += 1;
                    n;
                }
                if *vname.offset(n as isize) as libc::c_int != 0
                    || *(*(dap_obs[dap_ono as usize].do_nam).offset((v + d) as isize))
                        .offset(n as isize) as libc::c_int != '[' as i32
                {
                    break;
                }
                d += 1;
                d;
            }
            *dim = d;
            return v;
        }
        v += 1;
        v;
    }
    return -(1 as libc::c_int);
}
unsafe extern "C" fn dap_getline(
    mut fp: *mut DFILE,
    mut line: *mut libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut cc: libc::c_int = 0;
    c = -(1 as libc::c_int);
    match intype {
        0 | 2 => {
            l = 0 as libc::c_int;
            loop {
                c = dgetc(fp);
                if !(c != -(1 as libc::c_int)) {
                    break;
                }
                if c == '\n' as i32 {
                    break;
                }
                if c == '\r' as i32 {
                    cc = dgetc(fp);
                    if cc != '\n' as i32 {
                        undgetc(cc, fp);
                    }
                    break;
                } else if l < dap_linelen {
                    let fresh30 = l;
                    l = l + 1;
                    *line.offset(fresh30 as isize) = c as libc::c_char;
                } else {
                    *line.offset(l as isize) = '\0' as i32 as libc::c_char;
                    fprintf(
                        dap_err,
                        b"(dap_getline) line too long:\n%s\n\0" as *const u8
                            as *const libc::c_char,
                        line,
                    );
                    exit(1 as libc::c_int);
                }
            }
            *line.offset(l as isize) = '\0' as i32 as libc::c_char;
        }
        1 => {
            dgetc(fp);
            l = 0 as libc::c_int;
            while l < inlen {
                c = dgetc(fp);
                if c == -(1 as libc::c_int) {
                    break;
                }
                *line.offset(l as isize) = c as libc::c_char;
                l += 1;
                l;
            }
            if l < inlen {
                l = 0 as libc::c_int;
            }
            *line.offset(l as isize) = '\0' as i32 as libc::c_char;
        }
        _ => {
            fprintf(
                dap_err,
                b"(dap_getline) bad infile type: %d\n\0" as *const u8
                    as *const libc::c_char,
                intype,
            );
            exit(1 as libc::c_int);
        }
    }
    if l == 0 && c == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    return l;
}
pub unsafe extern "C" fn dap_swap() {
    let mut iv: libc::c_int = 0;
    let mut dbltmp: libc::c_double = 0.;
    let mut inttmp: libc::c_int = 0;
    let mut strtmp: libc::c_char = 0;
    let mut s: libc::c_int = 0;
    let mut so: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sp: *mut libc::c_char = 0 as *mut libc::c_char;
    iv = 0 as libc::c_int;
    while iv < dap_obs[dap_ono as usize].do_ivar {
        match *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize)
        {
            -1 => {
                dbltmp = *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dap_prev[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                *(dap_prev[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = dbltmp;
                if !(*(dap_obs[dap_ono as usize].do_dl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ))
                    .is_null()
                {
                    **(dap_obs[dap_ono as usize].do_dl)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) = *(dap_obs[dap_ono as usize].do_dbl)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        );
                }
            }
            0 => {
                inttmp = *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dap_prev[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                *(dap_prev[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = inttmp;
                if !(*(dap_obs[dap_ono as usize].do_il)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ))
                    .is_null()
                {
                    **(dap_obs[dap_ono as usize].do_il)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) = *(dap_obs[dap_ono as usize].do_int)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        );
                }
            }
            _ => {
                so = *(dap_obs[dap_ono as usize].do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                sp = *(dap_prev[dap_ono as usize].do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                s = 0 as libc::c_int;
                while s
                    < *(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        )
                {
                    strtmp = *so.offset(s as isize);
                    *so.offset(s as isize) = *sp.offset(s as isize);
                    *sp.offset(s as isize) = strtmp;
                    s += 1;
                    s;
                }
            }
        }
        iv += 1;
        iv;
    }
}
pub unsafe extern "C" fn dap_save() {
    let mut iv: libc::c_int = 0;
    iv = 0 as libc::c_int;
    while iv < dap_obs[dap_ono as usize].do_ivar {
        match *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize)
        {
            -1 => {
                *(dosave.do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
            }
            0 => {
                *(dosave.do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
            }
            _ => {
                if !(*(dosave.do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ))
                    .is_null()
                {
                    dap_free(
                        *(dosave.do_str)
                            .offset(
                                *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                    as isize,
                            ) as *mut libc::c_void,
                        b"dap_save: dosave.do_str[dap_obs[dap_ono].do_in[iv]]\0"
                            as *const u8 as *const libc::c_char as *mut libc::c_char,
                    );
                }
                let ref mut fresh31 = *(dosave.do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                *fresh31 = dap_malloc(
                    *(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) + 1 as libc::c_int,
                    b"dap_save: dap_obs[dap_ono].do_nam[dap_obs[dap_ono].do_in[iv]]\0"
                        as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
                strncpy(
                    *(dosave.do_str)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ),
                    *(dap_obs[dap_ono as usize].do_str)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ),
                    (*(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) + 1 as libc::c_int) as libc::c_ulong,
                );
            }
        }
        iv += 1;
        iv;
    }
}
pub unsafe extern "C" fn dap_rest() {
    let mut iv: libc::c_int = 0;
    iv = 0 as libc::c_int;
    while iv < dap_obs[dap_ono as usize].do_ivar {
        match *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize)
        {
            -1 => {
                *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dosave.do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                if !(*(dap_obs[dap_ono as usize].do_dl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ))
                    .is_null()
                {
                    **(dap_obs[dap_ono as usize].do_dl)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) = *(dap_obs[dap_ono as usize].do_dbl)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        );
                }
            }
            0 => {
                *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dosave.do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
                if !(*(dap_obs[dap_ono as usize].do_il)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ))
                    .is_null()
                {
                    **(dap_obs[dap_ono as usize].do_il)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) = *(dap_obs[dap_ono as usize].do_int)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        );
                }
            }
            _ => {
                strncpy(
                    *(dap_obs[dap_ono as usize].do_str)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ),
                    *(dosave.do_str)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ),
                    (*(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) + 1 as libc::c_int) as libc::c_ulong,
                );
            }
        }
        iv += 1;
        iv;
    }
}
pub unsafe extern "C" fn dap_ftell(mut fp: *mut DFILE) -> libc::c_long {
    if !((*fp).dfile_disk).is_null() {
        return ftell((*fp).dfile_disk);
    }
    return ((*(*fp).dfile_ram).rfile_pos).offset_from((*(*fp).dfile_ram).rfile_str)
        as libc::c_long;
}
pub unsafe extern "C" fn dap_mark() {
    filepos[dap_ono as usize] = dap_ftell(dap_in[dap_ono as usize]);
}
unsafe extern "C" fn dfseek(
    mut fp: *mut DFILE,
    mut pos: libc::c_long,
    mut mode: libc::c_int,
) {
    if !((*fp).dfile_disk).is_null() {
        fseek((*fp).dfile_disk, pos, mode);
    } else if ((*(*fp).dfile_ram).rfile_str).offset(pos as isize)
        < (*(*fp).dfile_ram).rfile_end
    {
        (*(*fp).dfile_ram)
            .rfile_pos = ((*(*fp).dfile_ram).rfile_str).offset(pos as isize);
    } else {
        fprintf(
            dap_err,
            b"(dfseek) seek past end of ramfile %s\n\0" as *const u8
                as *const libc::c_char,
            (*fp).dfile_name,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn dap_rewind() {
    if !(dap_in[dap_ono as usize]).is_null() {
        if filepos[dap_ono as usize] < dap_ftell(dap_in[dap_ono as usize]) {
            eof[dap_ono as usize] = 0 as libc::c_int;
        }
        dfseek(dap_in[dap_ono as usize], filepos[dap_ono as usize], 0 as libc::c_int);
        dap_obs[dap_ono as usize].do_valid = 0 as libc::c_int;
    } else {
        fprintf(
            dap_err,
            b"(dap_rewind) file (%d) is closed.\n\0" as *const u8 as *const libc::c_char,
            dap_ono,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn dap_blank(mut str: *mut libc::c_char) -> libc::c_int {
    let mut b: libc::c_int = 0;
    b = 0 as libc::c_int;
    while *str.offset(b as isize) as libc::c_int == ' ' as i32 {
        b += 1;
        b;
    }
    if *str.offset(b as isize) != 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
#[export_name = "skip"]
pub unsafe extern "C" fn skip_0(mut nlines: libc::c_int) {
    let mut line: *mut libc::c_char = 0 as *mut libc::c_char;
    line = dap_malloc(
        dap_linelen + 1 as libc::c_int,
        b"skip: line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    loop {
        nlines -= 1;
        if !(nlines >= 0 as libc::c_int) {
            break;
        }
        if (dap_in[dap_ono as usize]).is_null() || eof[dap_ono as usize] != 0 {
            fprintf(
                dap_err,
                b"(skip) tried to read past end of file (%d).\n\0" as *const u8
                    as *const libc::c_char,
                dap_ono,
            );
            exit(1 as libc::c_int);
        }
        if dap_getline(dap_in[dap_ono as usize], line) < 0 as libc::c_int {
            eof[dap_ono as usize] = 1 as libc::c_int;
        }
        lineno[dap_ono as usize] += 1;
        lineno[dap_ono as usize];
    }
    dap_free(
        line as *mut libc::c_void,
        b"skip: line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn step() -> libc::c_int {
    static mut stepinit: libc::c_int = 0 as libc::c_int;
    static mut line: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut value: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut iv: libc::c_int = 0;
    let mut nread: libc::c_int = 0;
    if stepinit == 0 {
        stepinit = 1 as libc::c_int;
        line = dap_malloc(
            dap_linelen + 1 as libc::c_int,
            b"step: line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        value = dap_malloc(
            dap_linelen + 1 as libc::c_int,
            b"step: value\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if (dap_in[dap_ono as usize]).is_null() || eof[dap_ono as usize] != 0 {
        fprintf(
            dap_err,
            b"(step) ERROR: tried to read past end of file (%s).\n\0" as *const u8
                as *const libc::c_char,
            if !(dap_in[dap_ono as usize]).is_null() {
                (*dap_in[dap_ono as usize]).dfile_name as *const libc::c_char
            } else {
                b"?\0" as *const u8 as *const libc::c_char
            },
        );
        exit(1 as libc::c_int);
    }
    iv = 0 as libc::c_int;
    while iv < dap_obs[dap_ono as usize].do_ivar {
        match *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize)
        {
            -1 => {
                *(dap_prev[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
            }
            0 => {
                *(dap_prev[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    ) = *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(iv as isize) as isize,
                    );
            }
            _ => {
                strncpy(
                    *(dap_prev[dap_ono as usize].do_str)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ),
                    *(dap_obs[dap_ono as usize].do_str)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ),
                    (*(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(iv as isize)
                                as isize,
                        ) + 1 as libc::c_int) as libc::c_ulong,
                );
            }
        }
        iv += 1;
        iv;
    }
    dap_prev[dap_ono as usize].do_valid = dap_obs[dap_ono as usize].do_valid;
    nread = dap_getline(dap_in[dap_ono as usize], line);
    if nread <= 0 as libc::c_int {
        eof[dap_ono as usize] = 1 as libc::c_int;
        dap_obs[dap_ono as usize].do_valid = 0 as libc::c_int;
        fprintf(
            stderr,
            b"(step) %d lines read from %s\n\0" as *const u8 as *const libc::c_char,
            lineno[dap_ono as usize] - 1 as libc::c_int,
            (*dap_in[dap_ono as usize]).dfile_name,
        );
        if nread == 0 {
            fputs(
                b"(step) WARNING: terminated on null line\n\0" as *const u8
                    as *const libc::c_char,
                stderr,
            );
        }
        fflush(stderr);
        fprintf(
            dap_log,
            b"(step) %d lines read from  %s\n\0" as *const u8 as *const libc::c_char,
            lineno[dap_ono as usize] - 1 as libc::c_int,
            (*dap_in[dap_ono as usize]).dfile_name,
        );
        if nread == 0 {
            fputs(
                b"(step) WARNING: terminated on null line\n\0" as *const u8
                    as *const libc::c_char,
                dap_log,
            );
        }
        return 0 as libc::c_int;
    }
    if nfields != 0 && dap_obs[dap_ono as usize].do_ivar != nfields {
        fprintf(
            dap_err,
            b"(step) ERROR: number of input variables %d different from number of fields specified %d for %s.\n\0"
                as *const u8 as *const libc::c_char,
            dap_obs[dap_ono as usize].do_ivar,
            nfields,
            (*dap_in[dap_ono as usize]).dfile_name,
        );
        exit(1 as libc::c_int);
    }
    v = 0 as libc::c_int;
    l = 0 as libc::c_int;
    while v < dap_obs[dap_ono as usize].do_ivar {
        if nfields != 0 {
            i = 0 as libc::c_int;
            while *line.offset((l + i) as isize) as libc::c_int != 0
                && i < *fieldwd.offset(v as isize)
            {
                *value.offset(i as isize) = *line.offset((l + i) as isize);
                i += 1;
                i;
            }
            *value.offset(i as isize) = '\0' as i32 as libc::c_char;
            if i < *fieldwd.offset(v as isize) {
                fprintf(
                    dap_log,
                    b"(step (%s:%d)) ERROR: got %d of %d characters for fixed length field for %s: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    (*dap_in[dap_ono as usize]).dfile_name,
                    lineno[dap_ono as usize],
                    i,
                    *fieldwd.offset(v as isize),
                    *(dap_obs[dap_ono as usize].do_nam)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ),
                    value,
                );
                fprintf(
                    dap_err,
                    b"(step (%s:%d)) ERROR: got %d of %d characters for fixed length field for %s: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    (*dap_in[dap_ono as usize]).dfile_name,
                    lineno[dap_ono as usize],
                    i,
                    *fieldwd.offset(v as isize),
                    *(dap_obs[dap_ono as usize].do_nam)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ),
                    value,
                );
                exit(1 as libc::c_int);
            }
        } else {
            i = 0 as libc::c_int;
            while *line.offset((l + i) as isize) as libc::c_int != 0
                && *line.offset((l + i) as isize) as libc::c_int != dap_delim
            {
                *value.offset(i as isize) = *line.offset((l + i) as isize);
                i += 1;
                i;
            }
            *value.offset(i as isize) = '\0' as i32 as libc::c_char;
        }
        if *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize)
            == 0 as libc::c_int
        {
            if intype == 2 as libc::c_int {
                *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ) = dap_getint(value);
            } else if sscanf(
                value,
                b" %d\0" as *const u8 as *const libc::c_char,
                (dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ),
            ) != 1 as libc::c_int
            {
                if dap_blank(value) != 0 {
                    *(dap_obs[dap_ono as usize].do_int)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ) = 0 as libc::c_int;
                } else {
                    fprintf(
                        dap_err,
                        b"(step (%s:%d)) ERROR: invalid integer data for %s: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        (*dap_in[dap_ono as usize]).dfile_name,
                        lineno[dap_ono as usize],
                        *(dap_obs[dap_ono as usize].do_nam)
                            .offset(
                                *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                    as isize,
                            ),
                        value,
                    );
                    exit(1 as libc::c_int);
                }
            }
            if dap_ono == 0
                && !(*(dap_obs[dap_ono as usize].do_il)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_in).offset(v as isize)
                            as isize,
                    ))
                    .is_null()
            {
                **(dap_obs[dap_ono as usize].do_il)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_in).offset(v as isize)
                            as isize,
                    ) = *(dap_obs[0 as libc::c_int as usize].do_int)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_in).offset(v as isize)
                            as isize,
                    );
            }
        } else if *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize)
            == -(1 as libc::c_int)
        {
            if intype == 2 as libc::c_int {
                dap_getdouble(value);
                *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ) = dap_double;
            } else if i == 0 || nfields != 0 && dap_blank(value) != 0
                || strcmp(value, b".\0" as *const u8 as *const libc::c_char) == 0
            {
                *(dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ) = 0.0f64 / 0.0f64;
            } else if sscanf(
                value,
                b" %lf\0" as *const u8 as *const libc::c_char,
                (dap_obs[dap_ono as usize].do_dbl)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ),
            ) != 1 as libc::c_int
            {
                fprintf(
                    dap_err,
                    b"(step (%s:%d)) ERROR: invalid double data for %s: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    (*dap_in[dap_ono as usize]).dfile_name,
                    lineno[dap_ono as usize],
                    *(dap_obs[dap_ono as usize].do_nam)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ),
                    value,
                );
                exit(1 as libc::c_int);
            }
            if dap_ono == 0
                && !(*(dap_obs[dap_ono as usize].do_dl)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_in).offset(v as isize)
                            as isize,
                    ))
                    .is_null()
            {
                **(dap_obs[dap_ono as usize].do_dl)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_in).offset(v as isize)
                            as isize,
                    ) = *(dap_obs[0 as libc::c_int as usize].do_dbl)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_in).offset(v as isize)
                            as isize,
                    );
            }
        } else if i
            <= *(dap_obs[dap_ono as usize].do_len)
                .offset(*(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize)
        {
            strcpy(
                *(dap_obs[dap_ono as usize].do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ),
                value,
            );
        } else {
            strncpy(
                *(dap_obs[dap_ono as usize].do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ),
                value,
                *(dap_obs[dap_ono as usize].do_len)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize,
                    ) as libc::c_ulong,
            );
            *(*(dap_obs[dap_ono as usize].do_str)
                .offset(*(dap_obs[dap_ono as usize].do_in).offset(v as isize) as isize))
                .offset(
                    *(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ) as isize,
                ) = '\0' as i32 as libc::c_char;
            if toolong < dap_toolong {
                fprintf(
                    dap_log,
                    b"(step (%s:%d)) WARNING: string data too long (%d) for %s (%d): %s\n\0"
                        as *const u8 as *const libc::c_char,
                    (*dap_in[dap_ono as usize]).dfile_name,
                    lineno[dap_ono as usize],
                    i,
                    *(dap_obs[dap_ono as usize].do_nam)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ),
                    *(dap_obs[dap_ono as usize].do_len)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_in).offset(v as isize)
                                as isize,
                        ),
                    value,
                );
                toolong += 1;
                toolong;
            }
        }
        l += i;
        if nfields == 0 && *line.offset(l as isize) as libc::c_int == dap_delim {
            l += 1;
            l;
        }
        v += 1;
        v;
    }
    dap_obs[dap_ono as usize].do_valid = 1 as libc::c_int;
    lineno[dap_ono as usize] += 1;
    lineno[dap_ono as usize];
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn dap_vd(
    mut varspec: *mut libc::c_char,
    mut invar: libc::c_int,
) -> libc::c_int {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut varnam: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sign: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut vlen: libc::c_int = 0;
    let mut redeclare: libc::c_int = 0;
    v = -(1 as libc::c_int);
    varnam = dap_malloc(
        strlen(varspec) as libc::c_int,
        b"dap_vd: varnam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = 0 as libc::c_int;
    while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    while *varspec.offset(s as isize) != 0 {
        if dap_obs[dap_ono as usize].do_nvar < dap_maxvar {
            i = 0 as libc::c_int;
            while *varspec.offset((s + i) as isize) as libc::c_int != 0
                && *varspec.offset((s + i) as isize) as libc::c_int != ' ' as i32
            {
                *varnam.offset(i as isize) = *varspec.offset((s + i) as isize);
                i += 1;
                i;
            }
            *varnam.offset(i as isize) = '\0' as i32 as libc::c_char;
            v = dap_varnum(varnam);
            if v < 0 as libc::c_int {
                redeclare = 0 as libc::c_int;
                v = dap_obs[dap_ono as usize].do_nvar;
                if !(*(dap_obs[dap_ono as usize].do_nam).offset(v as isize)).is_null() {
                    dap_free(
                        *(dap_obs[dap_ono as usize].do_nam).offset(v as isize)
                            as *mut libc::c_void,
                        b"dap_vd: dap_obs[dap_ono].do_nam[v]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                let ref mut fresh32 = *(dap_obs[dap_ono as usize].do_nam)
                    .offset(v as isize);
                *fresh32 = dap_malloc(
                    i + 1 as libc::c_int,
                    b"dap_vd: dap_obs[dap_ono].do_nam[v]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                strcpy(*(dap_obs[dap_ono as usize].do_nam).offset(v as isize), varnam);
                if invar != 0 {
                    let fresh33 = dap_obs[dap_ono as usize].do_ivar;
                    dap_obs[dap_ono as usize]
                        .do_ivar = dap_obs[dap_ono as usize].do_ivar + 1;
                    *(dap_obs[dap_ono as usize].do_in).offset(fresh33 as isize) = v;
                }
            } else {
                redeclare = 1 as libc::c_int;
            }
            while *varspec.offset((s + i) as isize) as libc::c_int != 0
                && *varspec.offset((s + i) as isize) as libc::c_int == ' ' as i32
            {
                i += 1;
                i;
            }
            if *varspec.offset((s + i) as isize) == 0 {
                fprintf(
                    dap_err,
                    b"(dap_vd) missing variable length: %s\n\0" as *const u8
                        as *const libc::c_char,
                    varspec.offset(s as isize),
                );
                exit(1 as libc::c_int);
            }
            s += i;
            if *varspec.offset(s as isize) as libc::c_int == '-' as i32 {
                sign = -(1 as libc::c_int);
                s += 1;
                s;
                while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
                    s += 1;
                    s;
                }
            } else {
                sign = 1 as libc::c_int;
            }
            i = 0 as libc::c_int;
            vlen = 0 as libc::c_int;
            while '0' as i32 <= *varspec.offset((s + i) as isize) as libc::c_int
                && *varspec.offset((s + i) as isize) as libc::c_int <= '9' as i32
            {
                vlen = 10 as libc::c_int * vlen
                    + *varspec.offset((s + i) as isize) as libc::c_int - '0' as i32;
                i += 1;
                i;
            }
            vlen *= sign;
            if redeclare != 0 {
                if *(dap_obs[dap_ono as usize].do_len).offset(v as isize) != vlen {
                    if vlen > 0 as libc::c_int
                        && *(dap_obs[dap_ono as usize].do_len).offset(v as isize)
                            > 0 as libc::c_int
                    {
                        fprintf(
                            dap_err,
                            b"(dap_vd) respecification of length of %s from %d to %d\n\0"
                                as *const u8 as *const libc::c_char,
                            varnam,
                            *(dap_obs[dap_ono as usize].do_len).offset(v as isize)
                                + 1 as libc::c_int,
                            vlen + 1 as libc::c_int,
                        );
                    } else {
                        fprintf(
                            dap_err,
                            b"(dap_vd) respecification of type of %s\n\0" as *const u8
                                as *const libc::c_char,
                            varnam,
                        );
                    }
                    exit(1 as libc::c_int);
                }
            } else {
                *(dap_obs[dap_ono as usize].do_len).offset(v as isize) = vlen;
            }
            if *varspec.offset((s + i) as isize) as libc::c_int != 0
                && *varspec.offset((s + i) as isize) as libc::c_int != ' ' as i32
            {
                fprintf(
                    dap_err,
                    b"(dap_vd) invalid variable length for %s: %s\n\0" as *const u8
                        as *const libc::c_char,
                    varnam,
                    varspec.offset(s as isize),
                );
                exit(1 as libc::c_int);
            }
            if redeclare == 0 && vlen > 0 as libc::c_int {
                if *(dap_obs[dap_ono as usize].do_sl).offset(v as isize) == 0
                    && !(*(dap_obs[dap_ono as usize].do_str).offset(v as isize))
                        .is_null()
                {
                    dap_free(
                        *(dap_obs[dap_ono as usize].do_str).offset(v as isize)
                            as *mut libc::c_void,
                        b"dap_vd: dap_obs[dap_ono].do_str[v]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                if dap_ono < 2 as libc::c_int
                    && !(*(dap_prev[dap_ono as usize].do_str).offset(v as isize))
                        .is_null()
                {
                    dap_free(
                        *(dap_prev[dap_ono as usize].do_str).offset(v as isize)
                            as *mut libc::c_void,
                        b"dap_vd: dap_prev[dap_ono].do_str[v]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
                let ref mut fresh34 = *(dap_obs[dap_ono as usize].do_str)
                    .offset(v as isize);
                *fresh34 = dap_malloc(
                    vlen + 1 as libc::c_int,
                    b"dap_vd: dap_obs[dap_ono].do_str[v]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
                *(dap_obs[dap_ono as usize].do_sl).offset(v as isize) = 0 as libc::c_int;
                if dap_ono < 2 as libc::c_int {
                    let ref mut fresh35 = *(dap_prev[dap_ono as usize].do_str)
                        .offset(v as isize);
                    *fresh35 = dap_malloc(
                        vlen + 1 as libc::c_int,
                        b"dap_vd: dap_prev[dap_ono].do_str[v]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                }
            }
            if vlen == -(1 as libc::c_int) {
                *(dap_obs[dap_ono as usize].do_dbl).offset(v as isize) = 0.0f64 / 0.0f64;
            } else if vlen == 0 as libc::c_int {
                *(dap_obs[dap_ono as usize].do_int)
                    .offset(v as isize) = 0 as libc::c_int;
            } else {
                *(*(dap_obs[dap_ono as usize].do_str).offset(v as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            }
            s += i;
            while *varspec.offset(s as isize) as libc::c_int == ' ' as i32 {
                s += 1;
                s;
            }
        } else {
            fprintf(
                dap_err,
                b"(dap_vd) too many variables: %s\n\0" as *const u8
                    as *const libc::c_char,
                varspec.offset(s as isize),
            );
            exit(1 as libc::c_int);
        }
        if redeclare == 0 {
            dap_obs[dap_ono as usize].do_nvar += 1;
            dap_obs[dap_ono as usize].do_nvar;
        }
    }
    dap_free(
        varnam as *mut libc::c_void,
        b"dap_vd: varnam\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return v;
}
pub unsafe extern "C" fn dap_dl(
    mut varname: *mut libc::c_char,
    mut dbl: *mut libc::c_double,
) {
    let mut v: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    v = dap_varnum(varname);
    if v >= 0 as libc::c_int {
        let ref mut fresh36 = *(dap_obs[dap_ono as usize].do_dl).offset(v as isize);
        *fresh36 = dbl;
        *dbl = 0.0f64 / 0.0f64;
    } else {
        v = dap_arrnum(varname, &mut dim);
        if v > 0 as libc::c_int {
            d = 0 as libc::c_int;
            while d < dim {
                let ref mut fresh37 = *(dap_obs[dap_ono as usize].do_dl)
                    .offset((v + d) as isize);
                *fresh37 = dbl.offset(d as isize);
                *dbl.offset(d as isize) = 0.0f64 / 0.0f64;
                d += 1;
                d;
            }
        } else {
            fprintf(
                dap_err,
                b"(dap_dl) unknown variable %s\n\0" as *const u8 as *const libc::c_char,
                varname,
            );
            exit(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn dap_il(
    mut varname: *mut libc::c_char,
    mut i: *mut libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    v = dap_varnum(varname);
    if v > 0 as libc::c_int {
        let ref mut fresh38 = *(dap_obs[dap_ono as usize].do_il).offset(v as isize);
        *fresh38 = i;
        *i = 0 as libc::c_int;
    } else {
        v = dap_arrnum(varname, &mut dim);
        if v > 0 as libc::c_int {
            d = 0 as libc::c_int;
            while d < dim {
                let ref mut fresh39 = *(dap_obs[dap_ono as usize].do_il)
                    .offset((v + d) as isize);
                *fresh39 = i.offset(d as isize);
                *i.offset(d as isize) = 0 as libc::c_int;
                d += 1;
                d;
            }
        } else {
            fprintf(
                dap_err,
                b"(dap_il) unknown variable %s\n\0" as *const u8 as *const libc::c_char,
                varname,
            );
            exit(1 as libc::c_int);
        }
    };
}
pub unsafe extern "C" fn dap_sl(
    mut varname: *mut libc::c_char,
    mut s: *mut libc::c_char,
) {
    let mut v: libc::c_int = 0;
    v = dap_varnum(varname);
    if v >= 0 as libc::c_int {
        dap_free(
            *(dap_obs[dap_ono as usize].do_str).offset(v as isize) as *mut libc::c_void,
            b"dap_sl: dap_obs[dap_ono].do_str[v]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        let ref mut fresh40 = *(dap_obs[dap_ono as usize].do_str).offset(v as isize);
        *fresh40 = s;
        *(dap_obs[dap_ono as usize].do_sl).offset(v as isize) = 1 as libc::c_int;
        *s.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    } else {
        fprintf(
            dap_err,
            b"(dap_sl) unknown variable %s\n\0" as *const u8 as *const libc::c_char,
            varname,
        );
        exit(1 as libc::c_int);
    };
}
pub unsafe extern "C" fn dap_name(
    mut dname: *mut libc::c_char,
    mut fname: *mut libc::c_char,
) {
    let mut statbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if fname.is_null() || *fname.offset(0 as libc::c_int as isize) == 0 {
        strcpy(dname, b"/dev/null\0" as *const u8 as *const libc::c_char);
        return;
    }
    strcpy(dname, dap_setdir);
    if stat(dname as *const libc::c_char, &mut statbuf) < 0 as libc::c_int {
        if mkdir(dname as *const libc::c_char, 0o700 as libc::c_int as mode_t)
            < 0 as libc::c_int
        {
            perror(dap_dapname);
            exit(1 as libc::c_int);
        }
    } else if statbuf.st_mode & 0o40000 as libc::c_int as libc::c_uint == 0 {
        fprintf(
            dap_err,
            b"%s: non-directory file exists: %s\n\0" as *const u8 as *const libc::c_char,
            dap_dapname,
            dname,
        );
        exit(1 as libc::c_int);
    }
    strcat(dname, b"/\0" as *const u8 as *const libc::c_char);
    strcat(dname, fname);
}
unsafe extern "C" fn dblcmp(
    mut d1: *mut libc::c_double,
    mut d2: *mut libc::c_double,
) -> libc::c_int {
    if *d1 < *d2 {
        return -(1 as libc::c_int)
    } else if *d1 > *d2 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn intcmp(
    mut i1: *mut libc::c_int,
    mut i2: *mut libc::c_int,
) -> libc::c_int {
    if *i1 < *i2 {
        return -(1 as libc::c_int)
    } else if *i1 > *i2 {
        return 1 as libc::c_int
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn stcmp(
    mut s1: *mut *mut libc::c_char,
    mut s2: *mut *mut libc::c_char,
) -> libc::c_int {
    return strcmp(*s1, *s2);
}
unsafe extern "C" fn findlev(
    mut class: libc::c_int,
    mut dlevel: *mut libc::c_double,
    mut ilevel: *mut libc::c_int,
    mut slevel: *mut *mut libc::c_char,
    mut nlevels: libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    if *(dap_obs[dap_ono as usize].do_len).offset(class as isize) == -(1 as libc::c_int)
    {
        v = 0 as libc::c_int;
        while v < nlevels {
            if *(dap_obs[dap_ono as usize].do_dbl).offset(class as isize)
                == *dlevel.offset(v as isize)
            {
                break;
            }
            v += 1;
            v;
        }
    } else if *(dap_obs[dap_ono as usize].do_len).offset(class as isize)
        == 0 as libc::c_int
    {
        v = 0 as libc::c_int;
        while v < nlevels {
            if *(dap_obs[dap_ono as usize].do_int).offset(class as isize)
                == *ilevel.offset(v as isize)
            {
                break;
            }
            v += 1;
            v;
        }
    } else {
        v = 0 as libc::c_int;
        while v < nlevels {
            if strcmp(
                *(dap_obs[dap_ono as usize].do_str).offset(class as isize),
                *slevel.offset(v as isize),
            ) == 0
            {
                break;
            }
            v += 1;
            v;
        }
    }
    if v < nlevels {
        return v;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn inclev(
    mut lev: *mut libc::c_int,
    mut nlevels: *mut libc::c_int,
    mut nclass: libc::c_int,
) -> libc::c_int {
    let mut c: libc::c_int = 0;
    c = nclass - 1 as libc::c_int;
    while c >= 0 as libc::c_int {
        if *lev.offset(c as isize) < *nlevels.offset(c as isize) - 1 as libc::c_int {
            break;
        }
        c -= 1;
        c;
    }
    if c < 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    let ref mut fresh41 = *lev.offset(c as isize);
    *fresh41 += 1;
    *fresh41;
    loop {
        c += 1;
        if !(c < nclass) {
            break;
        }
        *lev.offset(c as isize) = 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn dataset(
    mut oldname: *mut libc::c_char,
    mut newname: *mut libc::c_char,
    mut action: *mut libc::c_char,
) {
    static mut datsetinit: libc::c_int = 0 as libc::c_int;
    let mut dold: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dnew: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fold: libc::c_int = 0;
    let mut fnew: libc::c_int = 0;
    let mut doldf: *mut DFILE = 0 as *mut DFILE;
    let mut dnewf: *mut DFILE = 0 as *mut DFILE;
    let mut c: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut onum: libc::c_int = 0;
    let mut maxnamlen: libc::c_int = 0;
    static mut baseobs: dataobs = dataobs {
        do_int: 0 as *const libc::c_int as *mut libc::c_int,
        do_il: 0 as *const *mut libc::c_int as *mut *mut libc::c_int,
        do_dbl: 0 as *const libc::c_double as *mut libc::c_double,
        do_dl: 0 as *const *mut libc::c_double as *mut *mut libc::c_double,
        do_str: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        do_sl: 0 as *const libc::c_int as *mut libc::c_int,
        do_nam: 0 as *const *mut libc::c_char as *mut *mut libc::c_char,
        do_len: 0 as *const libc::c_int as *mut libc::c_int,
        do_in: 0 as *const libc::c_int as *mut libc::c_int,
        do_out: 0 as *const libc::c_int as *mut libc::c_int,
        do_ivar: 0,
        do_ovar: 0,
        do_nvar: 0,
        do_valid: 0,
    };
    let mut varspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldvmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newvmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut oldvar: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut newvar: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut nvar: libc::c_int = 0;
    let mut ncell: libc::c_int = 0;
    let mut nclass: libc::c_int = 0;
    let mut celllist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cellv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut classv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut slevelmem: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut slevel: *mut *mut *mut libc::c_char = 0 as *mut *mut *mut libc::c_char;
    let mut dlevelmem: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut dlevel: *mut *mut libc::c_double = 0 as *mut *mut libc::c_double;
    let mut ilevelmem: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ilevel: *mut *mut libc::c_int = 0 as *mut *mut libc::c_int;
    let mut inlev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut outlev: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nlevels: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut dcmp: Option::<unsafe extern "C" fn() -> libc::c_int> = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_double, *mut libc::c_double) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            dblcmp
                as unsafe extern "C" fn(
                    *mut libc::c_double,
                    *mut libc::c_double,
                ) -> libc::c_int,
        ),
    );
    let mut icmp: Option::<unsafe extern "C" fn() -> libc::c_int> = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            intcmp
                as unsafe extern "C" fn(
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
    );
    let mut scmp: Option::<unsafe extern "C" fn() -> libc::c_int> = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut *mut libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            stcmp
                as unsafe extern "C" fn(
                    *mut *mut libc::c_char,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
    );
    let mut vn: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut ndim: libc::c_int = 0;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut dimstr: [libc::c_char; 7] = [0; 7];
    let mut clearvar: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nclear: libc::c_int = 0;
    if datsetinit == 0 {
        datsetinit = 1 as libc::c_int;
        initdo(&mut baseobs);
    }
    if !(dap_in[0 as libc::c_int as usize]).is_null() {
        dfclose(dap_in[0 as libc::c_int as usize]);
        dap_in[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if !(dap_out[0 as libc::c_int as usize]).is_null() {
        dfclose(dap_out[0 as libc::c_int as usize]);
        dap_out[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    celllist = 0 as *mut libc::c_void as *mut libc::c_char;
    slevelmem = 0 as *mut libc::c_void as *mut *mut libc::c_char;
    cellv = 0 as *mut libc::c_void as *mut libc::c_int;
    classv = 0 as *mut libc::c_void as *mut libc::c_int;
    outlist = dap_malloc(
        dap_listlen,
        b"dataset: outlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    oldvmem = dap_malloc(
        dap_maxvar * (dap_namelen + 1 as libc::c_int),
        b"dataset: oldvmem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    oldvar = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dataset: oldvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    v = 0 as libc::c_int;
    while v < dap_maxvar {
        let ref mut fresh42 = *oldvar.offset(v as isize);
        *fresh42 = oldvmem.offset((v * (dap_namelen + 1 as libc::c_int)) as isize);
        v += 1;
        v;
    }
    newvmem = dap_malloc(
        dap_maxvar * (dap_namelen + 1 as libc::c_int),
        b"dataset: newvmem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    newvar = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dataset: newvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    v = 0 as libc::c_int;
    while v < dap_maxvar {
        let ref mut fresh43 = *newvar.offset(v as isize);
        *fresh43 = newvmem.offset((v * (dap_namelen + 1 as libc::c_int)) as isize);
        v += 1;
        v;
    }
    dold = dap_malloc(
        (strlen(oldname as *const libc::c_char))
            .wrapping_add(strlen(dap_setdir))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"dataset: dold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dnew = dap_malloc(
        (strlen(newname as *const libc::c_char))
            .wrapping_add(strlen(dap_setdir))
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"dataset: dnew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_name(dold, oldname);
    dap_name(dnew, newname);
    clearvar = 0 as *mut libc::c_int;
    if strcmp(
        action as *const libc::c_char,
        b"RENAME\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if *oldname.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
            if *newname.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
                fold = 0 as libc::c_int;
                while fold < dap_nrfiles {
                    if strcmp(
                        (*dfile.offset((4 as libc::c_int + fold) as isize)).dfile_name,
                        oldname.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    ) == 0
                    {
                        break;
                    }
                    fold += 1;
                    fold;
                }
                if fold < dap_nrfiles {
                    fnew = 0 as libc::c_int;
                    while fnew < dap_nrfiles {
                        if strcmp(
                            (*dfile.offset((4 as libc::c_int + fnew) as isize))
                                .dfile_name,
                            newname.offset(1 as libc::c_int as isize)
                                as *const libc::c_char,
                        ) == 0
                        {
                            break;
                        }
                        fnew += 1;
                        fnew;
                    }
                    if fnew < dap_nrfiles {
                        *((*dfile.offset((4 as libc::c_int + fnew) as isize)).dfile_name)
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    strcpy(
                        (*dfile.offset((4 as libc::c_int + fold) as isize)).dfile_name,
                        newname.offset(1 as libc::c_int as isize) as *const libc::c_char,
                    );
                } else {
                    fprintf(
                        dap_err,
                        b"(dataset) can't find ramfile %s\n\0" as *const u8
                            as *const libc::c_char,
                        oldname,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                fprintf(
                    dap_err,
                    b"(dataset) can't rename ramfile %s to disk file %s\n\0" as *const u8
                        as *const libc::c_char,
                    oldname,
                    newname,
                );
                exit(1 as libc::c_int);
            }
        } else if *newname.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32
        {
            fprintf(
                dap_err,
                b"(dataset) can't rename disk file %s to ramfile %s\n\0" as *const u8
                    as *const libc::c_char,
                oldname,
                newname,
            );
            exit(1 as libc::c_int);
        } else {
            rename(dold, dnew);
        }
    } else if strncmp(
        action as *const libc::c_char,
        b"COPY\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        v = 4 as libc::c_int;
        while *action.offset(v as isize) as libc::c_int == ' ' as i32 {
            v += 1;
            v;
        }
        nvar = 0 as libc::c_int;
        while *action.offset(v as isize) != 0 {
            c = 0 as libc::c_int;
            while *action.offset(v as isize) as libc::c_int != 0
                && *action.offset(v as isize) as libc::c_int != ' ' as i32
                && *action.offset(v as isize) as libc::c_int != '>' as i32
            {
                if c < dap_namelen {
                    let fresh44 = v;
                    v = v + 1;
                    let fresh45 = c;
                    c = c + 1;
                    *(*oldvar.offset(nvar as isize))
                        .offset(fresh45 as isize) = *action.offset(fresh44 as isize);
                } else {
                    fprintf(
                        dap_err,
                        b"(dataset) variable name too long: %s\n\0" as *const u8
                            as *const libc::c_char,
                        action,
                    );
                    exit(1 as libc::c_int);
                }
            }
            *(*oldvar.offset(nvar as isize))
                .offset(c as isize) = '\0' as i32 as libc::c_char;
            while *action.offset(v as isize) as libc::c_int == ' ' as i32 {
                v += 1;
                v;
            }
            if *action.offset(v as isize) as libc::c_int == '>' as i32 {
                v += 1;
                v;
                while *action.offset(v as isize) as libc::c_int == ' ' as i32 {
                    v += 1;
                    v;
                }
                c = 0 as libc::c_int;
                while *action.offset(v as isize) as libc::c_int != 0
                    && *action.offset(v as isize) as libc::c_int != ' ' as i32
                {
                    if c < dap_namelen {
                        let fresh46 = v;
                        v = v + 1;
                        let fresh47 = c;
                        c = c + 1;
                        *(*newvar.offset(nvar as isize))
                            .offset(fresh47 as isize) = *action.offset(fresh46 as isize);
                    } else {
                        fprintf(
                            dap_err,
                            b"(dataset) new variable name too long: %s\n\0" as *const u8
                                as *const libc::c_char,
                            action,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                *(*newvar.offset(nvar as isize))
                    .offset(c as isize) = '\0' as i32 as libc::c_char;
                while *action.offset(v as isize) as libc::c_int == ' ' as i32 {
                    v += 1;
                    v;
                }
            } else {
                strcpy(*newvar.offset(nvar as isize), *oldvar.offset(nvar as isize));
            }
            nvar += 1;
            nvar;
        }
        if nvar != 0 {
            inset(oldname);
            v = 0 as libc::c_int;
            *outlist.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
            while v < nvar {
                if v != 0 {
                    strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
                }
                strcat(outlist, *newvar.offset(v as isize));
                if strcmp(*oldvar.offset(v as isize), *newvar.offset(v as isize)) != 0 {
                    vn = dap_varnum(*oldvar.offset(v as isize));
                    if vn >= 0 as libc::c_int {
                        nv = dap_varnum(*newvar.offset(v as isize));
                        if nv >= 0 as libc::c_int {
                            *(*(dap_obs[0 as libc::c_int as usize].do_nam)
                                .offset(nv as isize))
                                .offset(
                                    0 as libc::c_int as isize,
                                ) = '0' as i32 as libc::c_char;
                        }
                        dap_free(
                            *(dap_obs[0 as libc::c_int as usize].do_nam)
                                .offset(vn as isize) as *mut libc::c_void,
                            b"dataset: dap_obs[0].do_nam[vn]\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        let ref mut fresh48 = *(dap_obs[0 as libc::c_int as usize]
                            .do_nam)
                            .offset(vn as isize);
                        *fresh48 = dap_malloc(
                            (strlen(*newvar.offset(v as isize)))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                            b"dataset: dap_obs[0].do_nam[vn]\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        strcpy(
                            *(dap_obs[0 as libc::c_int as usize].do_nam)
                                .offset(vn as isize),
                            *newvar.offset(v as isize),
                        );
                    } else {
                        vn = dap_arrnum(*oldvar.offset(v as isize), &mut dim);
                        if vn >= 0 as libc::c_int {
                            nv = dap_arrnum(*newvar.offset(v as isize), &mut ndim);
                            if nv >= 0 as libc::c_int {
                                c = 0 as libc::c_int;
                                while c < ndim {
                                    *(*(dap_obs[0 as libc::c_int as usize].do_nam)
                                        .offset((nv + c) as isize))
                                        .offset(
                                            0 as libc::c_int as isize,
                                        ) = '0' as i32 as libc::c_char;
                                    sprintf(
                                        dimstr.as_mut_ptr(),
                                        b"[%d]\0" as *const u8 as *const libc::c_char,
                                        c,
                                    );
                                    strcat(
                                        *(dap_obs[0 as libc::c_int as usize].do_nam)
                                            .offset((vn + c) as isize),
                                        dimstr.as_mut_ptr(),
                                    );
                                    c += 1;
                                    c;
                                }
                            }
                            c = 0 as libc::c_int;
                            while c < dim {
                                sprintf(
                                    dimstr.as_mut_ptr(),
                                    b"[%d]\0" as *const u8 as *const libc::c_char,
                                    c,
                                );
                                dap_free(
                                    *(dap_obs[0 as libc::c_int as usize].do_nam)
                                        .offset((vn + c) as isize) as *mut libc::c_void,
                                    b"dataset: dap_obs[0].do_nam[vn + c]\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                let ref mut fresh49 = *(dap_obs[0 as libc::c_int as usize]
                                    .do_nam)
                                    .offset((vn + c) as isize);
                                *fresh49 = dap_malloc(
                                    (strlen(*newvar.offset(v as isize)))
                                        .wrapping_add(strlen(dimstr.as_mut_ptr()))
                                        .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                        as libc::c_int,
                                    b"dataset: dap_obs[0].do_nam[vn + c]\0" as *const u8
                                        as *const libc::c_char as *mut libc::c_char,
                                );
                                strcpy(
                                    *(dap_obs[0 as libc::c_int as usize].do_nam)
                                        .offset((vn + c) as isize),
                                    *newvar.offset(v as isize),
                                );
                                strcat(
                                    *(dap_obs[0 as libc::c_int as usize].do_nam)
                                        .offset((vn + c) as isize),
                                    dimstr.as_mut_ptr(),
                                );
                                c += 1;
                                c;
                            }
                        } else {
                            fprintf(
                                dap_err,
                                b"(dataset) unknown variable %s\n\0" as *const u8
                                    as *const libc::c_char,
                                *oldvar.offset(v as isize),
                            );
                            exit(1 as libc::c_int);
                        }
                    }
                }
                v += 1;
                v;
            }
            outset(newname, outlist);
            while step() != 0 {
                output();
            }
        } else {
            doldf = dfopen(
                oldname,
                b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if doldf.is_null() {
                fprintf(
                    dap_err,
                    b"(dataset) can't read %s for copy.\n\0" as *const u8
                        as *const libc::c_char,
                    oldname,
                );
                exit(1 as libc::c_int);
            }
            dnewf = dfopen(
                newname,
                b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if dnewf.is_null() {
                fprintf(
                    dap_err,
                    b"(dataset) can't write %s for copy.\n\0" as *const u8
                        as *const libc::c_char,
                    newname,
                );
                exit(1 as libc::c_int);
            }
            loop {
                c = dgetc(doldf);
                if !(c != -(1 as libc::c_int)) {
                    break;
                }
                dap_putc(c, dnewf);
            }
            dfclose(dnewf);
            dfclose(doldf);
        }
    } else if strncmp(
        action as *const libc::c_char,
        b"FILL\0" as *const u8 as *const libc::c_char,
        4 as libc::c_int as libc::c_ulong,
    ) == 0
    {
        inset(oldname);
        celllist = dap_malloc(
            strlen(action as *const libc::c_char) as libc::c_int,
            b"dataset: celllist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        v = 4 as libc::c_int;
        c = 0 as libc::c_int;
        while *action.offset(v as isize) as libc::c_int != 0
            && *action.offset(v as isize) as libc::c_int != ':' as i32
        {
            let fresh50 = v;
            v = v + 1;
            let fresh51 = c;
            c = c + 1;
            *celllist.offset(fresh51 as isize) = *action.offset(fresh50 as isize);
        }
        *celllist.offset(c as isize) = '\0' as i32 as libc::c_char;
        if *action.offset(v as isize) == 0 {
            fprintf(
                dap_err,
                b"(dataset) missing ':' between variable lists in %s\n\0" as *const u8
                    as *const libc::c_char,
                action,
            );
            exit(1 as libc::c_int);
        }
        ncell = c / 2 as libc::c_int;
        cellv = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(ncell as libc::c_ulong) as libc::c_int,
            b"dataset: cellv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        ncell = dap_list(celllist, cellv, ncell);
        nclass = (strlen(action as *const libc::c_char))
            .wrapping_sub(v as libc::c_ulong)
            .wrapping_div(2 as libc::c_int as libc::c_ulong) as libc::c_int;
        classv = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: classv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        nclass = dap_list(
            action.offset(v as isize).offset(1 as libc::c_int as isize),
            classv,
            nclass,
        );
        inlev = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: inlev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        outlev = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: outlev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        slevelmem = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong)
                .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
            b"dataset: slevelmem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        slevel = dap_malloc(
            (::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: slevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut *mut libc::c_char;
        dlevelmem = dap_malloc(
            (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong)
                .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
            b"dataset: dlevelmem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as *mut libc::c_double;
        dlevel = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: dlevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_double;
        ilevelmem = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong)
                .wrapping_mul(dap_maxlev as libc::c_ulong) as libc::c_int,
            b"dataset: ilevelmem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as *mut libc::c_int;
        ilevel = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: ilevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_int;
        nlevels = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(nclass as libc::c_ulong) as libc::c_int,
            b"dataset: nlevels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        ) as *mut libc::c_int;
        c = 0 as libc::c_int;
        while c < nclass {
            let ref mut fresh52 = *slevel.offset(c as isize);
            *fresh52 = slevelmem.offset((c * dap_maxlev) as isize);
            let ref mut fresh53 = *dlevel.offset(c as isize);
            *fresh53 = dlevelmem.offset((c * dap_maxlev) as isize);
            let ref mut fresh54 = *ilevel.offset(c as isize);
            *fresh54 = ilevelmem.offset((c * dap_maxlev) as isize);
            c += 1;
            c;
        }
        outset(newname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
        c = 0 as libc::c_int;
        while c < nclass {
            *nlevels.offset(c as isize) = 0 as libc::c_int;
            c += 1;
            c;
        }
        dap_mark();
        while step() != 0 {
            c = 0 as libc::c_int;
            while c < nclass {
                v = findlev(
                    *classv.offset(c as isize),
                    *dlevel.offset(c as isize),
                    *ilevel.offset(c as isize),
                    *slevel.offset(c as isize),
                    *nlevels.offset(c as isize),
                );
                if v < 0 as libc::c_int {
                    if *nlevels.offset(c as isize) < dap_maxlev {
                        if *(dap_obs[dap_ono as usize].do_len)
                            .offset(*classv.offset(c as isize) as isize)
                            == -(1 as libc::c_int)
                        {
                            let ref mut fresh55 = *nlevels.offset(c as isize);
                            let fresh56 = *fresh55;
                            *fresh55 = *fresh55 + 1;
                            *(*dlevel.offset(c as isize))
                                .offset(
                                    fresh56 as isize,
                                ) = *(dap_obs[dap_ono as usize].do_dbl)
                                .offset(*classv.offset(c as isize) as isize);
                        } else if *(dap_obs[dap_ono as usize].do_len)
                            .offset(*classv.offset(c as isize) as isize)
                            == 0 as libc::c_int
                        {
                            let ref mut fresh57 = *nlevels.offset(c as isize);
                            let fresh58 = *fresh57;
                            *fresh57 = *fresh57 + 1;
                            *(*ilevel.offset(c as isize))
                                .offset(
                                    fresh58 as isize,
                                ) = *(dap_obs[dap_ono as usize].do_int)
                                .offset(*classv.offset(c as isize) as isize);
                        } else {
                            let ref mut fresh59 = *(*slevel.offset(c as isize))
                                .offset(*nlevels.offset(c as isize) as isize);
                            *fresh59 = dap_malloc(
                                (strlen(
                                    *(dap_obs[dap_ono as usize].do_str)
                                        .offset(*classv.offset(c as isize) as isize),
                                ))
                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                    as libc::c_int,
                                b"dataset: slevel[c][nlevels[c]]\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                            let ref mut fresh60 = *nlevels.offset(c as isize);
                            let fresh61 = *fresh60;
                            *fresh60 = *fresh60 + 1;
                            strcpy(
                                *(*slevel.offset(c as isize)).offset(fresh61 as isize),
                                *(dap_obs[dap_ono as usize].do_str)
                                    .offset(*classv.offset(c as isize) as isize),
                            );
                        }
                    } else {
                        fprintf(
                            dap_err,
                            b"(dataset) too many levels for %s\n\0" as *const u8
                                as *const libc::c_char,
                            *(dap_obs[dap_ono as usize].do_nam)
                                .offset(*classv.offset(c as isize) as isize),
                        );
                        exit(1 as libc::c_int);
                    }
                }
                c += 1;
                c;
            }
        }
        c = 0 as libc::c_int;
        while c < nclass {
            if *(dap_obs[dap_ono as usize].do_len)
                .offset(*classv.offset(c as isize) as isize) == -(1 as libc::c_int)
            {
                qsort(
                    *dlevel.offset(c as isize) as *mut libc::c_void,
                    *nlevels.offset(c as isize) as size_t,
                    ::std::mem::size_of::<libc::c_double>() as libc::c_ulong,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> libc::c_int>,
                        __compar_fn_t,
                    >(dcmp),
                );
            } else if *(dap_obs[dap_ono as usize].do_len)
                .offset(*classv.offset(c as isize) as isize) == 0 as libc::c_int
            {
                qsort(
                    *ilevel.offset(c as isize) as *mut libc::c_void,
                    *nlevels.offset(c as isize) as size_t,
                    ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> libc::c_int>,
                        __compar_fn_t,
                    >(icmp),
                );
            } else {
                qsort(
                    *slevel.offset(c as isize) as *mut libc::c_void,
                    *nlevels.offset(c as isize) as size_t,
                    ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
                    ::std::mem::transmute::<
                        Option::<unsafe extern "C" fn() -> libc::c_int>,
                        __compar_fn_t,
                    >(scmp),
                );
            }
            *outlev.offset(c as isize) = 0 as libc::c_int;
            c += 1;
            c;
        }
        dap_rewind();
        while step() != 0 {
            c = 0 as libc::c_int;
            while c < nclass {
                *inlev
                    .offset(
                        c as isize,
                    ) = findlev(
                    *classv.offset(c as isize),
                    *dlevel.offset(c as isize),
                    *ilevel.offset(c as isize),
                    *slevel.offset(c as isize),
                    *nlevels.offset(c as isize),
                );
                c += 1;
                c;
            }
            c = 0 as libc::c_int;
            while c < nclass {
                if *outlev.offset(c as isize) < *inlev.offset(c as isize) {
                    break;
                }
                c += 1;
                c;
            }
            if c < nclass {
                dap_save();
                v = 0 as libc::c_int;
                while v < ncell {
                    if *(dap_obs[dap_ono as usize].do_len)
                        .offset(*cellv.offset(v as isize) as isize)
                        == -(1 as libc::c_int)
                    {
                        *(dap_obs[dap_ono as usize].do_dbl)
                            .offset(*cellv.offset(v as isize) as isize) = 0.0f64;
                    } else if *(dap_obs[dap_ono as usize].do_len)
                        .offset(*cellv.offset(v as isize) as isize) == 0 as libc::c_int
                    {
                        *(dap_obs[dap_ono as usize].do_int)
                            .offset(
                                *cellv.offset(v as isize) as isize,
                            ) = 0 as libc::c_int;
                    } else {
                        *(*(dap_obs[dap_ono as usize].do_str)
                            .offset(*cellv.offset(v as isize) as isize))
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    v += 1;
                    v;
                }
                loop {
                    c = 0 as libc::c_int;
                    while c < nclass {
                        if *outlev.offset(c as isize) < *inlev.offset(c as isize) {
                            break;
                        }
                        c += 1;
                        c;
                    }
                    if c == nclass {
                        break;
                    }
                    v = c;
                    while v < nclass {
                        if *(dap_obs[dap_ono as usize].do_len)
                            .offset(*classv.offset(v as isize) as isize)
                            == -(1 as libc::c_int)
                        {
                            *(dap_obs[dap_ono as usize].do_dbl)
                                .offset(
                                    *classv.offset(v as isize) as isize,
                                ) = *(*dlevel.offset(v as isize))
                                .offset(*outlev.offset(v as isize) as isize);
                        } else if *(dap_obs[dap_ono as usize].do_len)
                            .offset(*classv.offset(v as isize) as isize)
                            == 0 as libc::c_int
                        {
                            *(dap_obs[dap_ono as usize].do_int)
                                .offset(
                                    *classv.offset(v as isize) as isize,
                                ) = *(*ilevel.offset(v as isize))
                                .offset(*outlev.offset(v as isize) as isize);
                        } else {
                            strcpy(
                                *(dap_obs[dap_ono as usize].do_str)
                                    .offset(*classv.offset(v as isize) as isize),
                                *(*slevel.offset(v as isize))
                                    .offset(*outlev.offset(v as isize) as isize),
                            );
                        }
                        v += 1;
                        v;
                    }
                    output();
                    if !(inclev(outlev, nlevels, nclass) != 0) {
                        break;
                    }
                }
                dap_rest();
            }
            output();
            inclev(outlev, nlevels, nclass);
        }
        c = 0 as libc::c_int;
        while c < nclass {
            v = 0 as libc::c_int;
            while v < *nlevels.offset(c as isize) {
                if *(dap_obs[dap_ono as usize].do_len)
                    .offset(*classv.offset(c as isize) as isize) > 0 as libc::c_int
                {
                    dap_free(
                        *(*slevel.offset(c as isize)).offset(v as isize)
                            as *mut libc::c_void,
                        b"dataset: slevel[c][v]\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                    );
                }
                v += 1;
                v;
            }
            *nlevels.offset(c as isize) = 0 as libc::c_int;
            c += 1;
            c;
        }
    } else if strcmp(
        action as *const libc::c_char,
        b"REMOVE\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if *oldname.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
            fold = 0 as libc::c_int;
            while fold < dap_nrfiles {
                if strcmp(
                    (*dfile.offset((4 as libc::c_int + fold) as isize)).dfile_name,
                    oldname.offset(1 as libc::c_int as isize) as *const libc::c_char,
                ) == 0
                {
                    dap_free(
                        (*dfile.offset((4 as libc::c_int + fold) as isize)).dfile_name
                            as *mut libc::c_void,
                        b"dataset: dfile[NDFILES + fold].dfile_name\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    let ref mut fresh62 = (*dfile
                        .offset((4 as libc::c_int + fold) as isize))
                        .dfile_name;
                    *fresh62 = 0 as *mut libc::c_char;
                    dap_free(
                        (*rfile.offset(fold as isize)).rfile_str as *mut libc::c_void,
                        b"dataset: rfile[fold].rfile_str\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    break;
                } else {
                    fold += 1;
                    fold;
                }
            }
        } else {
            unlink(dold);
        }
    } else if strcmp(
        action as *const libc::c_char,
        b"APPEND\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        dap_out[0 as libc::c_int
            as usize] = dfopen(
            newname,
            b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if (dap_out[0 as libc::c_int as usize]).is_null() {
            dap_out[0 as libc::c_int
                as usize] = dfopen(
                newname,
                b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if (dap_out[0 as libc::c_int as usize]).is_null() {
                fprintf(
                    dap_err,
                    b"(dataset) Can't create new data set for append: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    newname,
                );
                exit(1 as libc::c_int);
            }
            dap_in[0 as libc::c_int
                as usize] = dfopen(
                oldname,
                b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if (dap_in[0 as libc::c_int as usize]).is_null() {
                fprintf(
                    dap_err,
                    b"(dataset) can't read old data set for append: %s\n\0" as *const u8
                        as *const libc::c_char,
                    oldname,
                );
                exit(1 as libc::c_int);
            }
            loop {
                c = dgetc(dap_in[0 as libc::c_int as usize]);
                if !(c != -(1 as libc::c_int)) {
                    break;
                }
                dap_putc(c, dap_out[0 as libc::c_int as usize]);
            }
            dfclose(dap_in[0 as libc::c_int as usize]);
            dap_in[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
            dfclose(dap_out[0 as libc::c_int as usize]);
            dap_out[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
            return;
        } else {
            inset(newname);
            outset(
                b"dap_null\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            baseobs.do_nvar = dap_obs[0 as libc::c_int as usize].do_nvar;
            v = 0 as libc::c_int;
            while v < baseobs.do_nvar {
                *(baseobs.do_len)
                    .offset(
                        v as isize,
                    ) = *(dap_obs[0 as libc::c_int as usize].do_len).offset(v as isize);
                let ref mut fresh63 = *(baseobs.do_nam).offset(v as isize);
                *fresh63 = dap_malloc(
                    (strlen(
                        *(dap_obs[0 as libc::c_int as usize].do_nam).offset(v as isize),
                    ))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                    b"dataset: baseobs.do_nam[v]\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                strcpy(
                    *(baseobs.do_nam).offset(v as isize),
                    *(dap_obs[0 as libc::c_int as usize].do_nam).offset(v as isize),
                );
                v += 1;
                v;
            }
            dfclose(dap_out[0 as libc::c_int as usize]);
            dap_out[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
            inset(oldname);
            dap_obs[0 as libc::c_int as usize].do_ovar = 0 as libc::c_int;
            v = 0 as libc::c_int;
            maxnamlen = 0 as libc::c_int;
            while v < baseobs.do_nvar {
                if (maxnamlen as libc::c_ulong)
                    < strlen(*(baseobs.do_nam).offset(v as isize))
                {
                    maxnamlen = strlen(*(baseobs.do_nam).offset(v as isize))
                        as libc::c_int;
                }
                v += 1;
                v;
            }
            varspec = dap_malloc(
                maxnamlen + dap_intlen + 2 as libc::c_int,
                b"dataset: varspec\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
            clearvar = dap_malloc(
                (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                    .wrapping_mul(baseobs.do_nvar as libc::c_ulong) as libc::c_int,
                b"dataset: clearvar\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ) as *mut libc::c_int;
            v = 0 as libc::c_int;
            nclear = 0 as libc::c_int;
            while v < baseobs.do_nvar {
                onum = dap_varnum(*(baseobs.do_nam).offset(v as isize));
                if onum < 0 as libc::c_int {
                    sprintf(
                        varspec,
                        b"%s %d\0" as *const u8 as *const libc::c_char,
                        *(baseobs.do_nam).offset(v as isize),
                        *(baseobs.do_len).offset(v as isize),
                    );
                    onum = dap_vd(varspec, 0 as libc::c_int);
                    let fresh64 = nclear;
                    nclear = nclear + 1;
                    *clearvar.offset(fresh64 as isize) = onum;
                } else if *(dap_obs[0 as libc::c_int as usize].do_len)
                    .offset(onum as isize) != *(baseobs.do_len).offset(v as isize)
                {
                    fprintf(
                        stderr,
                        b"(dataset) variable %s has different lengths (%d appended to %d) in datasets\n\0"
                            as *const u8 as *const libc::c_char,
                        *(baseobs.do_nam).offset(v as isize),
                        *(dap_obs[0 as libc::c_int as usize].do_len)
                            .offset(onum as isize),
                        *(baseobs.do_len).offset(v as isize),
                    );
                    exit(1 as libc::c_int);
                }
                let fresh65 = dap_obs[0 as libc::c_int as usize].do_ovar;
                dap_obs[0 as libc::c_int as usize]
                    .do_ovar = dap_obs[0 as libc::c_int as usize].do_ovar + 1;
                *(dap_obs[0 as libc::c_int as usize].do_out)
                    .offset(fresh65 as isize) = onum;
                v += 1;
                v;
            }
            dap_out[0 as libc::c_int
                as usize] = dfopen(
                newname,
                b"a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            if (dap_out[0 as libc::c_int as usize]).is_null() {
                fprintf(
                    dap_err,
                    b"(dataset) can't append to new data set: %s\n\0" as *const u8
                        as *const libc::c_char,
                    newname,
                );
                exit(1 as libc::c_int);
            }
            while step() != 0 {
                v = 0 as libc::c_int;
                while v < nclear {
                    if !(*(dap_obs[0 as libc::c_int as usize].do_str)
                        .offset(*clearvar.offset(v as isize) as isize))
                        .is_null()
                    {
                        *(*(dap_obs[0 as libc::c_int as usize].do_str)
                            .offset(*clearvar.offset(v as isize) as isize))
                            .offset(
                                0 as libc::c_int as isize,
                            ) = '\0' as i32 as libc::c_char;
                    }
                    *(dap_obs[0 as libc::c_int as usize].do_sl)
                        .offset(
                            *clearvar.offset(v as isize) as isize,
                        ) = 0 as libc::c_int;
                    *(dap_obs[0 as libc::c_int as usize].do_int)
                        .offset(
                            *clearvar.offset(v as isize) as isize,
                        ) = 0 as libc::c_int;
                    *(dap_obs[0 as libc::c_int as usize].do_dbl)
                        .offset(*clearvar.offset(v as isize) as isize) = 0.0f64;
                    v += 1;
                    v;
                }
                output();
            }
            v = 0 as libc::c_int;
            while v < baseobs.do_nvar {
                dap_free(
                    *(baseobs.do_nam).offset(v as isize) as *mut libc::c_void,
                    b"dataset: baseobs.do_nam[v]\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
                let ref mut fresh66 = *(baseobs.do_nam).offset(v as isize);
                *fresh66 = 0 as *mut libc::c_char;
                v += 1;
                v;
            }
            dap_free(
                varspec as *mut libc::c_void,
                b"dataset: varspec\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
    } else {
        fprintf(
            dap_err,
            b"(dataset) unknown action: %s\n\0" as *const u8 as *const libc::c_char,
            action,
        );
        exit(1 as libc::c_int);
    }
    dap_free(
        dold as *mut libc::c_void,
        b"dataset: dold\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        dnew as *mut libc::c_void,
        b"dataset: dnew\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        oldvmem as *mut libc::c_void,
        b"dataset: oldvmem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        oldvar as *mut libc::c_void,
        b"dataset: oldvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        newvmem as *mut libc::c_void,
        b"dataset: newvmem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        newvar as *mut libc::c_void,
        b"dataset: newvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
        b"dataset: outlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !clearvar.is_null() {
        dap_free(
            clearvar as *mut libc::c_void,
            b"dataset: clearvar\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !celllist.is_null() {
        dap_free(
            celllist as *mut libc::c_void,
            b"dataset: celllist\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
    if !cellv.is_null() {
        dap_free(
            cellv as *mut libc::c_void,
            b"dataset: cellv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !classv.is_null() {
        dap_free(
            classv as *mut libc::c_void,
            b"dataset: classv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !slevelmem.is_null() {
        dap_free(
            slevelmem as *mut libc::c_void,
            b"dataset: slevelmem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        dap_free(
            slevel as *mut libc::c_void,
            b"dataset: slevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            dlevelmem as *mut libc::c_void,
            b"dataset: dlevelmem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        dap_free(
            dlevel as *mut libc::c_void,
            b"dataset: dlevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            ilevelmem as *mut libc::c_void,
            b"dataset: ilevelmem\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        dap_free(
            ilevel as *mut libc::c_void,
            b"dataset: ilevel\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            inlev as *mut libc::c_void,
            b"dataset: inlev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            outlev as *mut libc::c_void,
            b"dataset: outlev\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            nlevels as *mut libc::c_void,
            b"dataset: nlevels\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
    }
}
unsafe extern "C" fn getblock(mut fp: *mut DFILE) -> libc::c_int {
    let mut block: [libc::c_char; 32] = [0; 32];
    let mut b: libc::c_int = 0;
    block[0 as libc::c_int as usize] = dgetc(fp) as libc::c_char;
    if block[0 as libc::c_int as usize] as libc::c_int == '\r' as i32 {
        return 0 as libc::c_int;
    }
    b = 1 as libc::c_int;
    while b < 32 as libc::c_int {
        block[b as usize] = dgetc(fp) as libc::c_char;
        b += 1;
        b;
    }
    return block[16 as libc::c_int as usize] as libc::c_int & 0xff as libc::c_int;
}
pub unsafe extern "C" fn dap_clearobs(mut varspec: *mut libc::c_char) -> libc::c_int {
    let mut v: libc::c_int = 0;
    if dap_ono < 2 as libc::c_int {
        dap_prev[dap_ono as usize].do_ivar = 0 as libc::c_int;
        eof[dap_ono as usize] = 0 as libc::c_int;
        v = 0 as libc::c_int;
        while v < dap_maxvar {
            if !(*(dap_prev[dap_ono as usize].do_str).offset(v as isize)).is_null() {
                dap_free(
                    *(dap_prev[dap_ono as usize].do_str).offset(v as isize)
                        as *mut libc::c_void,
                    b"clearobs: dap_prev[dap_ono].do_str[v]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            let ref mut fresh67 = *(dap_prev[dap_ono as usize].do_str)
                .offset(v as isize);
            *fresh67 = 0 as *mut libc::c_char;
            v += 1;
            v;
        }
    }
    dap_obs[dap_ono as usize].do_ivar = 0 as libc::c_int;
    dap_obs[dap_ono as usize].do_ovar = 0 as libc::c_int;
    dap_obs[dap_ono as usize].do_nvar = 0 as libc::c_int;
    dap_obs[dap_ono as usize].do_valid = 0 as libc::c_int;
    v = 0 as libc::c_int;
    while v < dap_maxvar {
        if !(*(dap_obs[dap_ono as usize].do_nam).offset(v as isize)).is_null() {
            dap_free(
                *(dap_obs[dap_ono as usize].do_nam).offset(v as isize)
                    as *mut libc::c_void,
                b"clearobs: dap_obs[dap_ono].do_nam[v]\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        let ref mut fresh68 = *(dap_obs[dap_ono as usize].do_nam).offset(v as isize);
        *fresh68 = 0 as *mut libc::c_char;
        let ref mut fresh69 = *(dap_obs[dap_ono as usize].do_dl).offset(v as isize);
        *fresh69 = 0 as *mut libc::c_double;
        let ref mut fresh70 = *(dap_obs[dap_ono as usize].do_il).offset(v as isize);
        *fresh70 = 0 as *mut libc::c_int;
        if *(dap_obs[dap_ono as usize].do_sl).offset(v as isize) == 0
            && !(*(dap_obs[dap_ono as usize].do_str).offset(v as isize)).is_null()
        {
            dap_free(
                *(dap_obs[dap_ono as usize].do_str).offset(v as isize)
                    as *mut libc::c_void,
                b"clearobs: dap_obs[dap_ono].do_str[v]\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        let ref mut fresh71 = *(dap_obs[dap_ono as usize].do_str).offset(v as isize);
        *fresh71 = 0 as *mut libc::c_char;
        *(dap_obs[dap_ono as usize].do_sl).offset(v as isize) = 0 as libc::c_int;
        v += 1;
        v;
    }
    if !varspec.is_null() {
        dap_vd(varspec, 1 as libc::c_int);
    } else {
        dap_vd(
            b"_type_ 8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            0 as libc::c_int,
        );
    }
    v = dap_varnum(b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if v < 0 as libc::c_int {
        fputs(
            b"(clearobs) missing _type_ variable\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    let fresh72 = dap_obs[dap_ono as usize].do_ovar;
    dap_obs[dap_ono as usize].do_ovar = dap_obs[dap_ono as usize].do_ovar + 1;
    *(dap_obs[dap_ono as usize].do_out).offset(fresh72 as isize) = v;
    return v;
}
pub unsafe extern "C" fn infile(
    mut ifname: *mut libc::c_char,
    mut idelim: *mut libc::c_char,
) {
    static mut infinit: libc::c_int = 0 as libc::c_int;
    let mut v: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut infldlen: libc::c_int = 0;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut delim: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    static mut delimlen: libc::c_int = 0;
    if infinit == 0 {
        infinit = 1 as libc::c_int;
        delimlen = (dap_linelen + 1 as libc::c_int) / 8 as libc::c_int
            - 1 as libc::c_int;
        delim = dap_malloc(
            delimlen + 1 as libc::c_int,
            b"infile: delim\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        fieldwd = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"infile: fieldwd\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
    }
    if !(dap_in[dap_ono as usize]).is_null() {
        dfclose(dap_in[dap_ono as usize]);
        dap_in[dap_ono as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if !(dap_out[dap_ono as usize]).is_null() {
        dfclose(dap_out[dap_ono as usize]);
        dap_out[dap_ono as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if ifname.is_null() || *ifname.offset(0 as libc::c_int as isize) == 0 {
        fname = b"/dev/null\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        strcpy(delim, b"|\0" as *const u8 as *const libc::c_char);
    } else {
        fname = ifname;
        if idelim.is_null() || *idelim.offset(0 as libc::c_int as isize) == 0 {
            fprintf(
                dap_err,
                b"(infile) Delimiter string must be at least one character: %s\n\0"
                    as *const u8 as *const libc::c_char,
                delim,
            );
            exit(1 as libc::c_int);
        } else if strlen(idelim) <= delimlen as libc::c_ulong {
            strcpy(delim, idelim);
        } else {
            fprintf(
                dap_err,
                b"(infile) Delimiter string too long: %s\n\0" as *const u8
                    as *const libc::c_char,
                idelim,
            );
            exit(1 as libc::c_int);
        }
    }
    dap_in[dap_ono
        as usize] = dfopen(
        fname,
        b"rf\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (dap_in[dap_ono as usize]).is_null() {
        fprintf(
            dap_err,
            b"(infile) can't read data file: %s\n\0" as *const u8 as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    intype = 0 as libc::c_int;
    if strcmp(
        fname.offset(strlen(fname) as isize).offset(-(4 as libc::c_int as isize)),
        b".dbf\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        intype = 1 as libc::c_int;
        getblock(dap_in[dap_ono as usize]);
        inlen = 0 as libc::c_int;
        loop {
            infldlen = getblock(dap_in[dap_ono as usize]);
            if !(infldlen != 0) {
                break;
            }
            inlen += infldlen;
        }
    }
    dap_delim = *delim.offset(0 as libc::c_int as isize) as libc::c_int;
    nfields = 0 as libc::c_int;
    d = 1 as libc::c_int;
    while *delim.offset(d as isize) != 0 {
        if nfields < dap_maxvar {
            *fieldwd.offset(nfields as isize) = 0 as libc::c_int;
            while *delim.offset(d as isize) as libc::c_int != 0
                && *delim.offset(d as isize) as libc::c_int != dap_delim
            {
                let fresh73 = d;
                d = d + 1;
                *fieldwd
                    .offset(
                        nfields as isize,
                    ) = 10 as libc::c_int * *fieldwd.offset(nfields as isize)
                    + *delim.offset(fresh73 as isize) as libc::c_int - '0' as i32;
            }
            if *delim.offset(d as isize) != 0 {
                d += 1;
                d;
            }
        } else {
            fputs(
                b"(infile) too many field width specifiers.\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        nfields += 1;
        nfields;
    }
    v = dap_clearobs(0 as *mut libc::c_void as *mut libc::c_char);
    strcpy(
        *(dap_obs[dap_ono as usize].do_str).offset(v as isize),
        b"OBS\0" as *const u8 as *const libc::c_char,
    );
    lineno[dap_ono as usize] = 1 as libc::c_int;
    toolong = 0 as libc::c_int;
}
pub unsafe extern "C" fn input(mut varlist: *mut libc::c_char) {
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    static mut vname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut dim: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    if vname.is_null() {
        vname = dap_malloc(
            dap_namelen + 1 as libc::c_int,
            b"input: vname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    l = 0 as libc::c_int;
    while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
        l += 1;
        l;
    }
    while *varlist.offset(l as isize) != 0 {
        i = 0 as libc::c_int;
        while *varlist.offset((l + i) as isize) as libc::c_int != 0
            && *varlist.offset((l + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *vname.offset(i as isize) = *varlist.offset((l + i) as isize);
            } else {
                *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(input) variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
        v = dap_varnum(vname);
        if v >= 0 as libc::c_int {
            let fresh74 = dap_obs[dap_ono as usize].do_ivar;
            dap_obs[dap_ono as usize].do_ivar = dap_obs[dap_ono as usize].do_ivar + 1;
            *(dap_obs[dap_ono as usize].do_in).offset(fresh74 as isize) = v;
        } else {
            v = dap_arrnum(vname, &mut dim);
            if v >= 0 as libc::c_int {
                d = 0 as libc::c_int;
                while d < dim {
                    let fresh75 = dap_obs[dap_ono as usize].do_ivar;
                    dap_obs[dap_ono as usize]
                        .do_ivar = dap_obs[dap_ono as usize].do_ivar + 1;
                    *(dap_obs[dap_ono as usize].do_in).offset(fresh75 as isize) = v + d;
                    d += 1;
                    d;
                }
            } else {
                fprintf(
                    dap_err,
                    b"(input) unknown variable: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
        }
        l += i;
        while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
    }
}
pub unsafe extern "C" fn inset(mut fname: *mut libc::c_char) {
    let mut v: libc::c_int = 0;
    static mut varspec: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    let mut testd_0: libc::c_double = 0.;
    if varspec.is_null() {
        varspec = dap_malloc(
            dap_linelen + 1 as libc::c_int,
            b"inset: varspec\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if !(dap_in[dap_ono as usize]).is_null() {
        dfclose(dap_in[dap_ono as usize]);
        dap_in[dap_ono as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if !(dap_out[dap_ono as usize]).is_null() {
        dfclose(dap_out[dap_ono as usize]);
        dap_out[dap_ono as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if fname.is_null() {
        return;
    }
    dap_in[dap_ono
        as usize] = dfopen(
        fname,
        b"r\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (dap_in[dap_ono as usize]).is_null() {
        fprintf(
            dap_err,
            b"(inset) can't read data set: %s\n\0" as *const u8 as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    intype = 2 as libc::c_int;
    dap_delim = '\0' as i32;
    if dap_getline(dap_in[dap_ono as usize], varspec) < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(inset) data set empty: %s\n\0" as *const u8 as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    nfields = 0 as libc::c_int;
    dap_clearobs(varspec);
    dap_delim = '|' as i32;
    lineno[dap_ono as usize] = 1 as libc::c_int;
}
unsafe extern "C" fn fixlist(
    mut varl: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
) {
    static mut fixinit: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut inbrack: libc::c_int = 0;
    static mut vname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut dim: libc::c_int = 0;
    static mut outv: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
    let mut l0: libc::c_int = 0;
    let mut l1: libc::c_int = 0;
    let mut f1: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut vn: libc::c_int = 0;
    if fixinit == 0 {
        fixinit = 1 as libc::c_int;
        vname = dap_malloc(
            dap_namelen + 1 as libc::c_int,
            b"fixlist: vname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        outv = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"fixlist: outv\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
    }
    if varl.is_null() {
        fputs(
            b"(fixlist) missing variable list.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if varlist.is_null() {
        fputs(
            b"(fixlist) missing string for fixed variable list.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    inbrack = 0 as libc::c_int;
    l = 0 as libc::c_int;
    f = 0 as libc::c_int;
    while *varl.offset(l as isize) != 0 {
        if inbrack != 0 {
            if *varl.offset(l as isize) as libc::c_int == ']' as i32 {
                inbrack = 0 as libc::c_int;
            }
        } else if *varl.offset(l as isize) as libc::c_int == '[' as i32 {
            inbrack = 1 as libc::c_int;
        } else {
            let fresh76 = f;
            f = f + 1;
            *varlist.offset(fresh76 as isize) = *varl.offset(l as isize);
        }
        l += 1;
        l;
    }
    *varlist.offset(f as isize) = '\0' as i32 as libc::c_char;
    l = 0 as libc::c_int;
    while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
        l += 1;
        l;
    }
    l0 = 0 as libc::c_int;
    if *varlist.offset(l as isize) as libc::c_int == '!' as i32 {
        l += 1;
        l;
        while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
        let fresh77 = l0;
        l0 = l0 + 1;
        *varlist.offset(fresh77 as isize) = '!' as i32 as libc::c_char;
    }
    nv = 0 as libc::c_int;
    l1 = l0;
    while *varlist.offset(l as isize) != 0 {
        f = 0 as libc::c_int;
        while *varlist.offset((l + f) as isize) as libc::c_int != 0
            && *varlist.offset((l + f) as isize) as libc::c_int != ' ' as i32
        {
            if f < dap_namelen {
                *vname.offset(f as isize) = *varlist.offset((l + f) as isize);
            } else {
                *vname.offset(f as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(fixlist) variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
            f += 1;
            f;
        }
        *vname.offset(f as isize) = '\0' as i32 as libc::c_char;
        v = dap_varnum(vname);
        if v < 0 as libc::c_int
            && {
                v = dap_arrnum(vname, &mut dim);
                v < 0 as libc::c_int
            }
        {
            fprintf(
                dap_err,
                b"(fixlist) unknown variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
            exit(1 as libc::c_int);
        }
        vn = 0 as libc::c_int;
        while vn < nv {
            if *outv.offset(vn as isize) == v {
                break;
            }
            vn += 1;
            vn;
        }
        if vn == nv {
            let fresh78 = nv;
            nv = nv + 1;
            *outv.offset(fresh78 as isize) = v;
            if l1 > l0 {
                let fresh79 = l1;
                l1 = l1 + 1;
                *varlist.offset(fresh79 as isize) = ' ' as i32 as libc::c_char;
            }
            f1 = 0 as libc::c_int;
            while f1 < f {
                let fresh80 = l;
                l = l + 1;
                let fresh81 = l1;
                l1 = l1 + 1;
                *varlist.offset(fresh81 as isize) = *varlist.offset(fresh80 as isize);
                f1 += 1;
                f1;
            }
        } else {
            l += f;
        }
        while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
    }
    *varlist.offset(l1 as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn outset(
    mut fname: *mut libc::c_char,
    mut varl: *mut libc::c_char,
) {
    static mut outinit: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    static mut varlist: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut vname: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut v: libc::c_int = 0;
    let mut w: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut testd_0: libc::c_double = 0.;
    if !(dap_out[dap_ono as usize]).is_null() {
        dfclose(dap_out[dap_ono as usize]);
        dap_out[dap_ono as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if fname.is_null() {
        fputs(
            b"(outset) no dataset name.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if outinit == 0 {
        outinit = 1 as libc::c_int;
        varlist = dap_malloc(
            dap_listlen,
            b"outset: varlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        vname = dap_malloc(
            dap_listlen,
            b"outset: vname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dap_out[dap_ono
        as usize] = dfopen(
        fname,
        b"w\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if (dap_out[dap_ono as usize]).is_null() {
        fprintf(
            dap_err,
            b"(outset) Can't write data set: %s\n\0" as *const u8 as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    if varl.is_null() {
        fprintf(
            dap_err,
            b"(outset (%s)) Missing variable list.\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    fixlist(varl, varlist);
    if *varlist.offset(0 as libc::c_int as isize) != 0 {
        l = 0 as libc::c_int;
        while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
        if *varlist.offset(l as isize) as libc::c_int == '!' as i32 {
            l += 1;
            l;
            while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
                l += 1;
                l;
            }
            dap_obs[dap_ono as usize].do_ovar = dap_obs[dap_ono as usize].do_nvar;
            v = 0 as libc::c_int;
            while v < dap_obs[dap_ono as usize].do_nvar {
                *(dap_obs[dap_ono as usize].do_out).offset(v as isize) = v;
                v += 1;
                v;
            }
            while *varlist.offset(l as isize) != 0 {
                i = 0 as libc::c_int;
                while *varlist.offset((l + i) as isize) as libc::c_int != 0
                    && *varlist.offset((l + i) as isize) as libc::c_int != ' ' as i32
                {
                    *vname.offset(i as isize) = *varlist.offset((l + i) as isize);
                    i += 1;
                    i;
                }
                *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
                w = dap_varnum(vname);
                if w >= 0 as libc::c_int {
                    v = 0 as libc::c_int;
                    while v < dap_obs[dap_ono as usize].do_ovar {
                        if *(dap_obs[dap_ono as usize].do_out).offset(v as isize) == w {
                            break;
                        }
                        v += 1;
                        v;
                    }
                    if v == dap_obs[dap_ono as usize].do_ovar {
                        fprintf(
                            dap_err,
                            b"(outset (%s)) variable not in list of variables to exclude: %s\n\0"
                                as *const u8 as *const libc::c_char,
                            fname,
                            vname,
                        );
                        exit(1 as libc::c_int);
                    }
                    while v < dap_obs[dap_ono as usize].do_ovar - 1 as libc::c_int {
                        *(dap_obs[dap_ono as usize].do_out)
                            .offset(
                                v as isize,
                            ) = *(dap_obs[dap_ono as usize].do_out)
                            .offset((v + 1 as libc::c_int) as isize);
                        v += 1;
                        v;
                    }
                    dap_obs[dap_ono as usize].do_ovar -= 1;
                    dap_obs[dap_ono as usize].do_ovar;
                } else {
                    w = dap_arrnum(vname, &mut dim);
                    if w >= 0 as libc::c_int {
                        v = 0 as libc::c_int;
                        while v < dap_obs[dap_ono as usize].do_ovar {
                            if *(dap_obs[dap_ono as usize].do_out).offset(v as isize)
                                == w
                            {
                                break;
                            }
                            v += 1;
                            v;
                        }
                        while v < dap_obs[dap_ono as usize].do_ovar - dim {
                            *(dap_obs[dap_ono as usize].do_out)
                                .offset(
                                    v as isize,
                                ) = *(dap_obs[dap_ono as usize].do_out)
                                .offset((v + dim) as isize);
                            v += 1;
                            v;
                        }
                        dap_obs[dap_ono as usize].do_ovar -= dim;
                    } else {
                        fprintf(
                            dap_err,
                            b"(outset(%s)) unknown variable: %s\n\0" as *const u8
                                as *const libc::c_char,
                            fname,
                            vname,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                l += i;
                while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
                    l += 1;
                    l;
                }
            }
        } else {
            while *varlist.offset(l as isize) != 0 {
                i = 0 as libc::c_int;
                while *varlist.offset((l + i) as isize) as libc::c_int != 0
                    && *varlist.offset((l + i) as isize) as libc::c_int != ' ' as i32
                {
                    *vname.offset(i as isize) = *varlist.offset((l + i) as isize);
                    i += 1;
                    i;
                }
                *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
                v = dap_varnum(vname);
                if v >= 0 as libc::c_int {
                    let fresh82 = dap_obs[dap_ono as usize].do_ovar;
                    dap_obs[dap_ono as usize]
                        .do_ovar = dap_obs[dap_ono as usize].do_ovar + 1;
                    *(dap_obs[dap_ono as usize].do_out).offset(fresh82 as isize) = v;
                } else {
                    v = dap_arrnum(vname, &mut dim);
                    if v >= 0 as libc::c_int {
                        d = 0 as libc::c_int;
                        while d < dim {
                            let fresh83 = dap_obs[dap_ono as usize].do_ovar;
                            dap_obs[dap_ono as usize]
                                .do_ovar = dap_obs[dap_ono as usize].do_ovar + 1;
                            *(dap_obs[dap_ono as usize].do_out)
                                .offset(fresh83 as isize) = v + d;
                            d += 1;
                            d;
                        }
                    } else {
                        fprintf(
                            dap_err,
                            b"(outset(%s)) unknown variable: %s\n\0" as *const u8
                                as *const libc::c_char,
                            fname,
                            vname,
                        );
                        exit(1 as libc::c_int);
                    }
                }
                l += i;
                while *varlist.offset(l as isize) as libc::c_int == ' ' as i32 {
                    l += 1;
                    l;
                }
            }
        }
    } else {
        dap_obs[dap_ono as usize].do_ovar = dap_obs[dap_ono as usize].do_nvar;
        v = 0 as libc::c_int;
        while v < dap_obs[dap_ono as usize].do_nvar {
            *(dap_obs[dap_ono as usize].do_out).offset(v as isize) = v;
            v += 1;
            v;
        }
    }
    if dap_varnum(b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char)
        < 0 as libc::c_int
    {
        fprintf(
            dap_err,
            b"(outset (%s)) missing _type_ variable\n\0" as *const u8
                as *const libc::c_char,
            fname,
        );
        exit(1 as libc::c_int);
    }
    v = 0 as libc::c_int;
    while v < dap_obs[dap_ono as usize].do_ovar {
        w = v + 1 as libc::c_int;
        while w < dap_obs[dap_ono as usize].do_ovar {
            if *(dap_obs[dap_ono as usize].do_out).offset(v as isize)
                == *(dap_obs[dap_ono as usize].do_out).offset(w as isize)
            {
                fprintf(
                    dap_err,
                    b"(outset (%s)) duplicate variable in output list: %s\n\0"
                        as *const u8 as *const libc::c_char,
                    fname,
                    *(dap_obs[dap_ono as usize].do_nam)
                        .offset(
                            *(dap_obs[dap_ono as usize].do_out).offset(w as isize)
                                as isize,
                        ),
                );
                exit(1 as libc::c_int);
            }
            w += 1;
            w;
        }
        v += 1;
        v;
    }
    v = 0 as libc::c_int;
    first = 1 as libc::c_int;
    while v < dap_obs[dap_ono as usize].do_ovar {
        if first == 0 {
            dap_putc(' ' as i32, dap_out[dap_ono as usize]);
        }
        dputs(
            *(dap_obs[dap_ono as usize].do_nam)
                .offset(*(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize),
            b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            dap_out[dap_ono as usize],
        );
        dputi(
            *(dap_obs[dap_ono as usize].do_len)
                .offset(*(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize),
            dap_out[dap_ono as usize],
        );
        first = 0 as libc::c_int;
        v += 1;
        v;
    }
    dap_putc('\n' as i32, dap_out[dap_ono as usize]);
    dflush(dap_out[dap_ono as usize]);
    outline = 0 as libc::c_int;
}
pub unsafe extern "C" fn output() {
    let mut v: libc::c_int = 0;
    let mut first: libc::c_int = 0;
    v = dap_varnum(b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if v < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(output) missing _type_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    v = 0 as libc::c_int;
    first = 1 as libc::c_int;
    while v < dap_obs[dap_ono as usize].do_ovar {
        if first == 0 {
            dap_putc('|' as i32, dap_out[dap_ono as usize]);
        }
        if *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize)
            == 0 as libc::c_int
        {
            if dap_ono == 0
                && !(*(dap_obs[dap_ono as usize].do_il)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_out).offset(v as isize)
                            as isize,
                    ))
                    .is_null()
            {
                *(dap_obs[0 as libc::c_int as usize].do_int)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_out).offset(v as isize)
                            as isize,
                    ) = **(dap_obs[dap_ono as usize].do_il)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_out).offset(v as isize)
                            as isize,
                    );
            }
            dap_putint(
                *(dap_obs[dap_ono as usize].do_int)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize,
                    ),
                dap_out[dap_ono as usize],
            );
        } else if *(dap_obs[dap_ono as usize].do_len)
            .offset(*(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize)
            == -(1 as libc::c_int)
        {
            if dap_ono == 0
                && !(*(dap_obs[dap_ono as usize].do_dl)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_out).offset(v as isize)
                            as isize,
                    ))
                    .is_null()
            {
                *(dap_obs[0 as libc::c_int as usize].do_dbl)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_out).offset(v as isize)
                            as isize,
                    ) = **(dap_obs[dap_ono as usize].do_dl)
                    .offset(
                        *(dap_obs[0 as libc::c_int as usize].do_out).offset(v as isize)
                            as isize,
                    );
            }
            dap_double = *(dap_obs[dap_ono as usize].do_dbl)
                .offset(*(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize);
            dap_putdouble(dap_out[dap_ono as usize]);
        } else {
            dputs(
                *(dap_obs[dap_ono as usize].do_str)
                    .offset(
                        *(dap_obs[dap_ono as usize].do_out).offset(v as isize) as isize,
                    ),
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                dap_out[dap_ono as usize],
            );
        }
        first = 0 as libc::c_int;
        v += 1;
        v;
    }
    dap_putc('\n' as i32, dap_out[dap_ono as usize]);
    dflush(dap_out[dap_ono as usize]);
    if dap_outreport != 0
        && {
            outline += 1;
            outline % dap_outreport == 0
        }
    {
        fprintf(
            stderr,
            b"(output) %d lines written to %s...\n\0" as *const u8
                as *const libc::c_char,
            outline,
            (*dap_out[dap_ono as usize]).dfile_name,
        );
        fflush(stderr);
    }
}
unsafe extern "C" fn expand(
    mut varlist: *mut libc::c_char,
    mut varv: *mut libc::c_int,
    mut maxvars: libc::c_int,
) -> libc::c_int {
    let mut nvars: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut arrn: libc::c_int = 0;
    let mut dim: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut include: libc::c_int = 0;
    if varlist.is_null() {
        return 0 as libc::c_int;
    }
    if varv.is_null() {
        fputs(
            b"(expand) Missing variable index list.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    mname = dap_malloc(
        (strlen(varlist)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"expand: mname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    newname = dap_malloc(
        (strlen(varlist)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"expand: newname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    include = 1 as libc::c_int;
    m = 0 as libc::c_int;
    while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
        m += 1;
        m;
    }
    if *varlist.offset(m as isize) as libc::c_int == '!' as i32 {
        m += 1;
        m;
        while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
        include = -(1 as libc::c_int);
    }
    nvars = 0 as libc::c_int;
    while *varlist.offset(m as isize) != 0 {
        i = 0 as libc::c_int;
        while *varlist.offset((m + i) as isize) as libc::c_int != 0
            && *varlist.offset((m + i) as isize) as libc::c_int != ' ' as i32
            && *varlist.offset((m + i) as isize) as libc::c_int != '>' as i32
        {
            *mname.offset(i as isize) = *varlist.offset((m + i) as isize);
            i += 1;
            i;
        }
        *mname.offset(i as isize) = '\0' as i32 as libc::c_char;
        m += i;
        while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
        *newname.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        if *varlist.offset(m as isize) as libc::c_int == '>' as i32 {
            if include < 0 as libc::c_int {
                fputs(
                    b"(expand) Can't rename variables being excluded\n\0" as *const u8
                        as *const libc::c_char,
                    dap_err,
                );
                exit(1 as libc::c_int);
            }
            m += 1;
            m;
            while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
                m += 1;
                m;
            }
            i = 0 as libc::c_int;
            while *varlist.offset((m + i) as isize) as libc::c_int != 0
                && *varlist.offset((m + i) as isize) as libc::c_int != ' ' as i32
                && *varlist.offset((m + i) as isize) as libc::c_int != '>' as i32
            {
                *newname.offset(i as isize) = *varlist.offset((m + i) as isize);
                i += 1;
                i;
            }
            *newname.offset(i as isize) = '\0' as i32 as libc::c_char;
            m += i;
            while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
                m += 1;
                m;
            }
        }
        if nvars >= maxvars {
            fprintf(
                dap_err,
                b"(expand) More than %d variables: %s\n\0" as *const u8
                    as *const libc::c_char,
                maxvars,
                varlist,
            );
            exit(1 as libc::c_int);
        }
        arrn = dap_arrnum(mname, &mut dim);
        if arrn >= 0 as libc::c_int {
            d = 0 as libc::c_int;
            while d < dim {
                if *newname.offset(0 as libc::c_int as isize) != 0 {
                    dap_free(
                        *(dap_obs[dap_ono as usize].do_nam).offset(arrn as isize)
                            as *mut libc::c_void,
                        b"expand: dap_obs[dap_ono].do_nam[arrn]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    let ref mut fresh84 = *(dap_obs[dap_ono as usize].do_nam)
                        .offset(arrn as isize);
                    *fresh84 = dap_malloc(
                        (strlen(newname)).wrapping_add(6 as libc::c_int as libc::c_ulong)
                            as libc::c_int,
                        b"expand: dap_obs[dap_ono].do_nam[arrn]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    sprintf(
                        *(dap_obs[dap_ono as usize].do_nam).offset(arrn as isize),
                        b"%s[%d]\0" as *const u8 as *const libc::c_char,
                        newname,
                        d,
                    );
                }
                let fresh85 = arrn;
                arrn = arrn + 1;
                let fresh86 = nvars;
                nvars = nvars + 1;
                *varv.offset(fresh86 as isize) = fresh85;
                d += 1;
                d;
            }
        } else {
            let ref mut fresh87 = *varv.offset(nvars as isize);
            *fresh87 = dap_varnum(mname);
            if *fresh87 > 0 as libc::c_int {
                if *newname.offset(0 as libc::c_int as isize) != 0 {
                    dap_free(
                        *(dap_obs[dap_ono as usize].do_nam)
                            .offset(*varv.offset(nvars as isize) as isize)
                            as *mut libc::c_void,
                        b"expand: dap_obs[dap_ono].do_nam[varv[nvars]]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    let ref mut fresh88 = *(dap_obs[dap_ono as usize].do_nam)
                        .offset(*varv.offset(nvars as isize) as isize);
                    *fresh88 = dap_malloc(
                        (strlen(newname)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                            as libc::c_int,
                        b"expand: dap_obs[dap_ono].do_nam[varv[nvars]]\0" as *const u8
                            as *const libc::c_char as *mut libc::c_char,
                    );
                    strcpy(
                        *(dap_obs[dap_ono as usize].do_nam)
                            .offset(*varv.offset(nvars as isize) as isize),
                        newname,
                    );
                }
                nvars += 1;
                nvars;
            } else {
                fprintf(
                    dap_err,
                    b"(expand) Variable unknown: %s\n\0" as *const u8
                        as *const libc::c_char,
                    mname,
                );
                exit(1 as libc::c_int);
            }
        }
    }
    dap_free(
        mname as *mut libc::c_void,
        b"expand: mname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        newname as *mut libc::c_void,
        b"expand: newname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return include * nvars;
}
unsafe extern "C" fn varcat(mut to: *mut libc::c_char, mut from: *mut libc::c_char) {
    let mut t: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut ff: libc::c_int = 0;
    f = 0 as libc::c_int;
    while *from.offset(f as isize) as libc::c_int == ' ' as i32 {
        f += 1;
        f;
    }
    t = 0 as libc::c_int;
    while *to.offset(t as isize) != 0 {
        t += 1;
        t;
    }
    while *from.offset(f as isize) != 0 {
        ff = f;
        while *from.offset(ff as isize) as libc::c_int != 0
            && *from.offset(ff as isize) as libc::c_int != ' ' as i32
            && *from.offset(ff as isize) as libc::c_int != '>' as i32
        {
            ff += 1;
            ff;
        }
        while *from.offset(ff as isize) as libc::c_int == ' ' as i32 {
            ff += 1;
            ff;
        }
        if *from.offset(ff as isize) as libc::c_int == '>' as i32 {
            f = ff + 1 as libc::c_int;
            while *from.offset(f as isize) as libc::c_int == ' ' as i32 {
                f += 1;
                f;
            }
        }
        while *from.offset(f as isize) as libc::c_int != 0
            && *from.offset(f as isize) as libc::c_int != ' ' as i32
        {
            let fresh89 = f;
            f = f + 1;
            let fresh90 = t;
            t = t + 1;
            *to.offset(fresh90 as isize) = *from.offset(fresh89 as isize);
        }
        let fresh91 = t;
        t = t + 1;
        *to.offset(fresh91 as isize) = ' ' as i32 as libc::c_char;
        while *from.offset(f as isize) as libc::c_int == ' ' as i32 {
            f += 1;
            f;
        }
    }
    *to.offset(t as isize) = '\0' as i32 as libc::c_char;
}
pub unsafe extern "C" fn merge(
    mut fname1: *mut libc::c_char,
    mut vars1: *mut libc::c_char,
    mut fname2: *mut libc::c_char,
    mut vars2: *mut libc::c_char,
    mut marks: *mut libc::c_char,
    mut outname: *mut libc::c_char,
) {
    let mut varv1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut varv2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ovarv1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut ovarv2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvar1: libc::c_int = 0;
    let mut nvar2: libc::c_int = 0;
    let mut markv1: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut markv2: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut v1: libc::c_int = 0;
    let mut v2: libc::c_int = 0;
    let mut vv1: libc::c_int = 0;
    let mut vv2: libc::c_int = 0;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlist1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outlist2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut goon1: libc::c_int = 0;
    let mut ddiff: libc::c_double = 0.;
    let mut isdiff: libc::c_int = 0;
    let mut vars1null: libc::c_int = 0;
    let mut vars2null: libc::c_int = 0;
    let mut vars1a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vars2a: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut exclude1: libc::c_int = 0;
    let mut exclude2: libc::c_int = 0;
    let mut nvar1a: libc::c_int = 0;
    let mut nvar2a: libc::c_int = 0;
    let mut varv1a: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut varv2a: *mut libc::c_int = 0 as *mut libc::c_int;
    if fname1.is_null() || fname2.is_null() || outname.is_null() {
        fputs(
            b"(merge) Missing dataset name.\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if !vars1.is_null() && !(index(vars1, '[' as i32)).is_null() {
        fprintf(
            dap_err,
            b"(merge) Variable lists may not contain individual array elements: %s\n\0"
                as *const u8 as *const libc::c_char,
            vars1,
        );
        exit(1 as libc::c_int);
    }
    if !vars2.is_null() && !(index(vars2, '[' as i32)).is_null() {
        fprintf(
            dap_err,
            b"(merge) Variable lists may not contain individual array elements: %s\n\0"
                as *const u8 as *const libc::c_char,
            vars2,
        );
        exit(1 as libc::c_int);
    }
    vars1null = 0 as libc::c_int;
    vars2null = 0 as libc::c_int;
    outlist = dap_malloc(
        dap_listlen,
        b"merge: outlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    outlist1 = dap_malloc(
        dap_listlen,
        b"merge: outlist1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    outlist2 = dap_malloc(
        dap_listlen,
        b"merge:outlist2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varv1 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"merge: varv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    varv2 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"merge: varv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    ovarv1 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"merge: ovarv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    ovarv2 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"merge: ovarv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv1 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"merge: markv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv2 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"merge: markv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    dap_ono = 0 as libc::c_int;
    inset(fname1);
    if !vars1.is_null() && *vars1.offset(0 as libc::c_int as isize) == 0 {
        vars1null = 1 as libc::c_int;
        vars1 = dap_malloc(
            dap_listlen,
            b"merge: vars1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        *vars1.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        v1 = 0 as libc::c_int;
        while v1 < dap_obs[dap_ono as usize].do_nvar {
            if strcmp(
                *(dap_obs[dap_ono as usize].do_nam).offset(v1 as isize),
                b"_type_\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                strcat(vars1, b" \0" as *const u8 as *const libc::c_char);
                strcat(vars1, *(dap_obs[dap_ono as usize].do_nam).offset(v1 as isize));
            }
            v1 += 1;
            v1;
        }
    }
    nvar1 = expand(vars1, varv1, dap_maxvar);
    exclude1 = 0 as libc::c_int;
    if nvar1 < 0 as libc::c_int {
        exclude1 = 1 as libc::c_int;
        nvar1 = -nvar1;
        vars1a = dap_malloc(
            dap_listlen,
            b"merge: vars1a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        *vars1a.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        varv1a = dap_malloc(
            dap_maxvar,
            b"merge: varv1a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        v1 = 0 as libc::c_int;
        nvar1a = 0 as libc::c_int;
        while v1 < dap_obs[dap_ono as usize].do_nvar {
            if !(strcmp(
                *(dap_obs[dap_ono as usize].do_nam).offset(v1 as isize),
                b"_type_\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                vv1 = 0 as libc::c_int;
                while vv1 < nvar1 {
                    if *varv1.offset(vv1 as isize) == v1 {
                        break;
                    }
                    vv1 += 1;
                    vv1;
                }
                if vv1 == nvar1 {
                    strcat(vars1a, b" \0" as *const u8 as *const libc::c_char);
                    strcat(
                        vars1a,
                        *(dap_obs[dap_ono as usize].do_nam).offset(v1 as isize),
                    );
                    let fresh92 = nvar1a;
                    nvar1a = nvar1a + 1;
                    *varv1a.offset(fresh92 as isize) = v1;
                }
            }
            v1 += 1;
            v1;
        }
        if vars1null != 0 {
            dap_free(
                vars1 as *mut libc::c_void,
                b"merge: vars1\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        vars1 = vars1a;
        nvar1 = nvar1a;
        dap_free(
            varv1 as *mut libc::c_void,
            b"merge: varv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        varv1 = varv1a;
    }
    if !marks.is_null() {
        nmark = dap_list(marks, markv1, dap_maxvar);
    } else {
        nmark = 0 as libc::c_int;
    }
    dap_ono = 1 as libc::c_int;
    inset(fname2);
    if !vars2.is_null() && *vars2.offset(0 as libc::c_int as isize) == 0 {
        vars2null = 1 as libc::c_int;
        vars2 = dap_malloc(
            dap_listlen,
            b"merge: vars2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        *vars2.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        v2 = 0 as libc::c_int;
        while v2 < dap_obs[dap_ono as usize].do_nvar {
            if strcmp(
                *(dap_obs[dap_ono as usize].do_nam).offset(v2 as isize),
                b"_type_\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                strcat(vars2, b" \0" as *const u8 as *const libc::c_char);
                strcat(vars2, *(dap_obs[dap_ono as usize].do_nam).offset(v2 as isize));
            }
            v2 += 1;
            v2;
        }
    }
    nvar2 = expand(vars2, varv2, dap_maxvar);
    exclude2 = 0 as libc::c_int;
    if nvar2 < 0 as libc::c_int {
        exclude2 = 1 as libc::c_int;
        nvar2 = -nvar2;
        vars2a = dap_malloc(
            dap_listlen,
            b"merge: vars2a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        *vars2a.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        varv2a = dap_malloc(
            dap_maxvar,
            b"merge: varv2a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        v2 = 0 as libc::c_int;
        nvar2a = 0 as libc::c_int;
        while v2 < dap_obs[dap_ono as usize].do_nvar {
            if !(strcmp(
                *(dap_obs[dap_ono as usize].do_nam).offset(v2 as isize),
                b"_type_\0" as *const u8 as *const libc::c_char,
            ) == 0)
            {
                vv2 = 0 as libc::c_int;
                while vv2 < nvar2 {
                    if *varv2.offset(vv2 as isize) == v2 {
                        break;
                    }
                    vv2 += 1;
                    vv2;
                }
                if vv2 == nvar2 {
                    strcat(vars2a, b" \0" as *const u8 as *const libc::c_char);
                    strcat(
                        vars2a,
                        *(dap_obs[dap_ono as usize].do_nam).offset(v2 as isize),
                    );
                    let fresh93 = nvar2a;
                    nvar2a = nvar2a + 1;
                    *varv2a.offset(fresh93 as isize) = v2;
                }
            }
            v2 += 1;
            v2;
        }
        if vars2null != 0 {
            dap_free(
                vars2 as *mut libc::c_void,
                b"merge: vars2\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            );
        }
        vars2 = vars2a;
        nvar2 = nvar2a;
        dap_free(
            varv2 as *mut libc::c_void,
            b"merge: varv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        varv2 = varv2a;
    }
    dap_list(marks, markv2, dap_maxvar);
    v1 = 0 as libc::c_int;
    while v1 < nmark {
        if *(dap_obs[0 as libc::c_int as usize].do_len)
            .offset(*markv1.offset(v1 as isize) as isize)
            != *(dap_obs[1 as libc::c_int as usize].do_len)
                .offset(*markv2.offset(v1 as isize) as isize)
        {
            fprintf(
                dap_err,
                b"(merge) Part variables of different types: %s (%d) and %s (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                *(dap_obs[0 as libc::c_int as usize].do_nam)
                    .offset(*markv1.offset(v1 as isize) as isize),
                *(dap_obs[0 as libc::c_int as usize].do_len)
                    .offset(*markv1.offset(v1 as isize) as isize),
                *(dap_obs[1 as libc::c_int as usize].do_nam)
                    .offset(*markv2.offset(v1 as isize) as isize),
                *(dap_obs[1 as libc::c_int as usize].do_len)
                    .offset(*markv2.offset(v1 as isize) as isize),
            );
            exit(1 as libc::c_int);
        }
        v1 += 1;
        v1;
    }
    *outlist.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if !vars2.is_null() && nvar2 > 0 as libc::c_int {
        varcat(outlist, vars2);
    }
    *outlist2.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if !vars2.is_null() && nvar2 > 0 as libc::c_int {
        varcat(outlist2, vars2);
    }
    dap_ono = 2 as libc::c_int;
    dap_obs[2 as libc::c_int as usize].do_nvar = 0 as libc::c_int;
    dap_obs[2 as libc::c_int as usize].do_ovar = 0 as libc::c_int;
    let fresh94 = dap_obs[2 as libc::c_int as usize].do_ovar;
    dap_obs[2 as libc::c_int as usize]
        .do_ovar = dap_obs[2 as libc::c_int as usize].do_ovar + 1;
    *(dap_obs[2 as libc::c_int as usize].do_out)
        .offset(
            fresh94 as isize,
        ) = dap_vd(
        b"_type_ 8\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
    );
    v2 = 0 as libc::c_int;
    while v2 < nvar2 {
        if !(*(dap_obs[2 as libc::c_int as usize].do_nam)
            .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize))
            .is_null()
        {
            dap_free(
                *(dap_obs[2 as libc::c_int as usize].do_nam)
                    .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize)
                    as *mut libc::c_void,
                b"merge: dap_obs[2].do_nam[dap_obs[2].do_nvar]\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
        }
        let ref mut fresh95 = *(dap_obs[2 as libc::c_int as usize].do_nam)
            .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize);
        *fresh95 = dap_malloc(
            (strlen(
                *(dap_obs[1 as libc::c_int as usize].do_nam)
                    .offset(*varv2.offset(v2 as isize) as isize),
            ))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"merge: dap_obs[1].do_nam[varv2[v2]]\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
        );
        strcpy(
            *(dap_obs[2 as libc::c_int as usize].do_nam)
                .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize),
            *(dap_obs[1 as libc::c_int as usize].do_nam)
                .offset(*varv2.offset(v2 as isize) as isize),
        );
        *(dap_obs[2 as libc::c_int as usize].do_len)
            .offset(
                dap_obs[2 as libc::c_int as usize].do_nvar as isize,
            ) = *(dap_obs[1 as libc::c_int as usize].do_len)
            .offset(*varv2.offset(v2 as isize) as isize);
        dap_obs[2 as libc::c_int as usize].do_nvar += 1;
        dap_obs[2 as libc::c_int as usize].do_nvar;
        v2 += 1;
        v2;
    }
    *outlist1.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    if !vars1.is_null() && nvar1 > 0 as libc::c_int {
        strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
        varcat(outlist, vars1);
        varcat(outlist1, vars1);
    }
    v1 = 0 as libc::c_int;
    while v1 < nvar1 {
        v2 = 0 as libc::c_int;
        while v2 < nvar2 {
            if strcmp(
                *(dap_obs[0 as libc::c_int as usize].do_nam)
                    .offset(*varv1.offset(v1 as isize) as isize),
                *(dap_obs[1 as libc::c_int as usize].do_nam)
                    .offset(*varv2.offset(v2 as isize) as isize),
            ) == 0
            {
                break;
            }
            v2 += 1;
            v2;
        }
        if v2 < nvar2 {
            fprintf(
                dap_err,
                b"(merge) variable appears in lists for both %s and %s: %s\n\0"
                    as *const u8 as *const libc::c_char,
                fname1,
                fname2,
                *(dap_obs[0 as libc::c_int as usize].do_nam)
                    .offset(*varv1.offset(v1 as isize) as isize),
            );
            exit(1 as libc::c_int);
        } else {
            if !(*(dap_obs[2 as libc::c_int as usize].do_nam)
                .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize))
                .is_null()
            {
                dap_free(
                    *(dap_obs[2 as libc::c_int as usize].do_nam)
                        .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize)
                        as *mut libc::c_void,
                    b"merge: dap_obs[2].do_nam[dap_obs[2].do_nvar]\0" as *const u8
                        as *const libc::c_char as *mut libc::c_char,
                );
            }
            let ref mut fresh96 = *(dap_obs[2 as libc::c_int as usize].do_nam)
                .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize);
            *fresh96 = dap_malloc(
                (strlen(
                    *(dap_obs[0 as libc::c_int as usize].do_nam)
                        .offset(*varv1.offset(v1 as isize) as isize),
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
                b"merge: dap_obs[0].do_nam[varv1[v1]]\0" as *const u8
                    as *const libc::c_char as *mut libc::c_char,
            );
            strcpy(
                *(dap_obs[2 as libc::c_int as usize].do_nam)
                    .offset(dap_obs[2 as libc::c_int as usize].do_nvar as isize),
                *(dap_obs[0 as libc::c_int as usize].do_nam)
                    .offset(*varv1.offset(v1 as isize) as isize),
            );
            *(dap_obs[2 as libc::c_int as usize].do_len)
                .offset(
                    dap_obs[2 as libc::c_int as usize].do_nvar as isize,
                ) = *(dap_obs[0 as libc::c_int as usize].do_len)
                .offset(*varv1.offset(v1 as isize) as isize);
            dap_obs[2 as libc::c_int as usize].do_nvar += 1;
            dap_obs[2 as libc::c_int as usize].do_nvar;
        }
        v1 += 1;
        v1;
    }
    outset(outname, outlist);
    strcpy(
        *(dap_obs[2 as libc::c_int as usize].do_str)
            .offset(
                dap_varnum(
                    b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) as isize,
            ),
        b"OBS\0" as *const u8 as *const libc::c_char,
    );
    expand(outlist1, ovarv1, dap_maxvar);
    expand(outlist2, ovarv2, dap_maxvar);
    dap_ono = 0 as libc::c_int;
    goon1 = step();
    loop {
        dap_ono = 1 as libc::c_int;
        if !(step() != 0) {
            break;
        }
        while goon1 != 0 {
            isdiff = 0 as libc::c_int;
            if nmark != 0 {
                v1 = 0 as libc::c_int;
                while v1 < nmark {
                    if *(dap_obs[0 as libc::c_int as usize].do_len)
                        .offset(*markv1.offset(v1 as isize) as isize)
                        == -(1 as libc::c_int)
                    {
                        ddiff = *(dap_obs[0 as libc::c_int as usize].do_dbl)
                            .offset(*markv1.offset(v1 as isize) as isize)
                            - *(dap_obs[1 as libc::c_int as usize].do_dbl)
                                .offset(*markv2.offset(v1 as isize) as isize);
                        if ddiff < 0.0f64 {
                            isdiff = -(1 as libc::c_int);
                        } else if ddiff > 0.0f64 {
                            isdiff = 1 as libc::c_int;
                        } else {
                            isdiff = 0 as libc::c_int;
                        }
                        if isdiff != 0 {
                            break;
                        }
                    } else if *(dap_obs[0 as libc::c_int as usize].do_len)
                        .offset(*markv1.offset(v1 as isize) as isize) == 0 as libc::c_int
                    {
                        isdiff = *(dap_obs[0 as libc::c_int as usize].do_int)
                            .offset(*markv1.offset(v1 as isize) as isize)
                            - *(dap_obs[1 as libc::c_int as usize].do_int)
                                .offset(*markv2.offset(v1 as isize) as isize);
                        if isdiff != 0 {
                            break;
                        }
                    } else {
                        isdiff = strcmp(
                            *(dap_obs[0 as libc::c_int as usize].do_str)
                                .offset(*markv1.offset(v1 as isize) as isize),
                            *(dap_obs[1 as libc::c_int as usize].do_str)
                                .offset(*markv2.offset(v1 as isize) as isize),
                        );
                        if isdiff != 0 {
                            break;
                        }
                    }
                    v1 += 1;
                    v1;
                }
            }
            if isdiff < 0 as libc::c_int {
                dap_ono = 0 as libc::c_int;
                goon1 = step();
            } else {
                if isdiff > 0 as libc::c_int {
                    break;
                }
                v1 = 0 as libc::c_int;
                while v1 < nvar1 {
                    if *(dap_obs[0 as libc::c_int as usize].do_len)
                        .offset(*varv1.offset(v1 as isize) as isize)
                        == -(1 as libc::c_int)
                    {
                        *(dap_obs[2 as libc::c_int as usize].do_dbl)
                            .offset(
                                *ovarv1.offset(v1 as isize) as isize,
                            ) = *(dap_obs[0 as libc::c_int as usize].do_dbl)
                            .offset(*varv1.offset(v1 as isize) as isize);
                    } else if *(dap_obs[0 as libc::c_int as usize].do_len)
                        .offset(*varv1.offset(v1 as isize) as isize) == 0 as libc::c_int
                    {
                        *(dap_obs[2 as libc::c_int as usize].do_int)
                            .offset(
                                *ovarv1.offset(v1 as isize) as isize,
                            ) = *(dap_obs[0 as libc::c_int as usize].do_int)
                            .offset(*varv1.offset(v1 as isize) as isize);
                    } else {
                        if !(*(dap_obs[2 as libc::c_int as usize].do_str)
                            .offset(*ovarv1.offset(v1 as isize) as isize))
                            .is_null()
                        {
                            dap_free(
                                *(dap_obs[2 as libc::c_int as usize].do_str)
                                    .offset(*ovarv1.offset(v1 as isize) as isize)
                                    as *mut libc::c_void,
                                b"merge: dap_obs[2].do_str[ovarv1[v1]]\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        let ref mut fresh97 = *(dap_obs[2 as libc::c_int as usize]
                            .do_str)
                            .offset(*ovarv1.offset(v1 as isize) as isize);
                        *fresh97 = dap_malloc(
                            (strlen(
                                *(dap_obs[0 as libc::c_int as usize].do_str)
                                    .offset(*varv1.offset(v1 as isize) as isize),
                            ))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                            b"merge: dap_obs[0].do_str[ovarv1[v1]]\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        strcpy(
                            *(dap_obs[2 as libc::c_int as usize].do_str)
                                .offset(*ovarv1.offset(v1 as isize) as isize),
                            *(dap_obs[0 as libc::c_int as usize].do_str)
                                .offset(*varv1.offset(v1 as isize) as isize),
                        );
                    }
                    v1 += 1;
                    v1;
                }
                v2 = 0 as libc::c_int;
                while v2 < nvar2 {
                    if *(dap_obs[1 as libc::c_int as usize].do_len)
                        .offset(*varv2.offset(v2 as isize) as isize)
                        == -(1 as libc::c_int)
                    {
                        *(dap_obs[2 as libc::c_int as usize].do_dbl)
                            .offset(
                                *ovarv2.offset(v2 as isize) as isize,
                            ) = *(dap_obs[1 as libc::c_int as usize].do_dbl)
                            .offset(*varv2.offset(v2 as isize) as isize);
                    } else if *(dap_obs[1 as libc::c_int as usize].do_len)
                        .offset(*varv2.offset(v2 as isize) as isize) == 0 as libc::c_int
                    {
                        *(dap_obs[2 as libc::c_int as usize].do_int)
                            .offset(
                                *ovarv2.offset(v2 as isize) as isize,
                            ) = *(dap_obs[1 as libc::c_int as usize].do_int)
                            .offset(*varv2.offset(v2 as isize) as isize);
                    } else {
                        if !(*(dap_obs[2 as libc::c_int as usize].do_str)
                            .offset(*ovarv2.offset(v2 as isize) as isize))
                            .is_null()
                        {
                            dap_free(
                                *(dap_obs[2 as libc::c_int as usize].do_str)
                                    .offset(*ovarv2.offset(v2 as isize) as isize)
                                    as *mut libc::c_void,
                                b"merge: dap_obs[2].do_str[ovarv2[v2]]\0" as *const u8
                                    as *const libc::c_char as *mut libc::c_char,
                            );
                        }
                        let ref mut fresh98 = *(dap_obs[2 as libc::c_int as usize]
                            .do_str)
                            .offset(*ovarv2.offset(v2 as isize) as isize);
                        *fresh98 = dap_malloc(
                            (strlen(
                                *(dap_obs[1 as libc::c_int as usize].do_str)
                                    .offset(*varv2.offset(v2 as isize) as isize),
                            ))
                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                as libc::c_int,
                            b"merge: dap_obs[1].do_str[ovarv2[v2]]\0" as *const u8
                                as *const libc::c_char as *mut libc::c_char,
                        );
                        strcpy(
                            *(dap_obs[2 as libc::c_int as usize].do_str)
                                .offset(*ovarv2.offset(v2 as isize) as isize),
                            *(dap_obs[1 as libc::c_int as usize].do_str)
                                .offset(*varv2.offset(v2 as isize) as isize),
                        );
                    }
                    v2 += 1;
                    v2;
                }
                dap_ono = 2 as libc::c_int;
                output();
                dap_ono = 0 as libc::c_int;
                goon1 = step();
            }
            if nmark == 0 {
                break;
            }
        }
    }
    dap_ono = 0 as libc::c_int;
    if !(dap_in[0 as libc::c_int as usize]).is_null() {
        dfclose(dap_in[0 as libc::c_int as usize]);
        dap_in[0 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if !(dap_in[1 as libc::c_int as usize]).is_null() {
        dfclose(dap_in[1 as libc::c_int as usize]);
        dap_in[1 as libc::c_int as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    if !(dap_out[dap_ono as usize]).is_null() {
        dfclose(dap_out[dap_ono as usize]);
        dap_out[dap_ono as usize] = 0 as *mut libc::c_void as *mut DFILE;
    }
    dap_free(
        outlist as *mut libc::c_void,
        b"merge: outlist\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist1 as *mut libc::c_void,
        b"merge: outlist1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist2 as *mut libc::c_void,
        b"merge: outlist2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ovarv1 as *mut libc::c_void,
        b"merge: ovarv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ovarv2 as *mut libc::c_void,
        b"merge: ovarv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv1 as *mut libc::c_void,
        b"merge: markv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv2 as *mut libc::c_void,
        b"merge: markv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if vars1null != 0 {
        dap_free(
            vars1 as *mut libc::c_void,
            b"merge: vars1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if vars2null != 0 {
        dap_free(
            vars2 as *mut libc::c_void,
            b"merge: vars2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if exclude1 < 0 as libc::c_int {
        dap_free(
            vars1a as *mut libc::c_void,
            b"merge: vars1a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            varv1a as *mut libc::c_void,
            b"merge: varv1a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        dap_free(
            varv1 as *mut libc::c_void,
            b"merge: varv1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if exclude2 < 0 as libc::c_int {
        dap_free(
            vars2a as *mut libc::c_void,
            b"merge: vars2a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            varv2a as *mut libc::c_void,
            b"merge: varv2a\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        dap_free(
            varv2 as *mut libc::c_void,
            b"merge: varv2\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    };
}
pub unsafe extern "C" fn title(mut text: *mut libc::c_char) {
    dap_title = text;
}
pub unsafe extern "C" fn dap_head(mut markv: *mut libc::c_int, mut nmark: libc::c_int) {
    let mut t: time_t = 0;
    let mut m: libc::c_int = 0;
    fputs(
        b"\n=================================\0" as *const u8 as *const libc::c_char,
        dap_lst,
    );
    let fresh99 = pageno;
    pageno = pageno + 1;
    fprintf(dap_lst, b"\nDap %3d. \0" as *const u8 as *const libc::c_char, fresh99);
    time(&mut t);
    fputs(ctime(&mut t), dap_lst);
    putc('\n' as i32, dap_lst);
    if !dap_title.is_null() {
        fputs(dap_title, dap_lst);
        putc('\n' as i32, dap_lst);
    }
    if nmark != 0 {
        fprintf(dap_lst, b"\nFor: \0" as *const u8 as *const libc::c_char);
        m = 0 as libc::c_int;
        while m < nmark {
            match *(dap_obs[dap_ono as usize].do_len)
                .offset(*markv.offset(m as isize) as isize)
            {
                -1 => {
                    fprintf(
                        dap_lst,
                        b"%s = %g\0" as *const u8 as *const libc::c_char,
                        *(dap_obs[dap_ono as usize].do_nam)
                            .offset(*markv.offset(m as isize) as isize),
                        *(dap_obs[dap_ono as usize].do_dbl)
                            .offset(*markv.offset(m as isize) as isize),
                    );
                }
                0 => {
                    fprintf(
                        dap_lst,
                        b"%s = %d\0" as *const u8 as *const libc::c_char,
                        *(dap_obs[dap_ono as usize].do_nam)
                            .offset(*markv.offset(m as isize) as isize),
                        *(dap_obs[dap_ono as usize].do_int)
                            .offset(*markv.offset(m as isize) as isize),
                    );
                }
                _ => {
                    fprintf(
                        dap_lst,
                        b"%s = %s\0" as *const u8 as *const libc::c_char,
                        *(dap_obs[dap_ono as usize].do_nam)
                            .offset(*markv.offset(m as isize) as isize),
                        *(dap_obs[dap_ono as usize].do_str)
                            .offset(*markv.offset(m as isize) as isize),
                    );
                }
            }
            if m < nmark - 1 as libc::c_int {
                fputs(b", \0" as *const u8 as *const libc::c_char, dap_lst);
            }
            m += 1;
            m;
        }
        putc('\n' as i32, dap_lst);
    }
    putc('\n' as i32, dap_lst);
}
pub unsafe extern "C" fn extractWords(
    mut buffer: *mut libc::c_char,
    mut size: libc::c_long,
    mut delimiter: *mut libc::c_char,
) -> CharList {
    let mut bufferWord: *mut libc::c_char = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(size as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    memset(
        bufferWord as *mut libc::c_void,
        '\0' as i32,
        (size + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
    );
    let mut list: CharList = CharList {
        word: 0 as *mut libc::c_char,
        next: 0 as *mut CharList,
    };
    list.word = 0 as *mut libc::c_char;
    list.next = 0 as *mut CharList;
    let mut current: *mut CharList = &mut list;
    let mut i: libc::c_long = 0 as libc::c_long;
    let mut sSize: libc::c_long = 0 as libc::c_long;
    let mut sIndex: libc::c_long = 0 as libc::c_long;
    i = 0 as libc::c_long;
    while i < size {
        if !(*buffer.offset(i as isize) as libc::c_int == '\r' as i32
            || *buffer.offset(i as isize) as libc::c_int == '\t' as i32)
        {
            if *buffer.offset(i as isize) as libc::c_int
                == *delimiter.offset(0 as libc::c_int as isize) as libc::c_int
            {
                (*current)
                    .word = malloc(
                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                        .wrapping_mul(sIndex as libc::c_ulong)
                        .wrapping_add(5 as libc::c_int as libc::c_ulong),
                ) as *mut libc::c_char;
                strcpy((*current).word, bufferWord);
                (*current)
                    .next = malloc(::std::mem::size_of::<CharList>() as libc::c_ulong)
                    as *mut libc::c_char as *mut CharList;
                current = (*current).next;
                sSize = sIndex;
                sIndex = 0 as libc::c_int as libc::c_long;
                memset(
                    bufferWord as *mut libc::c_void,
                    '\0' as i32,
                    (size + 1 as libc::c_int as libc::c_long) as libc::c_ulong,
                );
            } else {
                *bufferWord.offset(sIndex as isize) = *buffer.offset(i as isize);
                sIndex += 1;
                sIndex;
            }
        }
        i += 1;
        i;
    }
    if sIndex > 0 as libc::c_int as libc::c_long {
        (*current)
            .word = malloc(
            (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                .wrapping_mul(strlen(bufferWord))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
        strcpy((*current).word, bufferWord);
        (*current).next = 0 as *mut CharList;
    }
    free(bufferWord as *mut libc::c_void);
    return list;
}
pub unsafe extern "C" fn cleanAttributeList(mut list: *mut AttributeList) {
    let mut actualatt: *mut AttributeList = list;
    loop {
        actualatt = list;
        let mut prev: *mut AttributeList = list;
        let mut prev2: *mut AttributeList = list;
        loop {
            prev2 = prev;
            prev = actualatt;
            actualatt = (*actualatt).next;
            if actualatt.is_null() {
                break;
            }
        }
        if prev != list {
            free((*prev).word as *mut libc::c_void);
            free(prev as *mut libc::c_void);
        }
        (*prev2).next = 0 as *mut AttributeList;
        if ((*list).next).is_null() {
            break;
        }
    };
}
pub unsafe extern "C" fn cleanCharList(mut list: *mut CharList) {
    let mut actualatt: *mut CharList = list;
    loop {
        actualatt = list;
        let mut prev: *mut CharList = list;
        let mut prev2: *mut CharList = list;
        loop {
            prev2 = prev;
            prev = actualatt;
            actualatt = (*actualatt).next;
            if actualatt.is_null() {
                break;
            }
        }
        if prev != list {
            free((*prev).word as *mut libc::c_void);
            free(prev as *mut libc::c_void);
        }
        (*prev2).next = 0 as *mut CharList;
        if ((*list).next).is_null() {
            break;
        }
    };
}
pub unsafe extern "C" fn import(
    mut fname: *mut libc::c_char,
    mut fileToLoad: *mut libc::c_char,
    mut format: *mut libc::c_char,
    mut delimiter: *mut libc::c_char,
    mut replace: libc::c_int,
    mut getnames: libc::c_int,
) -> libc::c_int {
    infile(fileToLoad, delimiter);
    let mut pFile: *mut FILE = 0 as *mut FILE;
    let mut lSize: libc::c_long = 0;
    let mut buffer: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut bufferline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: size_t = 0;
    if strcmp(format, b"CSV\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(format, b"DLM\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(format, b"csv\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(format, b"dlm\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(format, b"TAB\0" as *const u8 as *const libc::c_char) != 0
        && strcmp(format, b"tab\0" as *const u8 as *const libc::c_char) != 0
    {
        printf(
            b"Format not supported :%s\n\0" as *const u8 as *const libc::c_char,
            format,
        );
        exit(1 as libc::c_int);
    }
    if strlen(delimiter) == 0 as libc::c_int as libc::c_ulong {
        if strcmp(format, b"CSV\0" as *const u8 as *const libc::c_char) != 0
            || strcmp(format, b"csv\0" as *const u8 as *const libc::c_char) != 0
        {
            delimiter = b",\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if strcmp(format, b"DLM\0" as *const u8 as *const libc::c_char) != 0
            || strcmp(format, b"dlm\0" as *const u8 as *const libc::c_char) != 0
        {
            delimiter = b" \0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if strcmp(format, b"TAB\0" as *const u8 as *const libc::c_char) != 0
            || strcmp(format, b"tab\0" as *const u8 as *const libc::c_char) != 0
        {
            delimiter = b"\t\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
    }
    pFile = fopen(fileToLoad, b"rb\0" as *const u8 as *const libc::c_char);
    if pFile.is_null() {
        printf(
            b"File loading error :%s\n\0" as *const u8 as *const libc::c_char,
            fileToLoad,
        );
        exit(1 as libc::c_int);
    }
    fseek(pFile, 0 as libc::c_int as libc::c_long, 2 as libc::c_int);
    lSize = ftell(pFile);
    rewind(pFile);
    buffer = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(lSize as libc::c_ulong),
    ) as *mut libc::c_char;
    if buffer.is_null() {
        printf(b"Memory error \n\0" as *const u8 as *const libc::c_char);
        exit(2 as libc::c_int);
    }
    bufferline = malloc(
        (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(lSize as libc::c_ulong),
    ) as *mut libc::c_char;
    if bufferline.is_null() {
        printf(b"2nd Memory error \n\0" as *const u8 as *const libc::c_char);
        exit(2 as libc::c_int);
    }
    result = fread(
        buffer as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        lSize as libc::c_ulong,
        pFile,
    );
    if result != lSize as libc::c_ulong {
        printf(b"Reading error \n\0" as *const u8 as *const libc::c_char);
        exit(3 as libc::c_int);
    }
    let mut sIndex: libc::c_long = 0 as libc::c_long;
    let mut sSize: libc::c_long = 0 as libc::c_long;
    let mut i: libc::c_long = 0 as libc::c_long;
    let mut listatt: AttributeList = AttributeList {
        word: 0 as *mut libc::c_char,
        size: 0,
        type_0: 0,
        next: 0 as *mut AttributeList,
    };
    listatt.word = 0 as *mut libc::c_char;
    listatt.next = 0 as *mut AttributeList;
    let mut lineCnt: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_long;
    while i < lSize {
        if *buffer.offset(i as isize) as libc::c_int == '\n' as i32 {
            let mut list: CharList = extractWords(bufferline, sIndex, delimiter);
            let mut actual: *mut CharList = &mut list;
            let mut counter: libc::c_int = 0 as libc::c_int;
            loop {
                let mut word: *mut libc::c_char = (*actual).word;
                let mut sizeStr: libc::c_int = strlen(word) as libc::c_int;
                let mut actualatt: *mut AttributeList = &mut listatt;
                let mut prev: *mut AttributeList = actualatt;
                if lineCnt == 0 as libc::c_int {
                    if ((*actualatt).word).is_null() {
                        if getnames != 0 {
                            (*actualatt)
                                .word = malloc(
                                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                    .wrapping_mul(strlen(word))
                                    .wrapping_add(5 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            strcpy((*actualatt).word, word);
                        } else {
                            (*actualatt)
                                .word = malloc(
                                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                    .wrapping_mul(10 as libc::c_int as libc::c_ulong),
                            ) as *mut libc::c_char;
                            sprintf(
                                (*actualatt).word,
                                b"X%d\0" as *const u8 as *const libc::c_char,
                                counter,
                            );
                        }
                    } else {
                        loop {
                            let mut wordatt: *mut libc::c_char = (*actualatt).word;
                            if strcmp(wordatt, word) == 0 as libc::c_int {
                                break;
                            }
                            prev = actualatt;
                            actualatt = (*actualatt).next;
                            if actualatt.is_null() {
                                break;
                            }
                        }
                        if actualatt.is_null() {
                            (*prev)
                                .next = malloc(
                                ::std::mem::size_of::<AttributeList>() as libc::c_ulong,
                            ) as *mut AttributeList;
                            (*(*prev).next).word = 0 as *mut libc::c_char;
                            prev = (*prev).next;
                            if getnames != 0 {
                                (*prev)
                                    .word = malloc(
                                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(strlen(word))
                                        .wrapping_add(5 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                                strcpy((*prev).word, word);
                            } else {
                                (*prev)
                                    .word = malloc(
                                    (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                                        .wrapping_mul(10 as libc::c_int as libc::c_ulong),
                                ) as *mut libc::c_char;
                                sprintf(
                                    (*prev).word,
                                    b"X%d\0" as *const u8 as *const libc::c_char,
                                    counter,
                                );
                            }
                            (*prev).next = 0 as *mut AttributeList;
                        }
                    }
                }
                actualatt = &mut listatt;
                prev = actualatt;
                if getnames != 0 && lineCnt > 0 as libc::c_int || getnames == 0 {
                    let mut counterbis: libc::c_int = 0 as libc::c_int;
                    while !(counter == counterbis) {
                        actualatt = (*actualatt).next;
                        counterbis += 1;
                        counterbis;
                        if actualatt.is_null() {
                            break;
                        }
                    }
                    if !actualatt.is_null() {
                        if sizeStr > (*actualatt).size {
                            (*actualatt).size = sizeStr;
                        }
                        if (*actualatt).size > 1000 as libc::c_int {
                            (*actualatt).size = 1000 as libc::c_int;
                        }
                        let mut f: libc::c_float = 0.;
                        if sscanf(
                            word,
                            b"%f\0" as *const u8 as *const libc::c_char,
                            &mut f as *mut libc::c_float,
                        ) != 0 as libc::c_int
                        {
                            (*actualatt).type_0 = 1 as libc::c_int;
                        } else {
                            (*actualatt).type_0 = 0 as libc::c_int;
                        }
                    }
                }
                actual = (*actual).next;
                counter += 1;
                counter;
                if actual.is_null() {
                    break;
                }
            }
            sSize = sIndex;
            sIndex = 0 as libc::c_int as libc::c_long;
            lineCnt += 1;
            lineCnt;
            cleanCharList(&mut list);
        } else {
            *bufferline.offset(sIndex as isize) = *buffer.offset(i as isize);
            sIndex += 1;
            sIndex;
        }
        i += 1;
        i;
    }
    fclose(pFile);
    free(buffer as *mut libc::c_void);
    free(bufferline as *mut libc::c_void);
    let mut actualatt_0: *mut AttributeList = &mut listatt;
    let mut countcol: libc::c_int = 0 as libc::c_int;
    loop {
        let mut wordatt_0: *mut libc::c_char = (*actualatt_0).word;
        if !wordatt_0.is_null() {
            let mut sierr: libc::c_int = (strlen(wordatt_0))
                .wrapping_add(6 as libc::c_int as libc::c_ulong) as libc::c_int;
            let mut bufferer: *mut libc::c_char = malloc(
                (::std::mem::size_of::<libc::c_char>() as libc::c_ulong)
                    .wrapping_mul(sierr as libc::c_ulong),
            ) as *mut libc::c_char;
            if (*actualatt_0).type_0 == 1 as libc::c_int {
                sprintf(
                    bufferer,
                    b"%s -1\0" as *const u8 as *const libc::c_char,
                    wordatt_0,
                );
                dap_vd(bufferer, -(1 as libc::c_int));
            } else {
                let mut size: libc::c_int = 1000 as libc::c_int;
                if (*actualatt_0).size < 1000 as libc::c_int {
                    size = (*actualatt_0).size;
                }
                sprintf(
                    bufferer,
                    b"%s %d\0" as *const u8 as *const libc::c_char,
                    wordatt_0,
                    (*actualatt_0).size,
                );
                dap_vd(bufferer, size);
            }
        }
        countcol += 1;
        countcol;
        actualatt_0 = (*actualatt_0).next;
        if actualatt_0.is_null() {
            break;
        }
    }
    outset(fname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    skip(2 as libc::c_int);
    while step() != 0 {
        output();
    }
    cleanAttributeList(&mut listatt);
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args_os() {
        args.push(
            (::std::ffi::CString::new(
                ::std::os::unix::ffi::OsStrExt::as_bytes(arg.as_os_str()),
            ))
                .unwrap()
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
