use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type __dirstream;
    pub type exclist;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn rpl_free(ptr: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    fn call_arg_warn(call: *const libc::c_char, name: *const libc::c_char);
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    static mut selinux_context_option: libc::c_int;
    static mut acls_option: libc::c_int;
    static mut xattrs_option: libc::c_int;
    static mut verbose_option: libc::c_int;
    static mut stdlis: *mut FILE;
    static mut chdir_fd: libc::c_int;
    fn xheader_xattr_add(
        st: *mut tar_stat_info,
        key: *const libc::c_char,
        val: *const libc::c_char,
        len: size_t,
    );
    static mut warning_option: libc::c_int;
    fn setxattrat(
        dir_fd: libc::c_int,
        path: *const libc::c_char,
        name: *const libc::c_char,
        value: *const libc::c_void,
        size: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn lsetxattrat(
        dir_fd: libc::c_int,
        path: *const libc::c_char,
        name: *const libc::c_char,
        value: *const libc::c_void,
        size: size_t,
        flags: libc::c_int,
    ) -> libc::c_int;
    fn fgetxattr(
        __fd: libc::c_int,
        __name: *const libc::c_char,
        __value: *mut libc::c_void,
        __size: size_t,
    ) -> ssize_t;
    fn llistxattrat(
        dir_fd: libc::c_int,
        path: *const libc::c_char,
        list: *mut libc::c_char,
        size: size_t,
    ) -> ssize_t;
    fn lgetxattrat(
        dir_fd: libc::c_int,
        path: *const libc::c_char,
        name: *const libc::c_char,
        value: *mut libc::c_void,
        size: size_t,
    ) -> ssize_t;
    fn flistxattr(
        __fd: libc::c_int,
        __list: *mut libc::c_char,
        __size: size_t,
    ) -> ssize_t;
    fn lgetfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *mut *mut libc::c_char,
    ) -> libc::c_int;
    fn rpl_fgetfilecon(fd: libc::c_int, con: *mut *mut libc::c_char) -> libc::c_int;
    fn lsetfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *const libc::c_char,
    ) -> libc::c_int;
    fn setfileconat(
        dir_fd: libc::c_int,
        file: *const libc::c_char,
        con: *const libc::c_char,
    ) -> libc::c_int;
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
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
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
    pub temp: C2RustUnnamed_1,
    pub alignment_mask: size_t,
    pub chunkfun: C2RustUnnamed_0,
    pub freefun: C2RustUnnamed,
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
pub union C2RustUnnamed {
    pub plain: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub plain: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
    pub extra: Option::<
        unsafe extern "C" fn(*mut libc::c_void, size_t) -> *mut libc::c_void,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_1 {
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
pub const DEFAULT_MXFAST: C2RustUnnamed_2 = 128;
pub type C2RustUnnamed_2 = libc::c_uint;
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
pub struct xattrs_mask_map {
    pub masks: *mut *const libc::c_char,
    pub size: size_t,
    pub used: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub incl: xattrs_mask_map,
    pub excl: xattrs_mask_map,
}
#[inline]
unsafe extern "C" fn x2nrealloc(
    mut p: *mut libc::c_void,
    mut pn: *mut size_t,
    mut s: size_t,
) -> *mut libc::c_void {
    let mut n: size_t = *pn;
    if p.is_null() {
        if n == 0 {
            n = (DEFAULT_MXFAST as libc::c_int as libc::c_ulong).wrapping_div(s);
            n = (n as libc::c_ulong)
                .wrapping_add((n == 0) as libc::c_int as libc::c_ulong) as size_t
                as size_t;
        }
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            (18446744073709551615 as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
        })
            .wrapping_div(s) < n
        {
            xalloc_die();
        }
    } else {
        if (if (9223372036854775807 as libc::c_long as libc::c_ulong)
            < 18446744073709551615 as libc::c_ulong
        {
            9223372036854775807 as libc::c_long as libc::c_ulong
        } else {
            18446744073709551615 as libc::c_ulong
        })
            .wrapping_div(3 as libc::c_int as libc::c_ulong)
            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
            .wrapping_div(s) <= n
        {
            xalloc_die();
        }
        n = (n as libc::c_ulong)
            .wrapping_add(
                n
                    .wrapping_div(2 as libc::c_int as libc::c_ulong)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            ) as size_t as size_t;
    }
    *pn = n;
    return xrealloc(p, n.wrapping_mul(s));
}
static mut xattrs_setup: C2RustUnnamed_3 = C2RustUnnamed_3 {
    incl: xattrs_mask_map {
        masks: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        size: 0,
        used: 0,
    },
    excl: xattrs_mask_map {
        masks: 0 as *const *const libc::c_char as *mut *const libc::c_char,
        size: 0,
        used: 0,
    },
};
unsafe extern "C" fn acls_one_line(
    mut prefix: *const libc::c_char,
    mut delim: libc::c_char,
    mut aclstring: *const libc::c_char,
    mut len: size_t,
) {
    let mut stk: obstack = obstack {
        chunk_size: 0,
        chunk: 0 as *mut _obstack_chunk,
        object_base: 0 as *mut libc::c_char,
        next_free: 0 as *mut libc::c_char,
        chunk_limit: 0 as *mut libc::c_char,
        temp: C2RustUnnamed_1 { i: 0 },
        alignment_mask: 0,
        chunkfun: C2RustUnnamed_0 { plain: None },
        freefun: C2RustUnnamed { plain: None },
        extra_arg: 0 as *mut libc::c_void,
        use_extra_arg_maybe_empty_object_alloc_failed: [0; 1],
        c2rust_padding: [0; 7],
    };
    let mut pref_len: libc::c_int = strlen(prefix) as libc::c_int;
    let mut oldstring: *const libc::c_char = aclstring;
    let mut pos: libc::c_int = 0 as libc::c_int;
    if aclstring.is_null() || len == 0 {
        return;
    }
    _obstack_begin(
        &mut stk,
        0 as libc::c_int as size_t,
        0 as libc::c_int as size_t,
        Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
        Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
    );
    while pos as libc::c_ulong <= len {
        let mut move_0: libc::c_int = strcspn(
            aclstring,
            b",\n\0" as *const u8 as *const libc::c_char,
        ) as libc::c_int;
        if move_0 == 0 {
            break;
        }
        if oldstring != aclstring {
            let mut __o: *mut obstack = &mut stk;
            if ({
                let mut __o1: *const obstack = __o;
                ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                    as size_t
            }) < 1 as libc::c_int as libc::c_ulong
            {
                _obstack_newchunk(__o, 1 as libc::c_int as size_t);
            }
            let fresh0 = (*__o).next_free;
            (*__o).next_free = ((*__o).next_free).offset(1);
            *fresh0 = delim;
        }
        let mut __o_0: *mut obstack = &mut stk;
        let mut __len: size_t = pref_len as size_t;
        if ({
            let mut __o1: *const obstack = __o_0;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len
        {
            _obstack_newchunk(__o_0, __len);
        }
        memcpy(
            (*__o_0).next_free as *mut libc::c_void,
            prefix as *const libc::c_void,
            __len,
        );
        (*__o_0).next_free = ((*__o_0).next_free).offset(__len as isize);
        let mut __o_1: *mut obstack = &mut stk;
        let mut __len_0: size_t = move_0 as size_t;
        if ({
            let mut __o1: *const obstack = __o_1;
            ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long
                as size_t
        }) < __len_0
        {
            _obstack_newchunk(__o_1, __len_0);
        }
        memcpy(
            (*__o_1).next_free as *mut libc::c_void,
            aclstring as *const libc::c_void,
            __len_0,
        );
        (*__o_1).next_free = ((*__o_1).next_free).offset(__len_0 as isize);
        pos += move_0 + 1 as libc::c_int;
        aclstring = aclstring.offset((move_0 + 1 as libc::c_int) as isize);
    }
    let mut __o_2: *mut obstack = &mut stk;
    if ({
        let mut __o1: *const obstack = __o_2;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o_2, 1 as libc::c_int as size_t);
    }
    let fresh1 = (*__o_2).next_free;
    (*__o_2).next_free = ((*__o_2).next_free).offset(1);
    *fresh1 = '\0' as i32 as libc::c_char;
    fprintf(
        stdlis,
        b"%s\0" as *const u8 as *const libc::c_char,
        ({
            let mut __o1: *mut obstack = &mut stk as *mut obstack;
            let mut __value: *mut libc::c_void = (*__o1).object_base
                as *mut libc::c_void;
            if (*__o1).next_free == __value as *mut libc::c_char {
                (*__o1).set_maybe_empty_object(1 as libc::c_int as libc::c_uint);
            }
            (*__o1)
                .next_free = (if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
            {
                (*__o1).object_base
            } else {
                0 as *mut libc::c_char
            })
                .offset(
                    ((((*__o1).next_free)
                        .offset_from(
                            if (::std::mem::size_of::<ptrdiff_t>() as libc::c_ulong)
                                < ::std::mem::size_of::<*mut libc::c_void>()
                                    as libc::c_ulong
                            {
                                (*__o1).object_base
                            } else {
                                0 as *mut libc::c_char
                            },
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_add((*__o1).alignment_mask) & !(*__o1).alignment_mask)
                        as isize,
                );
            if ((*__o1).next_free).offset_from((*__o1).chunk as *mut libc::c_char)
                as libc::c_long as size_t
                > ((*__o1).chunk_limit).offset_from((*__o1).chunk as *mut libc::c_char)
                    as libc::c_long as size_t
            {
                (*__o1).next_free = (*__o1).chunk_limit;
            }
            (*__o1).object_base = (*__o1).next_free;
            __value
        }) as *mut libc::c_char,
    );
    let mut __o_3: *mut obstack = &mut stk;
    let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
    if __obj > (*__o_3).chunk as *mut libc::c_void
        && __obj < (*__o_3).chunk_limit as *mut libc::c_void
    {
        (*__o_3).object_base = __obj as *mut libc::c_char;
        (*__o_3).next_free = (*__o_3).object_base;
    } else {
        _obstack_free(__o_3, __obj);
    };
}
pub unsafe extern "C" fn xattrs_acls_get(
    mut parentfd: libc::c_int,
    mut file_name: *const libc::c_char,
    mut st: *mut tar_stat_info,
    mut fd: libc::c_int,
    mut xisfile: libc::c_int,
) {
    if acls_option > 0 as libc::c_int {
        static mut done: libc::c_int = 0 as libc::c_int;
        if done == 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"POSIX ACL support is not available\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        done = 1 as libc::c_int;
    }
}
pub unsafe extern "C" fn xattrs_acls_set(
    mut st: *const tar_stat_info,
    mut file_name: *const libc::c_char,
    mut typeflag: libc::c_char,
) {
    if acls_option > 0 as libc::c_int && typeflag as libc::c_int != '2' as i32 {
        static mut done: libc::c_int = 0 as libc::c_int;
        if done == 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"POSIX ACL support is not available\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
        }
        done = 1 as libc::c_int;
    }
}
unsafe extern "C" fn mask_map_realloc(mut map: *mut xattrs_mask_map) {
    if (*map).used == (*map).size {
        if (*map).size == 0 as libc::c_int as libc::c_ulong {
            (*map).size = 4 as libc::c_int as size_t;
        }
        (*map)
            .masks = x2nrealloc(
            (*map).masks as *mut libc::c_void,
            &mut (*map).size,
            ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
        ) as *mut *const libc::c_char;
    }
}
pub unsafe extern "C" fn xattrs_mask_add(mut mask: *const libc::c_char, mut incl: bool) {
    let mut mask_map: *mut xattrs_mask_map = if incl as libc::c_int != 0 {
        &mut xattrs_setup.incl
    } else {
        &mut xattrs_setup.excl
    };
    mask_map_realloc(mask_map);
    let fresh2 = (*mask_map).used;
    (*mask_map).used = ((*mask_map).used).wrapping_add(1);
    let ref mut fresh3 = *((*mask_map).masks).offset(fresh2 as isize);
    *fresh3 = mask;
}
unsafe extern "C" fn clear_mask_map(mut mask_map: *mut xattrs_mask_map) {
    if (*mask_map).size != 0 {
        rpl_free((*mask_map).masks as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn xattrs_clear_setup() {
    clear_mask_map(&mut xattrs_setup.incl);
    clear_mask_map(&mut xattrs_setup.excl);
}
pub unsafe extern "C" fn xattrs_xattrs_get(
    mut parentfd: libc::c_int,
    mut file_name: *const libc::c_char,
    mut st: *mut tar_stat_info,
    mut fd: libc::c_int,
) {
    if xattrs_option > 0 as libc::c_int {
        static mut xsz: size_t = 1024 as libc::c_int as size_t;
        static mut xatrs: *mut libc::c_char = 0 as *const libc::c_char
            as *mut libc::c_char;
        let mut xret: ssize_t = -(1 as libc::c_int) as ssize_t;
        if xatrs.is_null() {
            xatrs = x2nrealloc(
                xatrs as *mut libc::c_void,
                &mut xsz,
                1 as libc::c_int as size_t,
            ) as *mut libc::c_char;
        }
        while (if fd == 0 as libc::c_int {
            xret = llistxattrat(parentfd, file_name, xatrs, xsz);
            (xret == -(1 as libc::c_int) as libc::c_long) as libc::c_int
        } else {
            xret = flistxattr(fd, xatrs, xsz);
            (xret == -(1 as libc::c_int) as libc::c_long) as libc::c_int
        }) != 0 && *__errno_location() == 34 as libc::c_int
        {
            xatrs = x2nrealloc(
                xatrs as *mut libc::c_void,
                &mut xsz,
                1 as libc::c_int as size_t,
            ) as *mut libc::c_char;
        }
        if xret == -(1 as libc::c_int) as libc::c_long {
            call_arg_warn(
                if fd == 0 as libc::c_int {
                    b"llistxattrat\0" as *const u8 as *const libc::c_char
                } else {
                    b"flistxattr\0" as *const u8 as *const libc::c_char
                },
                file_name,
            );
        } else {
            let mut attr: *const libc::c_char = xatrs;
            static mut asz: size_t = 1024 as libc::c_int as size_t;
            static mut val: *mut libc::c_char = 0 as *const libc::c_char
                as *mut libc::c_char;
            if val.is_null() {
                val = x2nrealloc(
                    val as *mut libc::c_void,
                    &mut asz,
                    1 as libc::c_int as size_t,
                ) as *mut libc::c_char;
            }
            while xret > 0 as libc::c_int as libc::c_long {
                let mut len: size_t = strlen(attr);
                let mut aret: ssize_t = 0 as libc::c_int as ssize_t;
                while (if fd == 0 as libc::c_int {
                    aret = lgetxattrat(
                        parentfd,
                        file_name,
                        attr,
                        val as *mut libc::c_void,
                        asz,
                    );
                    (aret == -(1 as libc::c_int) as libc::c_long) as libc::c_int
                } else {
                    aret = fgetxattr(fd, attr, val as *mut libc::c_void, asz);
                    (aret == -(1 as libc::c_int) as libc::c_long) as libc::c_int
                }) != 0 && *__errno_location() == 34 as libc::c_int
                {
                    val = x2nrealloc(
                        val as *mut libc::c_void,
                        &mut asz,
                        1 as libc::c_int as size_t,
                    ) as *mut libc::c_char;
                }
                if aret != -(1 as libc::c_int) as libc::c_long {
                    if !xattrs_masked_out(attr, 1 as libc::c_int != 0) {
                        xheader_xattr_add(st, attr, val, aret as size_t);
                    }
                } else if *__errno_location() != 61 as libc::c_int {
                    call_arg_warn(
                        if fd == 0 as libc::c_int {
                            b"lgetxattrat\0" as *const u8 as *const libc::c_char
                        } else {
                            b"fgetxattr\0" as *const u8 as *const libc::c_char
                        },
                        file_name,
                    );
                }
                attr = attr
                    .offset(
                        len.wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
                    );
                xret = (xret as libc::c_ulong)
                    .wrapping_sub(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
                    as ssize_t as ssize_t;
            }
        }
    }
}
unsafe extern "C" fn xattrs__fd_set(
    mut st: *const tar_stat_info,
    mut file_name: *const libc::c_char,
    mut typeflag: libc::c_char,
    mut attr: *const libc::c_char,
    mut ptr: *const libc::c_char,
    mut len: size_t,
) {
    if !ptr.is_null() {
        let mut sysname: *const libc::c_char = b"setxattrat\0" as *const u8
            as *const libc::c_char;
        let mut ret: libc::c_int = -(1 as libc::c_int);
        if typeflag as libc::c_int != '2' as i32 {
            ret = setxattrat(
                chdir_fd,
                file_name,
                attr,
                ptr as *const libc::c_void,
                len,
                0 as libc::c_int,
            );
        } else {
            sysname = b"lsetxattr\0" as *const u8 as *const libc::c_char;
            ret = lsetxattrat(
                chdir_fd,
                file_name,
                attr,
                ptr as *const libc::c_void,
                len,
                0 as libc::c_int,
            );
        }
        if ret == -(1 as libc::c_int) {
            if warning_option & 0x200000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Cannot set '%s' extended attribute for file '%s'\0"
                            as *const u8 as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    sysname,
                    attr,
                    file_name,
                );
            }
        }
    }
}
pub unsafe extern "C" fn xattrs_selinux_get(
    mut parentfd: libc::c_int,
    mut file_name: *const libc::c_char,
    mut st: *mut tar_stat_info,
    mut fd: libc::c_int,
) {
    if selinux_context_option > 0 as libc::c_int {
        let mut result: libc::c_int = if fd != 0 {
            rpl_fgetfilecon(fd, &mut (*st).cntx_name)
        } else {
            lgetfileconat(parentfd, file_name, &mut (*st).cntx_name)
        };
        if result == -(1 as libc::c_int) && *__errno_location() != 61 as libc::c_int
            && *__errno_location() != 95 as libc::c_int
        {
            call_arg_warn(
                if fd != 0 {
                    b"fgetfilecon\0" as *const u8 as *const libc::c_char
                } else {
                    b"lgetfileconat\0" as *const u8 as *const libc::c_char
                },
                file_name,
            );
        }
    }
}
pub unsafe extern "C" fn xattrs_selinux_set(
    mut st: *const tar_stat_info,
    mut file_name: *const libc::c_char,
    mut typeflag: libc::c_char,
) {
    if selinux_context_option > 0 as libc::c_int {
        let mut sysname: *const libc::c_char = b"setfilecon\0" as *const u8
            as *const libc::c_char;
        let mut ret: libc::c_int = 0;
        if ((*st).cntx_name).is_null() {
            return;
        }
        if typeflag as libc::c_int != '2' as i32 {
            ret = setfileconat(chdir_fd, file_name, (*st).cntx_name);
            sysname = b"setfileconat\0" as *const u8 as *const libc::c_char;
        } else {
            ret = lsetfileconat(chdir_fd, file_name, (*st).cntx_name);
            sysname = b"lsetfileconat\0" as *const u8 as *const libc::c_char;
        }
        if ret == -(1 as libc::c_int) {
            if warning_option & 0x200000 as libc::c_int != 0 {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    *__errno_location(),
                    dcgettext(
                        0 as *const libc::c_char,
                        b"%s: Cannot set SELinux context for file '%s'\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    sysname,
                    file_name,
                );
            }
        }
    }
}
unsafe extern "C" fn xattrs_matches_mask(
    mut kw: *const libc::c_char,
    mut mm: *mut xattrs_mask_map,
) -> bool {
    let mut i: libc::c_int = 0;
    if (*mm).size == 0 {
        return 0 as libc::c_int != 0;
    }
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < (*mm).used {
        if fnmatch(*((*mm).masks).offset(i as isize), kw, 0 as libc::c_int)
            == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn xattrs_kw_included(
    mut kw: *const libc::c_char,
    mut archiving: bool,
) -> bool {
    if xattrs_setup.incl.size != 0 {
        return xattrs_matches_mask(kw, &mut xattrs_setup.incl)
    } else if archiving {
        return 1 as libc::c_int != 0
    } else {
        return strncmp(
            kw,
            b"user.\0" as *const u8 as *const libc::c_char,
            (::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int
    };
}
unsafe extern "C" fn xattrs_kw_excluded(
    mut kw: *const libc::c_char,
    mut archiving: bool,
) -> bool {
    return if xattrs_setup.excl.size != 0 {
        xattrs_matches_mask(kw, &mut xattrs_setup.excl) as libc::c_int
    } else {
        0 as libc::c_int
    } != 0;
}
unsafe extern "C" fn xattrs_masked_out(
    mut kw: *const libc::c_char,
    mut archiving: bool,
) -> bool {
    return if xattrs_kw_included(kw, archiving) as libc::c_int != 0 {
        xattrs_kw_excluded(kw, archiving) as libc::c_int
    } else {
        1 as libc::c_int
    } != 0;
}
pub unsafe extern "C" fn xattrs_xattrs_set(
    mut st: *const tar_stat_info,
    mut file_name: *const libc::c_char,
    mut typeflag: libc::c_char,
    mut later_run: libc::c_int,
) {
    if xattrs_option > 0 as libc::c_int {
        let mut scan: size_t = 0 as libc::c_int as size_t;
        if (*st).xattr_map_size == 0 {
            return;
        }
        while scan < (*st).xattr_map_size {
            let mut keyword: *mut libc::c_char = (*((*st).xattr_map)
                .offset(scan as isize))
                .xkey;
            keyword = keyword
                .offset(
                    strlen(b"SCHILY.xattr.\0" as *const u8 as *const libc::c_char)
                        as isize,
                );
            if !(typeflag as libc::c_int == '0' as i32
                && later_run
                    == (strcmp(
                        keyword,
                        b"security.capability\0" as *const u8 as *const libc::c_char,
                    ) != 0) as libc::c_int)
            {
                if !xattrs_masked_out(keyword, 0 as libc::c_int != 0) {
                    xattrs__fd_set(
                        st,
                        file_name,
                        typeflag,
                        keyword,
                        (*((*st).xattr_map).offset(scan as isize)).xval_ptr,
                        (*((*st).xattr_map).offset(scan as isize)).xval_len,
                    );
                }
            }
            scan = scan.wrapping_add(1);
            scan;
        }
    }
}
pub unsafe extern "C" fn xattrs_print_char(
    mut st: *const tar_stat_info,
    mut output: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if verbose_option < 2 as libc::c_int {
        *output = 0 as libc::c_int as libc::c_char;
        return;
    }
    if xattrs_option > 0 as libc::c_int || selinux_context_option > 0 as libc::c_int
        || acls_option > 0 as libc::c_int
    {
        *output = ' ' as i32 as libc::c_char;
        *output.offset(1 as libc::c_int as isize) = 0 as libc::c_int as libc::c_char;
    }
    if xattrs_option > 0 as libc::c_int && (*st).xattr_map_size != 0 {
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*st).xattr_map_size {
            let mut keyword: *mut libc::c_char = ((*((*st).xattr_map).offset(i as isize))
                .xkey)
                .offset(
                    strlen(b"SCHILY.xattr.\0" as *const u8 as *const libc::c_char)
                        as isize,
                );
            if !xattrs_masked_out(keyword, 0 as libc::c_int != 0) {
                *output = '*' as i32 as libc::c_char;
                break;
            } else {
                i += 1;
                i;
            }
        }
    }
    if selinux_context_option > 0 as libc::c_int && !((*st).cntx_name).is_null() {
        *output = '.' as i32 as libc::c_char;
    }
    if acls_option > 0 as libc::c_int && ((*st).acls_a_len != 0 || (*st).acls_d_len != 0)
    {
        *output = '+' as i32 as libc::c_char;
    }
}
pub unsafe extern "C" fn xattrs_print(mut st: *const tar_stat_info) {
    if verbose_option < 3 as libc::c_int {
        return;
    }
    if selinux_context_option > 0 as libc::c_int && !((*st).cntx_name).is_null() {
        fprintf(
            stdlis,
            b"  s: %s\n\0" as *const u8 as *const libc::c_char,
            (*st).cntx_name,
        );
    }
    if acls_option > 0 as libc::c_int && ((*st).acls_a_len != 0 || (*st).acls_d_len != 0)
    {
        fprintf(stdlis, b"  a: \0" as *const u8 as *const libc::c_char);
        acls_one_line(
            b"\0" as *const u8 as *const libc::c_char,
            ',' as i32 as libc::c_char,
            (*st).acls_a_ptr,
            (*st).acls_a_len,
        );
        if (*st).acls_a_len != 0 && (*st).acls_d_len != 0 {
            fprintf(stdlis, b",\0" as *const u8 as *const libc::c_char);
        }
        acls_one_line(
            b"default:\0" as *const u8 as *const libc::c_char,
            ',' as i32 as libc::c_char,
            (*st).acls_d_ptr,
            (*st).acls_d_len,
        );
        fprintf(stdlis, b"\n\0" as *const u8 as *const libc::c_char);
    }
    if xattrs_option > 0 as libc::c_int && (*st).xattr_map_size != 0 {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while (i as libc::c_ulong) < (*st).xattr_map_size {
            let mut keyword: *mut libc::c_char = ((*((*st).xattr_map).offset(i as isize))
                .xkey)
                .offset(
                    strlen(b"SCHILY.xattr.\0" as *const u8 as *const libc::c_char)
                        as isize,
                );
            if !xattrs_masked_out(keyword, 0 as libc::c_int != 0) {
                fprintf(
                    stdlis,
                    b"  x: %lu %s\n\0" as *const u8 as *const libc::c_char,
                    (*((*st).xattr_map).offset(i as isize)).xval_len,
                    keyword,
                );
            }
            i += 1;
            i;
        }
    }
}
