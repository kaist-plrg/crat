use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type __dirstream;
    pub type exclist;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memmove(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strtok(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    fn getpid() -> __pid_t;
    fn getenv(__name: *const libc::c_char) -> *mut libc::c_char;
    fn safer_name_suffix(
        file_name: *const libc::c_char,
        link_target: bool,
        absolute_names: bool,
    ) -> *mut libc::c_char;
    fn fatal_exit() -> !;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __strtol_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_long;
    fn __strtoul_internal(
        __nptr: *const libc::c_char,
        __endptr: *mut *mut libc::c_char,
        __base: libc::c_int,
        __group: libc::c_int,
    ) -> libc::c_ulong;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
    fn dir_name(file: *const libc::c_char) -> *mut libc::c_char;
    fn strip_trailing_slashes(file: *mut libc::c_char) -> bool;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xcalloc(n: size_t, s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn x2realloc(p: *mut libc::c_void, pn: *mut size_t) -> *mut libc::c_void;
    fn xmemdup(p: *const libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn imaxtostr(_: intmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    fn umaxtostr(_: uintmax_t, _: *mut libc::c_char) -> *mut libc::c_char;
    static mut error_hook: Option::<unsafe extern "C" fn() -> ()>;
    static mut exit_status: libc::c_int;
    fn fnmatch(
        __pattern: *const libc::c_char,
        __name: *const libc::c_char,
        __flags: libc::c_int,
    ) -> libc::c_int;
    static mut absolute_names_option: bool;
    static mut posixly_correct: bool;
    static mut start_time: timespec;
    static mut volume_label: *mut libc::c_char;
    static mut continued_file_name: *mut libc::c_char;
    static mut continued_file_size: uintmax_t;
    static mut continued_file_offset: uintmax_t;
    fn find_next_block() -> *mut block;
    fn set_next_block_after(block: *mut block);
    fn simple_finish_header(header: *mut block);
    fn start_private_header(
        name: *const libc::c_char,
        size: size_t,
        t: time_t,
    ) -> *mut block;
    fn dumpdir_size(p: *const libc::c_char) -> size_t;
    fn assign_string(dest: *mut *mut libc::c_char, src: *const libc::c_char);
    fn sysinttostr(
        _: uintmax_t,
        _: intmax_t,
        _: uintmax_t,
        buf: *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn strtosysint(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: intmax_t,
        _: uintmax_t,
    ) -> intmax_t;
    fn code_timespec(ts: timespec, sbuf: *mut libc::c_char) -> *const libc::c_char;
    fn decode_timespec(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: bool,
    ) -> timespec;
    fn usage(_: libc::c_int);
    fn _obstack_newchunk(_: *mut obstack, _: size_t);
    fn _obstack_free(_: *mut obstack, _: *mut libc::c_void);
    fn _obstack_begin(
        _: *mut obstack,
        _: size_t,
        _: size_t,
        _: Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>,
        _: Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>,
    ) -> libc::c_int;
    fn utf8_convert(
        to_utf: bool,
        input: *const libc::c_char,
        output: *mut *mut libc::c_char,
    ) -> bool;
    fn quote(arg: *const libc::c_char) -> *const libc::c_char;
    static mut warning_option: libc::c_int;
}
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type gid_t = __gid_t;
pub type uid_t = __uid_t;
pub type off_t = __off_t;
pub type time_t = __time_t;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
pub type DIR = __dirstream;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct posix_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 155],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse {
    pub offset: [libc::c_char; 12],
    pub numbytes: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sparse_header {
    pub sp: [sparse; 21],
    pub isextended: libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct oldgnu_header {
    pub unused_pad1: [libc::c_char; 345],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub longnames: [libc::c_char; 4],
    pub unused_pad2: libc::c_char,
    pub sp: [sparse; 4],
    pub isextended: libc::c_char,
    pub realsize: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_header {
    pub name: [libc::c_char; 100],
    pub mode: [libc::c_char; 8],
    pub uid: [libc::c_char; 8],
    pub gid: [libc::c_char; 8],
    pub size: [libc::c_char; 12],
    pub mtime: [libc::c_char; 12],
    pub chksum: [libc::c_char; 8],
    pub typeflag: libc::c_char,
    pub linkname: [libc::c_char; 100],
    pub magic: [libc::c_char; 6],
    pub version: [libc::c_char; 2],
    pub uname: [libc::c_char; 32],
    pub gname: [libc::c_char; 32],
    pub devmajor: [libc::c_char; 8],
    pub devminor: [libc::c_char; 8],
    pub prefix: [libc::c_char; 131],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_in_header {
    pub fill: [libc::c_char; 345],
    pub prefix: [libc::c_char; 1],
    pub fill2: libc::c_char,
    pub fill3: [libc::c_char; 8],
    pub isextended: libc::c_char,
    pub sp: [sparse; 4],
    pub realsize: [libc::c_char; 12],
    pub offset: [libc::c_char; 12],
    pub atime: [libc::c_char; 12],
    pub ctime: [libc::c_char; 12],
    pub mfill: [libc::c_char; 8],
    pub xmagic: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct star_ext_header {
    pub sp: [sparse; 21],
    pub isextended: libc::c_char,
}
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
pub union block {
    pub buffer: [libc::c_char; 512],
    pub header: posix_header,
    pub star_header: star_header,
    pub oldgnu_header: oldgnu_header,
    pub sparse_header: sparse_header,
    pub star_in_header: star_in_header,
    pub star_ext_header: star_ext_header,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct keyword_list {
    pub next: *mut keyword_list,
    pub pattern: *mut libc::c_char,
    pub value: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xhdr_tab {
    pub keyword: *const libc::c_char,
    pub coder: Option::<
        unsafe extern "C" fn(
            *const tar_stat_info,
            *const libc::c_char,
            *mut xheader,
            *const libc::c_void,
        ) -> (),
    >,
    pub decoder: Option::<
        unsafe extern "C" fn(
            *mut tar_stat_info,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    pub flags: libc::c_int,
    pub prefix: bool,
}
pub const pax_global_header: C2RustUnnamed_3 = 1;
pub const pax_file_header: C2RustUnnamed_3 = 0;
pub type C2RustUnnamed_3 = libc::c_uint;
#[inline]
unsafe extern "C" fn strtoimax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> intmax_t {
    return __strtol_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn strtoumax(
    mut nptr: *const libc::c_char,
    mut endptr: *mut *mut libc::c_char,
    mut base: libc::c_int,
) -> uintmax_t {
    return __strtoul_internal(nptr, endptr, base, 0 as libc::c_int);
}
#[inline]
unsafe extern "C" fn valid_timespec(mut t: timespec) -> bool {
    return 0 as libc::c_int as libc::c_long <= t.tv_nsec;
}
static mut global_header_count: size_t = 0;
unsafe extern "C" fn x_obstack_grow(
    mut xhdr: *mut xheader,
    mut ptr: *const libc::c_char,
    mut length: size_t,
) {
    let mut __o: *mut obstack = (*xhdr).stk;
    let mut __len: size_t = length;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    memcpy((*__o).next_free as *mut libc::c_void, ptr as *const libc::c_void, __len);
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    (*xhdr)
        .size = ((*xhdr).size as libc::c_ulong).wrapping_add(length) as size_t as size_t;
}
unsafe extern "C" fn x_obstack_1grow(mut xhdr: *mut xheader, mut c: libc::c_char) {
    let mut __o: *mut obstack = (*xhdr).stk;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < 1 as libc::c_int as libc::c_ulong
    {
        _obstack_newchunk(__o, 1 as libc::c_int as size_t);
    }
    let fresh0 = (*__o).next_free;
    (*__o).next_free = ((*__o).next_free).offset(1);
    *fresh0 = c;
    (*xhdr).size = ((*xhdr).size).wrapping_add(1);
    (*xhdr).size;
}
unsafe extern "C" fn x_obstack_blank(mut xhdr: *mut xheader, mut length: size_t) {
    let mut __o: *mut obstack = (*xhdr).stk;
    let mut __len: size_t = length;
    if ({
        let mut __o1: *const obstack = __o;
        ((*__o1).chunk_limit).offset_from((*__o1).next_free) as libc::c_long as size_t
    }) < __len
    {
        _obstack_newchunk(__o, __len);
    }
    (*__o).next_free = ((*__o).next_free).offset(__len as isize);
    (*xhdr)
        .size = ((*xhdr).size as libc::c_ulong).wrapping_add(length) as size_t as size_t;
}
static mut keyword_pattern_list: *mut keyword_list = 0 as *const keyword_list
    as *mut keyword_list;
static mut keyword_global_override_list: *mut keyword_list = 0 as *const keyword_list
    as *mut keyword_list;
static mut keyword_override_list: *mut keyword_list = 0 as *const keyword_list
    as *mut keyword_list;
static mut global_header_override_list: *mut keyword_list = 0 as *const keyword_list
    as *mut keyword_list;
static mut exthdr_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut exthdr_mtime_option: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut exthdr_mtime: time_t = 0;
static mut globexthdr_name: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut globexthdr_mtime_option: *mut libc::c_char = 0 as *const libc::c_char
    as *mut libc::c_char;
static mut globexthdr_mtime: time_t = 0;
pub unsafe extern "C" fn xheader_keyword_deleted_p(mut kw: *const libc::c_char) -> bool {
    let mut kp: *mut keyword_list = 0 as *mut keyword_list;
    kp = keyword_pattern_list;
    while !kp.is_null() {
        if fnmatch((*kp).pattern, kw, 0 as libc::c_int) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        kp = (*kp).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn xheader_keyword_override_p(
    mut keyword: *const libc::c_char,
) -> bool {
    let mut kp: *mut keyword_list = 0 as *mut keyword_list;
    kp = keyword_override_list;
    while !kp.is_null() {
        if strcmp((*kp).pattern, keyword) == 0 as libc::c_int {
            return 1 as libc::c_int != 0;
        }
        kp = (*kp).next;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn xheader_list_append(
    mut root: *mut *mut keyword_list,
    mut kw: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    let mut kp: *mut keyword_list = xmalloc(
        ::std::mem::size_of::<keyword_list>() as libc::c_ulong,
    ) as *mut keyword_list;
    (*kp).pattern = xstrdup(kw);
    (*kp).value = if !value.is_null() { xstrdup(value) } else { 0 as *mut libc::c_char };
    (*kp).next = *root;
    *root = kp;
}
unsafe extern "C" fn xheader_list_destroy(mut root: *mut *mut keyword_list) {
    if !root.is_null() {
        let mut kw: *mut keyword_list = *root;
        while !kw.is_null() {
            let mut next: *mut keyword_list = (*kw).next;
            rpl_free((*kw).pattern as *mut libc::c_void);
            rpl_free((*kw).value as *mut libc::c_void);
            rpl_free(kw as *mut libc::c_void);
            kw = next;
        }
        *root = 0 as *mut keyword_list;
    }
}
unsafe extern "C" fn xheader_set_single_keyword(mut kw: *mut libc::c_char) {
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"Keyword %s is unknown or not yet implemented\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        kw,
    );
    usage(2 as libc::c_int);
}
unsafe extern "C" fn assign_time_option(
    mut sval: *mut *mut libc::c_char,
    mut tval: *mut time_t,
    mut input: *const libc::c_char,
) {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: timespec = decode_timespec(input, &mut p, 0 as libc::c_int != 0);
    if !valid_timespec(t) || *p as libc::c_int != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Time stamp is out of allowed range\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit_status = 2 as libc::c_int;
    } else {
        *tval = t.tv_sec;
        assign_string(sval, input);
    };
}
unsafe extern "C" fn xheader_set_keyword_equal(
    mut kw: *mut libc::c_char,
    mut eq: *mut libc::c_char,
) {
    let mut global: bool = 1 as libc::c_int != 0;
    let mut p: *mut libc::c_char = eq;
    if eq == kw {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed pax option: %s\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            quote(kw),
        );
        usage(2 as libc::c_int);
    }
    if *eq.offset(-(1 as libc::c_int) as isize) as libc::c_int == ':' as i32 {
        p = p.offset(-1);
        p;
        global = 0 as libc::c_int != 0;
    }
    while p > kw
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(-1);
        p;
    }
    *p = 0 as libc::c_int as libc::c_char;
    p = eq.offset(1 as libc::c_int as isize);
    while *p as libc::c_int != 0
        && *(*__ctype_b_loc()).offset(*p as libc::c_uchar as libc::c_int as isize)
            as libc::c_int & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        p = p.offset(1);
        p;
    }
    if strcmp(kw, b"delete\0" as *const u8 as *const libc::c_char) == 0 as libc::c_int {
        if xheader_protected_pattern_p(p) {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Pattern %s cannot be used\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                quote(p),
            );
            usage(2 as libc::c_int);
        }
        xheader_list_append(&mut keyword_pattern_list, p, 0 as *const libc::c_char);
    } else if strcmp(kw, b"exthdr.name\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        assign_string(&mut exthdr_name, p);
    } else if strcmp(kw, b"globexthdr.name\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        assign_string(&mut globexthdr_name, p);
    } else if strcmp(kw, b"exthdr.mtime\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        assign_time_option(&mut exthdr_mtime_option, &mut exthdr_mtime, p);
    } else if strcmp(kw, b"globexthdr.mtime\0" as *const u8 as *const libc::c_char)
        == 0 as libc::c_int
    {
        assign_time_option(&mut globexthdr_mtime_option, &mut globexthdr_mtime, p);
    } else {
        if xheader_protected_keyword_p(kw) {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Keyword %s cannot be overridden\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                kw,
            );
            usage(2 as libc::c_int);
        }
        if global {
            xheader_list_append(&mut keyword_global_override_list, kw, p);
        } else {
            xheader_list_append(&mut keyword_override_list, kw, p);
        }
    };
}
pub unsafe extern "C" fn xheader_set_option(mut string: *mut libc::c_char) {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    token = strtok(string, b",\0" as *const u8 as *const libc::c_char);
    while !token.is_null() {
        let mut p: *mut libc::c_char = strchr(token, '=' as i32);
        if p.is_null() {
            xheader_set_single_keyword(token);
        } else {
            xheader_set_keyword_equal(token, p);
        }
        token = strtok(
            0 as *mut libc::c_char,
            b",\0" as *const u8 as *const libc::c_char,
        );
    }
}
pub unsafe extern "C" fn xheader_format_name(
    mut st: *mut tar_stat_info,
    mut fmt: *const libc::c_char,
    mut n: size_t,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut q: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dir: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut base: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut pidbuf: [libc::c_char; 21] = [0; 21];
    let mut pptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut nbuf: [libc::c_char; 21] = [0; 21];
    let mut nptr: *const libc::c_char = 0 as *const libc::c_char;
    len = 0 as libc::c_int as size_t;
    p = fmt;
    while *p != 0 {
        if *p as libc::c_int == '%' as i32
            && *p.offset(1 as libc::c_int as isize) as libc::c_int != 0
        {
            p = p.offset(1);
            match *p as libc::c_int {
                37 => {
                    len = len.wrapping_add(1);
                    len;
                }
                100 => {
                    if !st.is_null() {
                        if dirp.is_null() {
                            dirp = dir_name((*st).orig_file_name);
                        }
                        dir = safer_name_suffix(
                            dirp,
                            0 as libc::c_int != 0,
                            absolute_names_option,
                        );
                        len = (len as libc::c_ulong).wrapping_add(strlen(dir)) as size_t
                            as size_t;
                    }
                }
                102 => {
                    if !st.is_null() {
                        base = last_component((*st).orig_file_name);
                        len = (len as libc::c_ulong).wrapping_add(strlen(base)) as size_t
                            as size_t;
                    }
                }
                112 => {
                    pptr = umaxtostr(getpid() as uintmax_t, pidbuf.as_mut_ptr());
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            pidbuf
                                .as_mut_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(-(1 as libc::c_int as isize))
                                .offset_from(pptr) as libc::c_long as libc::c_ulong,
                        ) as size_t as size_t;
                }
                110 => {
                    nptr = umaxtostr(n, nbuf.as_mut_ptr());
                    len = (len as libc::c_ulong)
                        .wrapping_add(
                            nbuf
                                .as_mut_ptr()
                                .offset(
                                    ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong
                                        as isize,
                                )
                                .offset(-(1 as libc::c_int as isize))
                                .offset_from(nptr) as libc::c_long as libc::c_ulong,
                        ) as size_t as size_t;
                }
                _ => {
                    len = (len as libc::c_ulong)
                        .wrapping_add(2 as libc::c_int as libc::c_ulong) as size_t
                        as size_t;
                }
            }
        } else {
            len = len.wrapping_add(1);
            len;
        }
        p = p.offset(1);
        p;
    }
    buf = xmalloc(len.wrapping_add(1 as libc::c_int as libc::c_ulong))
        as *mut libc::c_char;
    q = buf;
    p = fmt;
    while *p != 0 {
        if *p as libc::c_int == '%' as i32 {
            match *p.offset(1 as libc::c_int as isize) as libc::c_int {
                37 => {
                    let fresh1 = p;
                    p = p.offset(1);
                    let fresh2 = q;
                    q = q.offset(1);
                    *fresh2 = *fresh1;
                    p = p.offset(1);
                    p;
                }
                100 => {
                    if !dir.is_null() {
                        q = stpcpy(q, dir);
                    }
                    p = p.offset(2 as libc::c_int as isize);
                }
                102 => {
                    if !base.is_null() {
                        q = stpcpy(q, base);
                    }
                    p = p.offset(2 as libc::c_int as isize);
                }
                112 => {
                    q = stpcpy(q, pptr);
                    p = p.offset(2 as libc::c_int as isize);
                }
                110 => {
                    q = stpcpy(q, nptr);
                    p = p.offset(2 as libc::c_int as isize);
                }
                _ => {
                    let fresh3 = p;
                    p = p.offset(1);
                    let fresh4 = q;
                    q = q.offset(1);
                    *fresh4 = *fresh3;
                    if *p != 0 {
                        let fresh5 = p;
                        p = p.offset(1);
                        let fresh6 = q;
                        q = q.offset(1);
                        *fresh6 = *fresh5;
                    }
                }
            }
        } else {
            let fresh7 = p;
            p = p.offset(1);
            let fresh8 = q;
            q = q.offset(1);
            *fresh8 = *fresh7;
        }
    }
    rpl_free(dirp as *mut libc::c_void);
    while q > buf && *q.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
    {
        q = q.offset(-1);
        q;
    }
    *q = 0 as libc::c_int as libc::c_char;
    return buf;
}
static mut header_template: [[*const libc::c_char; 2]; 2] = [
    [
        b"%d/PaxHeaders/%f\0" as *const u8 as *const libc::c_char,
        b"%d/PaxHeaders.%p/%f\0" as *const u8 as *const libc::c_char,
    ],
    [
        b"/GlobalHead.%n\0" as *const u8 as *const libc::c_char,
        b"/GlobalHead.%p.%n\0" as *const u8 as *const libc::c_char,
    ],
];
pub unsafe extern "C" fn xheader_xhdr_name(
    mut st: *mut tar_stat_info,
) -> *mut libc::c_char {
    if exthdr_name.is_null() {
        assign_string(
            &mut exthdr_name,
            header_template[pax_file_header as libc::c_int
                as usize][posixly_correct as usize],
        );
    }
    return xheader_format_name(st, exthdr_name, 0 as libc::c_int as size_t);
}
pub unsafe extern "C" fn xheader_ghdr_name() -> *mut libc::c_char {
    if globexthdr_name.is_null() {
        let mut len: size_t = 0;
        let mut global_header_template: *const libc::c_char = header_template[pax_global_header
            as libc::c_int as usize][posixly_correct as usize];
        let mut tmp: *const libc::c_char = getenv(
            b"TMPDIR\0" as *const u8 as *const libc::c_char,
        );
        if tmp.is_null() {
            tmp = b"/tmp\0" as *const u8 as *const libc::c_char;
        }
        len = (strlen(tmp))
            .wrapping_add(strlen(global_header_template))
            .wrapping_add(1 as libc::c_int as libc::c_ulong);
        globexthdr_name = xmalloc(len) as *mut libc::c_char;
        strcpy(globexthdr_name, tmp);
        strcat(globexthdr_name, global_header_template);
    }
    return xheader_format_name(
        0 as *mut tar_stat_info,
        globexthdr_name,
        global_header_count.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
}
pub unsafe extern "C" fn xheader_write(
    mut type_0: libc::c_char,
    mut name: *mut libc::c_char,
    mut t: time_t,
    mut xhdr: *mut xheader,
) {
    let mut header: *mut block = 0 as *mut block;
    let mut size: size_t = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    size = (*xhdr).size;
    match type_0 as libc::c_int {
        103 => {
            if !globexthdr_mtime_option.is_null() {
                t = globexthdr_mtime;
            }
        }
        120 => {
            if !exthdr_mtime_option.is_null() {
                t = exthdr_mtime;
            }
        }
        _ => {}
    }
    header = start_private_header(name, size, t);
    (*header).header.typeflag = type_0;
    simple_finish_header(header);
    p = (*xhdr).buffer;
    loop {
        let mut len: size_t = 0;
        header = find_next_block();
        len = 512 as libc::c_int as size_t;
        if len > size {
            len = size;
        }
        memcpy(
            ((*header).buffer).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            len,
        );
        if len < 512 as libc::c_int as libc::c_ulong {
            memset(
                ((*header).buffer).as_mut_ptr().offset(len as isize)
                    as *mut libc::c_void,
                0 as libc::c_int,
                (512 as libc::c_int as libc::c_ulong).wrapping_sub(len),
            );
        }
        p = p.offset(len as isize);
        size = (size as libc::c_ulong).wrapping_sub(len) as size_t as size_t;
        set_next_block_after(header);
        if !(size > 0 as libc::c_int as libc::c_ulong) {
            break;
        }
    }
    xheader_destroy(xhdr);
    if type_0 as libc::c_int == 'g' as i32 {
        global_header_count = global_header_count.wrapping_add(1);
        global_header_count;
    }
}
pub unsafe extern "C" fn xheader_write_global(mut xhdr: *mut xheader) {
    if !keyword_global_override_list.is_null() {
        let mut kp: *mut keyword_list = 0 as *mut keyword_list;
        xheader_init(xhdr);
        kp = keyword_global_override_list;
        while !kp.is_null() {
            code_string((*kp).value, (*kp).pattern, xhdr);
            kp = (*kp).next;
        }
    }
    if !((*xhdr).stk).is_null() {
        let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
        xheader_finish(xhdr);
        name = xheader_ghdr_name();
        xheader_write('g' as i32 as libc::c_char, name, start_time.tv_sec, xhdr);
        rpl_free(name as *mut libc::c_void);
    }
}
pub unsafe extern "C" fn xheader_forbid_global() {
    if !keyword_global_override_list.is_null() {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"can't update global extended header record\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        usage(2 as libc::c_int);
    }
}
pub unsafe extern "C" fn xheader_xattr_init(mut st: *mut tar_stat_info) {
    (*st).xattr_map = 0 as *mut xattr_array;
    (*st).xattr_map_size = 0 as libc::c_int as size_t;
    (*st).acls_a_ptr = 0 as *mut libc::c_char;
    (*st).acls_a_len = 0 as libc::c_int as size_t;
    (*st).acls_d_ptr = 0 as *mut libc::c_char;
    (*st).acls_d_len = 0 as libc::c_int as size_t;
    (*st).cntx_name = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn xheader_xattr_free(
    mut xattr_map: *mut xattr_array,
    mut xattr_map_size: size_t,
) {
    let mut scan: size_t = 0 as libc::c_int as size_t;
    while scan < xattr_map_size {
        rpl_free((*xattr_map.offset(scan as isize)).xkey as *mut libc::c_void);
        rpl_free((*xattr_map.offset(scan as isize)).xval_ptr as *mut libc::c_void);
        scan = scan.wrapping_add(1);
        scan;
    }
    rpl_free(xattr_map as *mut libc::c_void);
}
unsafe extern "C" fn xheader_xattr__add(
    mut xattr_map: *mut *mut xattr_array,
    mut xattr_map_size: *mut size_t,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
    mut len: size_t,
) {
    let fresh9 = *xattr_map_size;
    *xattr_map_size = (*xattr_map_size).wrapping_add(1);
    let mut pos: size_t = fresh9;
    *xattr_map = xrealloc(
        *xattr_map as *mut libc::c_void,
        (*xattr_map_size)
            .wrapping_mul(::std::mem::size_of::<xattr_array>() as libc::c_ulong),
    ) as *mut xattr_array;
    let ref mut fresh10 = (*(*xattr_map).offset(pos as isize)).xkey;
    *fresh10 = xstrdup(key);
    let ref mut fresh11 = (*(*xattr_map).offset(pos as isize)).xval_ptr;
    *fresh11 = xmemdup(
        val as *const libc::c_void,
        len.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    (*(*xattr_map).offset(pos as isize)).xval_len = len;
}
unsafe extern "C" fn xattr_decode_keyword(mut keyword: *mut libc::c_char) {
    let mut kpr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut kpl: *mut libc::c_char = 0 as *mut libc::c_char;
    kpl = keyword;
    kpr = kpl;
    loop {
        if *kpr as libc::c_int == '%' as i32 {
            if *kpr.offset(1 as libc::c_int as isize) as libc::c_int == '3' as i32
                && *kpr.offset(2 as libc::c_int as isize) as libc::c_int == 'D' as i32
            {
                *kpl = '=' as i32 as libc::c_char;
                kpr = kpr.offset(3 as libc::c_int as isize);
                kpl = kpl.offset(1);
                kpl;
                continue;
            } else if *kpr.offset(1 as libc::c_int as isize) as libc::c_int == '2' as i32
                && *kpr.offset(2 as libc::c_int as isize) as libc::c_int == '5' as i32
            {
                *kpl = '%' as i32 as libc::c_char;
                kpr = kpr.offset(3 as libc::c_int as isize);
                kpl = kpl.offset(1);
                kpl;
                continue;
            }
        }
        *kpl = *kpr;
        if *kpr as libc::c_int == 0 as libc::c_int {
            break;
        }
        kpr = kpr.offset(1);
        kpr;
        kpl = kpl.offset(1);
        kpl;
    };
}
pub unsafe extern "C" fn xheader_xattr_add(
    mut st: *mut tar_stat_info,
    mut key: *const libc::c_char,
    mut val: *const libc::c_char,
    mut len: size_t,
) {
    let mut klen: size_t = strlen(key);
    let mut xkey: *mut libc::c_char = xmalloc(
        (strlen(b"SCHILY.xattr.\0" as *const u8 as *const libc::c_char))
            .wrapping_add(klen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    let mut tmp: *mut libc::c_char = xkey;
    tmp = stpcpy(tmp, b"SCHILY.xattr.\0" as *const u8 as *const libc::c_char);
    stpcpy(tmp, key);
    xheader_xattr__add(&mut (*st).xattr_map, &mut (*st).xattr_map_size, xkey, val, len);
    rpl_free(xkey as *mut libc::c_void);
}
pub unsafe extern "C" fn xheader_xattr_copy(
    mut st: *const tar_stat_info,
    mut xattr_map: *mut *mut xattr_array,
    mut xattr_map_size: *mut size_t,
) {
    let mut scan: size_t = 0 as libc::c_int as size_t;
    *xattr_map = 0 as *mut xattr_array;
    *xattr_map_size = 0 as libc::c_int as size_t;
    while scan < (*st).xattr_map_size {
        let mut key: *mut libc::c_char = (*((*st).xattr_map).offset(scan as isize)).xkey;
        let mut val: *mut libc::c_char = (*((*st).xattr_map).offset(scan as isize))
            .xval_ptr;
        let mut len: size_t = (*((*st).xattr_map).offset(scan as isize)).xval_len;
        xheader_xattr__add(xattr_map, xattr_map_size, key, val, len);
        scan = scan.wrapping_add(1);
        scan;
    }
}
unsafe extern "C" fn locate_handler(
    mut keyword: *const libc::c_char,
) -> *const xhdr_tab {
    let mut p: *const xhdr_tab = 0 as *const xhdr_tab;
    p = xhdr_tab.as_ptr();
    while !((*p).keyword).is_null() {
        if (*p).prefix {
            if strncmp((*p).keyword, keyword, strlen((*p).keyword)) == 0 as libc::c_int {
                return p;
            }
        } else if strcmp((*p).keyword, keyword) == 0 as libc::c_int {
            return p
        }
        p = p.offset(1);
        p;
    }
    return 0 as *const xhdr_tab;
}
unsafe extern "C" fn xheader_protected_pattern_p(
    mut pattern: *const libc::c_char,
) -> bool {
    let mut p: *const xhdr_tab = 0 as *const xhdr_tab;
    p = xhdr_tab.as_ptr();
    while !((*p).keyword).is_null() {
        if !(*p).prefix && (*p).flags & 0x1 as libc::c_int != 0
            && fnmatch(pattern, (*p).keyword, 0 as libc::c_int) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn xheader_protected_keyword_p(
    mut keyword: *const libc::c_char,
) -> bool {
    let mut p: *const xhdr_tab = 0 as *const xhdr_tab;
    p = xhdr_tab.as_ptr();
    while !((*p).keyword).is_null() {
        if !(*p).prefix && (*p).flags & 0x1 as libc::c_int != 0
            && strcmp((*p).keyword, keyword) == 0 as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn decode_record(
    mut xhdr: *mut xheader,
    mut ptr: *mut *mut libc::c_char,
    mut handler: Option::<
        unsafe extern "C" fn(
            *mut libc::c_void,
            *const libc::c_char,
            *const libc::c_char,
            size_t,
        ) -> (),
    >,
    mut data: *mut libc::c_void,
) -> bool {
    let mut start: *mut libc::c_char = *ptr;
    let mut p: *mut libc::c_char = start;
    let mut len: size_t = 0;
    let mut len_lim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut keyword: *const libc::c_char = 0 as *const libc::c_char;
    let mut nextp: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len_max: size_t = ((*xhdr).buffer)
        .offset((*xhdr).size as isize)
        .offset_from(start) as libc::c_long as size_t;
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
        p = p.offset(1);
        p;
    }
    if !((*p as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
        <= 9 as libc::c_int as libc::c_uint)
    {
        if *p != 0 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed extended header: missing length\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            exit_status = 2 as libc::c_int;
        }
        return 0 as libc::c_int != 0;
    }
    len = strtoumax(p, &mut len_lim, 10 as libc::c_int);
    if len_max < len {
        let mut len_len: libc::c_int = len_lim.offset_from(p) as libc::c_long
            as libc::c_int;
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Extended header length %*s is out of range\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            len_len,
            p,
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    nextp = start.offset(len as isize);
    p = len_lim;
    while *p as libc::c_int == ' ' as i32 || *p as libc::c_int == '\t' as i32 {
        p = p.offset(1);
        p;
    }
    if p == len_lim {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed extended header: missing blank after length\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    keyword = p;
    p = strchr(p, '=' as i32);
    if !(!p.is_null() && p < nextp) {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed extended header: missing equal sign\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    if *nextp.offset(-(1 as libc::c_int) as isize) as libc::c_int != '\n' as i32 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed extended header: missing newline\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    let ref mut fresh12 = *nextp.offset(-(1 as libc::c_int) as isize);
    *fresh12 = '\0' as i32 as libc::c_char;
    *p = *fresh12;
    handler
        .unwrap()(
        data,
        keyword,
        p.offset(1 as libc::c_int as isize),
        (nextp.offset_from(p) as libc::c_long - 2 as libc::c_int as libc::c_long)
            as size_t,
    );
    *p = '=' as i32 as libc::c_char;
    *nextp.offset(-(1 as libc::c_int) as isize) = '\n' as i32 as libc::c_char;
    *ptr = nextp;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn run_override_list(
    mut kp: *mut keyword_list,
    mut st: *mut tar_stat_info,
) {
    while !kp.is_null() {
        let mut t: *const xhdr_tab = locate_handler((*kp).pattern);
        if !t.is_null() {
            ((*t).decoder).unwrap()(st, (*t).keyword, (*kp).value, strlen((*kp).value));
        }
        kp = (*kp).next;
    }
}
unsafe extern "C" fn decx(
    mut data: *mut libc::c_void,
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
    mut size: size_t,
) {
    let mut t: *const xhdr_tab = 0 as *const xhdr_tab;
    let mut st: *mut tar_stat_info = data as *mut tar_stat_info;
    if xheader_keyword_deleted_p(keyword) as libc::c_int != 0
        || xheader_keyword_override_p(keyword) as libc::c_int != 0
    {
        return;
    }
    t = locate_handler(keyword);
    if !t.is_null() {
        ((*t).decoder).unwrap()(st, keyword, value, size);
    } else if warning_option & 0x20000 as libc::c_int != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Ignoring unknown extended header keyword '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            keyword,
        );
    }
}
pub unsafe extern "C" fn xheader_decode(mut st: *mut tar_stat_info) {
    run_override_list(keyword_global_override_list, st);
    run_override_list(global_header_override_list, st);
    if (*st).xhdr.size != 0 {
        let mut p: *mut libc::c_char = ((*st).xhdr.buffer)
            .offset(512 as libc::c_int as isize);
        while decode_record(
            &mut (*st).xhdr,
            &mut p,
            Some(
                decx
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            st as *mut libc::c_void,
        ) {}
    }
    run_override_list(keyword_override_list, st);
    (*st).archive_file_size = (*st).stat.st_size;
    if (*st).real_size_set {
        (*st).stat.st_size = (*st).real_size;
    }
}
unsafe extern "C" fn decg(
    mut data: *mut libc::c_void,
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
    mut size: size_t,
) {
    let mut kwl: *mut *mut keyword_list = data as *mut *mut keyword_list;
    let mut tab: *const xhdr_tab = locate_handler(keyword);
    if !tab.is_null() && (*tab).flags & 0x2 as libc::c_int != 0 {
        ((*tab).decoder).unwrap()(data as *mut tar_stat_info, keyword, value, size);
    } else {
        xheader_list_append(kwl, keyword, value);
    };
}
pub unsafe extern "C" fn xheader_decode_global(mut xhdr: *mut xheader) {
    if (*xhdr).size != 0 {
        let mut p: *mut libc::c_char = ((*xhdr).buffer)
            .offset(512 as libc::c_int as isize);
        xheader_list_destroy(&mut global_header_override_list);
        while decode_record(
            xhdr,
            &mut p,
            Some(
                decg
                    as unsafe extern "C" fn(
                        *mut libc::c_void,
                        *const libc::c_char,
                        *const libc::c_char,
                        size_t,
                    ) -> (),
            ),
            &mut global_header_override_list as *mut *mut keyword_list
                as *mut libc::c_void,
        ) {}
    }
}
unsafe extern "C" fn xheader_init(mut xhdr: *mut xheader) {
    if ((*xhdr).stk).is_null() {
        (*xhdr)
            .stk = xmalloc(::std::mem::size_of::<obstack>() as libc::c_ulong)
            as *mut obstack;
        _obstack_begin(
            (*xhdr).stk,
            0 as libc::c_int as size_t,
            0 as libc::c_int as size_t,
            Some(xmalloc as unsafe extern "C" fn(size_t) -> *mut libc::c_void),
            Some(rpl_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
    }
}
pub unsafe extern "C" fn xheader_store(
    mut keyword: *const libc::c_char,
    mut st: *mut tar_stat_info,
    mut data: *const libc::c_void,
) {
    let mut t: *const xhdr_tab = 0 as *const xhdr_tab;
    if !((*st).xhdr.buffer).is_null() {
        return;
    }
    t = locate_handler(keyword);
    if t.is_null() || ((*t).coder).is_none() {
        return;
    }
    if xheader_keyword_deleted_p(keyword) {
        return;
    }
    xheader_init(&mut (*st).xhdr);
    if !xheader_keyword_override_p(keyword) {
        ((*t).coder).unwrap()(st, keyword, &mut (*st).xhdr, data);
    }
}
pub unsafe extern "C" fn xheader_read(
    mut xhdr: *mut xheader,
    mut p: *mut block,
    mut size: off_t,
) {
    let mut j: size_t = 0 as libc::c_int as size_t;
    if size < 0 as libc::c_int as libc::c_long {
        size = 0 as libc::c_int as off_t;
    }
    if (18446744073709551615 as libc::c_ulong)
        .wrapping_sub(512 as libc::c_int as libc::c_ulong) <= size as libc::c_ulong
    {
        xalloc_die();
    }
    size += 512 as libc::c_int as libc::c_long;
    (*xhdr).size = size as size_t;
    (*xhdr)
        .buffer = xmalloc((size + 1 as libc::c_int as libc::c_long) as size_t)
        as *mut libc::c_char;
    *((*xhdr).buffer).offset(size as isize) = '\0' as i32 as libc::c_char;
    loop {
        let mut len: size_t = size as size_t;
        if len > 512 as libc::c_int as libc::c_ulong {
            len = 512 as libc::c_int as size_t;
        }
        if p.is_null() {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Unexpected EOF in archive\0" as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
            );
            fatal_exit();
        }
        memcpy(
            &mut *((*xhdr).buffer).offset(j as isize) as *mut libc::c_char
                as *mut libc::c_void,
            ((*p).buffer).as_mut_ptr() as *const libc::c_void,
            len,
        );
        set_next_block_after(p);
        p = find_next_block();
        j = (j as libc::c_ulong).wrapping_add(len) as size_t as size_t;
        size = (size as libc::c_ulong).wrapping_sub(len) as off_t as off_t;
        if !(size > 0 as libc::c_int as libc::c_long) {
            break;
        }
    };
}
unsafe extern "C" fn xattr_encode_keyword(
    mut keyword: *const libc::c_char,
) -> *mut libc::c_char {
    static mut encode_buffer: *mut libc::c_char = 0 as *const libc::c_char
        as *mut libc::c_char;
    static mut encode_buffer_size: size_t = 0 as libc::c_int as size_t;
    let mut bp: size_t = 0;
    if encode_buffer.is_null() {
        encode_buffer_size = 256 as libc::c_int as size_t;
        encode_buffer = xmalloc(encode_buffer_size) as *mut libc::c_char;
    } else {
        *encode_buffer = 0 as libc::c_int as libc::c_char;
    }
    bp = 0 as libc::c_int as size_t;
    while *keyword as libc::c_int != 0 as libc::c_int {
        let mut c: libc::c_char = *keyword;
        if bp.wrapping_add(2 as libc::c_int as libc::c_ulong) >= encode_buffer_size {
            encode_buffer = x2realloc(
                encode_buffer as *mut libc::c_void,
                &mut encode_buffer_size,
            ) as *mut libc::c_char;
        }
        if c as libc::c_int == '%' as i32 {
            strcpy(
                encode_buffer.offset(bp as isize),
                b"%25\0" as *const u8 as *const libc::c_char,
            );
            bp = (bp as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else if c as libc::c_int == '=' as i32 {
            strcpy(
                encode_buffer.offset(bp as isize),
                b"%3D\0" as *const u8 as *const libc::c_char,
            );
            bp = (bp as libc::c_ulong).wrapping_add(2 as libc::c_int as libc::c_ulong)
                as size_t as size_t;
        } else {
            *encode_buffer.offset(bp as isize) = c;
        }
        bp = bp.wrapping_add(1);
        bp;
        keyword = keyword.offset(1);
        keyword;
    }
    *encode_buffer.offset(bp as isize) = 0 as libc::c_int as libc::c_char;
    return encode_buffer;
}
unsafe extern "C" fn xheader_print_n(
    mut xhdr: *mut xheader,
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
    mut vsize: size_t,
) {
    let mut p: size_t = 0;
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut nbuf: [libc::c_char; 21] = [0; 21];
    let mut np: *const libc::c_char = 0 as *const libc::c_char;
    let mut len: size_t = 0;
    let mut klen: size_t = 0;
    keyword = xattr_encode_keyword(keyword);
    klen = strlen(keyword);
    len = klen.wrapping_add(vsize).wrapping_add(3 as libc::c_int as libc::c_ulong);
    loop {
        p = n;
        np = umaxtostr(len.wrapping_add(p), nbuf.as_mut_ptr());
        n = nbuf
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize))
            .offset_from(np) as libc::c_long as size_t;
        if !(n != p) {
            break;
        }
    }
    x_obstack_grow(xhdr, np, n);
    x_obstack_1grow(xhdr, ' ' as i32 as libc::c_char);
    x_obstack_grow(xhdr, keyword, klen);
    x_obstack_1grow(xhdr, '=' as i32 as libc::c_char);
    x_obstack_grow(xhdr, value, vsize);
    x_obstack_1grow(xhdr, '\n' as i32 as libc::c_char);
}
unsafe extern "C" fn xheader_print(
    mut xhdr: *mut xheader,
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
) {
    xheader_print_n(xhdr, keyword, value, strlen(value));
}
pub unsafe extern "C" fn xheader_finish(mut xhdr: *mut xheader) {
    let mut kp: *mut keyword_list = 0 as *mut keyword_list;
    kp = keyword_override_list;
    while !kp.is_null() {
        code_string((*kp).value, (*kp).pattern, xhdr);
        kp = (*kp).next;
    }
    (*xhdr)
        .buffer = ({
        let mut __o1: *mut obstack = (*xhdr).stk;
        let mut __value: *mut libc::c_void = (*__o1).object_base as *mut libc::c_void;
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
                            < ::std::mem::size_of::<*mut libc::c_void>() as libc::c_ulong
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
    }) as *mut libc::c_char;
}
pub unsafe extern "C" fn xheader_destroy(mut xhdr: *mut xheader) {
    if !((*xhdr).stk).is_null() {
        let mut __o: *mut obstack = (*xhdr).stk;
        let mut __obj: *mut libc::c_void = 0 as *mut libc::c_void;
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut libc::c_char;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
        rpl_free((*xhdr).stk as *mut libc::c_void);
        (*xhdr).stk = 0 as *mut obstack;
    } else {
        rpl_free((*xhdr).buffer as *mut libc::c_void);
    }
    (*xhdr).buffer = 0 as *mut libc::c_char;
    (*xhdr).size = 0 as libc::c_int as size_t;
}
pub unsafe extern "C" fn xheader_string_begin(mut xhdr: *mut xheader) {
    (*xhdr).string_length = 0 as libc::c_int as uintmax_t;
}
pub unsafe extern "C" fn xheader_string_add(
    mut xhdr: *mut xheader,
    mut s: *const libc::c_char,
) {
    if !((*xhdr).buffer).is_null() {
        return;
    }
    xheader_init(xhdr);
    (*xhdr)
        .string_length = ((*xhdr).string_length as libc::c_ulong).wrapping_add(strlen(s))
        as uintmax_t as uintmax_t;
    x_obstack_grow(xhdr, s, strlen(s));
}
pub unsafe extern "C" fn xheader_string_end(
    mut xhdr: *mut xheader,
    mut keyword: *const libc::c_char,
) -> bool {
    let mut len: uintmax_t = 0;
    let mut p: uintmax_t = 0;
    let mut n: uintmax_t = 0 as libc::c_int as uintmax_t;
    let mut size: size_t = 0;
    let mut nbuf: [libc::c_char; 21] = [0; 21];
    let mut np: *const libc::c_char = 0 as *const libc::c_char;
    let mut cp: *mut libc::c_char = 0 as *mut libc::c_char;
    if !((*xhdr).buffer).is_null() {
        return 0 as libc::c_int != 0;
    }
    xheader_init(xhdr);
    len = (strlen(keyword))
        .wrapping_add((*xhdr).string_length)
        .wrapping_add(3 as libc::c_int as libc::c_ulong);
    loop {
        p = n;
        np = umaxtostr(len.wrapping_add(p), nbuf.as_mut_ptr());
        n = nbuf
            .as_mut_ptr()
            .offset(
                ::std::mem::size_of::<[libc::c_char; 21]>() as libc::c_ulong as isize,
            )
            .offset(-(1 as libc::c_int as isize))
            .offset_from(np) as libc::c_long as uintmax_t;
        if !(n != p) {
            break;
        }
    }
    p = (strlen(keyword))
        .wrapping_add(n)
        .wrapping_add(2 as libc::c_int as libc::c_ulong);
    size = p;
    if size != p {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Generated keyword/value pair is too long (keyword=%s, length=%s)\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            keyword,
            nbuf.as_mut_ptr(),
        );
        exit_status = 2 as libc::c_int;
        let mut __o: *mut obstack = (*xhdr).stk;
        let mut __obj: *mut libc::c_void = {
            let mut __o1: *mut obstack = (*xhdr).stk;
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
        };
        if __obj > (*__o).chunk as *mut libc::c_void
            && __obj < (*__o).chunk_limit as *mut libc::c_void
        {
            (*__o).object_base = __obj as *mut libc::c_char;
            (*__o).next_free = (*__o).object_base;
        } else {
            _obstack_free(__o, __obj);
        }
        return 0 as libc::c_int != 0;
    }
    x_obstack_blank(xhdr, p);
    x_obstack_1grow(xhdr, '\n' as i32 as libc::c_char);
    cp = ((*(*xhdr).stk).next_free as *mut libc::c_void as *mut libc::c_char)
        .offset(-((*xhdr).string_length as isize))
        .offset(-(p as isize))
        .offset(-(1 as libc::c_int as isize));
    memmove(
        cp.offset(p as isize) as *mut libc::c_void,
        cp as *const libc::c_void,
        (*xhdr).string_length,
    );
    cp = stpcpy(cp, np);
    let fresh13 = cp;
    cp = cp.offset(1);
    *fresh13 = ' ' as i32 as libc::c_char;
    cp = stpcpy(cp, keyword);
    let fresh14 = cp;
    cp = cp.offset(1);
    *fresh14 = '=' as i32 as libc::c_char;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn out_of_range_header(
    mut keyword: *const libc::c_char,
    mut value: *const libc::c_char,
    mut minval: intmax_t,
    mut maxval: uintmax_t,
) {
    let mut minval_buf: [libc::c_char; 21] = [0; 21];
    let mut maxval_buf: [libc::c_char; 21] = [0; 21];
    let mut minval_string: *mut libc::c_char = imaxtostr(
        minval,
        minval_buf.as_mut_ptr(),
    );
    let mut maxval_string: *mut libc::c_char = umaxtostr(
        maxval,
        maxval_buf.as_mut_ptr(),
    );
    if error_hook.is_some() {
        error_hook.unwrap()();
    }
    error(
        0 as libc::c_int,
        0 as libc::c_int,
        dcgettext(
            0 as *const libc::c_char,
            b"Extended header %s=%s is out of range %s..%s\0" as *const u8
                as *const libc::c_char,
            5 as libc::c_int,
        ),
        keyword,
        value,
        minval_string,
        maxval_string,
    );
    exit_status = 2 as libc::c_int;
}
unsafe extern "C" fn code_string(
    mut string: *const libc::c_char,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
) {
    let mut outstr: *mut libc::c_char = 0 as *mut libc::c_char;
    if !utf8_convert(1 as libc::c_int != 0, string, &mut outstr) {
        outstr = xstrdup(string);
    }
    xheader_print(xhdr, keyword, outstr);
    rpl_free(outstr as *mut libc::c_void);
}
unsafe extern "C" fn decode_string(
    mut string: *mut *mut libc::c_char,
    mut arg: *const libc::c_char,
) {
    if !(*string).is_null() {
        rpl_free(*string as *mut libc::c_void);
        *string = 0 as *mut libc::c_char;
    }
    if !utf8_convert(0 as libc::c_int != 0, arg, string) {
        assign_string(string, arg);
    }
}
unsafe extern "C" fn code_time(
    mut t: timespec,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
) {
    let mut buf: [libc::c_char; 32] = [0; 32];
    xheader_print(xhdr, keyword, code_timespec(t, buf.as_mut_ptr()));
}
unsafe extern "C" fn decode_time(
    mut ts: *mut timespec,
    mut arg: *const libc::c_char,
    mut keyword: *const libc::c_char,
) -> bool {
    let mut arg_lim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut t: timespec = decode_timespec(arg, &mut arg_lim, 1 as libc::c_int != 0);
    if !valid_timespec(t) {
        if arg < arg_lim as *const libc::c_char && *arg_lim == 0 {
            out_of_range_header(
                keyword,
                arg,
                !if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                },
                (if (0 as libc::c_int as time_t) < -(1 as libc::c_int) as time_t {
                    -(1 as libc::c_int) as time_t
                } else {
                    (((1 as libc::c_int as time_t)
                        << (::std::mem::size_of::<time_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as uintmax_t,
            );
        } else {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed extended header: invalid %s=%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                keyword,
                arg,
            );
            exit_status = 2 as libc::c_int;
        }
        return 0 as libc::c_int != 0;
    }
    *ts = t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn code_signed_num(
    mut value: uintmax_t,
    mut keyword: *const libc::c_char,
    mut minval: intmax_t,
    mut maxval: uintmax_t,
    mut xhdr: *mut xheader,
) {
    let mut sbuf: [libc::c_char; 21] = [0; 21];
    xheader_print(xhdr, keyword, sysinttostr(value, minval, maxval, sbuf.as_mut_ptr()));
}
unsafe extern "C" fn code_num(
    mut value: uintmax_t,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
) {
    code_signed_num(
        value,
        keyword,
        0 as libc::c_int as intmax_t,
        18446744073709551615 as libc::c_ulong,
        xhdr,
    );
}
unsafe extern "C" fn decode_signed_num(
    mut num: *mut intmax_t,
    mut arg: *const libc::c_char,
    mut minval: intmax_t,
    mut maxval: uintmax_t,
    mut keyword: *const libc::c_char,
) -> bool {
    let mut arg_lim: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut u: intmax_t = strtosysint(arg, &mut arg_lim, minval, maxval);
    if *__errno_location() == 22 as libc::c_int || *arg_lim as libc::c_int != 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed extended header: invalid %s=%s\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            keyword,
            arg,
        );
        exit_status = 2 as libc::c_int;
        return 0 as libc::c_int != 0;
    }
    if *__errno_location() == 34 as libc::c_int {
        out_of_range_header(keyword, arg, minval, maxval);
        return 0 as libc::c_int != 0;
    }
    *num = u;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn decode_num(
    mut num: *mut uintmax_t,
    mut arg: *const libc::c_char,
    mut maxval: uintmax_t,
    mut keyword: *const libc::c_char,
) -> bool {
    let mut i: intmax_t = 0;
    if !decode_signed_num(&mut i, arg, 0 as libc::c_int as intmax_t, maxval, keyword) {
        return 0 as libc::c_int != 0;
    }
    *num = i as uintmax_t;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn dummy_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {}
unsafe extern "C" fn dummy_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {}
unsafe extern "C" fn atime_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_time((*st).atime, keyword, xhdr);
}
unsafe extern "C" fn atime_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if decode_time(&mut ts, arg, keyword) {
        (*st).atime = ts;
    }
}
unsafe extern "C" fn gid_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_signed_num(
        (*st).stat.st_gid as uintmax_t,
        keyword,
        !if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } as intmax_t,
        (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
        xhdr,
    );
}
unsafe extern "C" fn gid_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: intmax_t = 0;
    if decode_signed_num(
        &mut u,
        arg,
        !if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } as intmax_t,
        (if (0 as libc::c_int as gid_t) < -(1 as libc::c_int) as gid_t {
            -(1 as libc::c_int) as gid_t
        } else {
            ((1 as libc::c_int as gid_t)
                << (::std::mem::size_of::<gid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
        keyword,
    ) {
        (*st).stat.st_gid = u as __gid_t;
    }
}
unsafe extern "C" fn gname_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_string((*st).gname, keyword, xhdr);
}
unsafe extern "C" fn gname_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    decode_string(&mut (*st).gname, arg);
}
unsafe extern "C" fn linkpath_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_string((*st).link_name, keyword, xhdr);
}
unsafe extern "C" fn linkpath_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    decode_string(&mut (*st).link_name, arg);
}
unsafe extern "C" fn ctime_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_time((*st).ctime, keyword, xhdr);
}
unsafe extern "C" fn ctime_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if decode_time(&mut ts, arg, keyword) {
        (*st).ctime = ts;
    }
}
unsafe extern "C" fn mtime_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    let mut mtime: *const timespec = data as *const timespec;
    code_time(if !mtime.is_null() { *mtime } else { (*st).mtime }, keyword, xhdr);
}
unsafe extern "C" fn mtime_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    if decode_time(&mut ts, arg, keyword) {
        (*st).mtime = ts;
    }
}
unsafe extern "C" fn path_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_string((*st).file_name, keyword, xhdr);
}
unsafe extern "C" fn raw_path_decoder(
    mut st: *mut tar_stat_info,
    mut arg: *const libc::c_char,
) {
    decode_string(&mut (*st).orig_file_name, arg);
    decode_string(&mut (*st).file_name, arg);
    (*st).had_trailing_slash = strip_trailing_slashes((*st).file_name);
}
unsafe extern "C" fn path_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    if !(*st).sparse_name_done {
        raw_path_decoder(st, arg);
    }
}
unsafe extern "C" fn sparse_path_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    (*st).sparse_name_done = 1 as libc::c_int != 0;
    raw_path_decoder(st, arg);
}
unsafe extern "C" fn size_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_num((*st).stat.st_size as uintmax_t, keyword, xhdr);
}
unsafe extern "C" fn size_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        keyword,
    ) {
        (*st).stat.st_size = u as __off_t;
    }
}
unsafe extern "C" fn uid_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_signed_num(
        (*st).stat.st_uid as uintmax_t,
        keyword,
        !if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } as intmax_t,
        (if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
        xhdr,
    );
}
unsafe extern "C" fn uid_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: intmax_t = 0;
    if decode_signed_num(
        &mut u,
        arg,
        !if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        } as intmax_t,
        (if (0 as libc::c_int as uid_t) < -(1 as libc::c_int) as uid_t {
            -(1 as libc::c_int) as uid_t
        } else {
            ((1 as libc::c_int as uid_t)
                << (::std::mem::size_of::<uid_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
        keyword,
    ) {
        (*st).stat.st_uid = u as __uid_t;
    }
}
unsafe extern "C" fn uname_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_string((*st).uname, keyword, xhdr);
}
unsafe extern "C" fn uname_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    decode_string(&mut (*st).uname, arg);
}
unsafe extern "C" fn sparse_size_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    size_coder(st, keyword, xhdr, data);
}
unsafe extern "C" fn sparse_size_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        keyword,
    ) {
        (*st).real_size_set = 1 as libc::c_int != 0;
        (*st).real_size = u as off_t;
    }
}
unsafe extern "C" fn sparse_numblocks_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_num((*st).sparse_map_avail, keyword, xhdr);
}
unsafe extern "C" fn sparse_numblocks_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(&mut u, arg, 18446744073709551615 as libc::c_ulong, keyword) {
        (*st).sparse_map_size = u;
        (*st)
            .sparse_map = xcalloc(u, ::std::mem::size_of::<sp_array>() as libc::c_ulong)
            as *mut sp_array;
        (*st).sparse_map_avail = 0 as libc::c_int as size_t;
    }
}
unsafe extern "C" fn sparse_offset_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    let mut pi: *const size_t = data as *const size_t;
    code_num(
        (*((*st).sparse_map).offset(*pi as isize)).offset as uintmax_t,
        keyword,
        xhdr,
    );
}
unsafe extern "C" fn sparse_offset_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        keyword,
    ) {
        if (*st).sparse_map_avail < (*st).sparse_map_size {
            (*((*st).sparse_map).offset((*st).sparse_map_avail as isize))
                .offset = u as off_t;
        } else {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed extended header: excess %s=%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                b"GNU.sparse.offset\0" as *const u8 as *const libc::c_char,
                arg,
            );
            exit_status = 2 as libc::c_int;
        }
    }
}
unsafe extern "C" fn sparse_numbytes_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    let mut pi: *const size_t = data as *const size_t;
    code_num(
        (*((*st).sparse_map).offset(*pi as isize)).numbytes as uintmax_t,
        keyword,
        xhdr,
    );
}
unsafe extern "C" fn sparse_numbytes_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) as uintmax_t,
        keyword,
    ) {
        if (*st).sparse_map_avail < (*st).sparse_map_size {
            let fresh15 = (*st).sparse_map_avail;
            (*st).sparse_map_avail = ((*st).sparse_map_avail).wrapping_add(1);
            (*((*st).sparse_map).offset(fresh15 as isize)).numbytes = u as off_t;
        } else {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed extended header: excess %s=%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                keyword,
                arg,
            );
            exit_status = 2 as libc::c_int;
        }
    }
}
unsafe extern "C" fn sparse_map_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut offset: libc::c_int = 1 as libc::c_int;
    let mut e: sp_array = sp_array { offset: 0, numbytes: 0 };
    (*st).sparse_map_avail = 0 as libc::c_int as size_t;
    loop {
        let mut u: intmax_t = 0;
        let mut delim: *mut libc::c_char = 0 as *mut libc::c_char;
        if !((*arg as libc::c_uint).wrapping_sub('0' as i32 as libc::c_uint)
            <= 9 as libc::c_int as libc::c_uint)
        {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed extended header: invalid %s=%s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                keyword,
                arg,
            );
            exit_status = 2 as libc::c_int;
            return;
        }
        *__errno_location() = 0 as libc::c_int;
        u = strtoimax(arg, &mut delim, 10 as libc::c_int);
        if (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
            -(1 as libc::c_int) as off_t
        } else {
            (((1 as libc::c_int as off_t)
                << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                - 1 as libc::c_int as libc::c_long) * 2 as libc::c_int as libc::c_long
                + 1 as libc::c_int as libc::c_long
        }) < u
        {
            u = if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                -(1 as libc::c_int) as off_t
            } else {
                (((1 as libc::c_int as off_t)
                    << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                    - 1 as libc::c_int as libc::c_long)
                    * 2 as libc::c_int as libc::c_long + 1 as libc::c_int as libc::c_long
            };
            *__errno_location() = 34 as libc::c_int;
        }
        if offset != 0 {
            e.offset = u;
            if *__errno_location() == 34 as libc::c_int {
                out_of_range_header(
                    keyword,
                    arg,
                    0 as libc::c_int as intmax_t,
                    (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    }) as uintmax_t,
                );
                return;
            }
        } else {
            e.numbytes = u;
            if *__errno_location() == 34 as libc::c_int {
                out_of_range_header(
                    keyword,
                    arg,
                    0 as libc::c_int as intmax_t,
                    (if (0 as libc::c_int as off_t) < -(1 as libc::c_int) as off_t {
                        -(1 as libc::c_int) as off_t
                    } else {
                        (((1 as libc::c_int as off_t)
                            << (::std::mem::size_of::<off_t>() as libc::c_ulong)
                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                            - 1 as libc::c_int as libc::c_long)
                            * 2 as libc::c_int as libc::c_long
                            + 1 as libc::c_int as libc::c_long
                    }) as uintmax_t,
                );
                return;
            }
            if (*st).sparse_map_avail < (*st).sparse_map_size {
                let fresh16 = (*st).sparse_map_avail;
                (*st).sparse_map_avail = ((*st).sparse_map_avail).wrapping_add(1);
                *((*st).sparse_map).offset(fresh16 as isize) = e;
            } else {
                if error_hook.is_some() {
                    error_hook.unwrap()();
                }
                error(
                    0 as libc::c_int,
                    0 as libc::c_int,
                    dcgettext(
                        0 as *const libc::c_char,
                        b"Malformed extended header: excess %s=%s\0" as *const u8
                            as *const libc::c_char,
                        5 as libc::c_int,
                    ),
                    keyword,
                    arg,
                );
                exit_status = 2 as libc::c_int;
                return;
            }
        }
        offset = (offset == 0) as libc::c_int;
        if *delim as libc::c_int == 0 as libc::c_int {
            break;
        }
        if *delim as libc::c_int != ',' as i32 {
            if error_hook.is_some() {
                error_hook.unwrap()();
            }
            error(
                0 as libc::c_int,
                0 as libc::c_int,
                dcgettext(
                    0 as *const libc::c_char,
                    b"Malformed extended header: invalid %s: unexpected delimiter %c\0"
                        as *const u8 as *const libc::c_char,
                    5 as libc::c_int,
                ),
                keyword,
                *delim as libc::c_int,
            );
            exit_status = 2 as libc::c_int;
            return;
        }
        arg = delim.offset(1 as libc::c_int as isize);
    }
    if offset == 0 {
        if error_hook.is_some() {
            error_hook.unwrap()();
        }
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"Malformed extended header: invalid %s: odd number of values\0"
                    as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            keyword,
        );
        exit_status = 2 as libc::c_int;
    }
}
unsafe extern "C" fn dumpdir_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    xheader_print_n(
        xhdr,
        keyword,
        data as *const libc::c_char,
        dumpdir_size(data as *const libc::c_char),
    );
}
unsafe extern "C" fn dumpdir_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    (*st).dumpdir = xmalloc(size) as *mut libc::c_char;
    memcpy((*st).dumpdir as *mut libc::c_void, arg as *const libc::c_void, size);
}
unsafe extern "C" fn volume_label_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_string(data as *const libc::c_char, keyword, xhdr);
}
unsafe extern "C" fn volume_label_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    decode_string(&mut volume_label, arg);
}
unsafe extern "C" fn volume_size_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    let mut v: *const off_t = data as *const off_t;
    code_num(*v as uintmax_t, keyword, xhdr);
}
unsafe extern "C" fn volume_size_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
            -(1 as libc::c_int) as uintmax_t
        } else {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        },
        keyword,
    ) {
        continued_file_size = u;
    }
}
unsafe extern "C" fn volume_offset_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    let mut v: *const off_t = data as *const off_t;
    code_num(*v as uintmax_t, keyword, xhdr);
}
unsafe extern "C" fn volume_offset_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        if (0 as libc::c_int as uintmax_t) < -(1 as libc::c_int) as uintmax_t {
            -(1 as libc::c_int) as uintmax_t
        } else {
            ((1 as libc::c_int as uintmax_t)
                << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                .wrapping_add(1 as libc::c_int as libc::c_ulong)
        },
        keyword,
    ) {
        continued_file_offset = u;
    }
}
unsafe extern "C" fn volume_filename_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    decode_string(&mut continued_file_name, arg);
}
unsafe extern "C" fn xattr_selinux_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_string((*st).cntx_name, keyword, xhdr);
}
unsafe extern "C" fn xattr_selinux_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    decode_string(&mut (*st).cntx_name, arg);
}
unsafe extern "C" fn xattr_acls_a_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    xheader_print_n(xhdr, keyword, (*st).acls_a_ptr, (*st).acls_a_len);
}
unsafe extern "C" fn xattr_acls_a_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    (*st)
        .acls_a_ptr = xmemdup(
        arg as *const libc::c_void,
        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    (*st).acls_a_len = size;
}
unsafe extern "C" fn xattr_acls_d_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    xheader_print_n(xhdr, keyword, (*st).acls_d_ptr, (*st).acls_d_len);
}
unsafe extern "C" fn xattr_acls_d_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    (*st)
        .acls_d_ptr = xmemdup(
        arg as *const libc::c_void,
        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    (*st).acls_d_len = size;
}
unsafe extern "C" fn xattr_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    let mut xattr_map: *mut xattr_array = (*st).xattr_map;
    let mut off: *const size_t = data as *const size_t;
    xheader_print_n(
        xhdr,
        keyword,
        (*xattr_map.offset(*off as isize)).xval_ptr,
        (*xattr_map.offset(*off as isize)).xval_len,
    );
}
unsafe extern "C" fn xattr_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut xstr: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut xkey: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut klen_raw: size_t = strlen(keyword);
    let mut fresh17 = ::std::vec::from_elem(
        0,
        klen_raw.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    xkey = fresh17.leak().as_mut_ptr() as *mut libc::c_char;
    memcpy(
        xkey as *mut libc::c_void,
        keyword as *const libc::c_void,
        klen_raw.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    let mut fresh18 = ::std::vec::from_elem(
        0,
        size.wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    xstr = fresh18.leak().as_mut_ptr() as *mut libc::c_char;
    memcpy(
        xstr as *mut libc::c_void,
        arg as *const libc::c_void,
        size.wrapping_add(1 as libc::c_int as libc::c_ulong),
    );
    xattr_decode_keyword(xkey);
    xheader_xattr_add(
        st,
        xkey
            .offset(
                strlen(b"SCHILY.xattr.\0" as *const u8 as *const libc::c_char) as isize,
            ),
        xstr,
        size,
    );
}
unsafe extern "C" fn sparse_major_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_num((*st).sparse_major as uintmax_t, keyword, xhdr);
}
unsafe extern "C" fn sparse_major_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        (if (0 as libc::c_int as libc::c_uint) < -(1 as libc::c_int) as libc::c_uint {
            -(1 as libc::c_int) as libc::c_uint
        } else {
            ((1 as libc::c_int as libc::c_uint)
                << (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
        keyword,
    ) {
        (*st).sparse_major = u as libc::c_uint;
    }
}
unsafe extern "C" fn sparse_minor_coder(
    mut st: *const tar_stat_info,
    mut keyword: *const libc::c_char,
    mut xhdr: *mut xheader,
    mut data: *const libc::c_void,
) {
    code_num((*st).sparse_minor as uintmax_t, keyword, xhdr);
}
unsafe extern "C" fn sparse_minor_decoder(
    mut st: *mut tar_stat_info,
    mut keyword: *const libc::c_char,
    mut arg: *const libc::c_char,
    mut size: size_t,
) {
    let mut u: uintmax_t = 0;
    if decode_num(
        &mut u,
        arg,
        (if (0 as libc::c_int as libc::c_uint) < -(1 as libc::c_int) as libc::c_uint {
            -(1 as libc::c_int) as libc::c_uint
        } else {
            ((1 as libc::c_int as libc::c_uint)
                << (::std::mem::size_of::<libc::c_uint>() as libc::c_ulong)
                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                .wrapping_sub(1 as libc::c_int as libc::c_uint)
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add(1 as libc::c_int as libc::c_uint)
        }) as uintmax_t,
        keyword,
    ) {
        (*st).sparse_minor = u as libc::c_uint;
    }
}
pub static mut xhdr_tab: [xhdr_tab; 31] = unsafe {
    [
        {
            let mut init = xhdr_tab {
                keyword: b"atime\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    atime_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    atime_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"comment\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    dummy_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    dummy_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"charset\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    dummy_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    dummy_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"ctime\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    ctime_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    ctime_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"gid\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    gid_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    gid_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"gname\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    gname_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    gname_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"linkpath\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    linkpath_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    linkpath_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"mtime\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    mtime_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    mtime_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"path\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    path_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    path_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"size\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    size_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    size_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"uid\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    uid_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    uid_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"uname\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    uname_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    uname_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.name\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    path_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_path_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.major\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_major_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_major_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.minor\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_minor_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_minor_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.realsize\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_size_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_size_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.numblocks\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_numblocks_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_numblocks_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.size\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_size_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_size_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.offset\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_offset_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_offset_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.numbytes\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    sparse_numbytes_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    sparse_numbytes_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.sparse.map\0" as *const u8 as *const libc::c_char,
                coder: None,
                decoder: Some(
                    sparse_map_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.dumpdir\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    dumpdir_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    dumpdir_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.volume.label\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    volume_label_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    volume_label_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.volume.filename\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    volume_label_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    volume_filename_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.volume.size\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    volume_size_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    volume_size_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"GNU.volume.offset\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    volume_offset_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    volume_offset_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0x1 as libc::c_int | 0x2 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"RHT.security.selinux\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    xattr_selinux_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    xattr_selinux_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"SCHILY.acl.access\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    xattr_acls_a_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    xattr_acls_a_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"SCHILY.acl.default\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    xattr_acls_d_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    xattr_acls_d_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: b"SCHILY.xattr\0" as *const u8 as *const libc::c_char,
                coder: Some(
                    xattr_coder
                        as unsafe extern "C" fn(
                            *const tar_stat_info,
                            *const libc::c_char,
                            *mut xheader,
                            *const libc::c_void,
                        ) -> (),
                ),
                decoder: Some(
                    xattr_decoder
                        as unsafe extern "C" fn(
                            *mut tar_stat_info,
                            *const libc::c_char,
                            *const libc::c_char,
                            size_t,
                        ) -> (),
                ),
                flags: 0 as libc::c_int,
                prefix: 1 as libc::c_int != 0,
            };
            init
        },
        {
            let mut init = xhdr_tab {
                keyword: 0 as *const libc::c_char,
                coder: None,
                decoder: None,
                flags: 0 as libc::c_int,
                prefix: 0 as libc::c_int != 0,
            };
            init
        },
    ]
};
