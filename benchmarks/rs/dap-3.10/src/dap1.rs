use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getpid() -> __pid_t;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn sqrt(_: libc::c_double) -> libc::c_double;
    fn fabs(_: libc::c_double) -> libc::c_double;
    fn finite(_: libc::c_double) -> libc::c_int;
    fn rand() -> libc::c_int;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn perror(__s: *const libc::c_char);
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn putc(__c: libc::c_int, __stream: *mut FILE) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn dap_ftell(fp: *mut DFILE) -> libc::c_long;
    fn dap_putc(c: libc::c_int, fp: *mut DFILE);
    fn title(text: *mut libc::c_char);
    fn step() -> libc::c_int;
    fn inset(fname: *mut libc::c_char);
    fn outset(fname: *mut libc::c_char, varlist: *mut libc::c_char);
    fn output();
    fn dap_free(ptr: *mut libc::c_void, mesg: *mut libc::c_char);
    fn dap_malloc(nbytes: libc::c_int, mesg: *mut libc::c_char) -> *mut libc::c_char;
    fn dap_rewind();
    fn dap_name(dname: *mut libc::c_char, fname: *mut libc::c_char);
    fn dap_mark();
    fn dap_varnum(vname: *mut libc::c_char) -> libc::c_int;
    fn dap_suffix(
        dst: *mut libc::c_char,
        src: *mut libc::c_char,
        suff: *mut libc::c_char,
    );
    fn dap_head(markv: *mut libc::c_int, nmark: libc::c_int);
    fn dap_swap();
    fn probt(t1: libc::c_double, di: libc::c_int) -> libc::c_double;
    fn dap_vd(varspec: *mut libc::c_char, invar: libc::c_int) -> libc::c_int;
    static mut dap_maxvar: libc::c_int;
    static mut dap_namelen: libc::c_int;
    static mut dap_listlen: libc::c_int;
    static mut dap_strlen: libc::c_int;
    static mut dap_setdir: *mut libc::c_char;
    static mut dap_linelen: libc::c_int;
    static mut dap_maxlines: libc::c_int;
    static mut dap_maxmem: libc::c_int;
    static mut dap_tmpdir: *mut libc::c_char;
    static mut dap_maxrows: libc::c_int;
    static mut dap_maxcols: libc::c_int;
    static mut dap_maxclab: libc::c_int;
    static mut dap_maxrowv: libc::c_int;
    static mut dap_maxcolv: libc::c_int;
    static mut dap_lablen: libc::c_int;
    static mut dap_dapname: *mut libc::c_char;
    static mut dap_in: [*mut DFILE; 2];
    static mut dap_out: [*mut DFILE; 2];
    static mut dap_obs: [dataobs; 0];
    static mut dap_prev: [dataobs; 0];
    static mut dap_ono: libc::c_int;
    static mut dap_lst: *mut FILE;
    static mut dap_err: *mut FILE;
    static mut dap_log: *mut FILE;
    static mut dap_title: *mut libc::c_char;
}
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
pub type ssize_t = __ssize_t;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
pub struct labnode {
    pub lab: *mut libc::c_char,
    pub labd: libc::c_int,
    pub laba: libc::c_int,
    pub labc: libc::c_int,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
static mut startmem: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut start: [*mut libc::c_int; 2] = [0 as *const libc::c_int
    as *mut libc::c_int; 2];
unsafe extern "C" fn sortparse(mut line_0: *mut libc::c_char, mut which: libc::c_int) {
    let mut v: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut newfield: libc::c_int = 0;
    v = 0 as libc::c_int;
    l = 0 as libc::c_int;
    newfield = 1 as libc::c_int;
    while *line_0.offset(l as isize) as libc::c_int != 0
        && *line_0.offset(l as isize) as libc::c_int != '\n' as i32
    {
        if newfield != 0 {
            let fresh0 = v;
            v = v + 1;
            *(start[which as usize]).offset(fresh0 as isize) = l;
        }
        newfield = (*line_0.offset(l as isize) as libc::c_int == '|' as i32)
            as libc::c_int;
        l += 1;
        l;
    }
}
static mut mod_0: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut nmods: libc::c_int = 0;
static mut varv: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut nvar: libc::c_int = 0;
unsafe extern "C" fn fieldcmp(
    mut f1: *mut libc::c_char,
    mut f2: *mut libc::c_char,
) -> libc::c_int {
    let mut f: libc::c_int = 0;
    f = 0 as libc::c_int;
    while *f1.offset(f as isize) as libc::c_int != 0
        && *f1.offset(f as isize) as libc::c_int != '|' as i32
        && *f1.offset(f as isize) as libc::c_int != '\n' as i32
        && *f1.offset(f as isize) as libc::c_int == *f2.offset(f as isize) as libc::c_int
    {
        f += 1;
        f;
    }
    if *f1.offset(f as isize) as libc::c_int == *f2.offset(f as isize) as libc::c_int {
        return 0 as libc::c_int;
    }
    if *f1.offset(f as isize) == 0 || *f1.offset(f as isize) as libc::c_int == '|' as i32
        || *f1.offset(f as isize) as libc::c_int == '\n' as i32
    {
        return -(1 as libc::c_int);
    }
    if *f2.offset(f as isize) == 0 || *f2.offset(f as isize) as libc::c_int == '|' as i32
        || *f2.offset(f as isize) as libc::c_int == '\n' as i32
    {
        return 1 as libc::c_int;
    }
    return *f1.offset(f as isize) as libc::c_int - *f2.offset(f as isize) as libc::c_int;
}
unsafe extern "C" fn sortcmp(
    mut e0: *mut *mut libc::c_char,
    mut e1: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    cmp = 0 as libc::c_int;
    sortparse(*e0, 0 as libc::c_int);
    sortparse(*e1, 1 as libc::c_int);
    v = 0 as libc::c_int;
    while v < nvar {
        cmp = fieldcmp(
            (*e0)
                .offset(
                    *(start[0 as libc::c_int as usize])
                        .offset(*varv.offset(v as isize) as isize) as isize,
                ),
            (*e1)
                .offset(
                    *(start[1 as libc::c_int as usize])
                        .offset(*varv.offset(v as isize) as isize) as isize,
                ),
        );
        if cmp != 0 {
            break;
        }
        v += 1;
        v;
    }
    if nmods != 0 && *mod_0.offset(v as isize) as libc::c_int == 'd' as i32 {
        cmp = -cmp;
    }
    return cmp;
}
unsafe extern "C" fn linediff(
    mut l1: *mut libc::c_char,
    mut l2: *mut libc::c_char,
) -> libc::c_int {
    let mut l: libc::c_int = 0;
    l = 0 as libc::c_int;
    while *l1.offset(l as isize) as libc::c_int != 0
        && *l1.offset(l as isize) as libc::c_int != '\n' as i32
        && *l1.offset(l as isize) as libc::c_int == *l2.offset(l as isize) as libc::c_int
    {
        l += 1;
        l;
    }
    return (*l1.offset(l as isize) as libc::c_int
        != *l2.offset(l as isize) as libc::c_int) as libc::c_int;
}
pub unsafe extern "C" fn sort(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut modifiers: *mut libc::c_char,
) {
    static mut sortinit: libc::c_int = 0 as libc::c_int;
    let mut unique_0: libc::c_int = 0;
    let mut lastun: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut vn: libc::c_int = 0;
    let mut dsrt0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dsrt: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dfile: *mut libc::c_char = 0 as *mut libc::c_char;
    static mut line_0: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    let mut newline: libc::c_int = 0;
    let mut nlines: libc::c_int = 0;
    let mut c: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut flen: libc::c_int = 0;
    let mut scmp: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    if sortinit == 0 {
        sortinit = 1 as libc::c_int;
        line_0 = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dap_maxlines as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        startmem = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        start[0 as libc::c_int as usize] = startmem;
        start[1 as libc::c_int as usize] = startmem.offset(dap_maxvar as isize);
        mod_0 = dap_malloc(
            dap_maxvar,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        varv = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
    }
    if fname.is_null() {
        fprintf(
            dap_err,
            b"%s:sort: no dataset name given\n\0" as *const u8 as *const libc::c_char,
            dap_dapname,
        );
        exit(1 as libc::c_int);
    }
    scmp = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut *mut libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            sortcmp
                as unsafe extern "C" fn(
                    *mut *mut libc::c_char,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
    );
    unique_0 = 0 as libc::c_int;
    if !modifiers.is_null() {
        l = 0 as libc::c_int;
        while *modifiers.offset(l as isize) as libc::c_int == ' ' as i32 {
            l += 1;
            l;
        }
        nmods = 0 as libc::c_int;
        unique_0 = 0 as libc::c_int;
        while *modifiers.offset(l as isize) != 0 {
            if *modifiers.offset(l as isize) as libc::c_int == 'u' as i32 {
                unique_0 = 1 as libc::c_int;
                l += 1;
                l;
            } else if *modifiers.offset(l as isize) as libc::c_int == 'i' as i32
                || *modifiers.offset(l as isize) as libc::c_int == 'd' as i32
            {
                while *modifiers.offset(l as isize) as libc::c_int == 'i' as i32
                    || *modifiers.offset(l as isize) as libc::c_int == 'd' as i32
                {
                    let fresh1 = l;
                    l = l + 1;
                    let fresh2 = nmods;
                    nmods = nmods + 1;
                    *mod_0.offset(fresh2 as isize) = *modifiers.offset(fresh1 as isize);
                }
            } else {
                fprintf(
                    dap_err,
                    b"(sort) Bad modifier(s): %s\n\0" as *const u8
                        as *const libc::c_char,
                    modifiers,
                );
                exit(1 as libc::c_int);
            }
            while *modifiers.offset(l as isize) as libc::c_int == ' ' as i32 {
                l += 1;
                l;
            }
        }
    } else {
        nmods = 0 as libc::c_int;
    }
    dsrt0 = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        dsrt0,
        fname,
        b".srt\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    outset(dsrt0, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    nvar = dap_list(varlist, varv, dap_maxvar);
    if nmods != 0 && nvar != nmods {
        fprintf(
            dap_err,
            b"(sort) Number of modifiers %d does not equal number of sort variables %d.\n\0"
                as *const u8 as *const libc::c_char,
            nmods,
            nvar,
        );
        exit(1 as libc::c_int);
    }
    if *fname.offset(0 as libc::c_int as isize) as libc::c_int == '<' as i32 {
        nlines = 0 as libc::c_int;
        i = dap_ftell(dap_in[0 as libc::c_int as usize]) as libc::c_int;
        newline = 1 as libc::c_int;
        while (i as libc::c_long)
            < ((*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_end)
                .offset_from((*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_str)
                as libc::c_long
        {
            if newline != 0 {
                if nlines < dap_maxlines {
                    let fresh3 = nlines;
                    nlines = nlines + 1;
                    let ref mut fresh4 = *line_0.offset(fresh3 as isize);
                    *fresh4 = ((*(*dap_in[0 as libc::c_int as usize]).dfile_ram)
                        .rfile_str)
                        .offset(i as isize);
                } else {
                    fprintf(
                        dap_err,
                        b"(sort) Too many lines in ramfile %s\n\0" as *const u8
                            as *const libc::c_char,
                        fname,
                    );
                    exit(1 as libc::c_int);
                }
            }
            newline = (*((*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_str)
                .offset(i as isize) as libc::c_int == '\n' as i32) as libc::c_int;
            i += 1;
            i;
        }
        qsort(
            line_0 as *mut libc::c_void,
            nlines as size_t,
            ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
            ::std::mem::transmute::<
                Option::<unsafe extern "C" fn() -> libc::c_int>,
                __compar_fn_t,
            >(scmp),
        );
        l = 0 as libc::c_int;
        lastun = -(1 as libc::c_int);
        while l < nlines {
            if unique_0 == 0 || lastun < 0 as libc::c_int
                || linediff(*line_0.offset(lastun as isize), *line_0.offset(l as isize))
                    != 0
            {
                c = *line_0.offset(l as isize);
                while c < (*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_end
                    && *c as libc::c_int != '\n' as i32
                {
                    dap_putc(*c as libc::c_int, dap_out[0 as libc::c_int as usize]);
                    c = c.offset(1);
                    c;
                }
                dap_putc('\n' as i32, dap_out[0 as libc::c_int as usize]);
                lastun = l;
            } else {
                c = *line_0.offset(l as isize);
                while c < (*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_end
                    && *c as libc::c_int != '\n' as i32
                {
                    c = c.offset(1);
                    c;
                }
            }
            l += 1;
            l;
        }
        flen = ((*(*dap_out[0 as libc::c_int as usize]).dfile_ram).rfile_end)
            .offset_from((*(*dap_out[0 as libc::c_int as usize]).dfile_ram).rfile_str)
            as libc::c_long as libc::c_int;
        memcpy(
            (*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_str
                as *mut libc::c_void,
            (*(*dap_out[0 as libc::c_int as usize]).dfile_ram).rfile_str
                as *const libc::c_void,
            flen as libc::c_ulong,
        );
        (*(*dap_in[0 as libc::c_int as usize]).dfile_ram)
            .rfile_end = ((*(*dap_in[0 as libc::c_int as usize]).dfile_ram).rfile_str)
            .offset(flen as isize);
    } else {
        inset(0 as *mut libc::c_char);
        dfile = dap_malloc(
            (strlen(fname))
                .wrapping_add(strlen(dap_setdir))
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_name(dfile, fname);
        dsrt = dap_malloc(
            (strlen(dsrt0))
                .wrapping_add(strlen(dap_setdir))
                .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_name(dsrt, dsrt0);
        dsort(dfile, dsrt, varv, nvar, unique_0, mod_0, nmods);
        dap_free(
            dsrt as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        dap_free(
            dfile as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    dap_free(
        dsrt0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn printhead(
    mut formstr: *mut *mut libc::c_char,
    mut space: libc::c_int,
    mut fname: *mut libc::c_char,
    mut varv_0: *mut libc::c_int,
    mut nvar_0: libc::c_int,
) {
    let mut v: libc::c_int = 0;
    let mut d: libc::c_int = 0;
    let mut ttext: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wastitle: libc::c_int = 0;
    ttext = dap_malloc(
        (strlen(fname)).wrapping_add(11 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if !dap_title.is_null() {
        wastitle = 1 as libc::c_int;
    } else {
        wastitle = 0 as libc::c_int;
        strcpy(ttext, b"Printing: \0" as *const u8 as *const libc::c_char);
        strcat(ttext, fname);
        title(ttext);
    }
    dap_head(0 as *mut libc::c_void as *mut libc::c_int, 0 as libc::c_int);
    if space == ' ' as i32 {
        fprintf(dap_lst, b"  Obs \0" as *const u8 as *const libc::c_char);
    }
    v = 0 as libc::c_int;
    while v < nvar_0 {
        if !(space != ' ' as i32
            && strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv_0.offset(v as isize) as isize),
                b"_type_\0" as *const u8 as *const libc::c_char,
            ) == 0)
        {
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*varv_0.offset(v as isize) as isize) <= 0 as libc::c_int
            {
                if space == ' ' as i32 {
                    fprintf(
                        dap_lst,
                        b"%12s\0" as *const u8 as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv_0.offset(v as isize) as isize),
                    );
                } else {
                    fprintf(
                        dap_lst,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv_0.offset(v as isize) as isize),
                    );
                }
            } else {
                fprintf(
                    dap_lst,
                    *formstr.offset(v as isize),
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*varv_0.offset(v as isize) as isize),
                );
            }
            if v < nvar_0 - 1 as libc::c_int {
                putc(space, dap_lst);
            }
        }
        v += 1;
        v;
    }
    putc('\n' as i32, dap_lst);
    if space == ' ' as i32 {
        fprintf(dap_lst, b"----- \0" as *const u8 as *const libc::c_char);
        v = 0 as libc::c_int;
        while v < nvar_0 {
            d = 0 as libc::c_int;
            while *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                .offset(*varv_0.offset(v as isize) as isize))
                .offset(d as isize) != 0
            {
                putc('-' as i32, dap_lst);
                d += 1;
                d;
            }
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*varv_0.offset(v as isize) as isize) <= 0 as libc::c_int
            {
                while d < 12 as libc::c_int {
                    putc('-' as i32, dap_lst);
                    d += 1;
                    d;
                }
            } else {
                while d
                    < *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                        .offset(*varv_0.offset(v as isize) as isize)
                {
                    putc('-' as i32, dap_lst);
                    d += 1;
                    d;
                }
            }
            putc(' ' as i32, dap_lst);
            v += 1;
            v;
        }
        putc('\n' as i32, dap_lst);
    }
    if wastitle == 0 {
        title(0 as *mut libc::c_char);
    }
    dap_free(
        ttext as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn print(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
) {
    let mut varlist1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut space: libc::c_int = 0;
    let mut varv_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvar_0: libc::c_int = 0;
    let mut typen: libc::c_int = 0;
    let mut formmem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut formstr: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut v: libc::c_int = 0;
    let mut lenstr: libc::c_int = 0;
    let mut obn: libc::c_int = 0;
    varv_0 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    inset(fname);
    space = ' ' as i32;
    if !varlist.is_null()
        && *varlist.offset(0 as libc::c_int as isize) as libc::c_int != 0
    {
        varlist1 = dap_malloc(
            (strlen(varlist)).wrapping_add(1 as libc::c_int as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        v = 0 as libc::c_int;
        while *varlist.offset(v as isize) != 0 {
            if *varlist.offset(v as isize) as libc::c_int == '\t' as i32
                || *varlist.offset(v as isize) as libc::c_int == ',' as i32
            {
                *varlist1.offset(v as isize) = ' ' as i32 as libc::c_char;
                if space == ' ' as i32 {
                    space = *varlist.offset(v as isize) as libc::c_int;
                } else if space != *varlist.offset(v as isize) as libc::c_int {
                    fputs(
                        b"(print) variable list contains more than one type of separator\n\0"
                            as *const u8 as *const libc::c_char,
                        dap_err,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                *varlist1.offset(v as isize) = *varlist.offset(v as isize);
            }
            v += 1;
            v;
        }
        *varlist1.offset(v as isize) = '\0' as i32 as libc::c_char;
        let ref mut fresh5 = *varv_0.offset(0 as libc::c_int as isize);
        *fresh5 = dap_varnum(
            b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if *fresh5 < 0 as libc::c_int {
            fputs(
                b"(print) Missing _type_ variable.\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        typen = *varv_0.offset(0 as libc::c_int as isize);
        nvar_0 = 1 as libc::c_int
            + dap_list(
                varlist1,
                varv_0.offset(1 as libc::c_int as isize),
                dap_maxvar - 1 as libc::c_int,
            );
        if nvar_0 == 1 as libc::c_int {
            nvar_0 = (*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nvar;
            v = 0 as libc::c_int;
            while v < nvar_0 {
                *varv_0.offset(v as isize) = v;
                v += 1;
                v;
            }
        }
        dap_free(
            varlist1 as *mut libc::c_void,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    } else {
        nvar_0 = (*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nvar;
        v = 0 as libc::c_int;
        while v < nvar_0 {
            *varv_0.offset(v as isize) = v;
            v += 1;
            v;
        }
    }
    formmem = dap_malloc(
        nvar_0 * 10 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    formstr = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    v = 0 as libc::c_int;
    while v < nvar_0 {
        let ref mut fresh6 = *formstr.offset(v as isize);
        *fresh6 = formmem.offset((10 as libc::c_int * v) as isize);
        v += 1;
        v;
    }
    v = 0 as libc::c_int;
    while v < nvar_0 {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv_0.offset(v as isize) as isize) == 0 as libc::c_int
        {
            lenstr = strlen(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv_0.offset(v as isize) as isize),
            ) as libc::c_int;
            if lenstr < 12 as libc::c_int {
                lenstr = 12 as libc::c_int;
            }
            if space != ' ' as i32 {
                lenstr = 0 as libc::c_int;
            }
            sprintf(
                *formstr.offset(v as isize),
                b"%%%dd\0" as *const u8 as *const libc::c_char,
                lenstr,
            );
        } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*varv_0.offset(v as isize) as isize) == -(1 as libc::c_int)
        {
            lenstr = strlen(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv_0.offset(v as isize) as isize),
            ) as libc::c_int;
            if lenstr < 12 as libc::c_int {
                lenstr = 12 as libc::c_int;
            }
            if space != ' ' as i32 {
                lenstr = 0 as libc::c_int;
            }
            sprintf(
                *formstr.offset(v as isize),
                b"%%%dg\0" as *const u8 as *const libc::c_char,
                lenstr,
            );
        } else {
            lenstr = strlen(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*varv_0.offset(v as isize) as isize),
            ) as libc::c_int;
            if lenstr
                < *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                    .offset(*varv_0.offset(v as isize) as isize)
            {
                lenstr = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_len)
                    .offset(*varv_0.offset(v as isize) as isize);
            }
            if space != ' ' as i32 {
                lenstr = 0 as libc::c_int;
            }
            sprintf(
                *formstr.offset(v as isize),
                b"%%-%ds\0" as *const u8 as *const libc::c_char,
                lenstr,
            );
        }
        v += 1;
        v;
    }
    printhead(formstr, space, fname, varv_0, nvar_0);
    obn = 1 as libc::c_int;
    while step() != 0 {
        if space == ' ' as i32 {
            fprintf(dap_lst, b"%5d \0" as *const u8 as *const libc::c_char, obn);
        }
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if !(space != ' ' as i32 && *varv_0.offset(v as isize) == typen) {
                if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                    .offset(*varv_0.offset(v as isize) as isize) == 0 as libc::c_int
                {
                    fprintf(
                        dap_lst,
                        *formstr.offset(v as isize),
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_int)
                            .offset(*varv_0.offset(v as isize) as isize),
                    );
                } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_len)
                    .offset(*varv_0.offset(v as isize) as isize) == -(1 as libc::c_int)
                {
                    fprintf(
                        dap_lst,
                        *formstr.offset(v as isize),
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_dbl)
                            .offset(*varv_0.offset(v as isize) as isize),
                    );
                } else {
                    fprintf(
                        dap_lst,
                        *formstr.offset(v as isize),
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*varv_0.offset(v as isize) as isize),
                    );
                }
                if v < nvar_0 - 1 as libc::c_int {
                    putc(space, dap_lst);
                }
            }
            v += 1;
            v;
        }
        putc('\n' as i32, dap_lst);
        obn += 1;
        obn;
    }
    fflush(dap_lst);
    dap_free(
        varv_0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        formmem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        formstr as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn dap_mnsparse(
    mut varlist: *mut libc::c_char,
    mut outlist: *mut libc::c_char,
    mut varv_0: *mut libc::c_int,
    mut wtvar: *mut libc::c_int,
    mut stats: *mut libc::c_int,
) -> libc::c_int {
    let mut v: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tmplist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vn: libc::c_int = 0;
    let mut wn: libc::c_int = 0;
    let mut nvar_0: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut nonly: libc::c_int = 0;
    if varlist.is_null() {
        fputs(
            b"(meansparse) Missing variable list.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    vname = dap_malloc(
        dap_namelen + 6 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    v = 0 as libc::c_int;
    while *varlist.offset(v as isize) != 0 {
        v += 1;
        v;
    }
    v -= 1;
    v;
    while v >= 0 as libc::c_int
        && *varlist.offset(v as isize) as libc::c_int == ' ' as i32
    {
        v -= 1;
        v;
    }
    nvar_0 = 0 as libc::c_int;
    tmplist = dap_malloc(
        dap_listlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    *tmplist.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    wn = -(1 as libc::c_int);
    nonly = 0 as libc::c_int;
    while v >= 0 as libc::c_int {
        i = v;
        while i >= 0 as libc::c_int
            && *varlist.offset(i as isize) as libc::c_int != ' ' as i32
            && *varlist.offset(i as isize) as libc::c_int != '*' as i32
        {
            i -= 1;
            i;
        }
        j = 0 as libc::c_int;
        while j < v - i {
            if j < dap_namelen {
                *vname
                    .offset(
                        j as isize,
                    ) = *varlist.offset((i + j + 1 as libc::c_int) as isize);
            } else {
                *vname.offset(j as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(meansparse) Variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
            }
            j += 1;
            j;
        }
        *vname.offset(j as isize) = '\0' as i32 as libc::c_char;
        while i >= 0 as libc::c_int
            && *varlist.offset(i as isize) as libc::c_int == ' ' as i32
        {
            i -= 1;
            i;
        }
        vn = dap_varnum(vname);
        if vn >= 0 as libc::c_int {
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(vn as isize) == -(1 as libc::c_int)
            {
                if *tmplist.offset(0 as libc::c_int as isize) != 0 {
                    strcat(tmplist, b" \0" as *const u8 as *const libc::c_char);
                }
                strcat(tmplist, vname);
            } else {
                fprintf(
                    dap_err,
                    b"(meansparse) Variable must be double: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
        } else {
            s = 0 as libc::c_int;
            while s
                < 0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int
            {
                if s != 0 as libc::c_int && *stats.offset(s as isize) != 0 {
                    fprintf(
                        dap_err,
                        b"(meansparse) Statistics other than N requested for unknown variable %s\n\0"
                            as *const u8 as *const libc::c_char,
                        vname,
                    );
                    exit(1 as libc::c_int);
                }
                s += 1;
                s;
            }
            strcpy(tmplist, vname);
            strcat(vname, b" -1\0" as *const u8 as *const libc::c_char);
            vn = dap_vd(vname, 0 as libc::c_int);
            nonly = 1 as libc::c_int;
        }
        v = i;
        if v >= 0 as libc::c_int
            && *varlist.offset(v as isize) as libc::c_int == '*' as i32
        {
            wn = vn;
            v -= 1;
            v;
            while v >= 0 as libc::c_int
                && *varlist.offset(v as isize) as libc::c_int == ' ' as i32
            {
                v -= 1;
                v;
            }
        } else {
            *wtvar.offset(nvar_0 as isize) = wn;
            let fresh7 = nvar_0;
            nvar_0 = nvar_0 + 1;
            *varv_0.offset(fresh7 as isize) = vn;
        }
    }
    i = 0 as libc::c_int;
    while *tmplist.offset(i as isize) != 0 {
        i += 1;
        i;
    }
    i -= 1;
    i;
    while i >= 0 as libc::c_int
        && *tmplist.offset(i as isize) as libc::c_int == ' ' as i32
    {
        i -= 1;
        i;
    }
    *outlist.offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
    while i >= 0 as libc::c_int {
        j = i;
        while j > 0 as libc::c_int
            && *tmplist.offset((j - 1 as libc::c_int) as isize) as libc::c_int
                != ' ' as i32
        {
            j -= 1;
            j;
        }
        k = 0 as libc::c_int;
        while k <= i - j {
            *vname.offset(k as isize) = *tmplist.offset((j + k) as isize);
            k += 1;
            k;
        }
        *vname.offset(k as isize) = '\0' as i32 as libc::c_char;
        if *outlist.offset(0 as libc::c_int as isize) != 0 {
            strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
        }
        strcat(outlist, vname);
        i = j - 1 as libc::c_int;
        while i >= 0 as libc::c_int
            && *tmplist.offset(i as isize) as libc::c_int == ' ' as i32
        {
            i -= 1;
            i;
        }
    }
    dap_free(
        vname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        tmplist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if nonly != 0 {
        return -nvar_0;
    }
    return nvar_0;
}
pub static mut dap_sttnm: [[libc::c_char; 9]; 43] = unsafe {
    [
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"N\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SUM\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SUMWT\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"MEAN\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"MIN\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"MAX\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"RANGE\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"STEPXXXX\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"VAR\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"VARM\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SD\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SEM\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"VARFREQ\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"VARMFREQ\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SDFREQ\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SEMFREQ\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"T\0\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"TPROB\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"QRANGE\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SIGN\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SPROB\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SRANK\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"SRPROB\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"NORMAL\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"NPROB\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"P1\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"P5\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"P10\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"Q1\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"MED\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"Q3\0\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"P90\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"P95\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"P99\0\0\0\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"PXXXXX\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"PXXXXX\0\0\0"),
        *::std::mem::transmute::<&[u8; 9], &mut [libc::c_char; 9]>(b"PXXXXX\0\0\0"),
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
        [0; 9],
    ]
};
pub unsafe extern "C" fn dap_stats(
    mut statlist: *mut libc::c_char,
    mut stats: *mut libc::c_int,
) {
    let mut s: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut stat_0: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut sn: libc::c_int = 0;
    let mut pctpt: libc::c_double = 0.;
    let mut pctptn: libc::c_int = 0;
    s = 0 as libc::c_int;
    while s
        < 0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
    {
        *stats.offset(s as isize) = 0 as libc::c_int;
        s += 1;
        s;
    }
    if statlist.is_null() {
        return;
    }
    if stats.is_null() {
        fputs(
            b"(dap_stats) Missing statistics index list.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    s = 0 as libc::c_int;
    while *statlist.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    stat_0 = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    pctptn = 0 as libc::c_int;
    while *statlist.offset(s as isize) != 0 {
        i = 0 as libc::c_int;
        while *statlist.offset((s + i) as isize) as libc::c_int != 0
            && *statlist.offset((s + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *stat_0.offset(i as isize) = *statlist.offset((s + i) as isize);
            } else {
                *stat_0.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(dap_stats) Statistic name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    stat_0,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *stat_0.offset(i as isize) = '\0' as i32 as libc::c_char;
        if strcmp(stat_0, b"STD\0" as *const u8 as *const libc::c_char) == 0 {
            strcpy(stat_0, b"SD\0" as *const u8 as *const libc::c_char);
        } else if strcmp(stat_0, b"STDERR\0" as *const u8 as *const libc::c_char) == 0 {
            strcpy(stat_0, b"SEM\0" as *const u8 as *const libc::c_char);
        } else if strcmp(stat_0, b"PRT\0" as *const u8 as *const libc::c_char) == 0 {
            strcpy(stat_0, b"TPROB\0" as *const u8 as *const libc::c_char);
        } else if strcmp(stat_0, b"MEDIAN\0" as *const u8 as *const libc::c_char) == 0 {
            strcpy(stat_0, b"MED\0" as *const u8 as *const libc::c_char);
        }
        sn = 0 as libc::c_int;
        while sn
            < 0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int - 9 as libc::c_int + pctptn
        {
            if strcmp(stat_0, (dap_sttnm[sn as usize]).as_mut_ptr()) == 0 {
                *stats.offset(sn as isize) = 1 as libc::c_int;
                break;
            } else {
                sn += 1;
                sn;
            }
        }
        if sn
            == 0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int - 9 as libc::c_int + pctptn
        {
            if strncmp(
                stat_0,
                b"STEP\0" as *const u8 as *const libc::c_char,
                4 as libc::c_int as libc::c_ulong,
            ) == 0
            {
                *stat_0.offset(8 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                strcpy(
                    (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                        + 1 as libc::c_int + 1 as libc::c_int) as usize])
                        .as_mut_ptr(),
                    stat_0,
                );
                *stats
                    .offset(
                        (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                            + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                            + 1 as libc::c_int + 1 as libc::c_int) as isize,
                    ) = 1 as libc::c_int;
            } else if *stat_0.offset(0 as libc::c_int as isize) as libc::c_int
                == 'P' as i32
                && sscanf(
                    stat_0.offset(1 as libc::c_int as isize),
                    b"%lf\0" as *const u8 as *const libc::c_char,
                    &mut pctpt as *mut libc::c_double,
                ) == 1 as libc::c_int
            {
                let fresh8 = pctptn;
                pctptn = pctptn + 1;
                if fresh8 < 9 as libc::c_int {
                    *stats.offset(sn as isize) = 1 as libc::c_int;
                    let fresh9 = sn;
                    sn = sn + 1;
                    strcpy((dap_sttnm[fresh9 as usize]).as_mut_ptr(), stat_0);
                } else {
                    fprintf(
                        dap_err,
                        b"(dap_stats) Too many user-defined statistics: %s\n\0"
                            as *const u8 as *const libc::c_char,
                        stat_0,
                    );
                    exit(1 as libc::c_int);
                }
            } else {
                fprintf(
                    dap_err,
                    b"(dap_stats) Invalid statistic name: %s\n\0" as *const u8
                        as *const libc::c_char,
                    stat_0,
                );
                exit(1 as libc::c_int);
            }
        }
        s += i;
        while *statlist.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
    }
    dap_free(
        stat_0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn meansout(
    mut varv_0: *mut libc::c_int,
    mut nvar_0: libc::c_int,
    mut nobs: *mut libc::c_int,
    mut sum: *mut libc::c_double,
    mut sumwt: *mut libc::c_double,
    mut min: *mut libc::c_double,
    mut max: *mut libc::c_double,
    mut ss: *mut libc::c_double,
    mut stats: *mut libc::c_int,
) {
    let mut dn: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut typevar: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut nsteps: libc::c_int = 0;
    let mut step_0: libc::c_int = 0;
    let mut range: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut fract: libc::c_double = 0.;
    dap_swap();
    dn = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    v = 0 as libc::c_int;
    while v < nvar_0 {
        *dn.offset(v as isize) = *nobs.offset(v as isize) as libc::c_double;
        v += 1;
        v;
    }
    typevar = dap_varnum(
        b"_type_\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if typevar < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(meansout) Missing _type_ variable\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if *stats.offset(0 as libc::c_int as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"N\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *dn.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats.offset((0 as libc::c_int + 1 as libc::c_int) as isize) != 0 {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SUM\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *sum.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats.offset((0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize)
        != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SUMWT\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *sumwt.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int)
                as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"MEAN\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *sum.offset(v as isize) / *sumwt.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"MIN\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *min.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"MAX\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *max.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"RANGE\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *nobs.offset(v as isize) >= 1 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *max.offset(v as isize) - *min.offset(v as isize);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"VAR\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *dn.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *ss.offset(v as isize) / (*dn.offset(v as isize) - 1.0f64);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SD\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *dn.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = sqrt(*ss.offset(v as isize) / (*dn.offset(v as isize) - 1.0f64));
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SEM\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *dn.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = sqrt(
                    *ss.offset(v as isize)
                        / (*dn.offset(v as isize) * (*dn.offset(v as isize) - 1.0f64)),
                );
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"VARM\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *dn.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *ss.offset(v as isize)
                    / (*dn.offset(v as isize) * (*dn.offset(v as isize) - 1.0f64));
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"VARFREQ\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *sumwt.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *ss.offset(v as isize) / (*sumwt.offset(v as isize) - 1.0f64);
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SDFREQ\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *sumwt.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = sqrt(
                    *ss.offset(v as isize) / (*sumwt.offset(v as isize) - 1.0f64),
                );
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SEMFREQ\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *sumwt.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = sqrt(
                    *ss.offset(v as isize)
                        / (*sumwt.offset(v as isize)
                            * (*sumwt.offset(v as isize) - 1.0f64)),
                );
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"SEMFREQ\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *sumwt.offset(v as isize) > 1.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *ss.offset(v as isize)
                    / (*sumwt.offset(v as isize) * (*sumwt.offset(v as isize) - 1.0f64));
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        if sscanf(
            (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as usize])
                .as_mut_ptr()
                .offset(4 as libc::c_int as isize),
            b"%d\0" as *const u8 as *const libc::c_char,
            &mut nsteps as *mut libc::c_int,
        ) == 1 as libc::c_int
        {
            range = dap_malloc(
                (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                    .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as *mut libc::c_double;
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(typevar as isize),
                b"STEP\0" as *const u8 as *const libc::c_char,
            );
            v = 0 as libc::c_int;
            while v < nvar_0 {
                if *nobs.offset(v as isize) >= 1 as libc::c_int {
                    *range
                        .offset(
                            v as isize,
                        ) = *max.offset(v as isize) - *min.offset(v as isize);
                } else {
                    *range.offset(v as isize) = 0.0f64 / 0.0f64;
                }
                v += 1;
                v;
            }
            step_0 = 0 as libc::c_int;
            while step_0 <= nsteps {
                fract = step_0 as libc::c_double / nsteps as libc::c_double;
                v = 0 as libc::c_int;
                while v < nvar_0 {
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(
                            *varv_0.offset(v as isize) as isize,
                        ) = *min.offset(v as isize) + *range.offset(v as isize) * fract;
                    v += 1;
                    v;
                }
                output();
                step_0 += 1;
                step_0;
            }
            dap_free(
                range as *mut libc::c_void,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        } else {
            fprintf(
                dap_err,
                b"(meansout) Bad number of steps: %s\n\0" as *const u8
                    as *const libc::c_char,
                (dap_sttnm[(0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                    + 1 as libc::c_int + 1 as libc::c_int) as usize])
                    .as_mut_ptr()
                    .offset(4 as libc::c_int as isize),
            );
            exit(1 as libc::c_int);
        }
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"T\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *sumwt.offset(v as isize) > 0.0f64 && *ss.offset(v as isize) > 0.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = *sum.offset(v as isize) / *sumwt.offset(v as isize)
                    * sqrt(
                        *sumwt.offset(v as isize) * (*dn.offset(v as isize) - 1.0f64)
                            / *ss.offset(v as isize),
                    );
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    if *stats
        .offset(
            (0 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int + 1 as libc::c_int
                + 1 as libc::c_int + 1 as libc::c_int) as isize,
        ) != 0
    {
        strcpy(
            *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(typevar as isize),
            b"TPROB\0" as *const u8 as *const libc::c_char,
        );
        v = 0 as libc::c_int;
        while v < nvar_0 {
            if *sumwt.offset(v as isize) > 0.0f64 && *ss.offset(v as isize) > 0.0f64 {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(
                        *varv_0.offset(v as isize) as isize,
                    ) = 2.0f64
                    * probt(
                        fabs(
                            *sum.offset(v as isize) / *sumwt.offset(v as isize)
                                * sqrt(
                                    *sumwt.offset(v as isize)
                                        * (*dn.offset(v as isize) - 1.0f64) / *ss.offset(v as isize),
                                ),
                        ),
                        *nobs.offset(v as isize) - 1 as libc::c_int,
                    );
            } else {
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*varv_0.offset(v as isize) as isize) = 0.0f64 / 0.0f64;
            }
            v += 1;
            v;
        }
        output();
    }
    dap_swap();
    dap_free(
        dn as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn dap_list(
    mut varlist: *mut libc::c_char,
    mut varv_0: *mut libc::c_int,
    mut maxvars: libc::c_int,
) -> libc::c_int {
    let mut nvars_0: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut mname: *mut libc::c_char = 0 as *mut libc::c_char;
    if varlist.is_null() {
        return 0 as libc::c_int;
    }
    if varv_0.is_null() {
        fputs(
            b"(dap_list) Missing variable index list.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    m = 0 as libc::c_int;
    while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
        m += 1;
        m;
    }
    mname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nvars_0 = 0 as libc::c_int;
    while *varlist.offset(m as isize) != 0 {
        i = 0 as libc::c_int;
        while *varlist.offset((m + i) as isize) as libc::c_int != 0
            && *varlist.offset((m + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *mname.offset(i as isize) = *varlist.offset((m + i) as isize);
            } else {
                *mname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(dap_list) variable name too long: %s\n\0" as *const u8
                        as *const libc::c_char,
                    mname,
                );
                exit(1 as libc::c_int);
            }
            i += 1;
            i;
        }
        *mname.offset(i as isize) = '\0' as i32 as libc::c_char;
        if nvars_0 >= maxvars {
            fprintf(
                dap_err,
                b"(dap_list) More than %d variables: %s\n\0" as *const u8
                    as *const libc::c_char,
                maxvars,
                varlist,
            );
            exit(1 as libc::c_int);
        }
        let fresh10 = nvars_0;
        nvars_0 = nvars_0 + 1;
        let ref mut fresh11 = *varv_0.offset(fresh10 as isize);
        *fresh11 = dap_varnum(mname);
        if *fresh11 < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(dap_list) variable unknown: %s\n\0" as *const u8
                    as *const libc::c_char,
                mname,
            );
            exit(1 as libc::c_int);
        }
        m += i;
        while *varlist.offset(m as isize) as libc::c_int == ' ' as i32 {
            m += 1;
            m;
        }
    }
    dap_free(
        mname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return nvars_0;
}
pub unsafe extern "C" fn dap_newpart(
    mut markv: *mut libc::c_int,
    mut nmark: libc::c_int,
) -> libc::c_int {
    let mut marked: libc::c_int = 0;
    let mut m: libc::c_int = 0;
    marked = 0 as libc::c_int;
    if (*dap_prev.as_mut_ptr().offset(0 as libc::c_int as isize)).do_valid != 0 {
        if (*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_valid != 0 {
            m = 0 as libc::c_int;
            while m < nmark {
                if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                    .offset(*markv.offset(m as isize) as isize) > 0 as libc::c_int
                {
                    if strcmp(
                        *((*dap_prev.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*markv.offset(m as isize) as isize),
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*markv.offset(m as isize) as isize),
                    ) != 0
                    {
                        marked = 1 as libc::c_int;
                    }
                } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_len)
                    .offset(*markv.offset(m as isize) as isize) == 0 as libc::c_int
                {
                    if *((*dap_prev.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_int)
                        .offset(*markv.offset(m as isize) as isize)
                        != *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_int)
                            .offset(*markv.offset(m as isize) as isize)
                    {
                        marked = 1 as libc::c_int;
                    }
                } else if *((*dap_prev.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_dbl)
                    .offset(*markv.offset(m as isize) as isize)
                    != *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                        .do_dbl)
                        .offset(*markv.offset(m as isize) as isize)
                {
                    marked = 1 as libc::c_int;
                }
                m += 1;
                m;
            }
        } else {
            marked = 1 as libc::c_int;
        }
    }
    return marked;
}
pub unsafe extern "C" fn means(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut statlist: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut stats: [libc::c_int; 43] = [0; 43];
    let mut nonly: libc::c_int = 0;
    let mut varv_0: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut markv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nvar_0: libc::c_int = 0;
    let mut nmark: libc::c_int = 0;
    let mut nobs: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut outlist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut wtvar: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut sum: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut sumwt: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut ss: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut min: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut max: *mut libc::c_double = 0 as *mut libc::c_double;
    let mut v: libc::c_int = 0;
    let mut wt: libc::c_double = 0.;
    let mut vtmp: libc::c_double = 0.;
    let mut tmp: libc::c_double = 0.;
    let mut nnan: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut more: libc::c_int = 0;
    if fname.is_null() {
        fputs(
            b"(means) Missing input dataset name.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_suffix(
        outname,
        fname,
        b".mns\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    varv_0 = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    markv = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    wtvar = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    outlist = dap_malloc(
        dap_listlen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    inset(fname);
    dap_stats(statlist, stats.as_mut_ptr());
    nvar_0 = dap_mnsparse(varlist, outlist, varv_0, wtvar, stats.as_mut_ptr());
    nonly = (nvar_0 < 0 as libc::c_int) as libc::c_int;
    if nonly != 0 {
        nvar_0 = -nvar_0;
    }
    nobs = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    nnan = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    if !marks.is_null() && *marks.offset(0 as libc::c_int as isize) as libc::c_int != 0 {
        strcat(outlist, b" \0" as *const u8 as *const libc::c_char);
        strcat(outlist, marks);
    }
    outset(outname, outlist);
    nmark = dap_list(marks, markv, dap_maxvar);
    sum = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    sumwt = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    ss = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    min = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    max = dap_malloc(
        (::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
            .wrapping_mul(nvar_0 as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_double;
    v = 0 as libc::c_int;
    while v < nvar_0 {
        *sum.offset(v as isize) = 0.0f64;
        *sumwt.offset(v as isize) = 0.0f64;
        *ss.offset(v as isize) = 0.0f64;
        *nobs.offset(v as isize) = 0 as libc::c_int;
        *nnan.offset(v as isize) = 0 as libc::c_int;
        v += 1;
        v;
    }
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            meansout(varv_0, nvar_0, nobs, sum, sumwt, min, max, ss, stats.as_mut_ptr());
            v = 0 as libc::c_int;
            while v < nvar_0 {
                if *nnan.offset(v as isize) != 0 {
                    dap_swap();
                    fprintf(
                        dap_log,
                        b"(means) %d NaNs for %s\n\0" as *const u8
                            as *const libc::c_char,
                        *nnan.offset(v as isize),
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_nam)
                            .offset(*varv_0.offset(v as isize) as isize),
                    );
                    dap_swap();
                }
                *sum.offset(v as isize) = 0.0f64;
                *sumwt.offset(v as isize) = 0.0f64;
                *ss.offset(v as isize) = 0.0f64;
                *nobs.offset(v as isize) = 0 as libc::c_int;
                *nnan.offset(v as isize) = 0 as libc::c_int;
                v += 1;
                v;
            }
        }
        v = 0 as libc::c_int;
        while v < nvar_0 {
            vtmp = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                .offset(*varv_0.offset(v as isize) as isize);
            if *wtvar.offset(v as isize) >= 0 as libc::c_int {
                wt = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                    .offset(*wtvar.offset(v as isize) as isize);
            } else {
                wt = 1.0f64;
            }
            if finite(vtmp) != 0 && finite(wt) != 0 {
                if *nobs.offset(v as isize) == 0 {
                    *min.offset(v as isize) = vtmp;
                    *max.offset(v as isize) = vtmp;
                } else {
                    if vtmp < *min.offset(v as isize) {
                        *min.offset(v as isize) = vtmp;
                    }
                    if vtmp > *max.offset(v as isize) {
                        *max.offset(v as isize) = vtmp;
                    }
                    tmp = *sum.offset(v as isize) - *sumwt.offset(v as isize) * vtmp;
                    *ss.offset(v as isize)
                        += tmp * tmp * wt
                            / (*sumwt.offset(v as isize)
                                * (*sumwt.offset(v as isize) + wt));
                }
                *sumwt.offset(v as isize) += wt;
                *sum.offset(v as isize) += vtmp * wt;
                let ref mut fresh12 = *nobs.offset(v as isize);
                *fresh12 += 1;
                *fresh12;
            } else if nonly != 0 {
                let ref mut fresh13 = *nobs.offset(v as isize);
                *fresh13 += 1;
                *fresh13;
            } else {
                let ref mut fresh14 = *nnan.offset(v as isize);
                *fresh14 += 1;
                *fresh14;
            }
            v += 1;
            v;
        }
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varv_0 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        markv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nobs as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outlist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        wtvar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sum as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sumwt as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        ss as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        min as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        max as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        nnan as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
static mut tabform: [libc::c_char; 7] = [0; 7];
static mut emptyform: [libc::c_char; 5] = [0; 5];
static mut cellwidth: libc::c_int = 0;
static mut tabvalmem: *mut libc::c_double = 0 as *const libc::c_double
    as *mut libc::c_double;
static mut tableval: *mut *mut libc::c_double = 0 as *const *mut libc::c_double
    as *mut *mut libc::c_double;
static mut valsetmem: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut valset: *mut *mut libc::c_int = 0 as *const *mut libc::c_int
    as *mut *mut libc::c_int;
static mut nrows: libc::c_int = 0;
static mut ncols: libc::c_int = 0;
static mut collabel: *mut labnode = 0 as *const labnode as *mut labnode;
static mut labroot: libc::c_int = 0;
static mut nextclab: libc::c_int = 0;
static mut rlabmem: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut rlptrmem: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut rowlabel: *mut *mut *mut libc::c_char = 0 as *const *mut *mut libc::c_char
    as *mut *mut *mut libc::c_char;
static mut rtitlesp: libc::c_int = 0;
static mut rowvar: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut nrowvar: libc::c_int = 0;
static mut colvar: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut ncolvar: libc::c_int = 0;
static mut colsort: libc::c_int = 0;
unsafe extern "C" fn newlab(mut lname: *mut libc::c_char) -> libc::c_int {
    if nextclab == dap_maxclab {
        fprintf(
            dap_err,
            b"(newlab) too many column labels: %s\n\0" as *const u8
                as *const libc::c_char,
            lname,
        );
        exit(1 as libc::c_int);
    }
    strcpy((*collabel.offset(nextclab as isize)).lab, lname as *const libc::c_char);
    (*collabel.offset(nextclab as isize)).labd = -(1 as libc::c_int);
    (*collabel.offset(nextclab as isize)).laba = -(1 as libc::c_int);
    (*collabel.offset(nextclab as isize)).labc = -(1 as libc::c_int);
    let fresh15 = nextclab;
    nextclab = nextclab + 1;
    return fresh15;
}
unsafe extern "C" fn nodecnt(mut clab: libc::c_int) -> libc::c_int {
    let mut across: libc::c_int = 0;
    let mut totalcnt: libc::c_int = 0;
    if (*collabel.offset(clab as isize)).labd < 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    across = (*collabel.offset(clab as isize)).labd;
    totalcnt = 0 as libc::c_int;
    while across >= 0 as libc::c_int {
        totalcnt += nodecnt(across);
        across = (*collabel.offset(across as isize)).laba;
    }
    return totalcnt;
}
unsafe extern "C" fn labelprint(mut name: *mut libc::c_char, mut width: libc::c_int) {
    static mut label: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
    let mut c: libc::c_int = 0;
    if label.is_null() {
        label = dap_malloc(
            dap_strlen + 1 as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    strcpy(label, name as *const libc::c_char);
    c = 0 as libc::c_int;
    while *label.offset(c as isize) as libc::c_int != 0 && c < width {
        c += 1;
        c;
    }
    while c < width {
        let fresh16 = c;
        c = c + 1;
        *label.offset(fresh16 as isize) = ' ' as i32 as libc::c_char;
    }
    *label.offset(c as isize) = '\0' as i32 as libc::c_char;
    fprintf(dap_lst, b"%s\0" as *const u8 as *const libc::c_char, label);
}
unsafe extern "C" fn divider(
    mut left: libc::c_int,
    mut conn: libc::c_int,
    mut sep: libc::c_int,
    mut right: libc::c_int,
    mut nblank: libc::c_int,
) {
    let mut col: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut connect: libc::c_int = 0;
    putc(left, dap_lst);
    col = 0 as libc::c_int;
    while col < nrowvar {
        if col < nblank {
            connect = ' ' as i32;
        } else {
            connect = conn;
        }
        c = 0 as libc::c_int;
        while c < rtitlesp {
            putc(connect, dap_lst);
            c += 1;
            c;
        }
        if col < nrowvar - 1 as libc::c_int {
            if col < nblank {
                putc(left, dap_lst);
            } else {
                putc(sep, dap_lst);
            }
        } else {
            putc(right, dap_lst);
        }
        col += 1;
        col;
    }
    col = 0 as libc::c_int;
    while col < ncols {
        c = 0 as libc::c_int;
        while c < cellwidth {
            putc(conn, dap_lst);
            c += 1;
            c;
        }
        if col < ncols - 1 as libc::c_int {
            putc(sep, dap_lst);
        } else {
            putc(right, dap_lst);
        }
        col += 1;
        col;
    }
    putc('\n' as i32, dap_lst);
}
unsafe extern "C" fn tableline(mut start_0: libc::c_int, mut depth: libc::c_int) {
    let mut across: libc::c_int = 0;
    let mut cnt: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    across = start_0;
    while across >= 0 as libc::c_int {
        if depth == 0 {
            labelprint((*collabel.offset(across as isize)).lab, cellwidth);
            cnt = nodecnt(across);
            c = 0 as libc::c_int;
            while c < (cnt - 1 as libc::c_int) * (cellwidth + 1 as libc::c_int) {
                putc(' ' as i32, dap_lst);
                c += 1;
                c;
            }
            putc('|' as i32, dap_lst);
        } else {
            tableline(
                (*collabel.offset(across as isize)).labd,
                depth - 1 as libc::c_int,
            );
        }
        across = (*collabel.offset(across as isize)).laba;
    }
}
unsafe extern "C" fn tablehead() {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    putc(' ' as i32, dap_lst);
    col = 0 as libc::c_int;
    while col < nrowvar {
        c = 0 as libc::c_int;
        while c < rtitlesp + 1 as libc::c_int {
            putc(' ' as i32, dap_lst);
            c += 1;
            c;
        }
        col += 1;
        col;
    }
    fputs(
        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
            .offset(*colvar.offset((ncolvar - 1 as libc::c_int) as isize) as isize),
        dap_lst,
    );
    if *colvar.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
        fputs(b" for \0" as *const u8 as *const libc::c_char, dap_lst);
        col = 0 as libc::c_int;
        while col < ncolvar - 1 as libc::c_int {
            fprintf(
                dap_lst,
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                    .offset(*colvar.offset(col as isize) as isize),
            );
            if col < ncolvar - 2 as libc::c_int {
                fprintf(dap_lst, b" / \0" as *const u8 as *const libc::c_char);
            }
            col += 1;
            col;
        }
    }
    putc('\n' as i32, dap_lst);
    divider('=' as i32, '=' as i32, '=' as i32, '=' as i32, 0 as libc::c_int);
    row = 0 as libc::c_int;
    while row < ncolvar - 1 as libc::c_int {
        if row < ncolvar - 2 as libc::c_int {
            putc('|' as i32, dap_lst);
            col = 0 as libc::c_int;
            while col < nrowvar {
                c = 0 as libc::c_int;
                while c < rtitlesp {
                    putc(' ' as i32, dap_lst);
                    c += 1;
                    c;
                }
                if col < nrowvar - 1 as libc::c_int {
                    putc(' ' as i32, dap_lst);
                } else {
                    putc('|' as i32, dap_lst);
                }
                col += 1;
                col;
            }
        } else {
            putc('|' as i32, dap_lst);
            col = 0 as libc::c_int;
            while col < nrowvar {
                labelprint(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_nam)
                        .offset(*rowvar.offset(col as isize) as isize),
                    rtitlesp,
                );
                putc('|' as i32, dap_lst);
                col += 1;
                col;
            }
        }
        tableline(labroot, row);
        putc('\n' as i32, dap_lst);
        if row < ncolvar - 2 as libc::c_int {
            divider('|' as i32, '-' as i32, '+' as i32, '|' as i32, 0 as libc::c_int);
        }
        row += 1;
        row;
    }
    divider('|' as i32, '=' as i32, '|' as i32, '|' as i32, 0 as libc::c_int);
}
unsafe extern "C" fn valprint(mut row: libc::c_int, mut node: libc::c_int) {
    while node >= 0 as libc::c_int {
        if (*collabel.offset(node as isize)).labd >= 0 as libc::c_int {
            valprint(row, (*collabel.offset(node as isize)).labd);
        } else {
            if *(*valset.offset(row as isize))
                .offset((*collabel.offset(node as isize)).labc as isize) != 0
            {
                fprintf(
                    dap_lst,
                    tabform.as_mut_ptr(),
                    *(*tableval.offset(row as isize))
                        .offset((*collabel.offset(node as isize)).labc as isize),
                );
            } else {
                fprintf(
                    dap_lst,
                    emptyform.as_mut_ptr(),
                    b"\0" as *const u8 as *const libc::c_char,
                );
            }
            putc('|' as i32, dap_lst);
        }
        node = (*collabel.offset(node as isize)).laba;
    }
}
unsafe extern "C" fn tableprint() {
    let mut row: libc::c_int = 0;
    let mut col: libc::c_int = 0;
    let mut isblank: libc::c_int = 0;
    let mut nextblank: libc::c_int = 0;
    let mut nblank: libc::c_int = 0;
    let mut nextnblank: libc::c_int = 0;
    row = 0 as libc::c_int;
    while row <= nrows {
        putc('|' as i32, dap_lst);
        nblank = 0 as libc::c_int;
        nextnblank = 0 as libc::c_int;
        col = 0 as libc::c_int;
        isblank = 1 as libc::c_int;
        nextblank = 1 as libc::c_int;
        while col < nrowvar {
            if isblank != 0
                && *(*(*rowlabel.offset(row as isize)).offset(col as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                nblank = col;
                isblank = 0 as libc::c_int;
            }
            if nextblank != 0 && row <= nrows - 1 as libc::c_int
                && *(*(*rowlabel.offset((row + 1 as libc::c_int) as isize))
                    .offset(col as isize))
                    .offset(0 as libc::c_int as isize) as libc::c_int != 0
            {
                nextnblank = col;
                nextblank = 0 as libc::c_int;
            }
            labelprint(*(*rowlabel.offset(row as isize)).offset(col as isize), rtitlesp);
            putc('|' as i32, dap_lst);
            col += 1;
            col;
        }
        valprint(row, labroot);
        putc('\n' as i32, dap_lst);
        if nextnblank != nblank {
            nblank = nextnblank;
        }
        if row <= nrows - 1 as libc::c_int {
            divider('|' as i32, '-' as i32, '+' as i32, '|' as i32, nblank);
        } else {
            divider('-' as i32, '-' as i32, '-' as i32, '-' as i32, 0 as libc::c_int);
        }
        row += 1;
        row;
    }
}
unsafe extern "C" fn findcol() -> libc::c_int {
    let mut varn: libc::c_int = 0;
    let mut node: libc::c_int = 0;
    let mut prevnode: libc::c_int = 0;
    let mut nextnode: libc::c_int = 0;
    let mut upnode: libc::c_int = 0;
    let mut cmp: libc::c_int = 0;
    let mut label: *mut libc::c_char = 0 as *mut libc::c_char;
    nextnode = -(1 as libc::c_int);
    if colsort != 0 && *colvar.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
        node = labroot;
        varn = 0 as libc::c_int;
        upnode = -(1 as libc::c_int);
        while varn < ncolvar - 1 as libc::c_int {
            if node >= 0 as libc::c_int {
                nextnode = (*collabel.offset(node as isize)).laba;
                while nextnode >= 0 as libc::c_int
                    && strcmp(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*colvar.offset(varn as isize) as isize),
                        (*collabel.offset(nextnode as isize)).lab,
                    ) >= 0 as libc::c_int
                {
                    node = nextnode;
                    nextnode = (*collabel.offset(nextnode as isize)).laba;
                }
                cmp = strcmp(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*colvar.offset(varn as isize) as isize),
                    (*collabel.offset(node as isize)).lab,
                );
            } else {
                cmp = -(1 as libc::c_int);
            }
            if cmp < 0 as libc::c_int {
                if upnode >= 0 as libc::c_int {
                    nextnode = node;
                    node = newlab(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*colvar.offset(varn as isize) as isize),
                    );
                    (*collabel.offset(upnode as isize)).labd = node;
                    (*collabel.offset(node as isize)).laba = nextnode;
                } else {
                    labroot = newlab(
                        *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                            .do_str)
                            .offset(*colvar.offset(varn as isize) as isize),
                    );
                    (*collabel.offset(labroot as isize)).laba = node;
                    node = labroot;
                }
            } else if cmp > 0 as libc::c_int {
                (*collabel.offset(node as isize))
                    .laba = newlab(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*colvar.offset(varn as isize) as isize),
                );
                node = (*collabel.offset(node as isize)).laba;
                (*collabel.offset(node as isize)).laba = nextnode;
            }
            upnode = node;
            node = (*collabel.offset(node as isize)).labd;
            varn += 1;
            varn;
        }
    } else {
        node = labroot;
        varn = 0 as libc::c_int;
        upnode = -(1 as libc::c_int);
        while varn < ncolvar - 1 as libc::c_int {
            if *colvar.offset(0 as libc::c_int as isize) >= 0 as libc::c_int {
                label = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize))
                    .do_str)
                    .offset(*colvar.offset(varn as isize) as isize);
            } else {
                label = b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
            }
            prevnode = -(1 as libc::c_int);
            while node >= 0 as libc::c_int
                && strcmp(label, (*collabel.offset(node as isize)).lab) != 0
            {
                prevnode = node;
                node = (*collabel.offset(node as isize)).laba;
            }
            if node < 0 as libc::c_int {
                node = newlab(label);
                if prevnode >= 0 as libc::c_int {
                    (*collabel.offset(prevnode as isize)).laba = node;
                } else if upnode >= 0 as libc::c_int {
                    (*collabel.offset(upnode as isize)).labd = node;
                } else {
                    labroot = node;
                }
            }
            upnode = node;
            node = (*collabel.offset(node as isize)).labd;
            varn += 1;
            varn;
        }
    }
    if (*collabel.offset(upnode as isize)).labc < 0 as libc::c_int {
        if ncols >= dap_maxcols {
            fputs(
                b"(findcol) too many columns in table\n\0" as *const u8
                    as *const libc::c_char,
                dap_err,
            );
            exit(1 as libc::c_int);
        }
        let fresh17 = ncols;
        ncols = ncols + 1;
        (*collabel.offset(upnode as isize)).labc = fresh17;
    }
    return (*collabel.offset(upnode as isize)).labc;
}
unsafe extern "C" fn tableform(mut tform: *mut libc::c_char) {
    let mut width: [libc::c_char; 7] = [0; 7];
    let mut w: libc::c_int = 0;
    let mut forg: libc::c_int = 0;
    strcpy(width.as_mut_ptr(), tform as *const libc::c_char);
    w = 0 as libc::c_int;
    while width[w as usize] as libc::c_int != 0
        && width[w as usize] as libc::c_int != '.' as i32
    {
        w += 1;
        w;
    }
    if width[w as usize] as libc::c_int == '.' as i32 {
        forg = 'f' as i32;
    } else {
        forg = 'g' as i32;
    }
    width[w as usize] = '\0' as i32 as libc::c_char;
    cellwidth = atoi(width.as_mut_ptr());
    strcpy(tabform.as_mut_ptr(), b"%\0" as *const u8 as *const libc::c_char);
    if forg == 'f' as i32 {
        strcat(tabform.as_mut_ptr(), tform as *const libc::c_char);
        strcat(tabform.as_mut_ptr(), b"f\0" as *const u8 as *const libc::c_char);
    } else {
        strcat(tabform.as_mut_ptr(), width.as_mut_ptr());
        strcat(tabform.as_mut_ptr(), b"g\0" as *const u8 as *const libc::c_char);
    }
    sprintf(
        emptyform.as_mut_ptr(),
        b"%%%ds\0" as *const u8 as *const libc::c_char,
        cellwidth,
    );
}
unsafe extern "C" fn specparse(
    mut rowvars: *mut libc::c_char,
    mut colvars: *mut libc::c_char,
    mut format: *mut libc::c_char,
) {
    let mut t: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut sp: libc::c_int = 0;
    let mut vname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut v: libc::c_int = 0;
    vname = dap_malloc(
        dap_namelen + 1 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    nrowvar = 0 as libc::c_int;
    ncolvar = 0 as libc::c_int;
    t = 0 as libc::c_int;
    while *rowvars.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    while *rowvars.offset(t as isize) != 0 {
        while *rowvars.offset(t as isize) as libc::c_int == ' ' as i32 {
            t += 1;
            t;
        }
        i = 0 as libc::c_int;
        while *rowvars.offset((t + i) as isize) as libc::c_int != 0
            && *rowvars.offset((t + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *vname.offset(i as isize) = *rowvars.offset((t + i) as isize);
            } else {
                *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(specparse) Row variable name too long: %s\n\0" as *const u8
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
            if nrowvar < dap_maxrowv {
                let fresh18 = nrowvar;
                nrowvar = nrowvar + 1;
                *rowvar.offset(fresh18 as isize) = v;
            } else {
                fprintf(
                    dap_err,
                    b"(specparse) Too many row variables: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                dap_err,
                b"(specparse) Unknown row variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
            exit(1 as libc::c_int);
        }
        t += i;
        while *rowvars.offset(t as isize) as libc::c_int == ' ' as i32 {
            t += 1;
            t;
        }
    }
    t = 0 as libc::c_int;
    while *colvars.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    while *colvars.offset(t as isize) != 0 {
        i = 0 as libc::c_int;
        while *colvars.offset((t + i) as isize) as libc::c_int != 0
            && *colvars.offset((t + i) as isize) as libc::c_int != ' ' as i32
        {
            if i < dap_namelen {
                *vname.offset(i as isize) = *colvars.offset((t + i) as isize);
            } else {
                *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
                fprintf(
                    dap_err,
                    b"(specparse) Column variable name too long: %s\n\0" as *const u8
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
            if ncolvar < dap_maxcolv {
                let fresh19 = ncolvar;
                ncolvar = ncolvar + 1;
                *colvar.offset(fresh19 as isize) = v;
            } else {
                fprintf(
                    dap_err,
                    b"(specparse) Too many column variables: %s\n\0" as *const u8
                        as *const libc::c_char,
                    vname,
                );
                exit(1 as libc::c_int);
            }
        } else {
            fprintf(
                dap_err,
                b"(specparse) Unknown column variable: %s\n\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
            exit(1 as libc::c_int);
        }
        t += i;
        while *colvars.offset(t as isize) as libc::c_int == ' ' as i32 {
            t += 1;
            t;
        }
    }
    t = 0 as libc::c_int;
    while *format.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    if *format.offset(t as isize) as libc::c_int == 's' as i32 {
        colsort = 1 as libc::c_int;
        t += 1;
        t;
        while *format.offset(t as isize) as libc::c_int == ' ' as i32 {
            t += 1;
            t;
        }
    } else {
        colsort = 0 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while *format.offset((t + i) as isize) as libc::c_int != 0
        && *format.offset((t + i) as isize) as libc::c_int != ' ' as i32
    {
        if i < dap_namelen {
            *vname.offset(i as isize) = *format.offset((t + i) as isize);
        } else {
            *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
            fprintf(
                dap_err,
                b"(specparse) Format too long %s\n\0" as *const u8
                    as *const libc::c_char,
                vname,
            );
            exit(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    *vname.offset(i as isize) = '\0' as i32 as libc::c_char;
    tableform(vname);
    t += i;
    while *format.offset(t as isize) as libc::c_int == ' ' as i32 {
        t += 1;
        t;
    }
    if *format.offset(t as isize) != 0 {
        sp = 0 as libc::c_int;
        while '0' as i32 <= *format.offset(t as isize) as libc::c_int
            && *format.offset(t as isize) as libc::c_int <= '9' as i32
        {
            sp = 10 as libc::c_int * sp + *format.offset(t as isize) as libc::c_int
                - '0' as i32;
            t += 1;
            t;
        }
        if *format.offset(t as isize) != 0 {
            fprintf(
                dap_err,
                b"(specparse) Extra character(s) at end of format: %s\n\0" as *const u8
                    as *const libc::c_char,
                format,
            );
            exit(1 as libc::c_int);
        }
        rtitlesp = (sp - 1 as libc::c_int) / nrowvar;
    }
    if tabform[0 as libc::c_int as usize] == 0 {
        fprintf(
            dap_err,
            b"(specparse) No format\n\0" as *const u8 as *const libc::c_char,
        );
        exit(1 as libc::c_int);
    }
    if ncolvar == 0 {
        fputs(
            b"(specparse) No column or analysis variable(s) specified.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if ncolvar < 2 as libc::c_int {
        *colvar
            .offset(
                1 as libc::c_int as isize,
            ) = *colvar.offset(0 as libc::c_int as isize);
        *colvar.offset(0 as libc::c_int as isize) = -(1 as libc::c_int);
        ncolvar = 2 as libc::c_int;
    }
    dap_free(
        vname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn table(
    mut fname: *mut libc::c_char,
    mut rowvars: *mut libc::c_char,
    mut colvars: *mut libc::c_char,
    mut format: *mut libc::c_char,
    mut marks: *mut libc::c_char,
) {
    static mut tabinit: libc::c_int = 0 as libc::c_int;
    static mut prevmem: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut prev: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
        as *mut *mut libc::c_char;
    let mut r: libc::c_int = 0;
    let mut s: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    static mut markv: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
    let mut nmark: libc::c_int = 0;
    let mut more: libc::c_int = 0;
    static mut nstring: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    if tabinit == 0 {
        tabinit = 1 as libc::c_int;
        valsetmem = dap_malloc(
            ((dap_maxrows * dap_maxcols) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        valset = dap_malloc(
            (dap_maxrows as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_int;
        r = 0 as libc::c_int;
        while r < dap_maxrows {
            let ref mut fresh20 = *valset.offset(r as isize);
            *fresh20 = valsetmem.offset((dap_maxcols * r) as isize);
            r += 1;
            r;
        }
        tabvalmem = dap_malloc(
            ((dap_maxrows * dap_maxcols) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_double;
        tableval = dap_malloc(
            (dap_maxrows as libc::c_ulong)
                .wrapping_mul(
                    ::std::mem::size_of::<*mut libc::c_double>() as libc::c_ulong,
                ) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_double;
        r = 0 as libc::c_int;
        while r < dap_maxrows {
            let ref mut fresh21 = *tableval.offset(r as isize);
            *fresh21 = tabvalmem.offset((dap_maxcols * r) as isize);
            r += 1;
            r;
        }
        collabel = dap_malloc(
            (dap_maxclab as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<labnode>() as libc::c_ulong)
                as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut labnode;
        c = 0 as libc::c_int;
        while c < dap_maxclab {
            let ref mut fresh22 = (*collabel.offset(c as isize)).lab;
            *fresh22 = dap_malloc(
                dap_lablen + 1 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
            c += 1;
            c;
        }
        rowvar = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        colvar = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        rlabmem = dap_malloc(
            dap_maxrows * dap_maxrowv * (dap_lablen + 1 as libc::c_int),
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        rlptrmem = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dap_maxrows as libc::c_ulong)
                .wrapping_mul(dap_maxrowv as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        rowlabel = dap_malloc(
            (::std::mem::size_of::<*mut *mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dap_maxrows as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut *mut libc::c_char;
        r = 0 as libc::c_int;
        while r < dap_maxrows {
            let ref mut fresh23 = *rowlabel.offset(r as isize);
            *fresh23 = rlptrmem.offset((r * dap_maxrowv) as isize);
            v = 0 as libc::c_int;
            while v < dap_maxrowv {
                let ref mut fresh24 = *(*rowlabel.offset(r as isize)).offset(v as isize);
                *fresh24 = rlabmem
                    .offset(
                        (r * (dap_maxrowv * (dap_lablen + 1 as libc::c_int))) as isize,
                    )
                    .offset((v * (dap_lablen + 1 as libc::c_int)) as isize);
                v += 1;
                v;
            }
            r += 1;
            r;
        }
        prevmem = dap_malloc(
            dap_maxrowv * (dap_lablen + 1 as libc::c_int),
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        prev = dap_malloc(
            (::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
                .wrapping_mul(dap_maxrowv as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut *mut libc::c_char;
        v = 0 as libc::c_int;
        while v < dap_maxrowv {
            let ref mut fresh25 = *prev.offset(v as isize);
            *fresh25 = prevmem.offset((v * (dap_lablen + 1 as libc::c_int)) as isize);
            v += 1;
            v;
        }
        markv = dap_malloc(
            (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
            b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) as *mut libc::c_int;
        nstring = dap_malloc(
            dap_strlen + 1 as libc::c_int,
            b"dap_strlen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
    }
    if fname.is_null() {
        fputs(
            b"(table) no dataset name given\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    inset(fname);
    tabform[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    rtitlesp = 8 as libc::c_int;
    if rowvars.is_null() || colvars.is_null() {
        fputs(
            b"(table) no row and/or column variables specified\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    if format.is_null() {
        fputs(
            b"(table) no format given\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    specparse(rowvars, colvars, format);
    nmark = dap_list(marks, markv, dap_maxvar);
    nextclab = 0 as libc::c_int;
    labroot = -(1 as libc::c_int);
    ncols = 0 as libc::c_int;
    r = 0 as libc::c_int;
    while r < nrowvar {
        *(*prev.offset(r as isize))
            .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*rowvar.offset(r as isize) as isize) <= 0 as libc::c_int
        {
            if !(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(*rowvar.offset(r as isize) as isize))
                .is_null()
            {
                dap_free(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*rowvar.offset(r as isize) as isize)
                        as *mut libc::c_void,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            let ref mut fresh26 = *((*dap_obs
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .do_str)
                .offset(*rowvar.offset(r as isize) as isize);
            *fresh26 = dap_malloc(
                rtitlesp + 1 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        r += 1;
        r;
    }
    c = 0 as libc::c_int;
    while c < ncolvar - 1 as libc::c_int {
        if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
            .offset(*colvar.offset(c as isize) as isize) <= 0 as libc::c_int
        {
            if !(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                .offset(*colvar.offset(c as isize) as isize))
                .is_null()
            {
                dap_free(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*colvar.offset(c as isize) as isize)
                        as *mut libc::c_void,
                    b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
                );
            }
            let ref mut fresh27 = *((*dap_obs
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize))
                .do_str)
                .offset(*colvar.offset(c as isize) as isize);
            *fresh27 = dap_malloc(
                rtitlesp + 1 as libc::c_int,
                b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            );
        }
        c += 1;
        c;
    }
    r = 0 as libc::c_int;
    while r < dap_maxrows {
        c = 0 as libc::c_int;
        while c < dap_maxcols {
            *(*valset.offset(r as isize)).offset(c as isize) = 0 as libc::c_int;
            c += 1;
            c;
        }
        r += 1;
        r;
    }
    nrows = -(1 as libc::c_int);
    more = 1 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(markv, nmark) != 0 {
            dap_swap();
            dap_head(markv, nmark);
            tablehead();
            tableprint();
            dap_swap();
            nextclab = 0 as libc::c_int;
            labroot = -(1 as libc::c_int);
            ncols = 0 as libc::c_int;
            r = 0 as libc::c_int;
            while r < nrowvar {
                *(*prev.offset(r as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                r += 1;
                r;
            }
            r = 0 as libc::c_int;
            while r < dap_maxrows {
                c = 0 as libc::c_int;
                while c < dap_maxcols {
                    *(*valset.offset(r as isize)).offset(c as isize) = 0 as libc::c_int;
                    c += 1;
                    c;
                }
                r += 1;
                r;
            }
            nrows = -(1 as libc::c_int);
        }
        r = 0 as libc::c_int;
        while r < nrowvar {
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*rowvar.offset(r as isize) as isize) == 0 as libc::c_int
            {
                sprintf(
                    nstring,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(*rowvar.offset(r as isize) as isize),
                );
                strncpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*rowvar.offset(r as isize) as isize),
                    nstring,
                    rtitlesp as libc::c_ulong,
                );
                *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*rowvar.offset(r as isize) as isize))
                    .offset(rtitlesp as isize) = '\0' as i32 as libc::c_char;
            } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*rowvar.offset(r as isize) as isize) == -(1 as libc::c_int)
            {
                sprintf(
                    nstring,
                    b"%g\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*rowvar.offset(r as isize) as isize),
                );
                strncpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*rowvar.offset(r as isize) as isize),
                    nstring,
                    rtitlesp as libc::c_ulong,
                );
                *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*rowvar.offset(r as isize) as isize))
                    .offset(rtitlesp as isize) = '\0' as i32 as libc::c_char;
            }
            r += 1;
            r;
        }
        r = 0 as libc::c_int;
        while r < nrowvar {
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*rowvar.offset(r as isize) as isize),
                *prev.offset(r as isize),
            ) != 0
            {
                break;
            }
            r += 1;
            r;
        }
        if r < nrowvar {
            nrows += 1;
            nrows;
            s = 0 as libc::c_int;
            while s < r {
                *(*(*rowlabel.offset(nrows as isize)).offset(s as isize))
                    .offset(0 as libc::c_int as isize) = '\0' as i32 as libc::c_char;
                s += 1;
                s;
            }
            while r < nrowvar {
                strcpy(
                    *prev.offset(r as isize),
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*rowvar.offset(r as isize) as isize),
                );
                strcpy(
                    *(*rowlabel.offset(nrows as isize)).offset(r as isize),
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*rowvar.offset(r as isize) as isize),
                );
                r += 1;
                r;
            }
        }
        if nrows < 0 as libc::c_int {
            fputs(b"(table) No rows.\n\0" as *const u8 as *const libc::c_char, dap_err);
            exit(1 as libc::c_int);
        }
        c = 0 as libc::c_int;
        while c < ncolvar - 1 as libc::c_int {
            if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*colvar.offset(c as isize) as isize) == 0 as libc::c_int
            {
                sprintf(
                    nstring,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_int)
                        .offset(*colvar.offset(c as isize) as isize),
                );
                strncpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*colvar.offset(c as isize) as isize),
                    nstring,
                    rtitlesp as libc::c_ulong,
                );
                *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*colvar.offset(c as isize) as isize))
                    .offset(rtitlesp as isize) = '\0' as i32 as libc::c_char;
            } else if *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_len)
                .offset(*colvar.offset(c as isize) as isize) == -(1 as libc::c_int)
            {
                sprintf(
                    nstring,
                    b"%g\0" as *const u8 as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
                        .offset(*colvar.offset(c as isize) as isize),
                );
                strncpy(
                    *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                        .offset(*colvar.offset(c as isize) as isize),
                    nstring,
                    rtitlesp as libc::c_ulong,
                );
                *(*((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_str)
                    .offset(*colvar.offset(c as isize) as isize))
                    .offset(rtitlesp as isize) = '\0' as i32 as libc::c_char;
            }
            c += 1;
            c;
        }
        c = findcol();
        *(*tableval.offset(nrows as isize))
            .offset(
                c as isize,
            ) = *((*dap_obs.as_mut_ptr().offset(0 as libc::c_int as isize)).do_dbl)
            .offset(*colvar.offset((ncolvar - 1 as libc::c_int) as isize) as isize);
        *(*valset.offset(nrows as isize)).offset(c as isize) = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn split(
    mut fname: *mut libc::c_char,
    mut varlist: *mut libc::c_char,
    mut classvalvars: *mut libc::c_char,
) {
    let mut skiplist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut classvar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut valuevar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut varname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut maxname: libc::c_int = 0;
    let mut var: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut nv: libc::c_int = 0;
    let mut vv: libc::c_int = 0;
    let mut vlen: libc::c_int = 0;
    let mut prevlen: libc::c_int = 0;
    let mut classv: libc::c_int = 0;
    let mut valuev: libc::c_int = 0;
    classvar = dap_malloc(
        (strlen(varlist)).wrapping_add(6 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    valuevar = dap_malloc(
        (strlen(varlist)).wrapping_add(6 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = 0 as libc::c_int;
    while *classvalvars.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    t = 0 as libc::c_int;
    while *classvalvars.offset(s as isize) as libc::c_int != 0
        && *classvalvars.offset(s as isize) as libc::c_int != ' ' as i32
    {
        let fresh28 = s;
        s = s + 1;
        let fresh29 = t;
        t = t + 1;
        *classvar.offset(fresh29 as isize) = *classvalvars.offset(fresh28 as isize);
    }
    *classvar.offset(t as isize) = '\0' as i32 as libc::c_char;
    if t == 0 {
        fputs(
            b"(split) No classification variable specified.\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    while *classvalvars.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    t = 0 as libc::c_int;
    while *classvalvars.offset(s as isize) as libc::c_int != 0
        && *classvalvars.offset(s as isize) as libc::c_int != ' ' as i32
    {
        let fresh30 = s;
        s = s + 1;
        let fresh31 = t;
        t = t + 1;
        *valuevar.offset(fresh31 as isize) = *classvalvars.offset(fresh30 as isize);
    }
    *valuevar.offset(t as isize) = '\0' as i32 as libc::c_char;
    if t == 0 {
        fprintf(
            dap_err,
            b"(split) No value variable specified: %s\n\0" as *const u8
                as *const libc::c_char,
            classvalvars,
        );
        exit(1 as libc::c_int);
    }
    var = dap_malloc(
        (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            .wrapping_mul(
                (strlen(varlist))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                    .wrapping_div(2 as libc::c_int as libc::c_ulong),
            ) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    varname = dap_malloc(
        (strlen(varlist)).wrapping_add(1 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    skiplist = dap_malloc(
        (strlen(varlist)).wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(skiplist, b"!\0" as *const u8 as *const libc::c_char);
    strcat(skiplist, varlist);
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(outname, fname);
    strcat(outname, b".spl\0" as *const u8 as *const libc::c_char);
    inset(fname);
    s = 0 as libc::c_int;
    while *varlist.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    nv = 0 as libc::c_int;
    prevlen = -(1 as libc::c_int) - 1 as libc::c_int;
    maxname = 0 as libc::c_int;
    while *varlist.offset(s as isize) != 0 {
        t = 0 as libc::c_int;
        while *varlist.offset(s as isize) as libc::c_int != 0
            && *varlist.offset(s as isize) as libc::c_int != ' ' as i32
        {
            let fresh32 = s;
            s = s + 1;
            let fresh33 = t;
            t = t + 1;
            *varname.offset(fresh33 as isize) = *varlist.offset(fresh32 as isize);
        }
        *varname.offset(t as isize) = '\0' as i32 as libc::c_char;
        if t > maxname {
            maxname = t;
        }
        let ref mut fresh34 = *var.offset(nv as isize);
        *fresh34 = dap_varnum(varname);
        if *fresh34 < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(split) Unknown variable: %s\n\0" as *const u8 as *const libc::c_char,
                varname,
            );
            exit(1 as libc::c_int);
        }
        vlen = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_len)
            .offset(*var.offset(nv as isize) as isize);
        if prevlen < -(1 as libc::c_int) {
            prevlen = vlen;
        } else if prevlen != vlen {
            fprintf(
                dap_err,
                b"(split) Length of %s (%d) differs from that of previous variables (%d)\n\0"
                    as *const u8 as *const libc::c_char,
                varname,
                vlen,
                prevlen,
            );
            exit(1 as libc::c_int);
        }
        while *varlist.offset(s as isize) as libc::c_int == ' ' as i32 {
            s += 1;
            s;
        }
        nv += 1;
        nv;
    }
    sprintf(
        classvar.offset(strlen(classvar) as isize),
        b" %d\0" as *const u8 as *const libc::c_char,
        maxname,
    );
    classv = dap_vd(classvar, 0 as libc::c_int);
    sprintf(
        valuevar.offset(strlen(valuevar) as isize),
        b" %d\0" as *const u8 as *const libc::c_char,
        vlen,
    );
    valuev = dap_vd(valuevar, 0 as libc::c_int);
    outset(outname, skiplist);
    while step() != 0 {
        vv = 0 as libc::c_int;
        while vv < nv {
            strcpy(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(classv as isize),
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_nam)
                    .offset(*var.offset(vv as isize) as isize),
            );
            if vlen == -(1 as libc::c_int) {
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(
                        valuev as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(*var.offset(vv as isize) as isize);
            } else if vlen == 0 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                    .offset(
                        valuev as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                    .offset(*var.offset(vv as isize) as isize);
            } else {
                strcpy(
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(valuev as isize),
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(*var.offset(vv as isize) as isize),
                );
            }
            output();
            vv += 1;
            vv;
        }
    }
    dap_free(
        classvar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        valuevar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        var as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        skiplist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn join(
    mut fname: *mut libc::c_char,
    mut partvars: *mut libc::c_char,
    mut valuevar: *mut libc::c_char,
) {
    let mut partvars1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut npart: libc::c_int = 0;
    let mut classvar: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut s: libc::c_int = 0;
    let mut t: libc::c_int = 0;
    let mut outname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skiplist: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cv: libc::c_int = 0;
    let mut vv: libc::c_int = 0;
    let mut nnew: libc::c_int = 0;
    let mut nv: libc::c_int = 0;
    let mut vlen: libc::c_int = 0;
    let mut partv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut newv: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut valv: libc::c_int = 0;
    let mut vallen: libc::c_int = 0;
    let mut varspec: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut more: libc::c_int = 0;
    let mut np: libc::c_int = 0;
    outname = dap_malloc(
        (strlen(fname)).wrapping_add(5 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    strcpy(outname, fname);
    strcat(outname, b".joi\0" as *const u8 as *const libc::c_char);
    newv = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(dap_maxvar as libc::c_ulong) as libc::c_int,
        b"dap_maxvar\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    partvars1 = dap_malloc(
        (strlen(partvars)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    s = 0 as libc::c_int;
    while *partvars.offset(s as isize) as libc::c_int == ' ' as i32 {
        s += 1;
        s;
    }
    npart = 0 as libc::c_int;
    while *partvars.offset(s as isize) != 0 {
        t = s;
        while *partvars.offset(t as isize) as libc::c_int != 0
            && *partvars.offset(t as isize) as libc::c_int != ' ' as i32
        {
            t += 1;
            t;
        }
        while *partvars.offset(t as isize) as libc::c_int == ' ' as i32 {
            t += 1;
            t;
        }
        if !(*partvars.offset(t as isize) != 0) {
            break;
        }
        s = t;
        npart += 1;
        npart;
    }
    strncpy(partvars1, partvars, s as libc::c_ulong);
    *partvars1.offset(s as isize) = '\0' as i32 as libc::c_char;
    classvar = dap_malloc(
        (strlen(partvars))
            .wrapping_sub(s as libc::c_ulong)
            .wrapping_add(2 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    t = 0 as libc::c_int;
    while *partvars.offset(s as isize) as libc::c_int != 0
        && *partvars.offset(s as isize) as libc::c_int != ' ' as i32
    {
        let fresh35 = s;
        s = s + 1;
        let fresh36 = t;
        t = t + 1;
        *classvar.offset(fresh36 as isize) = *partvars.offset(fresh35 as isize);
    }
    *classvar.offset(t as isize) = '\0' as i32 as libc::c_char;
    skiplist = dap_malloc(
        (strlen(classvar))
            .wrapping_add(strlen(valuevar))
            .wrapping_add(3 as libc::c_int as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if strcmp(classvar, b"_type_\0" as *const u8 as *const libc::c_char) != 0 {
        sprintf(
            skiplist,
            b"!%s %s\0" as *const u8 as *const libc::c_char,
            classvar,
            valuevar,
        );
    } else {
        sprintf(skiplist, b"!%s\0" as *const u8 as *const libc::c_char, valuevar);
    }
    inset(fname);
    cv = dap_varnum(classvar);
    if cv < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(join) Unknown variable: %s\n\0" as *const u8 as *const libc::c_char,
            classvar,
        );
        exit(1 as libc::c_int);
    }
    valv = dap_varnum(valuevar);
    if valv < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(join) Unknown variable: %s\n\0" as *const u8 as *const libc::c_char,
            valuevar,
        );
        exit(1 as libc::c_int);
    }
    vallen = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_len)
        .offset(valv as isize);
    vlen = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_len)
        .offset(cv as isize);
    varspec = dap_malloc(
        vlen + 5 as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if vlen <= 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(join) Variable %s not string variable (%d)\n\0" as *const u8
                as *const libc::c_char,
            classvar,
            vlen,
        );
        exit(1 as libc::c_int);
    }
    dap_mark();
    partv = dap_malloc(
        (::std::mem::size_of::<*mut libc::c_int>() as libc::c_ulong)
            .wrapping_mul(npart as libc::c_ulong) as libc::c_int,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    dap_list(partvars1, partv, npart);
    nnew = 0 as libc::c_int;
    while step() != 0 {
        if dap_newpart(partv, npart) != 0 {
            break;
        }
        strcpy(
            varspec,
            *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                .offset(cv as isize),
        );
        sprintf(
            varspec.offset(strlen(varspec) as isize),
            b" %d\0" as *const u8 as *const libc::c_char,
            vallen,
        );
        *newv.offset(nnew as isize) = dap_vd(varspec, 0 as libc::c_int);
        nnew += 1;
        nnew;
    }
    dap_rewind();
    outset(outname, skiplist);
    more = 1 as libc::c_int;
    nv = 0 as libc::c_int;
    while more != 0 {
        more = step();
        if dap_newpart(partv, npart) != 0 {
            if nv < nnew {
                fprintf(
                    dap_err,
                    b"(join) Too few lines in part:\0" as *const u8
                        as *const libc::c_char,
                );
                np = 0 as libc::c_int;
                while np < npart {
                    putc(' ' as i32, dap_err);
                    fputs(
                        *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                            .offset(*partv.offset(np as isize) as isize),
                        dap_err,
                    );
                    np += 1;
                    np;
                }
                putc('\n' as i32, dap_err);
                exit(1 as libc::c_int);
            }
            dap_swap();
            output();
            dap_swap();
            nv = 0 as libc::c_int;
        }
        if more != 0 {
            if nv >= nnew {
                fprintf(
                    dap_err,
                    b"(join) Too many lines at %s\n\0" as *const u8
                        as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(cv as isize),
                );
                exit(1 as libc::c_int);
            }
            if strcmp(
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_nam)
                    .offset(*newv.offset(nv as isize) as isize),
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                    .offset(cv as isize),
            ) != 0
            {
                fprintf(
                    dap_err,
                    b"(join) Missing or extra lines at %s\n\0" as *const u8
                        as *const libc::c_char,
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(cv as isize),
                );
                exit(1 as libc::c_int);
            }
            if vallen == -(1 as libc::c_int) {
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(
                        *newv.offset(nv as isize) as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_dbl)
                    .offset(valv as isize);
            } else if vallen == 0 as libc::c_int {
                *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                    .offset(
                        *newv.offset(nv as isize) as isize,
                    ) = *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_int)
                    .offset(valv as isize);
            } else {
                strcpy(
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(*newv.offset(nv as isize) as isize),
                    *((*dap_obs.as_mut_ptr().offset(dap_ono as isize)).do_str)
                        .offset(valv as isize),
                );
            }
        }
        nv += 1;
        nv;
    }
    dap_free(
        outname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        newv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partvars1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        classvar as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        skiplist as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        varspec as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        partv as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
static mut fieldstart: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut fieldlen: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut unfield: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut linelen: libc::c_int = 0;
static mut keylen: libc::c_int = 0;
static mut keyend: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut sortord: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut keymap: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut nvars: libc::c_int = 0;
static mut mem1: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut mem2: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut maxlines: libc::c_int = 0;
static mut line: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
static mut field: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
static mut nfields: libc::c_int = 0;
static mut unique: libc::c_int = 0;
static mut tmplate: [libc::c_char; 25] = unsafe {
    *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b"dap_tmp/sortppppppssssss\0")
};
static mut fpos: libc::c_ulong = 0;
unsafe extern "C" fn cleanup(mut nseg: libc::c_int) {
    let mut s: libc::c_int = 0;
    let mut segname: [libc::c_char; 25] = *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b"dap_tmp/sortppppppssssss\0");
    s = 0 as libc::c_int;
    while s < nseg {
        strcpy(segname.as_mut_ptr(), tmplate.as_mut_ptr());
        sprintf(
            segname.as_mut_ptr().offset(18 as libc::c_int as isize),
            b"%06d\0" as *const u8 as *const libc::c_char,
            s,
        );
        unlink(segname.as_mut_ptr());
        s += 1;
        s;
    }
}
unsafe extern "C" fn linecmp(
    mut s1: *mut *mut libc::c_char,
    mut s2: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut t1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut e1: *mut libc::c_char = 0 as *mut libc::c_char;
    e1 = (*s1).offset(keylen as isize).offset(-(1 as libc::c_int as isize));
    t1 = *s1;
    t2 = *s2;
    while t1 < e1 && *t1 as libc::c_int == *t2 as libc::c_int {
        t1 = t1.offset(1);
        t1;
        t2 = t2.offset(1);
        t2;
    }
    return if *keymap.offset(t1.offset_from(*s1) as libc::c_long as isize) != 0 {
        *t1 as libc::c_int - *t2 as libc::c_int
    } else {
        *t2 as libc::c_int - *t1 as libc::c_int
    };
}
unsafe extern "C" fn sortseg(mut orig: libc::c_int, mut s: libc::c_int) -> libc::c_int {
    let mut segname: [libc::c_char; 25] = *::std::mem::transmute::<
        &[u8; 25],
        &mut [libc::c_char; 25],
    >(b"dap_tmp/sortppppppssssss\0");
    let mut nread: ssize_t = 0;
    let mut seg: libc::c_int = 0;
    let mut m1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut m2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut nlines: libc::c_int = 0;
    let mut lstart1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut lstart2: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut lp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut mp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newfield: libc::c_int = 0;
    let mut cmp: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    cmp = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(
                *mut *mut libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            linecmp
                as unsafe extern "C" fn(
                    *mut *mut libc::c_char,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
        ),
    );
    nread = read(orig, mem1 as *mut libc::c_void, dap_maxmem as size_t);
    if nread == 0 {
        return 0 as libc::c_int;
    }
    strcpy(segname.as_mut_ptr(), tmplate.as_mut_ptr());
    sprintf(
        segname.as_mut_ptr().offset(18 as libc::c_int as isize),
        b"%06d\0" as *const u8 as *const libc::c_char,
        s,
    );
    seg = open(
        segname.as_mut_ptr(),
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if seg < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(dsort) can't write %s\n\0" as *const u8 as *const libc::c_char,
            segname.as_mut_ptr(),
        );
        exit(1 as libc::c_int);
    }
    lstart1 = mem1;
    lstart2 = mem2;
    nlines = 0 as libc::c_int;
    while lstart1 < mem1.offset(nread as isize)
        && lstart2.offset(linelen as isize) < mem2.offset(dap_maxmem as isize)
    {
        if !(nlines < maxlines) {
            break;
        }
        let fresh37 = nlines;
        nlines = nlines + 1;
        let ref mut fresh38 = *line.offset(fresh37 as isize);
        *fresh38 = lstart2;
        f = 0 as libc::c_int;
        m1 = lstart1;
        last = '\0' as i32;
        while f < nvars {
            m2 = lstart2
                .offset(
                    *fieldstart.offset(*unfield.offset(f as isize) as isize) as isize,
                );
            while m1 < mem1.offset(nread as isize) && *m1 as libc::c_int != '|' as i32
                && *m1 as libc::c_int != '\n' as i32
            {
                let fresh39 = m1;
                m1 = m1.offset(1);
                let fresh40 = m2;
                m2 = m2.offset(1);
                *fresh40 = *fresh39;
            }
            let fresh41 = m1;
            m1 = m1.offset(1);
            last = *fresh41 as libc::c_int;
            while m2
                < lstart2
                    .offset(
                        *fieldstart
                            .offset(
                                (*unfield.offset(f as isize) + 1 as libc::c_int) as isize,
                            ) as isize,
                    )
            {
                let fresh42 = m2;
                m2 = m2.offset(1);
                *fresh42 = '\0' as i32 as libc::c_char;
            }
            f += 1;
            f;
        }
        if last != '\n' as i32 {
            break;
        }
        lstart1 = m1;
        lstart2 = lstart2.offset(linelen as isize);
    }
    if last != '\n' as i32 {
        nlines -= 1;
        nlines;
    }
    nread = lstart1.offset_from(mem1) as libc::c_long;
    if nread == 0 {
        fputs(
            b"(dsort) line longer than buffer\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    fpos = fpos.wrapping_add(nread as libc::c_ulong);
    qsort(
        line as *mut libc::c_void,
        nlines as size_t,
        ::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            __compar_fn_t,
        >(cmp),
    );
    l = 0 as libc::c_int;
    while l < nlines {
        memcpy(
            mem1.offset((l * linelen) as isize) as *mut libc::c_void,
            *line.offset(l as isize) as *const libc::c_void,
            linelen as libc::c_ulong,
        );
        l += 1;
        l;
    }
    write(seg, mem1 as *const libc::c_void, (nlines * linelen) as size_t);
    close(seg);
    return nread as libc::c_int;
}
static mut nextline: *mut *mut libc::c_char = 0 as *const *mut libc::c_char
    as *mut *mut libc::c_char;
unsafe extern "C" fn nextlinecmp(
    mut s1: *mut libc::c_int,
    mut s2: *mut libc::c_int,
) -> libc::c_int {
    return linecmp(nextline.offset(*s1 as isize), nextline.offset(*s2 as isize));
}
unsafe extern "C" fn merge(mut nseg: libc::c_int, mut out: libc::c_int) {
    let mut s: libc::c_int = 0;
    let mut s1: libc::c_int = 0;
    let mut s2: libc::c_int = 0;
    let mut c: libc::c_int = 0;
    let mut l: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut segnamemem: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut segname: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut segfile: libc::c_int = 0;
    let mut chunksize: libc::c_int = 0;
    let mut chunkread: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut prevline: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut spos: *mut libc::c_ulong = 0 as *mut libc::c_ulong;
    let mut nmore: libc::c_int = 0;
    let mut outpos: libc::c_uint = 0;
    let mut segord: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut scmp: Option::<unsafe extern "C" fn() -> libc::c_int> = None;
    scmp = ::std::mem::transmute::<
        Option::<
            unsafe extern "C" fn(*mut libc::c_int, *mut libc::c_int) -> libc::c_int,
        >,
        Option::<unsafe extern "C" fn() -> libc::c_int>,
    >(
        Some(
            nextlinecmp
                as unsafe extern "C" fn(
                    *mut libc::c_int,
                    *mut libc::c_int,
                ) -> libc::c_int,
        ),
    );
    nextline = dap_malloc(
        (nseg as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int,
        b"nextline\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    prevline = 0 as *mut libc::c_char;
    segnamemem = dap_malloc(
        (nseg as libc::c_ulong)
            .wrapping_mul(
                (strlen(
                    b"dap_tmp/sortppppppssssss\0" as *const u8 as *const libc::c_char,
                ))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as libc::c_int,
        b"segnamemem\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    segname = dap_malloc(
        (nseg as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int,
        b"segname\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    spos = dap_malloc(
        (nseg as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
            as libc::c_int,
        b"spos\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_ulong;
    segord = dap_malloc(
        (nseg as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"segord\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    chunksize = dap_maxmem / (nseg * linelen) * linelen;
    if chunksize == 0 {
        fputs(
            b"(dsort) insufficient memory\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    chunkread = dap_malloc(
        (nseg as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"chunkread\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    s = 0 as libc::c_int;
    while s < nseg {
        let ref mut fresh43 = *segname.offset(s as isize);
        *fresh43 = segnamemem
            .offset(
                (s as libc::c_ulong)
                    .wrapping_mul(
                        (strlen(
                            b"dap_tmp/sortppppppssssss\0" as *const u8
                                as *const libc::c_char,
                        ))
                            .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    ) as isize,
            );
        strcpy(*segname.offset(s as isize), tmplate.as_mut_ptr());
        sprintf(
            (*segname.offset(s as isize)).offset(18 as libc::c_int as isize),
            b"%06d\0" as *const u8 as *const libc::c_char,
            s,
        );
        let ref mut fresh44 = *nextline.offset(s as isize);
        *fresh44 = mem1.offset((s * chunksize) as isize);
        *spos.offset(s as isize) = 0 as libc::c_int as libc::c_ulong;
        *chunkread.offset(s as isize) = 0 as libc::c_int;
        *segord.offset(s as isize) = s;
        s += 1;
        s;
    }
    s = 0 as libc::c_int;
    while s < nseg {
        segfile = open(*segname.offset(s as isize), 0 as libc::c_int);
        if segfile < 0 as libc::c_int {
            fprintf(
                dap_err,
                b"(dsort) can't read %s\n\0" as *const u8 as *const libc::c_char,
                *segname.offset(s as isize),
            );
            exit(1 as libc::c_int);
        }
        let ref mut fresh45 = *chunkread.offset(s as isize);
        *fresh45 = read(
            segfile,
            mem1.offset((s * chunksize) as isize) as *mut libc::c_void,
            chunksize as size_t,
        ) as libc::c_int;
        if *fresh45 > 0 as libc::c_int {
            let ref mut fresh46 = *spos.offset(s as isize);
            *fresh46 = (*fresh46)
                .wrapping_add(*chunkread.offset(s as isize) as libc::c_ulong);
        } else {
            fprintf(
                dap_err,
                b"(dsort) bad initial read of %s\n\0" as *const u8
                    as *const libc::c_char,
                *segname.offset(s as isize),
            );
            exit(1 as libc::c_int);
        }
        close(segfile);
        s += 1;
        s;
    }
    qsort(
        segord as *mut libc::c_void,
        nseg as size_t,
        ::std::mem::size_of::<libc::c_int>() as libc::c_ulong,
        ::std::mem::transmute::<
            Option::<unsafe extern "C" fn() -> libc::c_int>,
            __compar_fn_t,
        >(scmp),
    );
    nmore = nseg;
    outpos = 0 as libc::c_int as libc::c_uint;
    while nmore != 0 {
        if unique != 0 {
            if prevline.is_null() {
                prevline = dap_malloc(
                    linelen,
                    b"prevline\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
        }
        if unique == 0
            || !prevline.is_null()
                && linecmp(
                    nextline.offset(*segord.offset(0 as libc::c_int as isize) as isize),
                    &mut prevline,
                ) != 0
        {
            f = 0 as libc::c_int;
            while f < nvars {
                l = *fieldstart.offset(*unfield.offset(f as isize) as isize);
                while l
                    < *fieldstart
                        .offset(
                            (*unfield.offset(f as isize) + 1 as libc::c_int) as isize,
                        )
                {
                    if outpos == dap_maxmem as libc::c_uint {
                        write(out, mem2 as *const libc::c_void, outpos as size_t);
                        outpos = 0 as libc::c_int as libc::c_uint;
                    }
                    let ref mut fresh47 = *mem2.offset(outpos as isize);
                    *fresh47 = *(*nextline
                        .offset(*segord.offset(0 as libc::c_int as isize) as isize))
                        .offset(l as isize);
                    if *fresh47 == 0 {
                        break;
                    }
                    outpos = outpos.wrapping_add(1);
                    outpos;
                    l += 1;
                    l;
                }
                if outpos == dap_maxmem as libc::c_uint {
                    write(out, mem2 as *const libc::c_void, outpos as size_t);
                    outpos = 0 as libc::c_int as libc::c_uint;
                }
                if f < nvars - 1 as libc::c_int {
                    let fresh48 = outpos;
                    outpos = outpos.wrapping_add(1);
                    *mem2.offset(fresh48 as isize) = '|' as i32 as libc::c_char;
                } else {
                    let fresh49 = outpos;
                    outpos = outpos.wrapping_add(1);
                    *mem2.offset(fresh49 as isize) = '\n' as i32 as libc::c_char;
                }
                f += 1;
                f;
            }
        }
        if unique != 0 {
            if prevline.is_null() {
                prevline = dap_malloc(
                    linelen,
                    b"prevline\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                );
            }
            memcpy(
                prevline as *mut libc::c_void,
                *nextline.offset(*segord.offset(0 as libc::c_int as isize) as isize)
                    as *const libc::c_void,
                linelen as libc::c_ulong,
            );
        }
        let ref mut fresh50 = *nextline
            .offset(*segord.offset(0 as libc::c_int as isize) as isize);
        *fresh50 = (*fresh50).offset(linelen as isize);
        if *fresh50
            >= mem1
                .offset((*segord.offset(0 as libc::c_int as isize) * chunksize) as isize)
                .offset(
                    *chunkread.offset(*segord.offset(0 as libc::c_int as isize) as isize)
                        as isize,
                )
        {
            segfile = open(
                *segname.offset(*segord.offset(0 as libc::c_int as isize) as isize),
                0 as libc::c_int,
            );
            if segfile < 0 as libc::c_int {
                fprintf(
                    dap_err,
                    b"(dsort) can't read %s\n\0" as *const u8 as *const libc::c_char,
                    *segname.offset(*segord.offset(0 as libc::c_int as isize) as isize),
                );
                exit(1 as libc::c_int);
            }
            if lseek(
                segfile,
                *spos.offset(*segord.offset(0 as libc::c_int as isize) as isize)
                    as __off_t,
                1 as libc::c_int,
            ) > 0 as libc::c_int as libc::c_long
            {
                let ref mut fresh51 = *chunkread
                    .offset(*segord.offset(0 as libc::c_int as isize) as isize);
                *fresh51 = read(
                    segfile,
                    mem1
                        .offset(
                            (*segord.offset(0 as libc::c_int as isize) * chunksize)
                                as isize,
                        ) as *mut libc::c_void,
                    chunksize as size_t,
                ) as libc::c_int;
                if *fresh51 > 0 as libc::c_int {
                    let ref mut fresh52 = *spos
                        .offset(*segord.offset(0 as libc::c_int as isize) as isize);
                    *fresh52 = (*fresh52)
                        .wrapping_add(
                            *chunkread
                                .offset(*segord.offset(0 as libc::c_int as isize) as isize)
                                as libc::c_ulong,
                        );
                    let ref mut fresh53 = *nextline
                        .offset(*segord.offset(0 as libc::c_int as isize) as isize);
                    *fresh53 = mem1
                        .offset(
                            (*segord.offset(0 as libc::c_int as isize) * chunksize)
                                as isize,
                        );
                } else {
                    nmore -= 1;
                    nmore;
                }
            } else {
                *chunkread
                    .offset(
                        *segord.offset(0 as libc::c_int as isize) as isize,
                    ) = 0 as libc::c_int;
                nmore -= 1;
                nmore;
            }
            close(segfile);
        }
        if *chunkread.offset(*segord.offset(0 as libc::c_int as isize) as isize)
            <= 0 as libc::c_int
        {
            s = 0 as libc::c_int;
            while s < nmore {
                *segord
                    .offset(
                        s as isize,
                    ) = *segord.offset((s + 1 as libc::c_int) as isize);
                s += 1;
                s;
            }
        } else {
            s1 = 1 as libc::c_int;
            while s1 < nmore {
                if linecmp(
                    nextline.offset(*segord.offset(0 as libc::c_int as isize) as isize),
                    nextline.offset(*segord.offset(s1 as isize) as isize),
                ) <= 0 as libc::c_int
                {
                    break;
                }
                s1 += 1;
                s1;
            }
            s2 = *segord.offset(0 as libc::c_int as isize);
            s = 0 as libc::c_int;
            while s < s1 - 1 as libc::c_int {
                *segord
                    .offset(
                        s as isize,
                    ) = *segord.offset((s + 1 as libc::c_int) as isize);
                s += 1;
                s;
            }
            *segord.offset(s as isize) = s2;
        }
    }
    if outpos != 0 {
        write(out, mem2 as *const libc::c_void, outpos as size_t);
    }
    dap_free(
        nextline as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        prevline as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        segnamemem as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        segname as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        spos as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        chunkread as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        segord as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
unsafe extern "C" fn fieldfix(mut code: libc::c_int) -> libc::c_int {
    match code {
        -1 => return 12 as libc::c_int,
        0 => return 6 as libc::c_int,
        _ => return code,
    };
}
unsafe extern "C" fn fixheader(
    mut header: *mut libc::c_char,
    mut srt: libc::c_int,
) -> libc::c_int {
    let mut headerlen: libc::c_int = 0;
    let mut nspaces: libc::c_int = 0;
    let mut v: libc::c_int = 0;
    let mut gotsign: libc::c_int = 0;
    let mut sign: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut h1: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut newfield: libc::c_int = 0;
    let mut fieldlen1: libc::c_int = 0;
    let mut f: libc::c_int = 0;
    let mut f1: libc::c_int = 0;
    let mut f2: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut k1: libc::c_int = 0;
    let mut headerfield: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    headerfield = dap_malloc(
        (dap_maxvar as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int,
        b"headerfield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    fieldstart = dap_malloc(
        ((dap_maxvar + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"fieldstart\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    *fieldstart.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    fieldlen = dap_malloc(
        ((dap_maxvar + 1 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"fieldlen\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    headerlen = 0 as libc::c_int;
    nspaces = 0 as libc::c_int;
    nvars = 0 as libc::c_int;
    newfield = 1 as libc::c_int;
    linelen = 0 as libc::c_int;
    while headerlen < dap_linelen
        && *header.offset(headerlen as isize) as libc::c_int != '\n' as i32
    {
        if newfield != 0 {
            if nvars != 0 {
                let ref mut fresh54 = *fieldlen
                    .offset((nvars - 1 as libc::c_int) as isize);
                *fresh54 = fieldfix(sign * fieldlen1);
                linelen += *fresh54;
            }
            let fresh55 = nvars;
            nvars = nvars + 1;
            let ref mut fresh56 = *headerfield.offset(fresh55 as isize);
            *fresh56 = header.offset(headerlen as isize);
            newfield = 0 as libc::c_int;
        }
        if *header.offset(headerlen as isize) as libc::c_int == ' ' as i32 {
            nspaces += 1;
            if nspaces == 1 as libc::c_int {
                fieldlen1 = 0 as libc::c_int;
                gotsign = 0 as libc::c_int;
            } else {
                nspaces = 0 as libc::c_int;
                newfield = 1 as libc::c_int;
            }
        } else if nspaces == 1 as libc::c_int
            && *header.offset(headerlen as isize) as libc::c_int != ' ' as i32
        {
            if gotsign == 0 {
                if *header.offset(headerlen as isize) as libc::c_int == '-' as i32 {
                    sign = -(1 as libc::c_int);
                } else {
                    sign = 1 as libc::c_int;
                    gotsign = 1 as libc::c_int;
                }
            }
            if gotsign != 0 {
                fieldlen1 = 10 as libc::c_int * fieldlen1
                    + *header.offset(headerlen as isize) as libc::c_int - '0' as i32;
            }
            gotsign = 1 as libc::c_int;
        }
        headerlen += 1;
        headerlen;
    }
    let ref mut fresh57 = *fieldlen.offset((nvars - 1 as libc::c_int) as isize);
    *fresh57 = fieldfix(sign * fieldlen1);
    linelen += *fresh57;
    maxlines = dap_maxmem / linelen;
    line = dap_malloc(
        (maxlines as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            as libc::c_int,
        b"line\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut *mut libc::c_char;
    if headerlen == dap_linelen {
        *header
            .offset(
                (dap_linelen - 1 as libc::c_int) as isize,
            ) = '\0' as i32 as libc::c_char;
        fprintf(
            dap_err,
            b"(dsort) header line too long %s\n\0" as *const u8 as *const libc::c_char,
            header,
        );
        exit(1 as libc::c_int);
    }
    f = 0 as libc::c_int;
    f2 = nfields;
    while f < nvars {
        f1 = 0 as libc::c_int;
        while f1 < nfields {
            if *field.offset(f1 as isize) == f {
                break;
            }
            f1 += 1;
            f1;
        }
        if f1 == nfields {
            let fresh58 = f2;
            f2 = f2 + 1;
            *field.offset(fresh58 as isize) = f;
        }
        f += 1;
        f;
    }
    f = 0 as libc::c_int;
    while f < nvars {
        *unfield.offset(*field.offset(f as isize) as isize) = f;
        f += 1;
        f;
    }
    *fieldstart.offset(0 as libc::c_int as isize) = 0 as libc::c_int;
    f = 1 as libc::c_int;
    while f <= nvars {
        *fieldstart
            .offset(
                f as isize,
            ) = *fieldstart.offset((f - 1 as libc::c_int) as isize)
            + *fieldlen.offset(*field.offset((f - 1 as libc::c_int) as isize) as isize);
        f += 1;
        f;
    }
    keyend = dap_malloc(
        (nfields as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"keyend\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    f = 0 as libc::c_int;
    keylen = 0 as libc::c_int;
    while f < nfields {
        keylen += *fieldlen.offset(*field.offset(f as isize) as isize);
        *keyend.offset(f as isize) = keylen;
        f += 1;
        f;
    }
    keymap = dap_malloc(
        (keylen as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"keymap\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    f = 0 as libc::c_int;
    k = 0 as libc::c_int;
    while f < nfields {
        k1 = 0 as libc::c_int;
        while k1 < *fieldlen.offset(*field.offset(f as isize) as isize) {
            let fresh59 = k;
            k = k + 1;
            *keymap.offset(fresh59 as isize) = *sortord.offset(f as isize);
            k1 += 1;
            k1;
        }
        f += 1;
        f;
    }
    write(srt, header as *const libc::c_void, (headerlen + 1 as libc::c_int) as size_t);
    dap_free(
        headerfield as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    return headerlen + 1 as libc::c_int;
}
unsafe extern "C" fn dsort(
    mut origset: *mut libc::c_char,
    mut sortset: *mut libc::c_char,
    mut sortvar: *mut libc::c_int,
    mut nsort: libc::c_int,
    mut uniq: libc::c_int,
    mut mod_1: *mut libc::c_char,
    mut nmods_0: libc::c_int,
) {
    let mut f: libc::c_int = 0;
    let mut orig: libc::c_int = 0;
    let mut nseg: libc::c_int = 0;
    let mut srt: libc::c_int = 0;
    let mut header: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_int = 0;
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
    header = dap_malloc(
        dap_linelen,
        b"header\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    field = dap_malloc(
        (dap_maxvar as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"field\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    unfield = dap_malloc(
        (dap_maxvar as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"unfield\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    sortord = dap_malloc(
        (dap_maxvar as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"dsortord\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    nfields = nsort;
    if nmods_0 == 0 {
        f = 0 as libc::c_int;
        while f < nfields {
            *mod_1.offset(f as isize) = 'i' as i32 as libc::c_char;
            f += 1;
            f;
        }
    } else if nmods_0 != nfields {
        fprintf(
            dap_err,
            b"(dsort) nmods (%d) != nfields (%d)\n\0" as *const u8
                as *const libc::c_char,
            nmods_0,
            nfields,
        );
        exit(1 as libc::c_int);
    }
    unique = uniq;
    f = 0 as libc::c_int;
    while f < nfields {
        *sortord
            .offset(
                f as isize,
            ) = if *mod_1.offset(f as isize) as libc::c_int == 'i' as i32 {
            1 as libc::c_int
        } else {
            0 as libc::c_int
        };
        *field.offset(f as isize) = *sortvar.offset(f as isize);
        f += 1;
        f;
    }
    if stat(dap_tmpdir, &mut statbuf) < 0 as libc::c_int {
        if mkdir(dap_tmpdir, 0o700 as libc::c_int as mode_t) < 0 as libc::c_int {
            perror(dap_dapname);
            exit(1 as libc::c_int);
        }
    } else if statbuf.st_mode & 0o40000 as libc::c_int as libc::c_uint == 0 {
        fprintf(
            dap_err,
            b"%s: non-directory file exists: %s\n\0" as *const u8 as *const libc::c_char,
            dap_dapname,
            dap_tmpdir,
        );
        exit(1 as libc::c_int);
    }
    mem1 = dap_malloc(
        2 as libc::c_int * dap_maxmem,
        b"mem1\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    mem2 = mem1.offset(dap_maxmem as isize);
    if nfields == 0 {
        fputs(
            b"(dsort) no fields specified for sorting\n\0" as *const u8
                as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    orig = open(origset, 0 as libc::c_int);
    if orig < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(dsort) can't read %s\n\0" as *const u8 as *const libc::c_char,
            origset,
        );
        exit(1 as libc::c_int);
    }
    srt = open(
        sortset,
        0o1 as libc::c_int | 0o100 as libc::c_int | 0o1000 as libc::c_int,
        0o400 as libc::c_int | 0o200 as libc::c_int,
    );
    if srt < 0 as libc::c_int {
        fprintf(
            dap_err,
            b"(dsort) can't write %s\n\0" as *const u8 as *const libc::c_char,
            sortset,
        );
        exit(1 as libc::c_int);
    }
    if read(orig, header as *mut libc::c_void, dap_linelen as size_t)
        < 0 as libc::c_int as libc::c_long
    {
        fputs(
            b"(dsort) can't read header\n\0" as *const u8 as *const libc::c_char,
            dap_err,
        );
        exit(1 as libc::c_int);
    }
    fpos = fixheader(header, srt) as libc::c_ulong;
    lseek(orig, fpos as __off_t, 0 as libc::c_int);
    sprintf(
        tmplate.as_mut_ptr().offset(12 as libc::c_int as isize),
        b"%06d\0" as *const u8 as *const libc::c_char,
        getpid() % 1000000 as libc::c_int,
    );
    nseg = 0 as libc::c_int;
    while sortseg(orig, nseg) != 0 {
        lseek(orig, fpos as __off_t, 0 as libc::c_int);
        nseg += 1;
        nseg;
    }
    merge(nseg, srt);
    cleanup(nseg);
    dap_free(
        header as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        mem1 as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        fieldstart as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        line as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        field as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        unfield as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        fieldlen as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        sortord as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        keyend as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        keymap as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
pub unsafe extern "C" fn bubblesort(
    mut list: *mut libc::c_int,
    mut n: libc::c_int,
    mut order: libc::c_int,
) -> *mut libc::c_int {
    let mut returndata: *mut libc::c_int = dap_malloc(
        (n as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"return\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < n {
        *returndata.offset(i as isize) = *list.offset(i as isize);
        i += 1;
        i;
    }
    let mut swappedcount: libc::c_int = 0 as libc::c_int;
    let mut swapped: libc::c_int = 0 as libc::c_int;
    loop {
        swapped = 0 as libc::c_int;
        i = 1 as libc::c_int;
        i = 1 as libc::c_int;
        while i < n {
            if order > 0 as libc::c_int {
                if *returndata.offset((i - 1 as libc::c_int) as isize)
                    > *returndata.offset(i as isize)
                {
                    let mut act: libc::c_int = *returndata.offset(i as isize);
                    *returndata
                        .offset(
                            i as isize,
                        ) = *returndata.offset((i - 1 as libc::c_int) as isize);
                    *returndata.offset((i - 1 as libc::c_int) as isize) = act;
                    swapped = 1 as libc::c_int;
                    swappedcount += 1;
                    swappedcount;
                }
            } else if *returndata.offset((i - 1 as libc::c_int) as isize)
                < *returndata.offset(i as isize)
            {
                let mut act_0: libc::c_int = *returndata.offset(i as isize);
                *returndata
                    .offset(
                        i as isize,
                    ) = *returndata.offset((i - 1 as libc::c_int) as isize);
                *returndata.offset((i - 1 as libc::c_int) as isize) = act_0;
                swapped = 1 as libc::c_int;
                swappedcount += 1;
                swappedcount;
            }
            i += 1;
            i;
        }
        if !(swapped != 0) {
            break;
        }
    }
    printf(b"nb swapped =%d\n\0" as *const u8 as *const libc::c_char, swappedcount);
    return returndata;
}
pub unsafe extern "C" fn surveyselect(
    mut fname: *mut libc::c_char,
    mut outname: *mut libc::c_char,
    mut method: *mut libc::c_char,
    mut tirage: libc::c_int,
) {
    inset(fname);
    let mut nbLines: libc::c_int = 0 as libc::c_int;
    while step() != 0 {
        nbLines += 1;
        nbLines;
    }
    printf(
        b"nblines to read = %d, nb selected = %d\n\0" as *const u8
            as *const libc::c_char,
        nbLines,
        tirage,
    );
    let mut list: *mut libc::c_int = dap_malloc(
        (tirage as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
            as libc::c_int,
        b"list\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as *mut libc::c_int;
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut sysrand: libc::c_int = rand() % (nbLines / tirage);
    printf(b"reload data\n\0" as *const u8 as *const libc::c_char);
    printf(b"create index list based on method\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < tirage {
        if strcmp(method, b"SRS\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            let mut notalone: libc::c_int = 0 as libc::c_int;
            let mut choice: libc::c_int = 0 as libc::c_int;
            loop {
                choice = rand() % nbLines;
                let mut j: libc::c_int = 0 as libc::c_int;
                notalone = 0 as libc::c_int;
                j = 0 as libc::c_int;
                while j < i {
                    if *list.offset(j as isize) == choice {
                        notalone = 1 as libc::c_int;
                    }
                    j += 1;
                    j;
                }
                if !(notalone == 1 as libc::c_int) {
                    break;
                }
            }
            *list.offset(i as isize) = choice;
        } else if strcmp(method, b"SYS\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            if i == 0 as libc::c_int {
                *list.offset(i as isize) = sysrand;
            } else {
                *list
                    .offset(
                        i as isize,
                    ) = *list.offset((i - 1 as libc::c_int) as isize) + nbLines / tirage;
            }
        }
        i += 1;
        i;
    }
    printf(b"sort values\n\0" as *const u8 as *const libc::c_char);
    let mut listsorted: *mut libc::c_int = bubblesort(list, tirage, 1 as libc::c_int);
    let mut counter: libc::c_int = 0 as libc::c_int;
    let mut index: libc::c_int = 0 as libc::c_int;
    printf(b"set output\n\0" as *const u8 as *const libc::c_char);
    inset(fname);
    outset(outname, b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char);
    while step() != 0 {
        let mut founded: libc::c_int = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < tirage {
            if *listsorted.offset(i as isize) == counter {
                founded = 1 as libc::c_int;
            }
            i += 1;
            i;
        }
        if founded != 0 {
            output();
            index += 1;
            index;
        }
        counter += 1;
        counter;
    }
    dap_free(
        list as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    dap_free(
        listsorted as *mut libc::c_void,
        b"\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
}
