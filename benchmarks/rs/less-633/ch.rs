use ::libc;
extern "C" {
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn clear_eol();
    fn curr_ifile_changed() -> libc::c_int;
    fn set_filestate(ifile: *mut libc::c_void, filestate: *mut libc::c_void);
    fn get_filestate(ifile: *mut libc::c_void) -> *mut libc::c_void;
    fn filesize(f: libc::c_int) -> POSITION;
    fn sleep_ms(ms: libc::c_int);
    fn ixerror(fmt: *mut libc::c_char, parg: *mut PARG);
    fn wait_message() -> *mut libc::c_char;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn iread(fd: libc::c_int, buf: *mut libc::c_uchar, len: libc::c_uint) -> libc::c_int;
    fn ierror(fmt: *mut libc::c_char, parg: *mut PARG);
    fn fstatfs(__fildes: libc::c_int, __buf: *mut statfs) -> libc::c_int;
    static mut autobuf: libc::c_int;
    static mut sigs: libc::c_int;
    static mut secure: libc::c_int;
    static mut screen_trashed: libc::c_int;
    static mut follow_mode: libc::c_int;
    static mut waiting_for_data: libc::c_int;
    static helpdata: [libc::c_char; 0];
    static size_helpdata: libc::c_int;
    static mut curr_ifile: *mut libc::c_void;
    static mut logfile: libc::c_int;
    static mut namelogfile: *mut libc::c_char;
}
pub type __off_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __fsid_t {
    pub __val: [libc::c_int; 2],
}
pub type __fsblkcnt_t = libc::c_ulong;
pub type __fsfilcnt_t = libc::c_ulong;
pub type __fsword_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
pub type POSITION = off_t;
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
pub struct filestate {
    pub buflist: bufnode,
    pub hashtbl: [bufnode; 1024],
    pub file: libc::c_int,
    pub flags: libc::c_int,
    pub fpos: POSITION,
    pub nbufs: libc::c_int,
    pub block: BLOCKNUM,
    pub offset: libc::c_uint,
    pub fsize: POSITION,
}
pub type BLOCKNUM = POSITION;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bufnode {
    pub next: *mut bufnode,
    pub prev: *mut bufnode,
    pub hnext: *mut bufnode,
    pub hprev: *mut bufnode,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct buf {
    pub node: bufnode,
    pub block: BLOCKNUM,
    pub datasize: libc::c_uint,
    pub data: [libc::c_uchar; 8192],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statfs {
    pub f_type: __fsword_t,
    pub f_bsize: __fsword_t,
    pub f_blocks: __fsblkcnt_t,
    pub f_bfree: __fsblkcnt_t,
    pub f_bavail: __fsblkcnt_t,
    pub f_files: __fsfilcnt_t,
    pub f_ffree: __fsfilcnt_t,
    pub f_fsid: __fsid_t,
    pub f_namelen: __fsword_t,
    pub f_frsize: __fsword_t,
    pub f_flags: __fsword_t,
    pub f_spare: [__fsword_t; 4],
}
pub static mut ignore_eoi: libc::c_int = 0;
static mut thisfile: *mut filestate = 0 as *const filestate as *mut filestate;
static mut ch_ungotchar: libc::c_int = -(1 as libc::c_int);
static mut maxbufs: libc::c_int = -(1 as libc::c_int);
unsafe extern "C" fn ch_get() -> libc::c_int {
    let mut current_block: u64;
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut n: libc::c_int = 0;
    let mut read_again: libc::c_int = 0;
    let mut h: libc::c_int = 0;
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return -(1 as libc::c_int);
    }
    if (*thisfile).buflist.next != &mut (*thisfile).buflist as *mut bufnode {
        bp = (*thisfile).buflist.next as *mut buf;
        if (*thisfile).block == (*bp).block && (*thisfile).offset < (*bp).datasize {
            return (*bp).data[(*thisfile).offset as usize] as libc::c_int;
        }
    }
    waiting_for_data = 0 as libc::c_int;
    h = ((*thisfile).block & (1024 as libc::c_int - 1 as libc::c_int) as libc::c_long)
        as libc::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    loop {
        if !(bn
            != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize)
                as *mut bufnode)
        {
            current_block = 10048703153582371463;
            break;
        }
        bp = bn as *mut buf;
        if (*bp).block == (*thisfile).block {
            if (*thisfile).offset >= (*bp).datasize {
                current_block = 10048703153582371463;
                break;
            } else {
                current_block = 10610715554510520839;
                break;
            }
        } else {
            bn = (*bn).hnext;
        }
    }
    match current_block {
        10048703153582371463 => {
            if bn
                == &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize)
                    as *mut bufnode
            {
                if (*thisfile).buflist.prev == &mut (*thisfile).buflist as *mut bufnode
                    || (*((*thisfile).buflist.prev as *mut buf)).block
                        != -(1 as libc::c_int) as libc::c_long
                {
                    if autobuf != 0 && (*thisfile).flags & 0o1 as libc::c_int == 0
                        || (maxbufs < 0 as libc::c_int || (*thisfile).nbufs < maxbufs)
                    {
                        if ch_addbuf() != 0 {
                            autobuf = 0 as libc::c_int;
                        }
                    }
                }
                bn = (*thisfile).buflist.prev;
                bp = bn as *mut buf;
                (*(*bn).hnext).hprev = (*bn).hprev;
                (*(*bn).hprev).hnext = (*bn).hnext;
                (*bp).block = (*thisfile).block;
                (*bp).datasize = 0 as libc::c_int as libc::c_uint;
                (*bn).hnext = (*thisfile).hashtbl[h as usize].hnext;
                (*bn)
                    .hprev = &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize)
                    as *mut bufnode;
                (*(*thisfile).hashtbl[h as usize].hnext).hprev = bn;
                (*thisfile).hashtbl[h as usize].hnext = bn;
            }
            current_block = 4775909272756257391;
        }
        _ => {}
    }
    loop {
        match current_block {
            10610715554510520839 => {
                if (*thisfile).buflist.next != bn {
                    (*(*bn).next).prev = (*bn).prev;
                    (*(*bn).prev).next = (*bn).next;
                    (*bn).next = (*thisfile).buflist.next;
                    (*bn).prev = &mut (*thisfile).buflist;
                    (*(*thisfile).buflist.next).prev = bn;
                    (*thisfile).buflist.next = bn;
                    (*(*bn).hnext).hprev = (*bn).hprev;
                    (*(*bn).hprev).hnext = (*bn).hnext;
                    (*bn).hnext = (*thisfile).hashtbl[h as usize].hnext;
                    (*bn)
                        .hprev = &mut *((*thisfile).hashtbl)
                        .as_mut_ptr()
                        .offset(h as isize) as *mut bufnode;
                    (*(*thisfile).hashtbl[h as usize].hnext).hprev = bn;
                    (*thisfile).hashtbl[h as usize].hnext = bn;
                }
                if (*thisfile).offset < (*bp).datasize {
                    break;
                } else {
                    current_block = 4775909272756257391;
                }
            }
            _ => {
                pos = (*thisfile).block * 8192 as libc::c_int as libc::c_long
                    + (*bp).datasize as libc::c_long;
                len = ch_length();
                if len != -(1 as libc::c_int) as POSITION && pos >= len {
                    return -(1 as libc::c_int);
                }
                if pos != (*thisfile).fpos {
                    if (*thisfile).flags & 0o1 as libc::c_int == 0 {
                        return '?' as i32;
                    }
                    if lseek((*thisfile).file, pos, 0 as libc::c_int)
                        == -(1 as libc::c_int) as off_t
                    {
                        error(
                            b"seek error\0" as *const u8 as *const libc::c_char
                                as *mut libc::c_char,
                            0 as *mut libc::c_void as *mut PARG,
                        );
                        clear_eol();
                        return -(1 as libc::c_int);
                    }
                    (*thisfile).fpos = pos;
                }
                if ch_ungotchar != -(1 as libc::c_int) {
                    (*bp).data[(*bp).datasize as usize] = ch_ungotchar as libc::c_uchar;
                    n = 1 as libc::c_int;
                    ch_ungotchar = -(1 as libc::c_int);
                } else if (*thisfile).flags & 0o10 as libc::c_int != 0 {
                    (*bp)
                        .data[(*bp).datasize
                        as usize] = *helpdata.as_ptr().offset((*thisfile).fpos as isize)
                        as libc::c_uchar;
                    n = 1 as libc::c_int;
                } else {
                    n = iread(
                        (*thisfile).file,
                        &mut *((*bp).data).as_mut_ptr().offset((*bp).datasize as isize),
                        (8192 as libc::c_int as libc::c_uint)
                            .wrapping_sub((*bp).datasize),
                    );
                }
                read_again = 0 as libc::c_int;
                if n == -(2 as libc::c_int) {
                    (*thisfile).fsize = pos;
                    return -(1 as libc::c_int);
                }
                if n == -(3 as libc::c_int) {
                    read_again = 1 as libc::c_int;
                    n = 0 as libc::c_int;
                }
                if n < 0 as libc::c_int {
                    error(
                        b"read error\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                    clear_eol();
                    n = 0 as libc::c_int;
                }
                if secure == 0 && logfile >= 0 as libc::c_int && n > 0 as libc::c_int {
                    write(
                        logfile,
                        &mut *((*bp).data).as_mut_ptr().offset((*bp).datasize as isize)
                            as *mut libc::c_uchar as *mut libc::c_char
                            as *const libc::c_void,
                        n as size_t,
                    );
                }
                (*thisfile).fpos += n as libc::c_long;
                (*bp).datasize = ((*bp).datasize).wrapping_add(n as libc::c_uint);
                if n == 0 as libc::c_int {
                    if read_again == 0 {
                        (*thisfile).fsize = pos;
                    }
                    if ignore_eoi != 0 || read_again != 0 {
                        if waiting_for_data == 0 {
                            let mut parg: PARG = parg {
                                p_string: 0 as *mut libc::c_char,
                            };
                            parg.p_string = wait_message();
                            ixerror(
                                b"%s\0" as *const u8 as *const libc::c_char
                                    as *mut libc::c_char,
                                &mut parg,
                            );
                            waiting_for_data = 1 as libc::c_int;
                        }
                        sleep_ms(50 as libc::c_int);
                    }
                    if ignore_eoi != 0 && follow_mode == 1 as libc::c_int
                        && curr_ifile_changed() != 0
                    {
                        screen_trashed = 2 as libc::c_int;
                        return -(1 as libc::c_int);
                    }
                    if sigs != 0 {
                        return -(1 as libc::c_int);
                    }
                }
                current_block = 10610715554510520839;
            }
        }
    }
    return (*bp).data[(*thisfile).offset as usize] as libc::c_int;
}
pub unsafe extern "C" fn ch_ungetchar(mut c: libc::c_int) {
    if c != -(1 as libc::c_int) && ch_ungotchar != -(1 as libc::c_int) {
        error(
            b"ch_ungetchar overrun\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
    ch_ungotchar = c;
}
pub unsafe extern "C" fn end_logfile() {
    static mut tried: libc::c_int = 0 as libc::c_int;
    if logfile < 0 as libc::c_int {
        return;
    }
    if tried == 0 && (*thisfile).fsize == -(1 as libc::c_int) as POSITION {
        tried = 1 as libc::c_int;
        ierror(
            b"Finishing logfile\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        while ch_forw_get() != -(1 as libc::c_int) {
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                break;
            }
        }
    }
    close(logfile);
    logfile = -(1 as libc::c_int);
    free(namelogfile as *mut libc::c_void);
    namelogfile = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn sync_logfile() {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut warned: libc::c_int = 0 as libc::c_int;
    let mut block: BLOCKNUM = 0;
    let mut nblocks: BLOCKNUM = 0;
    nblocks = ((*thisfile).fpos + 8192 as libc::c_int as libc::c_long
        - 1 as libc::c_int as libc::c_long) / 8192 as libc::c_int as libc::c_long;
    block = 0 as libc::c_int as BLOCKNUM;
    while block < nblocks {
        let mut wrote: libc::c_int = 0 as libc::c_int;
        bn = (*thisfile).buflist.next;
        while bn != &mut (*thisfile).buflist as *mut bufnode {
            bp = bn as *mut buf;
            if (*bp).block == block {
                write(
                    logfile,
                    ((*bp).data).as_mut_ptr() as *mut libc::c_char
                        as *const libc::c_void,
                    (*bp).datasize as size_t,
                );
                wrote = 1 as libc::c_int;
                break;
            } else {
                bn = (*bn).next;
            }
        }
        if wrote == 0 && warned == 0 {
            error(
                b"Warning: log file is incomplete\0" as *const u8 as *const libc::c_char
                    as *mut libc::c_char,
                0 as *mut libc::c_void as *mut PARG,
            );
            warned = 1 as libc::c_int;
        }
        block += 1;
        block;
    }
}
unsafe extern "C" fn buffered(mut block: BLOCKNUM) -> libc::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut h: libc::c_int = 0;
    h = (block & (1024 as libc::c_int - 1 as libc::c_int) as libc::c_long)
        as libc::c_int;
    bn = (*thisfile).hashtbl[h as usize].hnext;
    while bn
        != &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize) as *mut bufnode
    {
        bp = bn as *mut buf;
        if (*bp).block == block {
            return 1 as libc::c_int;
        }
        bn = (*bn).hnext;
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ch_seek(mut pos: POSITION) -> libc::c_int {
    let mut new_block: BLOCKNUM = 0;
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as libc::c_int;
    }
    len = ch_length();
    if pos < 0 as libc::c_int as POSITION
        || len != -(1 as libc::c_int) as POSITION && pos > len
    {
        return 1 as libc::c_int;
    }
    new_block = pos / 8192 as libc::c_int as libc::c_long;
    if (*thisfile).flags & 0o1 as libc::c_int == 0 && pos != (*thisfile).fpos
        && buffered(new_block) == 0
    {
        if (*thisfile).fpos > pos {
            return 1 as libc::c_int;
        }
        while (*thisfile).fpos < pos {
            if ch_forw_get() == -(1 as libc::c_int) {
                return 1 as libc::c_int;
            }
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                return 1 as libc::c_int;
            }
        }
        return 0 as libc::c_int;
    }
    (*thisfile).block = new_block;
    (*thisfile).offset = (pos % 8192 as libc::c_int as libc::c_long) as libc::c_uint;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ch_end_seek() -> libc::c_int {
    let mut len: POSITION = 0;
    if thisfile.is_null() {
        return 0 as libc::c_int;
    }
    if (*thisfile).flags & 0o1 as libc::c_int != 0 {
        (*thisfile).fsize = filesize((*thisfile).file);
    }
    len = ch_length();
    if len != -(1 as libc::c_int) as POSITION {
        return ch_seek(len);
    }
    while ch_forw_get() != -(1 as libc::c_int) {
        if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            return 1 as libc::c_int;
        }
    }
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ch_end_buffer_seek() -> libc::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut buf_pos: POSITION = 0;
    let mut end_pos: POSITION = 0;
    if thisfile.is_null() || (*thisfile).flags & 0o1 as libc::c_int != 0 {
        return ch_end_seek();
    }
    end_pos = 0 as libc::c_int as POSITION;
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        bp = bn as *mut buf;
        buf_pos = (*bp).block * 8192 as libc::c_int as libc::c_long
            + (*bp).datasize as libc::c_long;
        if buf_pos > end_pos {
            end_pos = buf_pos;
        }
        bn = (*bn).next;
    }
    return ch_seek(end_pos);
}
pub unsafe extern "C" fn ch_beg_seek() -> libc::c_int {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    let mut firstbn: *mut bufnode = 0 as *mut bufnode;
    if ch_seek(0 as libc::c_int as POSITION) == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    firstbn = (*thisfile).buflist.next;
    if firstbn == &mut (*thisfile).buflist as *mut bufnode {
        return 1 as libc::c_int;
    }
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        if (*(bn as *mut buf)).block < (*(firstbn as *mut buf)).block {
            firstbn = bn;
        }
        bn = (*bn).next;
    }
    (*thisfile).block = (*(firstbn as *mut buf)).block;
    (*thisfile).offset = 0 as libc::c_int as libc::c_uint;
    return 0 as libc::c_int;
}
pub unsafe extern "C" fn ch_length() -> POSITION {
    if thisfile.is_null() {
        return -(1 as libc::c_int) as POSITION;
    }
    if ignore_eoi != 0 {
        return -(1 as libc::c_int) as POSITION;
    }
    if (*thisfile).flags & 0o10 as libc::c_int != 0 {
        return size_helpdata as POSITION;
    }
    if (*thisfile).flags & 0o20 as libc::c_int != 0 {
        return 0 as libc::c_int as POSITION;
    }
    return (*thisfile).fsize;
}
pub unsafe extern "C" fn ch_tell() -> POSITION {
    if thisfile.is_null() {
        return -(1 as libc::c_int) as POSITION;
    }
    return (*thisfile).block * 8192 as libc::c_int as libc::c_long
        + (*thisfile).offset as libc::c_long;
}
pub unsafe extern "C" fn ch_forw_get() -> libc::c_int {
    let mut c: libc::c_int = 0;
    if thisfile.is_null() {
        return -(1 as libc::c_int);
    }
    c = ch_get();
    if c == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    if (*thisfile).offset < (8192 as libc::c_int - 1 as libc::c_int) as libc::c_uint {
        (*thisfile).offset = ((*thisfile).offset).wrapping_add(1);
        (*thisfile).offset;
    } else {
        (*thisfile).block += 1;
        (*thisfile).block;
        (*thisfile).offset = 0 as libc::c_int as libc::c_uint;
    }
    return c;
}
pub unsafe extern "C" fn ch_back_get() -> libc::c_int {
    if thisfile.is_null() {
        return -(1 as libc::c_int);
    }
    if (*thisfile).offset > 0 as libc::c_int as libc::c_uint {
        (*thisfile).offset = ((*thisfile).offset).wrapping_sub(1);
        (*thisfile).offset;
    } else {
        if (*thisfile).block <= 0 as libc::c_int as libc::c_long {
            return -(1 as libc::c_int);
        }
        if (*thisfile).flags & 0o1 as libc::c_int == 0
            && buffered((*thisfile).block - 1 as libc::c_int as libc::c_long) == 0
        {
            return -(1 as libc::c_int);
        }
        (*thisfile).block -= 1;
        (*thisfile).block;
        (*thisfile).offset = (8192 as libc::c_int - 1 as libc::c_int) as libc::c_uint;
    }
    return ch_get();
}
pub unsafe extern "C" fn ch_setbufspace(mut bufspace: libc::c_int) {
    if bufspace < 0 as libc::c_int {
        maxbufs = -(1 as libc::c_int);
    } else {
        let mut lbufk: libc::c_int = 8192 as libc::c_int / 1024 as libc::c_int;
        maxbufs = bufspace / lbufk
            + (bufspace % lbufk != 0 as libc::c_int) as libc::c_int;
        if maxbufs < 1 as libc::c_int {
            maxbufs = 1 as libc::c_int;
        }
    };
}
pub unsafe extern "C" fn ch_flush() {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags & 0o1 as libc::c_int == 0 {
        (*thisfile).fsize = -(1 as libc::c_int) as POSITION;
        return;
    }
    bn = (*thisfile).buflist.next;
    while bn != &mut (*thisfile).buflist as *mut bufnode {
        (*(bn as *mut buf)).block = -(1 as libc::c_int) as BLOCKNUM;
        bn = (*bn).next;
    }
    (*thisfile).fsize = filesize((*thisfile).file);
    (*thisfile).fpos = 0 as libc::c_int as POSITION;
    (*thisfile).block = 0 as libc::c_int as BLOCKNUM;
    (*thisfile).offset = 0 as libc::c_int as libc::c_uint;
    if (*thisfile).fsize == 0 as libc::c_int as libc::c_long {
        let mut st: statfs = statfs {
            f_type: 0,
            f_bsize: 0,
            f_blocks: 0,
            f_bfree: 0,
            f_bavail: 0,
            f_files: 0,
            f_ffree: 0,
            f_fsid: __fsid_t { __val: [0; 2] },
            f_namelen: 0,
            f_frsize: 0,
            f_flags: 0,
            f_spare: [0; 4],
        };
        if fstatfs((*thisfile).file, &mut st) == 0 as libc::c_int {
            if st.f_type == 0x9fa0 as libc::c_int as libc::c_long {
                (*thisfile).fsize = -(1 as libc::c_int) as POSITION;
                (*thisfile).flags &= !(0o1 as libc::c_int);
            }
        }
    }
    if lseek((*thisfile).file, 0 as libc::c_int as off_t, 0 as libc::c_int)
        == -(1 as libc::c_int) as off_t
    {
        error(
            b"seek error to 0\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    }
}
unsafe extern "C" fn ch_addbuf() -> libc::c_int {
    let mut bp: *mut buf = 0 as *mut buf;
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    bp = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::std::mem::size_of::<buf>() as libc::c_ulong,
    ) as *mut buf;
    if bp.is_null() {
        return 1 as libc::c_int;
    }
    (*thisfile).nbufs += 1;
    (*thisfile).nbufs;
    (*bp).block = -(1 as libc::c_int) as BLOCKNUM;
    bn = &mut (*bp).node;
    (*bn).next = &mut (*thisfile).buflist;
    (*bn).prev = (*thisfile).buflist.prev;
    (*(*thisfile).buflist.prev).next = bn;
    (*thisfile).buflist.prev = bn;
    (*bn).hnext = (*thisfile).hashtbl[0 as libc::c_int as usize].hnext;
    (*bn)
        .hprev = &mut *((*thisfile).hashtbl)
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize) as *mut bufnode;
    (*(*thisfile).hashtbl[0 as libc::c_int as usize].hnext).hprev = bn;
    (*thisfile).hashtbl[0 as libc::c_int as usize].hnext = bn;
    return 0 as libc::c_int;
}
unsafe extern "C" fn init_hashtbl() {
    let mut h: libc::c_int = 0;
    h = 0 as libc::c_int;
    while h < 1024 as libc::c_int {
        (*thisfile)
            .hashtbl[h as usize]
            .hnext = &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize)
            as *mut bufnode;
        (*thisfile)
            .hashtbl[h as usize]
            .hprev = &mut *((*thisfile).hashtbl).as_mut_ptr().offset(h as isize)
            as *mut bufnode;
        h += 1;
        h;
    }
}
unsafe extern "C" fn ch_delbufs() {
    let mut bn: *mut bufnode = 0 as *mut bufnode;
    while (*thisfile).buflist.next != &mut (*thisfile).buflist as *mut bufnode {
        bn = (*thisfile).buflist.next;
        (*(*bn).next).prev = (*bn).prev;
        (*(*bn).prev).next = (*bn).next;
        free(bn as *mut buf as *mut libc::c_void);
    }
    (*thisfile).nbufs = 0 as libc::c_int;
    init_hashtbl();
}
pub unsafe extern "C" fn seekable(mut f: libc::c_int) -> libc::c_int {
    return (lseek(f, 1 as libc::c_int as off_t, 0 as libc::c_int)
        != -(1 as libc::c_int) as off_t) as libc::c_int;
}
pub unsafe extern "C" fn ch_set_eof() {
    if (*thisfile).fsize != -(1 as libc::c_int) as POSITION
        && (*thisfile).fsize < (*thisfile).fpos
    {
        (*thisfile).fsize = (*thisfile).fpos;
    }
}
pub unsafe extern "C" fn ch_init(mut f: libc::c_int, mut flags: libc::c_int) {
    thisfile = get_filestate(curr_ifile) as *mut filestate;
    if thisfile.is_null() {
        thisfile = ecalloc(
            1 as libc::c_int,
            ::std::mem::size_of::<filestate>() as libc::c_ulong as libc::c_uint,
        ) as *mut filestate;
        (*thisfile).buflist.prev = &mut (*thisfile).buflist;
        (*thisfile).buflist.next = (*thisfile).buflist.prev;
        (*thisfile).nbufs = 0 as libc::c_int;
        (*thisfile).flags = flags;
        (*thisfile).fpos = 0 as libc::c_int as POSITION;
        (*thisfile).block = 0 as libc::c_int as BLOCKNUM;
        (*thisfile).offset = 0 as libc::c_int as libc::c_uint;
        (*thisfile).file = -(1 as libc::c_int);
        (*thisfile).fsize = -(1 as libc::c_int) as POSITION;
        init_hashtbl();
        if flags & 0o1 as libc::c_int != 0 && seekable(f) == 0 {
            (*thisfile).flags &= !(0o1 as libc::c_int);
        }
        set_filestate(curr_ifile, thisfile as *mut libc::c_void);
    }
    if (*thisfile).file == -(1 as libc::c_int) {
        (*thisfile).file = f;
    }
    ch_flush();
}
pub unsafe extern "C" fn ch_close() {
    let mut keepstate: libc::c_int = 0 as libc::c_int;
    if thisfile.is_null() {
        return;
    }
    if (*thisfile).flags
        & (0o1 as libc::c_int | 0o4 as libc::c_int | 0o10 as libc::c_int) != 0
        && (*thisfile).flags & 0o2 as libc::c_int == 0
    {
        ch_delbufs();
    } else {
        keepstate = 1 as libc::c_int;
    }
    if (*thisfile).flags & 0o2 as libc::c_int == 0 {
        if (*thisfile).flags & (0o4 as libc::c_int | 0o10 as libc::c_int) == 0 {
            close((*thisfile).file);
        }
        (*thisfile).file = -(1 as libc::c_int);
    } else {
        keepstate = 1 as libc::c_int;
    }
    if keepstate == 0 {
        free(thisfile as *mut libc::c_void);
        thisfile = 0 as *mut filestate;
        set_filestate(curr_ifile, 0 as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn ch_getflags() -> libc::c_int {
    if thisfile.is_null() {
        return 0 as libc::c_int;
    }
    return (*thisfile).flags;
}
