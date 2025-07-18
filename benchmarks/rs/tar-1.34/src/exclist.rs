use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type wordsplit_node;
    pub type exclude;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn fatal_exit() -> !;
    fn open_error(_: *const libc::c_char);
    static mut exit_status: libc::c_int;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn base_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn quotearg_colon(arg: *const libc::c_char) -> *mut libc::c_char;
    fn wordsplit(
        s: *const libc::c_char,
        ws: *mut wordsplit_t,
        flags: libc::c_uint,
    ) -> libc::c_int;
    fn wordsplit_free(ws: *mut wordsplit_t);
    static mut excluded: *mut exclude;
    fn subfile_open(
        dir: *const tar_stat_info,
        file: *const libc::c_char,
        flags: libc::c_int,
    ) -> libc::c_int;
    static mut chdir_fd: libc::c_int;
    fn new_exclude() -> *mut exclude;
    fn free_exclude(_: *mut exclude);
    fn add_exclude(_: *mut exclude, _: *const libc::c_char, _: libc::c_int);
    fn add_exclude_fp(
        _: Option::<
            unsafe extern "C" fn(
                *mut exclude,
                *const libc::c_char,
                libc::c_int,
                *mut libc::c_void,
            ) -> (),
        >,
        _: *mut exclude,
        _: *mut FILE,
        _: libc::c_int,
        _: libc::c_char,
        _: *mut libc::c_void,
    ) -> libc::c_int;
    fn excluded_file_name(_: *const exclude, _: *const libc::c_char) -> bool;
    fn exclude_add_pattern_buffer(ex: *mut exclude, buf: *mut libc::c_char);
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
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct obstack {
    pub chunk_size: size_t,
    pub chunk: *mut _obstack_chunk,
    pub object_base: *mut libc::c_char,
    pub next_free: *mut libc::c_char,
    pub chunk_limit: *mut libc::c_char,
    pub temp: C2RustUnnamed_2,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_1,
    pub freefun: C2RustUnnamed_0,
    pub extra_arg: *mut libc::c_void,
    #[bitfield(name = "use_extra_arg", ty = "libc::c_uint", bits = "0..=0")]
    #[bitfield(name = "maybe_empty_object", ty = "libc::c_uint", bits = "1..=1")]
    #[bitfield(name = "alloc_failed", ty = "libc::c_uint", bits = "2..=2")]
    pub use_extra_arg_maybe_empty_object_alloc_failed: [u8; 1],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 7],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_2 {
    pub i: size_t,
    pub p: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _obstack_chunk {
    pub limit: *mut libc::c_char,
    pub prev: *mut _obstack_chunk,
    pub contents: [libc::c_char; 0],
}
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wordsplit {
    pub ws_wordc: size_t,
    pub ws_wordv: *mut *mut libc::c_char,
    pub ws_offs: size_t,
    pub ws_wordn: size_t,
    pub ws_flags: libc::c_uint,
    pub ws_options: libc::c_uint,
    pub ws_maxwords: size_t,
    pub ws_wordi: size_t,
    pub ws_delim: *const libc::c_char,
    pub ws_comment: *const libc::c_char,
    pub ws_escape: [*const libc::c_char; 2],
    pub ws_alloc_die: Option::<unsafe extern "C" fn(*mut wordsplit_t) -> ()>,
    pub ws_error: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_debug: Option::<unsafe extern "C" fn(*const libc::c_char, ...) -> ()>,
    pub ws_env: *mut *const libc::c_char,
    pub ws_envbuf: *mut *mut libc::c_char,
    pub ws_envidx: size_t,
    pub ws_envsiz: size_t,
    pub ws_getvar: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_closure: *mut libc::c_void,
    pub ws_command: Option::<
        unsafe extern "C" fn(
            *mut *mut libc::c_char,
            *const libc::c_char,
            size_t,
            *mut *mut libc::c_char,
            *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub ws_input: *const libc::c_char,
    pub ws_len: size_t,
    pub ws_endp: size_t,
    pub ws_errno: libc::c_int,
    pub ws_usererr: *mut libc::c_char,
    pub ws_head: *mut wordsplit_node,
    pub ws_tail: *mut wordsplit_node,
    pub ws_lvl: libc::c_int,
}
pub type wordsplit_t = wordsplit;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sp_array {
    pub offset: off_t,
    pub numbytes: off_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xheader {
    pub stk: *mut obstack,
    pub size: size_t,
    pub buffer: *mut libc::c_char,
    pub string_length: uintmax_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xattr_array {
    pub xkey: *mut libc::c_char,
    pub xval_ptr: *mut libc::c_char,
    pub xval_len: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tar_stat_info {
    pub orig_file_name: *mut libc::c_char,
    pub file_name: *mut libc::c_char,
    pub had_trailing_slash: bool,
    pub link_name: *mut libc::c_char,
    pub uname: *mut libc::c_char,
    pub gname: *mut libc::c_char,
    pub cntx_name: *mut libc::c_char,
    pub acls_a_ptr: *mut libc::c_char,
    pub acls_a_len: size_t,
    pub acls_d_ptr: *mut libc::c_char,
    pub acls_d_len: size_t,
    pub stat: stat,
    pub atime: timespec,
    pub mtime: timespec,
    pub ctime: timespec,
    pub archive_file_size: off_t,
    pub is_sparse: bool,
    pub sparse_major: libc::c_uint,
    pub sparse_minor: libc::c_uint,
    pub sparse_map_avail: size_t,
    pub sparse_map_size: size_t,
    pub sparse_map: *mut sp_array,
    pub real_size: off_t,
    pub real_size_set: bool,
    pub sparse_name_done: bool,
    pub xattr_map_size: size_t,
    pub xattr_map: *mut xattr_array,
    pub xhdr: xheader,
    pub is_dumpdir: bool,
    pub skipped: bool,
    pub dumpdir: *mut libc::c_char,
    pub parent: *mut tar_stat_info,
    pub dirstream: *mut DIR,
    pub fd: libc::c_int,
    pub exclude_list: *mut exclist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct exclist {
    pub next: *mut exclist,
    pub prev: *mut exclist,
    pub flags: libc::c_int,
    pub excluded: *mut exclude,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct excfile {
    pub next: *mut excfile,
    pub flags: libc::c_int,
    pub name: [libc::c_char; 1],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vcs_ignore_file {
    pub filename: *const libc::c_char,
    pub flags: libc::c_int,
    pub addfn: add_fn,
    pub initfn: Option::<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
    pub data: *mut libc::c_void,
}
pub type add_fn = Option::<
    unsafe extern "C" fn(
        *mut exclude,
        *const libc::c_char,
        libc::c_int,
        *mut libc::c_void,
    ) -> (),
>;
static mut excfile_head: *mut excfile = 0 as *const excfile as *mut excfile;
static mut excfile_tail: *mut excfile = 0 as *const excfile as *mut excfile;
pub unsafe extern "C" fn excfile_add(
    mut name: *const libc::c_char,
    mut flags: libc::c_int,
) {
    let mut p: *mut excfile = xmalloc(
        (::std::mem::size_of::<excfile>() as libc::c_ulong).wrapping_add(strlen(name)),
    ) as *mut excfile;
    (*p).next = 0 as *mut excfile;
    (*p).flags = flags;
    strcpy(((*p).name).as_mut_ptr(), name);
    if !excfile_tail.is_null() {
        (*excfile_tail).next = p;
    } else {
        excfile_head = p;
    }
    excfile_tail = p;
}
pub unsafe extern "C" fn info_attach_exclist(mut dir: *mut tar_stat_info) {
    let mut file: *mut excfile = 0 as *mut excfile;
    let mut head: *mut exclist = 0 as *mut exclist;
    let mut tail: *mut exclist = 0 as *mut exclist;
    let mut ent: *mut exclist = 0 as *mut exclist;
    let mut vcsfile: *mut vcs_ignore_file = 0 as *mut vcs_ignore_file;
    if !((*dir).exclude_list).is_null() {
        return;
    }
    file = excfile_head;
    while !file.is_null() {
        if faccessat(
            if !dir.is_null() { (*dir).fd } else { chdir_fd },
            ((*file).name).as_mut_ptr(),
            0 as libc::c_int,
            0 as libc::c_int,
        ) == 0 as libc::c_int
        {
            let mut fp: *mut FILE = 0 as *mut FILE;
            let mut ex: *mut exclude = 0 as *mut exclude;
            let mut fd: libc::c_int = subfile_open(
                dir,
                ((*file).name).as_mut_ptr(),
                0 as libc::c_int,
            );
            if fd == -(1 as libc::c_int) {
                open_error(((*file).name).as_mut_ptr());
            } else {
                fp = fdopen(fd, b"r\0" as *const u8 as *const libc::c_char);
                if fp.is_null() {
                    if error_hook.is_some() {
                        error_hook.unwrap()();
                    }
                    error(
                        0 as libc::c_int,
                        *__errno_location(),
                        dcgettext(
                            0 as *const libc::c_char,
                            b"%s: fdopen failed\0" as *const u8 as *const libc::c_char,
                            5 as libc::c_int,
                        ),
                        ((*file).name).as_mut_ptr(),
                    );
                    exit_status = 2 as libc::c_int;
                    close(fd);
                } else {
                    if ex.is_null() {
                        ex = new_exclude();
                    }
                    vcsfile = get_vcs_ignore_file(((*file).name).as_mut_ptr());
                    if ((*vcsfile).initfn).is_some() {
                        (*vcsfile).data = ((*vcsfile).initfn).unwrap()((*vcsfile).data);
                    }
                    if add_exclude_fp(
                        (*vcsfile).addfn,
                        ex,
                        fp,
                        (1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 28 as libc::c_int
                            | (1 as libc::c_int) << 30 as libc::c_int,
                        '\n' as i32 as libc::c_char,
                        (*vcsfile).data,
                    ) != 0
                    {
                        let mut e: libc::c_int = *__errno_location();
                        if error_hook.is_some() {
                            error_hook.unwrap()();
                        }
                        error(
                            0 as libc::c_int,
                            e,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            quotearg_colon(((*file).name).as_mut_ptr()),
                        );
                        fatal_exit();
                    }
                    fclose(fp);
                    ent = xmalloc(::std::mem::size_of::<exclist>() as libc::c_ulong)
                        as *mut exclist;
                    (*ent).excluded = ex;
                    (*ent)
                        .flags = if (*file).flags == 0 as libc::c_int {
                        (*file).flags
                    } else {
                        (*vcsfile).flags
                    };
                    (*ent).prev = tail;
                    (*ent).next = 0 as *mut exclist;
                    if !tail.is_null() {
                        (*tail).next = ent;
                    } else {
                        head = ent;
                    }
                    tail = ent;
                }
            }
        }
        file = (*file).next;
    }
    (*dir).exclude_list = head;
}
pub unsafe extern "C" fn info_free_exclist(mut dir: *mut tar_stat_info) {
    let mut ep: *mut exclist = (*dir).exclude_list;
    while !ep.is_null() {
        let mut next: *mut exclist = (*ep).next;
        free_exclude((*ep).excluded);
        rpl_free(ep as *mut libc::c_void);
        ep = next;
    }
    (*dir).exclude_list = 0 as *mut exclist;
}
pub unsafe extern "C" fn excluded_name(
    mut name: *const libc::c_char,
    mut st: *mut tar_stat_info,
) -> bool {
    let mut ep: *mut exclist = 0 as *mut exclist;
    let mut rname: *const libc::c_char = 0 as *const libc::c_char;
    let mut bname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut result: bool = false;
    let mut nr: libc::c_int = 0 as libc::c_int;
    name = name.offset(0 as libc::c_int as isize);
    if excluded_file_name(excluded, name) {
        return 1 as libc::c_int != 0;
    }
    if st.is_null() {
        return 0 as libc::c_int != 0;
    }
    result = 0 as libc::c_int != 0;
    while !st.is_null() && !result {
        ep = (*st).exclude_list;
        while !ep.is_null() {
            if !((*ep).flags & nr != 0) {
                result = excluded_file_name((*ep).excluded, name);
                if result {
                    break;
                }
                if rname.is_null() {
                    rname = name;
                    while *rname as libc::c_int == '.' as i32
                        && *rname.offset(1 as libc::c_int as isize) as libc::c_int
                            == '/' as i32
                    {
                        rname = rname.offset(2 as libc::c_int as isize);
                    }
                }
                result = excluded_file_name((*ep).excluded, rname);
                if result {
                    break;
                }
                if bname.is_null() {
                    bname = base_name(name);
                }
                result = excluded_file_name((*ep).excluded, bname);
                if result {
                    break;
                }
            }
            ep = (*ep).next;
        }
        st = (*st).parent;
        nr = 0x2 as libc::c_int;
    }
    rpl_free(bname as *mut libc::c_void);
    return result;
}
unsafe extern "C" fn cvs_addfn(
    mut ex: *mut exclude,
    mut pattern: *const libc::c_char,
    mut options: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut ws: wordsplit = wordsplit {
        ws_wordc: 0,
        ws_wordv: 0 as *mut *mut libc::c_char,
        ws_offs: 0,
        ws_wordn: 0,
        ws_flags: 0,
        ws_options: 0,
        ws_maxwords: 0,
        ws_wordi: 0,
        ws_delim: 0 as *const libc::c_char,
        ws_comment: 0 as *const libc::c_char,
        ws_escape: [0 as *const libc::c_char; 2],
        ws_alloc_die: None,
        ws_error: None,
        ws_debug: None,
        ws_env: 0 as *mut *const libc::c_char,
        ws_envbuf: 0 as *mut *mut libc::c_char,
        ws_envidx: 0,
        ws_envsiz: 0,
        ws_getvar: None,
        ws_closure: 0 as *mut libc::c_void,
        ws_command: None,
        ws_input: 0 as *const libc::c_char,
        ws_len: 0,
        ws_endp: 0,
        ws_errno: 0,
        ws_usererr: 0 as *mut libc::c_char,
        ws_head: 0 as *mut wordsplit_node,
        ws_tail: 0 as *mut wordsplit_node,
        ws_lvl: 0,
    };
    let mut i: size_t = 0;
    if wordsplit(
        pattern,
        &mut ws,
        (0x40 as libc::c_int | 0x4 as libc::c_int | 0x800 as libc::c_int) as libc::c_uint,
    ) != 0
    {
        return;
    }
    i = 0 as libc::c_int as size_t;
    while i < ws.ws_wordc {
        add_exclude(ex, *(ws.ws_wordv).offset(i as isize), options);
        i = i.wrapping_add(1);
        i;
    }
    wordsplit_free(&mut ws);
}
unsafe extern "C" fn git_addfn(
    mut ex: *mut exclude,
    mut pattern: *const libc::c_char,
    mut options: libc::c_int,
    mut data: *mut libc::c_void,
) {
    while *(*__ctype_b_loc()).offset(*pattern as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        pattern = pattern.offset(1);
        pattern;
    }
    if *pattern as libc::c_int == 0 as libc::c_int
        || *pattern as libc::c_int == '#' as i32
    {
        return;
    }
    if *pattern as libc::c_int == '\\' as i32
        && *pattern.offset(1 as libc::c_int as isize) as libc::c_int == '#' as i32
    {
        pattern = pattern.offset(1);
        pattern;
    }
    add_exclude(ex, pattern, options);
}
unsafe extern "C" fn bzr_addfn(
    mut ex: *mut exclude,
    mut pattern: *const libc::c_char,
    mut options: libc::c_int,
    mut data: *mut libc::c_void,
) {
    while *(*__ctype_b_loc()).offset(*pattern as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        pattern = pattern.offset(1);
        pattern;
    }
    if *pattern as libc::c_int == 0 as libc::c_int
        || *pattern as libc::c_int == '#' as i32
    {
        return;
    }
    if *pattern as libc::c_int == '!' as i32 {
        pattern = pattern.offset(1);
        if *pattern as libc::c_int == '!' as i32 {
            pattern = pattern.offset(1);
            pattern;
        } else {
            options |= (1 as libc::c_int) << 29 as libc::c_int;
        }
    }
    if strncmp(
        pattern,
        b"RE:\0" as *const u8 as *const libc::c_char,
        3 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        pattern = pattern.offset(3 as libc::c_int as isize);
        options &= !((1 as libc::c_int) << 28 as libc::c_int);
        options |= (1 as libc::c_int) << 27 as libc::c_int;
    }
    add_exclude(ex, pattern, options);
}
unsafe extern "C" fn hg_initfn(mut data: *mut libc::c_void) -> *mut libc::c_void {
    static mut hg_options: libc::c_int = 0;
    let mut hgopt: *mut libc::c_int = (if !data.is_null() {
        data
    } else {
        &mut hg_options as *mut libc::c_int as *mut libc::c_void
    }) as *mut libc::c_int;
    *hgopt = (1 as libc::c_int) << 27 as libc::c_int;
    return hgopt as *mut libc::c_void;
}
unsafe extern "C" fn hg_addfn(
    mut ex: *mut exclude,
    mut pattern: *const libc::c_char,
    mut options: libc::c_int,
    mut data: *mut libc::c_void,
) {
    let mut hgopt: *mut libc::c_int = data as *mut libc::c_int;
    let mut len: size_t = 0;
    while *(*__ctype_b_loc()).offset(*pattern as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
    {
        pattern = pattern.offset(1);
        pattern;
    }
    if *pattern as libc::c_int == 0 as libc::c_int
        || *pattern as libc::c_int == '#' as i32
    {
        return;
    }
    if strncmp(
        pattern,
        b"syntax:\0" as *const u8 as *const libc::c_char,
        7 as libc::c_int as libc::c_ulong,
    ) == 0 as libc::c_int
    {
        pattern = pattern.offset(7 as libc::c_int as isize);
        while *(*__ctype_b_loc()).offset(*pattern as libc::c_int as isize) as libc::c_int
            & _ISspace as libc::c_int as libc::c_ushort as libc::c_int != 0
        {
            pattern = pattern.offset(1);
            pattern;
        }
        if strcmp(pattern, b"regexp\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            *hgopt = (1 as libc::c_int) << 27 as libc::c_int;
        } else if strcmp(pattern, b"glob\0" as *const u8 as *const libc::c_char)
            == 0 as libc::c_int
        {
            *hgopt = (1 as libc::c_int) << 28 as libc::c_int;
        }
        return;
    }
    len = strlen(pattern);
    if *pattern.offset(len.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
        as libc::c_int == '/' as i32
    {
        let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
        len = len.wrapping_sub(1);
        len;
        p = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        memcpy(p as *mut libc::c_void, pattern as *const libc::c_void, len);
        *p.offset(len as isize) = 0 as libc::c_int as libc::c_char;
        pattern = p;
        exclude_add_pattern_buffer(ex, p);
        options
            |= (1 as libc::c_int) << 3 as libc::c_int
                | (1 as libc::c_int) << 26 as libc::c_int;
    }
    add_exclude(
        ex,
        pattern,
        (if *hgopt == (1 as libc::c_int) << 27 as libc::c_int {
            options & !((1 as libc::c_int) << 28 as libc::c_int)
        } else {
            options & !((1 as libc::c_int) << 27 as libc::c_int)
        }) | *hgopt,
    );
}
static mut vcs_ignore_files: [vcs_ignore_file; 5] = unsafe {
    [
        {
            let mut init = vcs_ignore_file {
                filename: b".cvsignore\0" as *const u8 as *const libc::c_char,
                flags: 0x2 as libc::c_int,
                addfn: Some(
                    cvs_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const libc::c_char,
                            libc::c_int,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: b".gitignore\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int,
                addfn: Some(
                    git_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const libc::c_char,
                            libc::c_int,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: b".bzrignore\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int,
                addfn: Some(
                    bzr_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const libc::c_char,
                            libc::c_int,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: b".hgignore\0" as *const u8 as *const libc::c_char,
                flags: 0 as libc::c_int,
                addfn: Some(
                    hg_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const libc::c_char,
                            libc::c_int,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: Some(
                    hg_initfn
                        as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void,
                ),
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
        {
            let mut init = vcs_ignore_file {
                filename: 0 as *const libc::c_char,
                flags: 0 as libc::c_int,
                addfn: Some(
                    git_addfn
                        as unsafe extern "C" fn(
                            *mut exclude,
                            *const libc::c_char,
                            libc::c_int,
                            *mut libc::c_void,
                        ) -> (),
                ),
                initfn: None,
                data: 0 as *const libc::c_void as *mut libc::c_void,
            };
            init
        },
    ]
};
unsafe extern "C" fn get_vcs_ignore_file(
    mut name: *const libc::c_char,
) -> *mut vcs_ignore_file {
    let mut p: *mut vcs_ignore_file = 0 as *mut vcs_ignore_file;
    p = vcs_ignore_files.as_mut_ptr();
    while !((*p).filename).is_null() {
        if strcmp((*p).filename, name) == 0 as libc::c_int {
            break;
        }
        p = p.offset(1);
        p;
    }
    return p;
}
pub unsafe extern "C" fn exclude_vcs_ignores() {
    let mut p: *mut vcs_ignore_file = 0 as *mut vcs_ignore_file;
    p = vcs_ignore_files.as_mut_ptr();
    while !((*p).filename).is_null() {
        excfile_add((*p).filename, 0 as libc::c_int);
        p = p.offset(1);
        p;
    }
}
