use ::libc;
extern "C" {
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn unlinkat(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn rmdir_error(_: *const libc::c_char);
    fn unlink_error(_: *const libc::c_char);
    static mut records_written: off_t;
    fn normalize_filename_x(name: *mut libc::c_char);
    fn tar_dirname() -> *const libc::c_char;
    static mut chdir_current: libc::c_int;
    static mut chdir_fd: libc::c_int;
    fn chdir_do(dir: libc::c_int);
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct deferred_unlink {
    pub next: *mut deferred_unlink,
    pub dir_idx: libc::c_int,
    pub file_name: *mut libc::c_char,
    pub is_dir: bool,
    pub records_written: off_t,
}
static mut dunlink_head: *mut deferred_unlink = 0 as *const deferred_unlink
    as *mut deferred_unlink;
static mut dunlink_tail: *mut deferred_unlink = 0 as *const deferred_unlink
    as *mut deferred_unlink;
static mut dunlink_count: size_t = 0;
static mut dunlink_avail: *mut deferred_unlink = 0 as *const deferred_unlink
    as *mut deferred_unlink;
static mut deferred_unlink_delay: size_t = 0 as libc::c_int as size_t;
unsafe extern "C" fn dunlink_alloc() -> *mut deferred_unlink {
    let mut p: *mut deferred_unlink = 0 as *mut deferred_unlink;
    if !dunlink_avail.is_null() {
        p = dunlink_avail;
        dunlink_avail = (*p).next;
        (*p).next = 0 as *mut deferred_unlink;
    } else {
        p = xmalloc(::std::mem::size_of::<deferred_unlink>() as libc::c_ulong)
            as *mut deferred_unlink;
    }
    return p;
}
unsafe extern "C" fn dunlink_insert(
    mut anchor: *mut deferred_unlink,
    mut p: *mut deferred_unlink,
) {
    if !anchor.is_null() {
        (*p).next = (*anchor).next;
        (*anchor).next = p;
    } else {
        (*p).next = dunlink_head;
        dunlink_head = p;
    }
    if ((*p).next).is_null() {
        dunlink_tail = p;
    }
    dunlink_count = dunlink_count.wrapping_add(1);
    dunlink_count;
}
unsafe extern "C" fn dunlink_reclaim(mut p: *mut deferred_unlink) {
    rpl_free((*p).file_name as *mut libc::c_void);
    (*p).next = dunlink_avail;
    dunlink_avail = p;
}
unsafe extern "C" fn flush_deferred_unlinks(mut force: bool) {
    let mut p: *mut deferred_unlink = 0 as *mut deferred_unlink;
    let mut prev: *mut deferred_unlink = 0 as *mut deferred_unlink;
    let mut saved_chdir: libc::c_int = chdir_current;
    let mut current_block_19: u64;
    p = dunlink_head;
    while !p.is_null() {
        let mut next: *mut deferred_unlink = (*p).next;
        if force as libc::c_int != 0
            || records_written as libc::c_ulong
                > ((*p).records_written as libc::c_ulong)
                    .wrapping_add(deferred_unlink_delay)
        {
            chdir_do((*p).dir_idx);
            if (*p).is_dir {
                let mut fname: *const libc::c_char = 0 as *const libc::c_char;
                if (*p).dir_idx != 0
                    && ((*p).is_dir as libc::c_int != 0
                        && (*((*p).file_name).offset(0 as libc::c_int as isize)
                            as libc::c_int == 0 as libc::c_int
                            || strcmp(
                                (*p).file_name,
                                b".\0" as *const u8 as *const libc::c_char,
                            ) == 0 as libc::c_int))
                {
                    prev = p;
                    p = next;
                    continue;
                } else {
                    fname = (*p).file_name;
                    if unlinkat(chdir_fd, fname, 0x200 as libc::c_int)
                        != 0 as libc::c_int
                    {
                        match *__errno_location() {
                            2 => {}
                            17 | 39 => {
                                current_block_19 = 8469801356334057207;
                                match current_block_19 {
                                    7996093731189617273 => {
                                        rmdir_error(fname);
                                    }
                                    _ => {
                                        prev = p;
                                        p = next;
                                        continue;
                                    }
                                }
                            }
                            _ => {
                                current_block_19 = 7996093731189617273;
                                match current_block_19 {
                                    7996093731189617273 => {
                                        rmdir_error(fname);
                                    }
                                    _ => {
                                        prev = p;
                                        p = next;
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                }
            } else if unlinkat(chdir_fd, (*p).file_name, 0 as libc::c_int)
                != 0 as libc::c_int && *__errno_location() != 2 as libc::c_int
            {
                unlink_error((*p).file_name);
            }
            dunlink_reclaim(p);
            dunlink_count = dunlink_count.wrapping_sub(1);
            dunlink_count;
            p = next;
            if !prev.is_null() {
                (*prev).next = p;
            } else {
                dunlink_head = p;
            }
        } else {
            prev = p;
            p = next;
        }
    }
    if dunlink_head.is_null() {
        dunlink_tail = 0 as *mut deferred_unlink;
    } else if force {
        p = dunlink_head;
        while !p.is_null() {
            let mut next_0: *mut deferred_unlink = (*p).next;
            let mut fname_0: *const libc::c_char = 0 as *const libc::c_char;
            chdir_do((*p).dir_idx);
            if (*p).dir_idx != 0
                && ((*p).is_dir as libc::c_int != 0
                    && (*((*p).file_name).offset(0 as libc::c_int as isize)
                        as libc::c_int == 0 as libc::c_int
                        || strcmp(
                            (*p).file_name,
                            b".\0" as *const u8 as *const libc::c_char,
                        ) == 0 as libc::c_int))
            {
                fname_0 = tar_dirname();
                chdir_do((*p).dir_idx - 1 as libc::c_int);
            } else {
                fname_0 = (*p).file_name;
            }
            if unlinkat(chdir_fd, fname_0, 0x200 as libc::c_int) != 0 as libc::c_int {
                if *__errno_location() != 2 as libc::c_int {
                    rmdir_error(fname_0);
                }
            }
            dunlink_reclaim(p);
            dunlink_count = dunlink_count.wrapping_sub(1);
            dunlink_count;
            p = next_0;
        }
        dunlink_tail = 0 as *mut deferred_unlink;
        dunlink_head = dunlink_tail;
    }
    chdir_do(saved_chdir);
}
pub unsafe extern "C" fn finish_deferred_unlinks() {
    flush_deferred_unlinks(1 as libc::c_int != 0);
    while !dunlink_avail.is_null() {
        let mut next: *mut deferred_unlink = (*dunlink_avail).next;
        rpl_free(dunlink_avail as *mut libc::c_void);
        dunlink_avail = next;
    }
}
pub unsafe extern "C" fn queue_deferred_unlink(
    mut name: *const libc::c_char,
    mut is_dir: bool,
) {
    let mut p: *mut deferred_unlink = 0 as *mut deferred_unlink;
    if !dunlink_head.is_null()
        && records_written as libc::c_ulong
            > ((*dunlink_head).records_written as libc::c_ulong)
                .wrapping_add(deferred_unlink_delay)
    {
        flush_deferred_unlinks(0 as libc::c_int != 0);
    }
    p = dunlink_alloc();
    (*p).next = 0 as *mut deferred_unlink;
    (*p).dir_idx = chdir_current;
    (*p).file_name = xstrdup(name);
    normalize_filename_x((*p).file_name);
    (*p).is_dir = is_dir;
    (*p).records_written = records_written;
    if (*p).is_dir as libc::c_int != 0
        && (*((*p).file_name).offset(0 as libc::c_int as isize) as libc::c_int
            == 0 as libc::c_int
            || strcmp((*p).file_name, b".\0" as *const u8 as *const libc::c_char)
                == 0 as libc::c_int)
    {
        let mut q: *mut deferred_unlink = 0 as *mut deferred_unlink;
        let mut prev: *mut deferred_unlink = 0 as *mut deferred_unlink;
        q = dunlink_head;
        prev = 0 as *mut deferred_unlink;
        while !q.is_null() {
            if (*q).is_dir as libc::c_int != 0
                && (*((*q).file_name).offset(0 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                    || strcmp((*q).file_name, b".\0" as *const u8 as *const libc::c_char)
                        == 0 as libc::c_int) && (*q).dir_idx < (*p).dir_idx
            {
                break;
            }
            prev = q;
            q = (*q).next;
        }
        if !q.is_null() {
            dunlink_insert(prev, p);
        } else {
            dunlink_insert(dunlink_tail, p);
        }
    } else {
        dunlink_insert(dunlink_tail, p);
    };
}
