use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn free(__ptr: *mut libc::c_void);
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn bell();
    fn clear_eol();
    fn putbs();
    fn prchar(c: LWCHAR) -> *mut libc::c_char;
    fn prutfchar(ch: LWCHAR) -> *mut libc::c_char;
    fn utf_len(ch: libc::c_int) -> libc::c_int;
    fn is_utf8_well_formed(ss: *mut libc::c_char, slen: libc::c_int) -> libc::c_int;
    fn step_char(
        pp: *mut *mut libc::c_char,
        dir: libc::c_int,
        limit: *const libc::c_char,
    ) -> LWCHAR;
    fn is_composing_char(ch: LWCHAR) -> libc::c_int;
    fn is_ubin_char(ch: LWCHAR) -> libc::c_int;
    fn is_wide_char(ch: LWCHAR) -> libc::c_int;
    fn is_combining_char(ch1: LWCHAR, ch2: LWCHAR) -> libc::c_int;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn lgetenv(var: *mut libc::c_char) -> *mut libc::c_char;
    fn is_dir(filename: *mut libc::c_char) -> libc::c_int;
    fn back_textlist(tlist: *mut textlist, prev: *mut libc::c_char) -> *mut libc::c_char;
    fn forw_textlist(tlist: *mut textlist, prev: *mut libc::c_char) -> *mut libc::c_char;
    fn init_textlist(tlist: *mut textlist, str: *mut libc::c_char);
    fn shell_quote(s: *mut libc::c_char) -> *mut libc::c_char;
    fn fcomplete(s: *mut libc::c_char) -> *mut libc::c_char;
    fn get_meta_escape() -> *mut libc::c_char;
    fn editchar(c: libc::c_int, flags: libc::c_int) -> libc::c_int;
    fn in_mca() -> libc::c_int;
    fn dirfile(
        dirname: *mut libc::c_char,
        filename: *mut libc::c_char,
        must_exist: libc::c_int,
    ) -> *mut libc::c_char;
    fn isnullenv(s: *mut libc::c_char) -> libc::c_int;
    fn save_marks(fout: *mut FILE, hdr: *mut libc::c_char);
    fn restore_mark(line: *mut libc::c_char);
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn getfraction(
        sp: *mut *mut libc::c_char,
        printopt: *mut libc::c_char,
        errp: *mut libc::c_int,
    ) -> libc::c_long;
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn putstr(s: *const libc::c_char);
    fn help_ckd_add(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn help_ckd_mul(
        r: *mut libc::c_void,
        a: uintmax,
        b: uintmax,
        rsize: libc::c_int,
        rsigned: libc::c_int,
    ) -> libc::c_int;
    fn fchmod(__fd: libc::c_int, __mode: __mode_t) -> libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut sc_width: libc::c_int;
    static mut utf_mode: libc::c_int;
    static mut no_hist_dups: libc::c_int;
    static mut marks_modified: libc::c_int;
    static mut secure: libc::c_int;
}
pub type __uintmax_t = libc::c_ulong;
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
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
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
pub type uintmax_t = __uintmax_t;
pub type uintmax = uintmax_t;
pub type LWCHAR = libc::c_ulong;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct textlist {
    pub string: *mut libc::c_char,
    pub endstring: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mlist {
    pub next: *mut mlist,
    pub prev: *mut mlist,
    pub curr_mp: *mut mlist,
    pub string: *mut libc::c_char,
    pub modified: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct save_ctx {
    pub mlist: *mut mlist,
    pub fout: *mut FILE,
}
#[inline]
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
static mut cmdbuf: [libc::c_char; 2048] = [0; 2048];
static mut cmd_col: libc::c_int = 0;
static mut prompt_col: libc::c_int = 0;
static mut cp: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut cmd_offset: libc::c_int = 0;
static mut literal: libc::c_int = 0;
pub static mut updown_match: libc::c_int = -(1 as libc::c_int);
static mut in_completion: libc::c_int = 0 as libc::c_int;
static mut tk_text: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tk_original: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut tk_ipoint: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tk_trial: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut tk_tlist: textlist = textlist {
    string: 0 as *const libc::c_char as *mut libc::c_char,
    endstring: 0 as *const libc::c_char as *mut libc::c_char,
};
pub static mut openquote: libc::c_char = '"' as i32 as libc::c_char;
pub static mut closequote: libc::c_char = '"' as i32 as libc::c_char;
pub static mut mlist_search: mlist = unsafe {
    {
        let mut init = mlist {
            next: &mlist_search as *const mlist as *mut mlist,
            prev: &mlist_search as *const mlist as *mut mlist,
            curr_mp: &mlist_search as *const mlist as *mut mlist,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            modified: 0 as libc::c_int,
        };
        init
    }
};
pub static mut ml_search: *mut libc::c_void = unsafe {
    &mlist_search as *const mlist as *mut mlist as *mut libc::c_void
};
pub static mut mlist_examine: mlist = unsafe {
    {
        let mut init = mlist {
            next: &mlist_examine as *const mlist as *mut mlist,
            prev: &mlist_examine as *const mlist as *mut mlist,
            curr_mp: &mlist_examine as *const mlist as *mut mlist,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            modified: 0 as libc::c_int,
        };
        init
    }
};
pub static mut ml_examine: *mut libc::c_void = unsafe {
    &mlist_examine as *const mlist as *mut mlist as *mut libc::c_void
};
pub static mut mlist_shell: mlist = unsafe {
    {
        let mut init = mlist {
            next: &mlist_shell as *const mlist as *mut mlist,
            prev: &mlist_shell as *const mlist as *mut mlist,
            curr_mp: &mlist_shell as *const mlist as *mut mlist,
            string: 0 as *const libc::c_char as *mut libc::c_char,
            modified: 0 as libc::c_int,
        };
        init
    }
};
pub static mut ml_shell: *mut libc::c_void = unsafe {
    &mlist_shell as *const mlist as *mut mlist as *mut libc::c_void
};
static mut curr_mlist: *mut mlist = 0 as *const mlist as *mut mlist;
static mut curr_cmdflags: libc::c_int = 0;
static mut cmd_mbc_buf: [libc::c_char; 6] = [0; 6];
static mut cmd_mbc_buf_len: libc::c_int = 0;
static mut cmd_mbc_buf_index: libc::c_int = 0;
pub unsafe extern "C" fn cmd_reset() {
    cp = cmdbuf.as_mut_ptr();
    *cp = '\0' as i32 as libc::c_char;
    cmd_col = 0 as libc::c_int;
    cmd_offset = 0 as libc::c_int;
    literal = 0 as libc::c_int;
    cmd_mbc_buf_len = 0 as libc::c_int;
    updown_match = -(1 as libc::c_int);
}
pub unsafe extern "C" fn clear_cmd() {
    prompt_col = 0 as libc::c_int;
    cmd_col = prompt_col;
    cmd_mbc_buf_len = 0 as libc::c_int;
    updown_match = -(1 as libc::c_int);
}
pub unsafe extern "C" fn cmd_putstr(mut s: *const libc::c_char) {
    let mut prev_ch: LWCHAR = 0 as libc::c_int as LWCHAR;
    let mut ch: LWCHAR = 0;
    let mut endline: *const libc::c_char = s.offset(strlen(s) as isize);
    while *s as libc::c_int != '\0' as i32 {
        let mut ns: *mut libc::c_char = s as *mut libc::c_char;
        let mut width: libc::c_int = 0;
        ch = step_char(&mut ns, 1 as libc::c_int, endline);
        while s < ns as *const libc::c_char {
            let fresh0 = s;
            s = s.offset(1);
            putchr(*fresh0 as libc::c_int);
        }
        if utf_mode == 0 {
            width = 1 as libc::c_int;
        } else if is_composing_char(ch) != 0 || is_combining_char(prev_ch, ch) != 0 {
            width = 0 as libc::c_int;
        } else {
            width = if is_wide_char(ch) != 0 {
                2 as libc::c_int
            } else {
                1 as libc::c_int
            };
        }
        cmd_col += width;
        prompt_col += width;
        prev_ch = ch;
    }
}
pub unsafe extern "C" fn len_cmdbuf() -> libc::c_int {
    let mut s: *mut libc::c_char = cmdbuf.as_mut_ptr();
    let mut endline: *mut libc::c_char = s.offset(strlen(s) as isize);
    let mut len: libc::c_int = 0 as libc::c_int;
    while *s as libc::c_int != '\0' as i32 {
        step_char(&mut s, 1 as libc::c_int, endline);
        len += 1;
        len;
    }
    return len;
}
unsafe extern "C" fn cmd_step_common(
    mut p: *mut libc::c_char,
    mut ch: LWCHAR,
    mut len: libc::c_int,
    mut pwidth: *mut libc::c_int,
    mut bswidth: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut pr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: libc::c_int = 0;
    if len == 1 as libc::c_int {
        pr = prchar(ch as libc::c_int as LWCHAR);
        width = strlen(pr) as libc::c_int;
    } else {
        pr = prutfchar(ch);
        if is_composing_char(ch) != 0 {
            width = 0 as libc::c_int;
        } else if is_ubin_char(ch) != 0 {
            width = strlen(pr) as libc::c_int;
        } else {
            let mut prev_ch: LWCHAR = step_char(
                &mut p,
                -(1 as libc::c_int),
                cmdbuf.as_mut_ptr(),
            );
            if is_combining_char(prev_ch, ch) != 0 {
                width = 0 as libc::c_int;
            } else {
                width = if is_wide_char(ch) != 0 {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            }
        }
    }
    if !pwidth.is_null() {
        *pwidth = width;
    }
    if !bswidth.is_null() {
        *bswidth = width;
    }
    return pr;
}
unsafe extern "C" fn cmd_step_right(
    mut pp: *mut *mut libc::c_char,
    mut pwidth: *mut libc::c_int,
    mut bswidth: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = *pp;
    let mut ch: LWCHAR = step_char(pp, 1 as libc::c_int, p.offset(strlen(p) as isize));
    return cmd_step_common(
        p,
        ch,
        (*pp).offset_from(p) as libc::c_long as libc::c_int,
        pwidth,
        bswidth,
    );
}
unsafe extern "C" fn cmd_step_left(
    mut pp: *mut *mut libc::c_char,
    mut pwidth: *mut libc::c_int,
    mut bswidth: *mut libc::c_int,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = *pp;
    let mut ch: LWCHAR = step_char(pp, -(1 as libc::c_int), cmdbuf.as_mut_ptr());
    return cmd_step_common(
        *pp,
        ch,
        p.offset_from(*pp) as libc::c_long as libc::c_int,
        pwidth,
        bswidth,
    );
}
unsafe extern "C" fn cmd_home() {
    while cmd_col > prompt_col {
        let mut width: libc::c_int = 0;
        let mut bswidth: libc::c_int = 0;
        cmd_step_left(&mut cp, &mut width, &mut bswidth);
        loop {
            let fresh1 = bswidth;
            bswidth = bswidth - 1;
            if !(fresh1 > 0 as libc::c_int) {
                break;
            }
            putbs();
        }
        cmd_col -= width;
    }
    cp = &mut *cmdbuf.as_mut_ptr().offset(cmd_offset as isize) as *mut libc::c_char;
}
pub unsafe extern "C" fn cmd_repaint(mut old_cp: *const libc::c_char) {
    if old_cp.is_null() {
        old_cp = cp;
        cmd_home();
    }
    clear_eol();
    while *cp as libc::c_int != '\0' as i32 {
        let mut np: *mut libc::c_char = cp;
        let mut width: libc::c_int = 0;
        let mut pr: *mut libc::c_char = cmd_step_right(
            &mut np,
            &mut width,
            0 as *mut libc::c_int,
        );
        if cmd_col + width >= sc_width {
            break;
        }
        cp = np;
        putstr(pr);
        cmd_col += width;
    }
    while *cp as libc::c_int != '\0' as i32 {
        let mut np_0: *mut libc::c_char = cp;
        let mut width_0: libc::c_int = 0;
        let mut pr_0: *mut libc::c_char = cmd_step_right(
            &mut np_0,
            &mut width_0,
            0 as *mut libc::c_int,
        );
        if width_0 > 0 as libc::c_int {
            break;
        }
        cp = np_0;
        putstr(pr_0);
    }
    while cp > old_cp as *mut libc::c_char {
        cmd_left();
    }
}
unsafe extern "C" fn cmd_lshift() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cols: libc::c_int = 0;
    s = cmdbuf.as_mut_ptr().offset(cmd_offset as isize);
    cols = 0 as libc::c_int;
    while cols < (sc_width - prompt_col) / 2 as libc::c_int
        && *s as libc::c_int != '\0' as i32
    {
        let mut width: libc::c_int = 0;
        cmd_step_right(&mut s, &mut width, 0 as *mut libc::c_int);
        cols += width;
    }
    while *s as libc::c_int != '\0' as i32 {
        let mut width_0: libc::c_int = 0;
        let mut ns: *mut libc::c_char = s;
        cmd_step_right(&mut ns, &mut width_0, 0 as *mut libc::c_int);
        if width_0 > 0 as libc::c_int {
            break;
        }
        s = ns;
    }
    cmd_offset = s.offset_from(cmdbuf.as_mut_ptr()) as libc::c_long as libc::c_int;
    save_cp = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
unsafe extern "C" fn cmd_rshift() {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut save_cp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut cols: libc::c_int = 0;
    s = cmdbuf.as_mut_ptr().offset(cmd_offset as isize);
    cols = 0 as libc::c_int;
    while cols < (sc_width - prompt_col) / 2 as libc::c_int && s > cmdbuf.as_mut_ptr() {
        let mut width: libc::c_int = 0;
        cmd_step_left(&mut s, &mut width, 0 as *mut libc::c_int);
        cols += width;
    }
    cmd_offset = s.offset_from(cmdbuf.as_mut_ptr()) as libc::c_long as libc::c_int;
    save_cp = cp;
    cmd_home();
    cmd_repaint(save_cp);
}
unsafe extern "C" fn cmd_right() -> libc::c_int {
    let mut pr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ncp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: libc::c_int = 0;
    if *cp as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    ncp = cp;
    pr = cmd_step_right(&mut ncp, &mut width, 0 as *mut libc::c_int);
    if cmd_col + width >= sc_width {
        cmd_lshift();
    } else if cmd_col + width == sc_width - 1 as libc::c_int
        && *cp.offset(1 as libc::c_int as isize) as libc::c_int != '\0' as i32
    {
        cmd_lshift();
    }
    cp = ncp;
    cmd_col += width;
    putstr(pr);
    while *cp as libc::c_int != '\0' as i32 {
        pr = cmd_step_right(&mut ncp, &mut width, 0 as *mut libc::c_int);
        if width > 0 as libc::c_int {
            break;
        }
        putstr(pr);
        cp = ncp;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_left() -> libc::c_int {
    let mut ncp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut width: libc::c_int = 0 as libc::c_int;
    let mut bswidth: libc::c_int = 0 as libc::c_int;
    if cp <= cmdbuf.as_mut_ptr() {
        return 0 as libc::c_int;
    }
    ncp = cp;
    while ncp > cmdbuf.as_mut_ptr() {
        cmd_step_left(&mut ncp, &mut width, &mut bswidth);
        if width > 0 as libc::c_int {
            break;
        }
    }
    if cmd_col < prompt_col + width {
        cmd_rshift();
    }
    cp = ncp;
    cmd_col -= width;
    loop {
        let fresh2 = bswidth;
        bswidth = bswidth - 1;
        if !(fresh2 > 0 as libc::c_int) {
            break;
        }
        putbs();
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_ichar(
    mut cs: *mut libc::c_char,
    mut clen: libc::c_int,
) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if (strlen(cmdbuf.as_mut_ptr())).wrapping_add(clen as libc::c_ulong)
        >= (::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
    {
        bell();
        return 2 as libc::c_int;
    }
    s = &mut *cmdbuf
        .as_mut_ptr()
        .offset(
            (strlen
                as unsafe extern "C" fn(
                    *const libc::c_char,
                ) -> libc::c_ulong)(cmdbuf.as_mut_ptr()) as isize,
        ) as *mut libc::c_char;
    while s >= cp {
        *s.offset(clen as isize) = *s.offset(0 as libc::c_int as isize);
        s = s.offset(-1);
        s;
    }
    s = cp;
    while s < cp.offset(clen as isize) {
        let fresh3 = cs;
        cs = cs.offset(1);
        *s = *fresh3;
        s = s.offset(1);
        s;
    }
    updown_match = -(1 as libc::c_int);
    cmd_repaint(cp);
    cmd_right();
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_erase() -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut clen: libc::c_int = 0;
    if cp == cmdbuf.as_mut_ptr() {
        return 1 as libc::c_int;
    }
    s = cp;
    cmd_left();
    clen = s.offset_from(cp) as libc::c_long as libc::c_int;
    s = cp;
    loop {
        *s.offset(0 as libc::c_int as isize) = *s.offset(clen as isize);
        if *s.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
            break;
        }
        s = s.offset(1);
        s;
    }
    updown_match = -(1 as libc::c_int);
    cmd_repaint(cp);
    if curr_cmdflags & 0o1 as libc::c_int != 0 && cp == cmdbuf.as_mut_ptr()
        && *cp as libc::c_int == '\0' as i32
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_delete() -> libc::c_int {
    if *cp as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    cmd_right();
    cmd_erase();
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_werase() -> libc::c_int {
    if cp > cmdbuf.as_mut_ptr()
        && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
    {
        while cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
        {
            cmd_erase();
        }
    } else {
        while cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32
        {
            cmd_erase();
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_wdelete() -> libc::c_int {
    if *cp as libc::c_int == ' ' as i32 {
        while *cp as libc::c_int == ' ' as i32 {
            cmd_delete();
        }
    } else {
        while *cp as libc::c_int != ' ' as i32 && *cp as libc::c_int != '\0' as i32 {
            cmd_delete();
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn cmd_kill() -> libc::c_int {
    if cmdbuf[0 as libc::c_int as usize] as libc::c_int == '\0' as i32 {
        return 1 as libc::c_int;
    }
    cmd_offset = 0 as libc::c_int;
    cmd_home();
    *cp = '\0' as i32 as libc::c_char;
    updown_match = -(1 as libc::c_int);
    cmd_repaint(cp);
    if curr_cmdflags & 0o1 as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn set_mlist(
    mut mlist: *mut libc::c_void,
    mut cmdflags: libc::c_int,
) {
    curr_mlist = mlist as *mut mlist;
    curr_cmdflags = cmdflags;
    if !curr_mlist.is_null() {
        (*curr_mlist).curr_mp = curr_mlist;
    }
}
unsafe extern "C" fn cmd_updown(mut action: libc::c_int) -> libc::c_int {
    let mut s: *const libc::c_char = 0 as *const libc::c_char;
    let mut ml: *mut mlist = 0 as *mut mlist;
    if curr_mlist.is_null() {
        bell();
        return 0 as libc::c_int;
    }
    if updown_match < 0 as libc::c_int {
        updown_match = cp.offset_from(cmdbuf.as_mut_ptr()) as libc::c_long
            as libc::c_int;
    }
    ml = (*curr_mlist).curr_mp;
    loop {
        ml = if action == 13 as libc::c_int { (*ml).prev } else { (*ml).next };
        if ml == curr_mlist {
            break;
        }
        if strncmp(cmdbuf.as_mut_ptr(), (*ml).string, updown_match as libc::c_ulong)
            == 0 as libc::c_int
        {
            (*curr_mlist).curr_mp = ml;
            s = (*ml).string;
            if s.is_null() {
                s = b"\0" as *const u8 as *const libc::c_char;
            }
            cmd_offset = 0 as libc::c_int;
            cmd_home();
            clear_eol();
            strcpy(cmdbuf.as_mut_ptr(), s);
            cp = cmdbuf.as_mut_ptr();
            while *cp as libc::c_int != '\0' as i32 {
                cmd_right();
            }
            return 0 as libc::c_int;
        }
    }
    bell();
    return 0 as libc::c_int;
}
unsafe extern "C" fn ml_link(mut mlist: *mut mlist, mut ml: *mut mlist) {
    (*ml).next = mlist;
    (*ml).prev = (*mlist).prev;
    (*(*mlist).prev).next = ml;
    (*mlist).prev = ml;
}
unsafe extern "C" fn ml_unlink(mut ml: *mut mlist) {
    (*(*ml).prev).next = (*ml).next;
    (*(*ml).next).prev = (*ml).prev;
}
pub unsafe extern "C" fn cmd_addhist(
    mut mlist: *mut mlist,
    mut cmd: *const libc::c_char,
    mut modified: libc::c_int,
) {
    let mut ml: *mut mlist = 0 as *mut mlist;
    if strlen(cmd) == 0 as libc::c_int as libc::c_ulong {
        return;
    }
    if no_hist_dups != 0 {
        let mut next: *mut mlist = 0 as *mut mlist;
        ml = (*mlist).next;
        while !((*ml).string).is_null() {
            next = (*ml).next;
            if strcmp((*ml).string, cmd) == 0 as libc::c_int {
                ml_unlink(ml);
                free((*ml).string as *mut libc::c_void);
                free(ml as *mut libc::c_void);
            }
            ml = next;
        }
    }
    ml = (*mlist).prev;
    if ml == mlist || strcmp((*ml).string, cmd) != 0 as libc::c_int {
        ml = ecalloc(
            1 as libc::c_int,
            ::std::mem::size_of::<mlist>() as libc::c_ulong as libc::c_uint,
        ) as *mut mlist;
        (*ml).string = save(cmd);
        (*ml).modified = modified;
        ml_link(mlist, ml);
    }
    (*mlist).curr_mp = (*ml).next;
}
pub unsafe extern "C" fn cmd_accept() {
    if curr_mlist.is_null() || curr_mlist == ml_examine as *mut mlist {
        return;
    }
    cmd_addhist(curr_mlist, cmdbuf.as_mut_ptr(), 1 as libc::c_int);
    (*curr_mlist).modified = 1 as libc::c_int;
}
unsafe extern "C" fn cmd_edit(mut c: libc::c_int) -> libc::c_int {
    let mut action: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    flags = 0 as libc::c_int;
    if curr_mlist.is_null() {
        flags |= 0o2 as libc::c_int;
    }
    if curr_mlist == ml_search as *mut mlist || curr_mlist.is_null() {
        flags |= 0o4 as libc::c_int;
    }
    action = editchar(c, flags);
    match action {
        101 => return 0 as libc::c_int,
        3 => {
            in_completion = 0 as libc::c_int;
            return cmd_right();
        }
        4 => {
            in_completion = 0 as libc::c_int;
            return cmd_left();
        }
        6 => {
            in_completion = 0 as libc::c_int;
            while *cp as libc::c_int != '\0' as i32 && *cp as libc::c_int != ' ' as i32 {
                cmd_right();
            }
            while *cp as libc::c_int == ' ' as i32 {
                cmd_right();
            }
            return 0 as libc::c_int;
        }
        5 => {
            in_completion = 0 as libc::c_int;
            while cp > cmdbuf.as_mut_ptr()
                && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int == ' ' as i32
            {
                cmd_left();
            }
            while cp > cmdbuf.as_mut_ptr()
                && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32
            {
                cmd_left();
            }
            return 0 as libc::c_int;
        }
        9 => {
            in_completion = 0 as libc::c_int;
            cmd_offset = 0 as libc::c_int;
            cmd_home();
            cmd_repaint(cp);
            return 0 as libc::c_int;
        }
        10 => {
            in_completion = 0 as libc::c_int;
            while *cp as libc::c_int != '\0' as i32 {
                cmd_right();
            }
            return 0 as libc::c_int;
        }
        7 => {
            in_completion = 0 as libc::c_int;
            return 0 as libc::c_int;
        }
        1 => {
            in_completion = 0 as libc::c_int;
            return cmd_erase();
        }
        2 => {
            in_completion = 0 as libc::c_int;
            return cmd_kill();
        }
        20 => {
            in_completion = 0 as libc::c_int;
            cmd_kill();
            return 1 as libc::c_int;
        }
        11 => {
            in_completion = 0 as libc::c_int;
            return cmd_werase();
        }
        8 => {
            in_completion = 0 as libc::c_int;
            return cmd_delete();
        }
        12 => {
            in_completion = 0 as libc::c_int;
            return cmd_wdelete();
        }
        19 => {
            literal = 1 as libc::c_int;
            return 0 as libc::c_int;
        }
        13 | 14 => {
            in_completion = 0 as libc::c_int;
            return cmd_updown(action);
        }
        17 | 18 | 15 => return cmd_complete(action),
        _ => {
            in_completion = 0 as libc::c_int;
            return 3 as libc::c_int;
        }
    };
}
unsafe extern "C" fn cmd_istr(mut str: *mut libc::c_char) -> libc::c_int {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut action: libc::c_int = 0;
    let mut endline: *mut libc::c_char = str.offset(strlen(str) as isize);
    s = str;
    while *s as libc::c_int != '\0' as i32 {
        let mut os: *mut libc::c_char = s;
        step_char(&mut s, 1 as libc::c_int, endline);
        action = cmd_ichar(os, s.offset_from(os) as libc::c_long as libc::c_int);
        if action != 0 as libc::c_int {
            return action;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn delimit_word() -> *mut libc::c_char {
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut delim_quoted: libc::c_int = 0 as libc::c_int;
    let mut meta_quoted: libc::c_int = 0 as libc::c_int;
    let mut esc: *const libc::c_char = get_meta_escape();
    let mut esclen: libc::c_int = strlen(esc) as libc::c_int;
    if *cp as libc::c_int != ' ' as i32 && *cp as libc::c_int != '\0' as i32 {
        while *cp as libc::c_int != ' ' as i32 && *cp as libc::c_int != '\0' as i32 {
            cmd_right();
        }
    } else {
        cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int != ' ' as i32;
    }
    if cp == cmdbuf.as_mut_ptr() {
        return 0 as *mut libc::c_char;
    }
    word = cmdbuf.as_mut_ptr();
    while word < cp {
        if *word as libc::c_int != ' ' as i32 {
            break;
        }
        word = word.offset(1);
        word;
    }
    if word >= cp {
        return cp;
    }
    p = cmdbuf.as_mut_ptr();
    while p < cp {
        if meta_quoted != 0 {
            meta_quoted = 0 as libc::c_int;
        } else if esclen > 0 as libc::c_int && p.offset(esclen as isize) < cp
            && strncmp(p, esc, esclen as libc::c_ulong) == 0 as libc::c_int
        {
            meta_quoted = 1 as libc::c_int;
            p = p.offset((esclen - 1 as libc::c_int) as isize);
        } else if delim_quoted != 0 {
            if *p as libc::c_int == closequote as libc::c_int {
                delim_quoted = 0 as libc::c_int;
            }
        } else if *p as libc::c_int == openquote as libc::c_int {
            delim_quoted = 1 as libc::c_int;
        } else if *p as libc::c_int == ' ' as i32 {
            word = p.offset(1 as libc::c_int as isize);
        }
        p = p.offset(1);
        p;
    }
    return word;
}
unsafe extern "C" fn init_compl() {
    let mut word: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
    if !tk_text.is_null() {
        free(tk_text as *mut libc::c_void);
        tk_text = 0 as *mut libc::c_char;
    }
    word = delimit_word();
    if word.is_null() {
        return;
    }
    tk_ipoint = word;
    if !tk_original.is_null() {
        free(tk_original as *mut libc::c_void);
    }
    tk_original = ecalloc(
        (cp.offset_from(word) as libc::c_long + 1 as libc::c_int as libc::c_long)
            as libc::c_int,
        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as libc::c_uint,
    ) as *mut libc::c_char;
    strncpy(tk_original, word, cp.offset_from(word) as libc::c_long as libc::c_ulong);
    c = *cp;
    *cp = '\0' as i32 as libc::c_char;
    if *word as libc::c_int != openquote as libc::c_int {
        tk_text = fcomplete(word);
    } else {
        let mut qword: *mut libc::c_char = shell_quote(
            word.offset(1 as libc::c_int as isize),
        );
        if qword.is_null() {
            tk_text = fcomplete(word.offset(1 as libc::c_int as isize));
        } else {
            tk_text = fcomplete(qword);
            free(qword as *mut libc::c_void);
        }
    }
    *cp = c;
}
unsafe extern "C" fn next_compl(
    mut action: libc::c_int,
    mut prev: *mut libc::c_char,
) -> *mut libc::c_char {
    match action {
        17 => return forw_textlist(&mut tk_tlist, prev),
        18 => return back_textlist(&mut tk_tlist, prev),
        _ => {}
    }
    return b"?\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
unsafe extern "C" fn cmd_complete(mut action: libc::c_int) -> libc::c_int {
    let mut current_block: u64;
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    if in_completion == 0 || action == 15 as libc::c_int {
        init_compl();
        if tk_text.is_null() {
            bell();
            return 0 as libc::c_int;
        }
        if action == 15 as libc::c_int {
            tk_trial = tk_text;
        } else {
            in_completion = 1 as libc::c_int;
            init_textlist(&mut tk_tlist, tk_text);
            tk_trial = next_compl(action, 0 as *mut libc::c_void as *mut libc::c_char);
        }
    } else {
        tk_trial = next_compl(action, tk_trial);
    }
    while cp > tk_ipoint {
        cmd_erase();
    }
    if tk_trial.is_null() {
        in_completion = 0 as libc::c_int;
        if cmd_istr(tk_original) != 0 as libc::c_int {
            current_block = 10993025135213339818;
        } else {
            current_block = 2719512138335094285;
        }
    } else if cmd_istr(tk_trial) != 0 as libc::c_int {
        current_block = 10993025135213339818;
    } else if is_dir(tk_trial) != 0 {
        if cp > cmdbuf.as_mut_ptr()
            && *cp.offset(-(1 as libc::c_int) as isize) as libc::c_int
                == closequote as libc::c_int
        {
            cmd_erase();
        }
        s = lgetenv(
            b"LESSSEPARATOR\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if s.is_null() {
            s = b"/\0" as *const u8 as *const libc::c_char as *mut libc::c_char;
        }
        if cmd_istr(s) != 0 as libc::c_int {
            current_block = 10993025135213339818;
        } else {
            current_block = 2719512138335094285;
        }
    } else {
        current_block = 2719512138335094285;
    }
    match current_block {
        2719512138335094285 => return 0 as libc::c_int,
        _ => {
            in_completion = 0 as libc::c_int;
            bell();
            return 0 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn cmd_char(mut c: libc::c_int) -> libc::c_int {
    let mut action: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    if utf_mode == 0 {
        cmd_mbc_buf[0 as libc::c_int as usize] = c as libc::c_char;
        len = 1 as libc::c_int;
    } else {
        let mut current_block_24: u64;
        if cmd_mbc_buf_len == 0 as libc::c_int {
            current_block_24 = 768829615447722766;
        } else if c & 0xc0 as libc::c_int == 0x80 as libc::c_int {
            let fresh4 = cmd_mbc_buf_index;
            cmd_mbc_buf_index = cmd_mbc_buf_index + 1;
            cmd_mbc_buf[fresh4 as usize] = c as libc::c_char;
            if cmd_mbc_buf_index < cmd_mbc_buf_len {
                return 0 as libc::c_int;
            }
            if is_utf8_well_formed(cmd_mbc_buf.as_mut_ptr(), cmd_mbc_buf_index) == 0 {
                cmd_mbc_buf_len = 0 as libc::c_int;
                bell();
                return 2 as libc::c_int;
            }
            current_block_24 = 5601891728916014340;
        } else {
            cmd_mbc_buf_len = 0 as libc::c_int;
            bell();
            current_block_24 = 768829615447722766;
        }
        match current_block_24 {
            768829615447722766 => {
                cmd_mbc_buf_index = 1 as libc::c_int;
                *cmd_mbc_buf.as_mut_ptr() = c as libc::c_char;
                if c & 0x80 as libc::c_int == 0 as libc::c_int {
                    cmd_mbc_buf_len = 1 as libc::c_int;
                } else if c & 0xc0 as libc::c_int == 0xc0 as libc::c_int
                    && !(c & 0xfe as libc::c_int == 0xfe as libc::c_int)
                {
                    cmd_mbc_buf_len = utf_len(c);
                    return 0 as libc::c_int;
                } else {
                    bell();
                    return 2 as libc::c_int;
                }
            }
            _ => {}
        }
        len = cmd_mbc_buf_len;
        cmd_mbc_buf_len = 0 as libc::c_int;
    }
    if literal != 0 {
        literal = 0 as libc::c_int;
        return cmd_ichar(cmd_mbc_buf.as_mut_ptr(), len);
    }
    if in_mca() != 0 && len == 1 as libc::c_int {
        action = cmd_edit(c);
        match action {
            0 | 1 => return action,
            3 | _ => {}
        }
    }
    return cmd_ichar(cmd_mbc_buf.as_mut_ptr(), len);
}
pub unsafe extern "C" fn cmd_int(mut frac: *mut libc::c_long) -> LINENUM {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: LINENUM = 0 as libc::c_int as LINENUM;
    let mut err: libc::c_int = 0;
    p = cmdbuf.as_mut_ptr();
    while *p as libc::c_int >= '0' as i32 && *p as libc::c_int <= '9' as i32 {
        if help_ckd_mul(
            &mut n as *mut LINENUM as *mut libc::c_void,
            n as uintmax,
            10 as libc::c_int as uintmax,
            ::std::mem::size_of::<LINENUM>() as libc::c_ulong as libc::c_int,
            (((if 1 as libc::c_int != 0 { 0 as libc::c_int as libc::c_long } else { n })
                - 1 as libc::c_int as libc::c_long) < 0 as libc::c_int as libc::c_long)
                as libc::c_int,
        ) != 0
            || help_ckd_add(
                &mut n as *mut LINENUM as *mut libc::c_void,
                n as uintmax,
                (*p as libc::c_int - '0' as i32) as uintmax,
                ::std::mem::size_of::<LINENUM>() as libc::c_ulong as libc::c_int,
                (((if 1 as libc::c_int != 0 {
                    0 as libc::c_int as libc::c_long
                } else {
                    n
                }) - 1 as libc::c_int as libc::c_long)
                    < 0 as libc::c_int as libc::c_long) as libc::c_int,
            ) != 0
        {
            error(
                b"Integer is too big\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
            return 0 as libc::c_int as LINENUM;
        }
        p = p.offset(1);
        p;
    }
    *frac = 0 as libc::c_int as libc::c_long;
    let fresh5 = p;
    p = p.offset(1);
    if *fresh5 as libc::c_int == '.' as i32 {
        *frac = getfraction(&mut p, 0 as *mut libc::c_char, &mut err);
    }
    return n;
}
pub unsafe extern "C" fn get_cmdbuf() -> *mut libc::c_char {
    if cmd_mbc_buf_index < cmd_mbc_buf_len {
        return 0 as *mut libc::c_char;
    }
    return cmdbuf.as_mut_ptr();
}
pub unsafe extern "C" fn cmd_lastpattern() -> *mut libc::c_char {
    if curr_mlist.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*(*(*curr_mlist).curr_mp).prev).string;
}
unsafe extern "C" fn mlist_size(mut ml: *mut mlist) -> libc::c_int {
    let mut size: libc::c_int = 0 as libc::c_int;
    ml = (*ml).next;
    while !((*ml).string).is_null() {
        size += 1;
        size;
        ml = (*ml).next;
    }
    return size;
}
unsafe extern "C" fn histfile_find(mut must_exist: libc::c_int) -> *mut libc::c_char {
    let mut home: *mut libc::c_char = lgetenv(
        b"HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = dirfile(
        lgetenv(
            b"XDG_STATE_HOME\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ),
        &*(b".lesshst\0" as *const u8 as *const libc::c_char)
            .offset(1 as libc::c_int as isize) as *const libc::c_char
            as *mut libc::c_char,
        must_exist,
    );
    if name.is_null() {
        let mut dir: *mut libc::c_char = dirfile(
            home,
            b".local/state\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            1 as libc::c_int,
        );
        if !dir.is_null() {
            name = dirfile(
                dir,
                &*(b".lesshst\0" as *const u8 as *const libc::c_char)
                    .offset(1 as libc::c_int as isize) as *const libc::c_char
                    as *mut libc::c_char,
                must_exist,
            );
            free(dir as *mut libc::c_void);
        }
    }
    if name.is_null() {
        name = dirfile(
            lgetenv(
                b"XDG_DATA_HOME\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
            ),
            &*(b".lesshst\0" as *const u8 as *const libc::c_char)
                .offset(1 as libc::c_int as isize) as *const libc::c_char
                as *mut libc::c_char,
            must_exist,
        );
    }
    if name.is_null() {
        name = dirfile(
            home,
            b".lesshst\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
            must_exist,
        );
    }
    return name;
}
unsafe extern "C" fn histfile_name(mut must_exist: libc::c_int) -> *mut libc::c_char {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    name = lgetenv(
        b"LESSHISTFILE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
    );
    if isnullenv(name) == 0 {
        if strcmp(name, b"-\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int
            || strcmp(name, b"/dev/null\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int
        {
            return 0 as *mut libc::c_char;
        }
        return save(name);
    }
    if strcmp(
        b".lesshst\0" as *const u8 as *const libc::c_char,
        b"\0" as *const u8 as *const libc::c_char,
    ) == 0 as libc::c_int
        || strcmp(
            b".lesshst\0" as *const u8 as *const libc::c_char,
            b"-\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
    {
        return 0 as *mut libc::c_char;
    }
    name = 0 as *mut libc::c_char;
    if must_exist == 0 {
        name = histfile_find(1 as libc::c_int);
    }
    if name.is_null() {
        name = histfile_find(must_exist);
    }
    return name;
}
unsafe extern "C" fn read_cmdhist2(
    mut action: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut mlist, *mut libc::c_char) -> (),
    >,
    mut uparam: *mut libc::c_void,
    mut skip_search: libc::c_int,
    mut skip_shell: libc::c_int,
) {
    let mut ml: *mut mlist = 0 as *mut mlist;
    let mut line: [libc::c_char; 2048] = [0; 2048];
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skip: *mut libc::c_int = 0 as *mut libc::c_int;
    filename = histfile_name(1 as libc::c_int);
    if filename.is_null() {
        return;
    }
    f = fopen(filename, b"r\0" as *const u8 as *const libc::c_char);
    free(filename as *mut libc::c_void);
    if f.is_null() {
        return;
    }
    if (fgets(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
        || strncmp(
            line.as_mut_ptr(),
            b".less-history-file:\0" as *const u8 as *const libc::c_char,
            strlen(b".less-history-file:\0" as *const u8 as *const libc::c_char),
        ) != 0 as libc::c_int
    {
        fclose(f);
        return;
    }
    while !(fgets(
        line.as_mut_ptr(),
        ::std::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong as libc::c_int,
        f,
    ))
        .is_null()
    {
        p = line.as_mut_ptr();
        while *p as libc::c_int != '\0' as i32 {
            if *p as libc::c_int == '\n' as i32 || *p as libc::c_int == '\r' as i32 {
                *p = '\0' as i32 as libc::c_char;
                break;
            } else {
                p = p.offset(1);
                p;
            }
        }
        if strcmp(line.as_mut_ptr(), b".search\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            ml = &mut mlist_search;
            skip = &mut skip_search;
        } else if strcmp(
            line.as_mut_ptr(),
            b".shell\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ml = &mut mlist_shell;
            skip = &mut skip_shell;
        } else if strcmp(
            line.as_mut_ptr(),
            b".mark\0" as *const u8 as *const libc::c_char,
        ) == 0 as libc::c_int
        {
            ml = 0 as *mut mlist;
        } else if *line.as_mut_ptr() as libc::c_int == '"' as i32 {
            if !ml.is_null() {
                if !skip.is_null() && *skip > 0 as libc::c_int {
                    *skip -= 1;
                    *skip;
                } else {
                    (Some(action.unwrap()))
                        .unwrap()(
                        uparam,
                        ml,
                        line.as_mut_ptr().offset(1 as libc::c_int as isize),
                    );
                }
            }
        } else if *line.as_mut_ptr() as libc::c_int == 'm' as i32 {
            (Some(action.unwrap())).unwrap()(uparam, 0 as *mut mlist, line.as_mut_ptr());
        }
    }
    fclose(f);
}
unsafe extern "C" fn read_cmdhist(
    mut action: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut mlist, *mut libc::c_char) -> (),
    >,
    mut uparam: *mut libc::c_void,
    mut skip_search: libc::c_int,
    mut skip_shell: libc::c_int,
) {
    if secure != 0 {
        return;
    }
    read_cmdhist2(action, uparam, skip_search, skip_shell);
    (Some(action.unwrap())).unwrap()(uparam, 0 as *mut mlist, 0 as *mut libc::c_char);
}
unsafe extern "C" fn addhist_init(
    mut uparam: *mut libc::c_void,
    mut ml: *mut mlist,
    mut string: *mut libc::c_char,
) {
    if !ml.is_null() {
        cmd_addhist(ml, string, 0 as libc::c_int);
    } else if !string.is_null() {
        restore_mark(string);
    }
}
pub unsafe extern "C" fn init_cmdhist() {
    read_cmdhist(
        Some(
            addhist_init
                as unsafe extern "C" fn(
                    *mut libc::c_void,
                    *mut mlist,
                    *mut libc::c_char,
                ) -> (),
        ),
        0 as *mut libc::c_void,
        0 as libc::c_int,
        0 as libc::c_int,
    );
}
unsafe extern "C" fn write_mlist_header(mut ml: *mut mlist, mut f: *mut FILE) {
    if ml == &mut mlist_search as *mut mlist {
        fprintf(
            f,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b".search\0" as *const u8 as *const libc::c_char,
        );
    } else if ml == &mut mlist_shell as *mut mlist {
        fprintf(
            f,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b".shell\0" as *const u8 as *const libc::c_char,
        );
    }
}
unsafe extern "C" fn write_mlist(mut ml: *mut mlist, mut f: *mut FILE) {
    ml = (*ml).next;
    while !((*ml).string).is_null() {
        if !((*ml).modified == 0) {
            fprintf(f, b"\"%s\n\0" as *const u8 as *const libc::c_char, (*ml).string);
            (*ml).modified = 0 as libc::c_int;
        }
        ml = (*ml).next;
    }
    (*ml).modified = 0 as libc::c_int;
}
unsafe extern "C" fn make_tempname(
    mut filename: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut lastch: libc::c_char = 0;
    let mut tempname: *mut libc::c_char = ecalloc(
        1 as libc::c_int,
        (strlen(filename)).wrapping_add(1 as libc::c_int as libc::c_ulong)
            as libc::c_uint,
    ) as *mut libc::c_char;
    strcpy(tempname, filename);
    lastch = *tempname
        .offset(
            (strlen(tempname)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        );
    *tempname
        .offset(
            (strlen(tempname)).wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
        ) = (if lastch as libc::c_int == 'Q' as i32 { 'Z' as i32 } else { 'Q' as i32 })
        as libc::c_char;
    return tempname;
}
unsafe extern "C" fn copy_hist(
    mut uparam: *mut libc::c_void,
    mut ml: *mut mlist,
    mut string: *mut libc::c_char,
) {
    let mut ctx: *mut save_ctx = uparam as *mut save_ctx;
    if !ml.is_null() && ml != (*ctx).mlist {
        if !((*ctx).mlist).is_null() {
            write_mlist((*ctx).mlist, (*ctx).fout);
        }
        (*ctx).mlist = ml;
        write_mlist_header((*ctx).mlist, (*ctx).fout);
    }
    if string.is_null() {
        if mlist_search.modified != 0 {
            write_mlist_header(&mut mlist_search, (*ctx).fout);
            write_mlist(&mut mlist_search, (*ctx).fout);
        }
        if mlist_shell.modified != 0 {
            write_mlist_header(&mut mlist_shell, (*ctx).fout);
            write_mlist(&mut mlist_shell, (*ctx).fout);
        }
    } else if !ml.is_null() {
        fprintf((*ctx).fout, b"\"%s\n\0" as *const u8 as *const libc::c_char, string);
    }
}
unsafe extern "C" fn make_file_private(mut f: *mut FILE) {
    let mut do_chmod: libc::c_int = 1 as libc::c_int;
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
    let mut r: libc::c_int = fstat(fileno(f), &mut statbuf);
    if r < 0 as libc::c_int
        || !(statbuf.st_mode & 0o170000 as libc::c_int as libc::c_uint
            == 0o100000 as libc::c_int as libc::c_uint)
    {
        do_chmod = 0 as libc::c_int;
    }
    if do_chmod != 0 {
        fchmod(fileno(f), 0o600 as libc::c_int as __mode_t);
    }
}
unsafe extern "C" fn histfile_modified() -> libc::c_int {
    if mlist_search.modified != 0 {
        return 1 as libc::c_int;
    }
    if mlist_shell.modified != 0 {
        return 1 as libc::c_int;
    }
    if marks_modified != 0 {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn save_cmdhist() {
    let mut histname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut tempname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut skip_search: libc::c_int = 0;
    let mut skip_shell: libc::c_int = 0;
    let mut ctx: save_ctx = save_ctx {
        mlist: 0 as *mut mlist,
        fout: 0 as *mut FILE,
    };
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut fout: *mut FILE = 0 as *mut FILE;
    let mut histsize: libc::c_int = 0 as libc::c_int;
    if secure != 0 || histfile_modified() == 0 {
        return;
    }
    histname = histfile_name(0 as libc::c_int);
    if histname.is_null() {
        return;
    }
    tempname = make_tempname(histname);
    fout = fopen(tempname, b"w\0" as *const u8 as *const libc::c_char);
    if !fout.is_null() {
        make_file_private(fout);
        s = lgetenv(
            b"LESSHISTSIZE\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        if !s.is_null() {
            histsize = atoi(s);
        }
        if histsize <= 0 as libc::c_int {
            histsize = 100 as libc::c_int;
        }
        skip_search = mlist_size(&mut mlist_search) - histsize;
        skip_shell = mlist_size(&mut mlist_shell) - histsize;
        fprintf(
            fout,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            b".less-history-file:\0" as *const u8 as *const libc::c_char,
        );
        ctx.fout = fout;
        ctx.mlist = 0 as *mut mlist;
        read_cmdhist(
            Some(
                copy_hist
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *mut mlist,
                        *mut libc::c_char,
                    ) -> (),
            ),
            &mut ctx as *mut save_ctx as *mut libc::c_void,
            skip_search,
            skip_shell,
        );
        save_marks(
            fout,
            b".mark\0" as *const u8 as *const libc::c_char as *mut libc::c_char,
        );
        fclose(fout);
        rename(tempname, histname);
    }
    free(tempname as *mut libc::c_void);
    free(histname as *mut libc::c_void);
}
