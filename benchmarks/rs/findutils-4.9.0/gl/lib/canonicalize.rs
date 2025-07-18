use ::libc;
extern "C" {
    pub type hash_table;
    fn rpl_free(ptr: *mut libc::c_void);
    fn __errno_location() -> *mut libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
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
    fn rawmemchr(__s: *const libc::c_void, __c: libc::c_int) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn faccessat(
        __fd: libc::c_int,
        __file: *const libc::c_char,
        __type: libc::c_int,
        __flag: libc::c_int,
    ) -> libc::c_int;
    fn getcwd(__buf: *mut libc::c_char, __size: size_t) -> *mut libc::c_char;
    fn readlink(
        __path: *const libc::c_char,
        __buf: *mut libc::c_char,
        __len: size_t,
    ) -> ssize_t;
    fn gl_scratch_buffer_grow(buffer: *mut scratch_buffer) -> bool;
    fn gl_scratch_buffer_dupfree(
        buffer: *mut scratch_buffer,
        size: size_t,
    ) -> *mut libc::c_void;
    fn gl_scratch_buffer_grow_preserve(buffer: *mut scratch_buffer) -> bool;
    fn hash_free(table: *mut Hash_table);
    fn record_file(ht: *mut Hash_table, file: *const libc::c_char, stats: *const stat);
    fn seen_file(
        ht: *const Hash_table,
        file: *const libc::c_char,
        stats: *const stat,
    ) -> bool;
    fn hash_initialize(
        candidate: size_t,
        tuning: *const Hash_tuning,
        hasher: Hash_hasher,
        comparator: Hash_comparator,
        data_freer: Hash_data_freer,
    ) -> *mut Hash_table;
    fn triple_hash(x: *const libc::c_void, table_size: size_t) -> size_t;
    fn triple_compare_ino_str(x: *const libc::c_void, y: *const libc::c_void) -> bool;
    fn triple_free(x: *mut libc::c_void);
    fn xalloc_die();
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type ssize_t = __ssize_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type ptrdiff_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct max_align_t {
    pub __clang_max_align_nonce1: libc::c_longlong,
    pub __clang_max_align_nonce2: f128::f128,
}
pub type canonicalize_mode_t = libc::c_uint;
pub const CAN_NOLINKS: canonicalize_mode_t = 4;
pub const CAN_MISSING: canonicalize_mode_t = 2;
pub const CAN_ALL_BUT_LAST: canonicalize_mode_t = 1;
pub const CAN_EXISTING: canonicalize_mode_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scratch_buffer {
    pub data: *mut libc::c_void,
    pub length: size_t,
    pub __space: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __align: max_align_t,
    pub __c: [libc::c_char; 1024],
}
pub type Hash_table = hash_table;
pub type idx_t = ptrdiff_t;
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
pub type Hash_tuning = hash_tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_tuning {
    pub shrink_threshold: libc::c_float,
    pub shrink_factor: libc::c_float,
    pub growth_threshold: libc::c_float,
    pub growth_factor: libc::c_float,
    pub is_n_buckets: bool,
}
pub type Hash_data_freer = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type Hash_comparator = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> bool,
>;
pub type Hash_hasher = Option::<
    unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
>;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
#[inline]
unsafe extern "C" fn scratch_buffer_init(mut buffer: *mut scratch_buffer) {
    (*buffer).data = ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void;
    (*buffer).length = ::std::mem::size_of::<C2RustUnnamed>() as libc::c_ulong;
}
#[inline]
unsafe extern "C" fn scratch_buffer_free(mut buffer: *mut scratch_buffer) {
    if (*buffer).data != ((*buffer).__space.__c).as_mut_ptr() as *mut libc::c_void {
        rpl_free((*buffer).data);
    }
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_grow(mut buffer: *mut scratch_buffer) -> bool {
    return gl_scratch_buffer_grow(buffer) as libc::c_long != 0;
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_dupfree(
    mut buffer: *mut scratch_buffer,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut r: *mut libc::c_void = gl_scratch_buffer_dupfree(buffer, size);
    return if (r != 0 as *mut libc::c_void) as libc::c_int as libc::c_long != 0 {
        r
    } else {
        0 as *mut libc::c_void
    };
}
#[inline(always)]
unsafe extern "C" fn scratch_buffer_grow_preserve(
    mut buffer: *mut scratch_buffer,
) -> bool {
    return gl_scratch_buffer_grow_preserve(buffer) as libc::c_long != 0;
}
unsafe extern "C" fn file_accessible(mut file: *const libc::c_char) -> bool {
    return faccessat(-(100 as libc::c_int), file, 0 as libc::c_int, 0x200 as libc::c_int)
        == 0 as libc::c_int;
}
unsafe extern "C" fn suffix_requires_dir_check(mut end: *const libc::c_char) -> bool {
    while *end as libc::c_int == '/' as i32 {
        loop {
            end = end.offset(1);
            end;
            if !(*end as libc::c_int == '/' as i32) {
                break;
            }
        }
        let fresh0 = end;
        end = end.offset(1);
        match *fresh0 as libc::c_int {
            0 => return 1 as libc::c_int != 0,
            46 => {}
            _ => return 0 as libc::c_int != 0,
        }
        if *end == 0
            || *end as libc::c_int == '.' as i32
                && (*end.offset(1 as libc::c_int as isize) == 0
                    || *end.offset(1 as libc::c_int as isize) as libc::c_int
                        == '/' as i32)
        {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
static mut dir_suffix: [libc::c_char; 2] = unsafe {
    *::std::mem::transmute::<&[u8; 2], &[libc::c_char; 2]>(b"/\0")
};
unsafe extern "C" fn dir_check(
    mut dir: *mut libc::c_char,
    mut dirend: *mut libc::c_char,
) -> bool {
    strcpy(dirend, dir_suffix.as_ptr());
    return file_accessible(dir);
}
unsafe extern "C" fn multiple_bits_set(mut i: canonicalize_mode_t) -> bool {
    return i as libc::c_uint
        & (i as libc::c_uint).wrapping_sub(1 as libc::c_int as libc::c_uint)
        != 0 as libc::c_int as libc::c_uint;
}
unsafe extern "C" fn seen_triple(
    mut ht: *mut *mut Hash_table,
    mut filename: *const libc::c_char,
    mut st: *const stat,
) -> bool {
    if (*ht).is_null() {
        let mut initial_capacity: idx_t = 7 as libc::c_int as idx_t;
        *ht = hash_initialize(
            initial_capacity as size_t,
            0 as *const Hash_tuning,
            Some(
                triple_hash
                    as unsafe extern "C" fn(*const libc::c_void, size_t) -> size_t,
            ),
            Some(
                triple_compare_ino_str
                    as unsafe extern "C" fn(
                        *const libc::c_void,
                        *const libc::c_void,
                    ) -> bool,
            ),
            Some(triple_free as unsafe extern "C" fn(*mut libc::c_void) -> ()),
        );
        if (*ht).is_null() {
            xalloc_die();
        }
    }
    if seen_file(*ht, filename, st) {
        return 1 as libc::c_int != 0;
    }
    record_file(*ht, filename, st);
    return 0 as libc::c_int != 0;
}
unsafe extern "C" fn canonicalize_filename_mode_stk(
    mut name: *const libc::c_char,
    mut can_mode: canonicalize_mode_t,
    mut rname_buf: *mut scratch_buffer,
) -> *mut libc::c_char {
    let mut current_block: u64;
    let mut dest: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *const libc::c_char = 0 as *const libc::c_char;
    let mut end: *const libc::c_char = 0 as *const libc::c_char;
    let mut ht: *mut Hash_table = 0 as *mut Hash_table;
    let mut logical: bool = can_mode as libc::c_uint
        & CAN_NOLINKS as libc::c_int as libc::c_uint != 0 as libc::c_int as libc::c_uint;
    let mut num_links: libc::c_int = 0 as libc::c_int;
    let mut can_exist: canonicalize_mode_t = (can_mode as libc::c_uint
        & (CAN_EXISTING as libc::c_int | CAN_ALL_BUT_LAST as libc::c_int
            | CAN_MISSING as libc::c_int) as libc::c_uint) as canonicalize_mode_t;
    if multiple_bits_set(can_exist) {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    if name.is_null() {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    if *name.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        *__errno_location() = 2 as libc::c_int;
        return 0 as *mut libc::c_char;
    }
    let mut extra_buffer: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    let mut link_buffer: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    scratch_buffer_init(&mut extra_buffer);
    scratch_buffer_init(&mut link_buffer);
    scratch_buffer_init(rname_buf);
    let mut rname_on_stack: *mut libc::c_char = (*rname_buf).data as *mut libc::c_char;
    let mut rname: *mut libc::c_char = rname_on_stack;
    let mut end_in_extra_buffer: bool = 0 as libc::c_int != 0;
    let mut failed: bool = 1 as libc::c_int != 0;
    let mut prefix_len: idx_t = 0 as libc::c_int as idx_t;
    if !(*name.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32) {
        loop {
            if !(getcwd(rname, (*rname_buf).length)).is_null() {
                current_block = 14401909646449704462;
                break;
            }
            match *__errno_location() {
                34 => {
                    if scratch_buffer_grow(rname_buf) {
                        rname = (*rname_buf).data as *mut libc::c_char;
                        continue;
                    } else {
                        current_block = 8153372852136612846;
                    }
                }
                12 => {
                    current_block = 8153372852136612846;
                }
                _ => {
                    current_block = 10545010776557921714;
                }
            }
            match current_block {
                8153372852136612846 => {
                    xalloc_die();
                }
                _ => {}
            }
            dest = rname;
            current_block = 5438892885942363852;
            break;
        }
        match current_block {
            5438892885942363852 => {}
            _ => {
                dest = rawmemchr(rname as *const libc::c_void, '\0' as i32)
                    as *mut libc::c_char;
                start = name;
                prefix_len = 0 as libc::c_int as idx_t;
                current_block = 8704759739624374314;
            }
        }
    } else {
        dest = mempcpy(
            rname as *mut libc::c_void,
            name as *const libc::c_void,
            prefix_len as libc::c_ulong,
        ) as *mut libc::c_char;
        let fresh1 = dest;
        dest = dest.offset(1);
        *fresh1 = '/' as i32 as libc::c_char;
        start = name.offset(prefix_len as isize);
        current_block = 8704759739624374314;
    }
    loop {
        match current_block {
            5438892885942363852 => {
                if !ht.is_null() {
                    hash_free(ht);
                }
                break;
            }
            _ => {
                if *start != 0 {
                    while *start as libc::c_int == '/' as i32 {
                        start = start.offset(1);
                        start;
                    }
                    end = start;
                    while *end as libc::c_int != 0
                        && !(*end as libc::c_int == '/' as i32)
                    {
                        end = end.offset(1);
                        end;
                    }
                    let mut startlen: idx_t = end.offset_from(start) as libc::c_long;
                    if !(startlen == 0 as libc::c_int as libc::c_long) {
                        if !(startlen == 1 as libc::c_int as libc::c_long
                            && *start.offset(0 as libc::c_int as isize) as libc::c_int
                                == '.' as i32)
                        {
                            if startlen == 2 as libc::c_int as libc::c_long
                                && *start.offset(0 as libc::c_int as isize) as libc::c_int
                                    == '.' as i32
                                && *start.offset(1 as libc::c_int as isize) as libc::c_int
                                    == '.' as i32
                            {
                                if dest
                                    > rname
                                        .offset(prefix_len as isize)
                                        .offset(1 as libc::c_int as isize)
                                {
                                    dest = dest.offset(-1);
                                    dest;
                                    while dest > rname
                                        && !(*dest.offset(-(1 as libc::c_int) as isize)
                                            as libc::c_int == '/' as i32)
                                    {
                                        dest = dest.offset(-1);
                                        dest;
                                    }
                                }
                                if 0 as libc::c_int != 0
                                    && dest == rname.offset(1 as libc::c_int as isize)
                                    && prefix_len == 0 && *dest as libc::c_int == '/' as i32
                                    && !(*dest.offset(1 as libc::c_int as isize) as libc::c_int
                                        == '/' as i32)
                                {
                                    dest = dest.offset(1);
                                    dest;
                                }
                            } else {
                                if !(*dest.offset(-(1 as libc::c_int) as isize)
                                    as libc::c_int == '/' as i32)
                                {
                                    let fresh3 = dest;
                                    dest = dest.offset(1);
                                    *fresh3 = '/' as i32 as libc::c_char;
                                }
                                while (rname
                                    .offset((*rname_buf).length as isize)
                                    .offset_from(dest) as libc::c_long as libc::c_ulong)
                                    < (startlen as libc::c_ulong)
                                        .wrapping_add(
                                            ::std::mem::size_of::<[libc::c_char; 2]>() as libc::c_ulong,
                                        )
                                {
                                    let mut dest_offset: idx_t = dest.offset_from(rname)
                                        as libc::c_long;
                                    if !scratch_buffer_grow_preserve(rname_buf) {
                                        xalloc_die();
                                    }
                                    rname = (*rname_buf).data as *mut libc::c_char;
                                    dest = rname.offset(dest_offset as isize);
                                }
                                dest = mempcpy(
                                    dest as *mut libc::c_void,
                                    start as *const libc::c_void,
                                    startlen as libc::c_ulong,
                                ) as *mut libc::c_char;
                                *dest = '\0' as i32 as libc::c_char;
                                let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
                                let mut n: ssize_t = -(1 as libc::c_int) as ssize_t;
                                if !logical {
                                    loop {
                                        buf = link_buffer.data as *mut libc::c_char;
                                        let mut bufsize: idx_t = link_buffer.length as idx_t;
                                        n = readlink(
                                            rname,
                                            buf,
                                            (bufsize - 1 as libc::c_int as libc::c_long) as size_t,
                                        );
                                        if n < bufsize - 1 as libc::c_int as libc::c_long {
                                            break;
                                        }
                                        if !scratch_buffer_grow(&mut link_buffer) {
                                            xalloc_die();
                                        }
                                    }
                                }
                                if 0 as libc::c_int as libc::c_long <= n {
                                    if num_links < 20 as libc::c_int {
                                        num_links += 1;
                                        num_links;
                                        current_block = 9437375157805982253;
                                    } else if *start != 0 {
                                        let mut st: stat = stat {
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
                                        *dest
                                            .offset(-startlen as isize) = '\0' as i32 as libc::c_char;
                                        if stat(
                                            if *rname as libc::c_int != 0 {
                                                rname as *const libc::c_char
                                            } else {
                                                b".\0" as *const u8 as *const libc::c_char
                                            },
                                            &mut st,
                                        ) != 0 as libc::c_int
                                        {
                                            current_block = 5438892885942363852;
                                            continue;
                                        }
                                        *dest.offset(-startlen as isize) = *start;
                                        if seen_triple(&mut ht, start, &mut st) {
                                            if can_exist as libc::c_uint
                                                == CAN_MISSING as libc::c_int as libc::c_uint
                                            {
                                                current_block = 1118134448028020070;
                                            } else {
                                                *__errno_location() = 40 as libc::c_int;
                                                current_block = 5438892885942363852;
                                                continue;
                                            }
                                        } else {
                                            current_block = 9437375157805982253;
                                        }
                                    } else {
                                        current_block = 9437375157805982253;
                                    }
                                    match current_block {
                                        1118134448028020070 => {}
                                        _ => {
                                            *buf.offset(n as isize) = '\0' as i32 as libc::c_char;
                                            let mut extra_buf: *mut libc::c_char = extra_buffer.data
                                                as *mut libc::c_char;
                                            let mut end_idx: idx_t = 0;
                                            if end_in_extra_buffer {
                                                end_idx = end.offset_from(extra_buf) as libc::c_long;
                                            }
                                            let mut len: size_t = strlen(end);
                                            if if (if (if 1 as libc::c_int != 0 {
                                                0 as libc::c_int as libc::c_ulong
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    len
                                                })
                                                    .wrapping_add(n as libc::c_ulong)
                                            })
                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                < 0 as libc::c_int as libc::c_ulong
                                            {
                                                !((if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        len
                                                    })
                                                        .wrapping_add(n as libc::c_ulong)
                                                })
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                        .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                        .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                    .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                            } else {
                                                (if 1 as libc::c_int != 0 {
                                                    0 as libc::c_int as libc::c_ulong
                                                } else {
                                                    (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        len
                                                    })
                                                        .wrapping_add(n as libc::c_ulong)
                                                })
                                                    .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                            }) < 0 as libc::c_int as libc::c_ulong
                                            {
                                                if n < 0 as libc::c_int as libc::c_long {
                                                    (len
                                                        < (if (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                len
                                                            })
                                                                .wrapping_add(n as libc::c_ulong)
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            < 0 as libc::c_int as libc::c_ulong
                                                        {
                                                            !((if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    len
                                                                })
                                                                    .wrapping_add(n as libc::c_ulong)
                                                            })
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                                << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                    .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                    .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                                .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                                .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                (if 1 as libc::c_int != 0 {
                                                                    0 as libc::c_int as libc::c_ulong
                                                                } else {
                                                                    len
                                                                })
                                                                    .wrapping_add(n as libc::c_ulong)
                                                            })
                                                                .wrapping_add(0 as libc::c_int as libc::c_ulong)
                                                        })
                                                            .wrapping_sub(n as libc::c_ulong)) as libc::c_int
                                                } else {
                                                    ((if (if 1 as libc::c_int != 0 {
                                                        0 as libc::c_int as libc::c_ulong
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            len
                                                        })
                                                            .wrapping_add(n as libc::c_ulong)
                                                    })
                                                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                        < 0 as libc::c_int as libc::c_ulong
                                                    {
                                                        ((if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                len
                                                            })
                                                                .wrapping_add(n as libc::c_ulong)
                                                        })
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                            << (::std::mem::size_of::<libc::c_ulong>() as libc::c_ulong)
                                                                .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                                                .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                            .wrapping_mul(2 as libc::c_int as libc::c_ulong)
                                                            .wrapping_add(1 as libc::c_int as libc::c_ulong)
                                                    } else {
                                                        (if 1 as libc::c_int != 0 {
                                                            0 as libc::c_int as libc::c_ulong
                                                        } else {
                                                            (if 1 as libc::c_int != 0 {
                                                                0 as libc::c_int as libc::c_ulong
                                                            } else {
                                                                len
                                                            })
                                                                .wrapping_add(n as libc::c_ulong)
                                                        })
                                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                                    })
                                                        .wrapping_sub(n as libc::c_ulong) < len) as libc::c_int
                                                }
                                            } else if len < 0 as libc::c_int as libc::c_ulong {
                                                (n as libc::c_ulong <= len.wrapping_add(n as libc::c_ulong))
                                                    as libc::c_int
                                            } else if n < 0 as libc::c_int as libc::c_long {
                                                (len <= len.wrapping_add(n as libc::c_ulong)) as libc::c_int
                                            } else {
                                                (len.wrapping_add(n as libc::c_ulong) < n as libc::c_ulong)
                                                    as libc::c_int
                                            } != 0
                                            {
                                                xalloc_die();
                                            }
                                            while extra_buffer.length
                                                <= len.wrapping_add(n as libc::c_ulong)
                                            {
                                                if !scratch_buffer_grow_preserve(&mut extra_buffer) {
                                                    xalloc_die();
                                                }
                                                extra_buf = extra_buffer.data as *mut libc::c_char;
                                            }
                                            if end_in_extra_buffer {
                                                end = extra_buf.offset(end_idx as isize);
                                            }
                                            memmove(
                                                &mut *extra_buf.offset(n as isize) as *mut libc::c_char
                                                    as *mut libc::c_void,
                                                end as *const libc::c_void,
                                                len.wrapping_add(1 as libc::c_int as libc::c_ulong),
                                            );
                                            end = memcpy(
                                                extra_buf as *mut libc::c_void,
                                                buf as *const libc::c_void,
                                                n as libc::c_ulong,
                                            ) as *const libc::c_char;
                                            name = end;
                                            end_in_extra_buffer = 1 as libc::c_int != 0;
                                            if *buf.offset(0 as libc::c_int as isize) as libc::c_int
                                                == '/' as i32
                                            {
                                                let mut pfxlen: idx_t = 0 as libc::c_int as idx_t;
                                                dest = mempcpy(
                                                    rname as *mut libc::c_void,
                                                    buf as *const libc::c_void,
                                                    pfxlen as libc::c_ulong,
                                                ) as *mut libc::c_char;
                                                let fresh4 = dest;
                                                dest = dest.offset(1);
                                                *fresh4 = '/' as i32 as libc::c_char;
                                                prefix_len = pfxlen;
                                            } else {
                                                if dest
                                                    > rname
                                                        .offset(prefix_len as isize)
                                                        .offset(1 as libc::c_int as isize)
                                                {
                                                    dest = dest.offset(-1);
                                                    dest;
                                                    while dest > rname
                                                        && !(*dest.offset(-(1 as libc::c_int) as isize)
                                                            as libc::c_int == '/' as i32)
                                                    {
                                                        dest = dest.offset(-1);
                                                        dest;
                                                    }
                                                }
                                                if 0 as libc::c_int != 0
                                                    && dest == rname.offset(1 as libc::c_int as isize)
                                                    && *dest as libc::c_int == '/' as i32
                                                    && !(*dest.offset(1 as libc::c_int as isize) as libc::c_int
                                                        == '/' as i32) && prefix_len == 0
                                                {
                                                    dest = dest.offset(1);
                                                    dest;
                                                }
                                            }
                                        }
                                    }
                                } else if !(can_exist as libc::c_uint
                                    == CAN_MISSING as libc::c_int as libc::c_uint
                                    || (if suffix_requires_dir_check(end) as libc::c_int != 0 {
                                        dir_check(rname, dest) as libc::c_int
                                    } else {
                                        if !logical {
                                            (*__errno_location() == 22 as libc::c_int) as libc::c_int
                                        } else {
                                            (*end as libc::c_int != 0
                                                || file_accessible(rname) as libc::c_int != 0)
                                                as libc::c_int
                                        }
                                    }) != 0
                                    || can_exist as libc::c_uint
                                        == CAN_ALL_BUT_LAST as libc::c_int as libc::c_uint
                                        && *__errno_location() == 2 as libc::c_int
                                        && *end
                                            .offset(
                                                strspn(end, b"/\0" as *const u8 as *const libc::c_char)
                                                    as isize,
                                            ) == 0)
                                {
                                    current_block = 5438892885942363852;
                                    continue;
                                }
                            }
                        }
                        start = end;
                        current_block = 8704759739624374314;
                        continue;
                    }
                }
                if dest
                    > rname.offset(prefix_len as isize).offset(1 as libc::c_int as isize)
                    && *dest.offset(-(1 as libc::c_int) as isize) as libc::c_int
                        == '/' as i32
                {
                    dest = dest.offset(-1);
                    dest;
                }
                if 0 as libc::c_int != 0
                    && dest == rname.offset(1 as libc::c_int as isize) && prefix_len == 0
                    && *dest as libc::c_int == '/' as i32
                    && !(*dest.offset(1 as libc::c_int as isize) as libc::c_int
                        == '/' as i32)
                {
                    dest = dest.offset(1);
                    dest;
                }
                failed = 0 as libc::c_int != 0;
                current_block = 5438892885942363852;
            }
        }
    }
    scratch_buffer_free(&mut extra_buffer);
    scratch_buffer_free(&mut link_buffer);
    if failed {
        scratch_buffer_free(rname_buf);
        return 0 as *mut libc::c_char;
    }
    let fresh6 = dest;
    dest = dest.offset(1);
    *fresh6 = '\0' as i32 as libc::c_char;
    let mut result: *mut libc::c_char = scratch_buffer_dupfree(
        rname_buf,
        dest.offset_from(rname) as libc::c_long as size_t,
    ) as *mut libc::c_char;
    if result.is_null() {
        xalloc_die();
    }
    return result;
}
pub unsafe extern "C" fn canonicalize_filename_mode(
    mut name: *const libc::c_char,
    mut can_mode: canonicalize_mode_t,
) -> *mut libc::c_char {
    let mut rname_buffer: scratch_buffer = scratch_buffer {
        data: 0 as *mut libc::c_void,
        length: 0,
        __space: C2RustUnnamed {
            __align: max_align_t {
                __clang_max_align_nonce1: 0,
                __clang_max_align_nonce2: f128::f128::ZERO,
            },
        },
    };
    return canonicalize_filename_mode_stk(name, can_mode, &mut rname_buffer);
}
