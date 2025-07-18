use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type dep;
    pub type commands;
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fputs(__s: *const libc::c_char, __stream: *mut FILE) -> libc::c_int;
    fn puts(__s: *const libc::c_char) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
    fn free(__ptr: *mut libc::c_void);
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
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn mempcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xrealloc(_: *mut libc::c_void, _: size_t) -> *mut libc::c_void;
    fn find_percent(_: *mut libc::c_char) -> *mut libc::c_char;
    fn dir_file_exists_p(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dir_name(_: *const libc::c_char) -> *const libc::c_char;
    fn strcache_add(str: *const libc::c_char) -> *const libc::c_char;
    static mut stopchar_map: [libc::c_ushort; 0];
    fn strcache_add_len(str: *const libc::c_char, len: size_t) -> *const libc::c_char;
    fn lookup_file(name: *const libc::c_char) -> *mut file;
    fn file_timestamp_cons(
        _: *const libc::c_char,
        _: time_t,
        _: libc::c_long,
    ) -> uintmax_t;
    fn variable_expand(line: *const libc::c_char) -> *mut libc::c_char;
    fn pattern_matches(
        pattern: *const libc::c_char,
        percent: *const libc::c_char,
        str: *const libc::c_char,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type time_t = __time_t;
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
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct file {
    pub name: *const libc::c_char,
    pub hname: *const libc::c_char,
    pub vpath: *const libc::c_char,
    pub deps: *mut dep,
    pub cmds: *mut commands,
    pub stem: *const libc::c_char,
    pub also_make: *mut dep,
    pub prev: *mut file,
    pub last: *mut file,
    pub renamed: *mut file,
    pub variables: *mut variable_set_list,
    pub pat_variables: *mut variable_set_list,
    pub parent: *mut file,
    pub double_colon: *mut file,
    pub last_mtime: uintmax_t,
    pub mtime_before_update: uintmax_t,
    pub considered: libc::c_uint,
    pub command_flags: libc::c_int,
    #[bitfield(name = "update_status", ty = "update_status", bits = "0..=1")]
    #[bitfield(name = "command_state", ty = "cmd_state", bits = "2..=3")]
    #[bitfield(name = "builtin", ty = "libc::c_uint", bits = "4..=4")]
    #[bitfield(name = "precious", ty = "libc::c_uint", bits = "5..=5")]
    #[bitfield(name = "loaded", ty = "libc::c_uint", bits = "6..=6")]
    #[bitfield(name = "unloaded", ty = "libc::c_uint", bits = "7..=7")]
    #[bitfield(name = "low_resolution_time", ty = "libc::c_uint", bits = "8..=8")]
    #[bitfield(name = "tried_implicit", ty = "libc::c_uint", bits = "9..=9")]
    #[bitfield(name = "updating", ty = "libc::c_uint", bits = "10..=10")]
    #[bitfield(name = "updated", ty = "libc::c_uint", bits = "11..=11")]
    #[bitfield(name = "is_target", ty = "libc::c_uint", bits = "12..=12")]
    #[bitfield(name = "cmd_target", ty = "libc::c_uint", bits = "13..=13")]
    #[bitfield(name = "phony", ty = "libc::c_uint", bits = "14..=14")]
    #[bitfield(name = "intermediate", ty = "libc::c_uint", bits = "15..=15")]
    #[bitfield(name = "is_explicit", ty = "libc::c_uint", bits = "16..=16")]
    #[bitfield(name = "secondary", ty = "libc::c_uint", bits = "17..=17")]
    #[bitfield(name = "notintermediate", ty = "libc::c_uint", bits = "18..=18")]
    #[bitfield(name = "dontcare", ty = "libc::c_uint", bits = "19..=19")]
    #[bitfield(name = "ignore_vpath", ty = "libc::c_uint", bits = "20..=20")]
    #[bitfield(name = "pat_searched", ty = "libc::c_uint", bits = "21..=21")]
    #[bitfield(name = "no_diag", ty = "libc::c_uint", bits = "22..=22")]
    #[bitfield(name = "was_shuffled", ty = "libc::c_uint", bits = "23..=23")]
    #[bitfield(name = "snapped", ty = "libc::c_uint", bits = "24..=24")]
    pub update_status_command_state_builtin_precious_loaded_unloaded_low_resolution_time_tried_implicit_updating_updated_is_target_cmd_target_phony_intermediate_is_explicit_secondary_notintermediate_dontcare_ignore_vpath_pat_searched_no_diag_was_shuffled_snapped: [u8; 4],
    #[bitfield(padding)]
    pub c2rust_padding: [u8; 4],
}
pub type cmd_state = libc::c_uint;
pub const cs_finished: cmd_state = 3;
pub const cs_running: cmd_state = 2;
pub const cs_deps_running: cmd_state = 1;
pub const cs_not_started: cmd_state = 0;
pub type update_status = libc::c_uint;
pub const us_failed: update_status = 3;
pub const us_question: update_status = 2;
pub const us_none: update_status = 1;
pub const us_success: update_status = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set_list {
    pub next: *mut variable_set_list,
    pub set: *mut variable_set,
    pub next_is_parent: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct variable_set {
    pub table: hash_table,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct hash_table {
    pub ht_vec: *mut *mut libc::c_void,
    pub ht_hash_1: hash_func_t,
    pub ht_hash_2: hash_func_t,
    pub ht_compare: hash_cmp_func_t,
    pub ht_size: libc::c_ulong,
    pub ht_capacity: libc::c_ulong,
    pub ht_fill: libc::c_ulong,
    pub ht_empty_slots: libc::c_ulong,
    pub ht_collisions: libc::c_ulong,
    pub ht_lookups: libc::c_ulong,
    pub ht_rehashes: libc::c_uint,
}
pub type hash_cmp_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type hash_func_t = Option::<
    unsafe extern "C" fn(*const libc::c_void) -> libc::c_ulong,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct vpath {
    pub next: *mut vpath,
    pub pattern: *const libc::c_char,
    pub percent: *const libc::c_char,
    pub patlen: size_t,
    pub searchpath: *mut *const libc::c_char,
    pub maxlen: size_t,
}
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
static mut vpaths: *mut vpath = 0 as *const vpath as *mut vpath;
static mut general_vpath: *mut vpath = 0 as *const vpath as *mut vpath;
static mut gpaths: *mut vpath = 0 as *const vpath as *mut vpath;
pub unsafe extern "C" fn build_vpath_lists() {
    let mut new: *mut vpath = 0 as *mut vpath;
    let mut old: *mut vpath = 0 as *mut vpath;
    let mut nexto: *mut vpath = 0 as *mut vpath;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    old = vpaths;
    while !old.is_null() {
        nexto = (*old).next;
        (*old).next = new;
        new = old;
        old = nexto;
    }
    vpaths = new;
    p = variable_expand(b"$(strip $(VPATH))\0" as *const u8 as *const libc::c_char);
    if *p as libc::c_int != '\0' as i32 {
        let mut save_vpaths: *mut vpath = vpaths;
        let mut gp: [libc::c_char; 2] = *::std::mem::transmute::<
            &[u8; 2],
            &mut [libc::c_char; 2],
        >(b"%\0");
        vpaths = 0 as *mut vpath;
        construct_vpath_list(gp.as_mut_ptr(), p);
        general_vpath = vpaths;
        vpaths = save_vpaths;
    }
    p = variable_expand(b"$(strip $(GPATH))\0" as *const u8 as *const libc::c_char);
    if *p as libc::c_int != '\0' as i32 {
        let mut save_vpaths_0: *mut vpath = vpaths;
        let mut gp_0: [libc::c_char; 2] = *::std::mem::transmute::<
            &[u8; 2],
            &mut [libc::c_char; 2],
        >(b"%\0");
        vpaths = 0 as *mut vpath;
        construct_vpath_list(gp_0.as_mut_ptr(), p);
        gpaths = vpaths;
        vpaths = save_vpaths_0;
    }
}
pub unsafe extern "C" fn construct_vpath_list(
    mut pattern: *mut libc::c_char,
    mut dirpath: *mut libc::c_char,
) {
    let mut elem: libc::c_uint = 0;
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut vpath: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut maxvpath: size_t = 0;
    let mut maxelem: libc::c_uint = 0;
    let mut percent: *const libc::c_char = 0 as *const libc::c_char;
    if !pattern.is_null() {
        percent = find_percent(pattern);
    }
    if dirpath.is_null() {
        let mut path: *mut vpath = 0 as *mut vpath;
        let mut lastpath: *mut vpath = 0 as *mut vpath;
        lastpath = 0 as *mut vpath;
        path = vpaths;
        while !path.is_null() {
            let mut next: *mut vpath = (*path).next;
            if pattern.is_null()
                || (percent.is_null() && ((*path).percent).is_null()
                    || percent.offset_from(pattern) as libc::c_long
                        == ((*path).percent).offset_from((*path).pattern)
                            as libc::c_long)
                    && (pattern == (*path).pattern as *mut libc::c_char
                        || *pattern as libc::c_int == *(*path).pattern as libc::c_int
                            && (*pattern as libc::c_int == '\0' as i32
                                || strcmp(
                                    pattern.offset(1 as libc::c_int as isize),
                                    ((*path).pattern).offset(1 as libc::c_int as isize),
                                ) == 0))
            {
                if lastpath.is_null() {
                    vpaths = (*path).next;
                } else {
                    (*lastpath).next = next;
                }
                free((*path).searchpath as *mut libc::c_void);
                free(path as *mut libc::c_void);
            } else {
                lastpath = path;
            }
            path = next;
        }
        return;
    }
    while *stopchar_map.as_mut_ptr().offset(*dirpath as libc::c_uchar as isize)
        as libc::c_int & (0x2 as libc::c_int | 0x40 as libc::c_int) != 0 as libc::c_int
    {
        dirpath = dirpath.offset(1);
        dirpath;
    }
    maxelem = 2 as libc::c_int as libc::c_uint;
    p = dirpath;
    while *p as libc::c_int != '\0' as i32 {
        let fresh0 = p;
        p = p.offset(1);
        if *stopchar_map.as_mut_ptr().offset(*fresh0 as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x40 as libc::c_int)
            != 0 as libc::c_int
        {
            maxelem = maxelem.wrapping_add(1);
            maxelem;
        }
    }
    vpath = xmalloc(
        (maxelem as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong),
    ) as *mut *const libc::c_char;
    maxvpath = 0 as libc::c_int as size_t;
    elem = 0 as libc::c_int as libc::c_uint;
    p = dirpath;
    while *p as libc::c_int != '\0' as i32 {
        let mut v: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut len: size_t = 0;
        v = p;
        while *p as libc::c_int != '\0' as i32 && *p as libc::c_int != ':' as i32
            && !(*stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
                as libc::c_int & 0x2 as libc::c_int != 0 as libc::c_int)
        {
            p = p.offset(1);
            p;
        }
        len = p.offset_from(v) as libc::c_long as size_t;
        if len > 1 as libc::c_int as libc::c_ulong
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int == '/' as i32
        {
            len = len.wrapping_sub(1);
            len;
        }
        if len > 1 as libc::c_int as libc::c_ulong || *v as libc::c_int != '.' as i32 {
            let fresh1 = elem;
            elem = elem.wrapping_add(1);
            let ref mut fresh2 = *vpath.offset(fresh1 as isize);
            *fresh2 = dir_name(strcache_add_len(v, len));
            if len > maxvpath {
                maxvpath = len;
            }
        }
        while *stopchar_map.as_mut_ptr().offset(*p as libc::c_uchar as isize)
            as libc::c_int & (0x2 as libc::c_int | 0x40 as libc::c_int)
            != 0 as libc::c_int
        {
            p = p.offset(1);
            p;
        }
    }
    if elem > 0 as libc::c_int as libc::c_uint {
        let mut path_0: *mut vpath = 0 as *mut vpath;
        if elem < maxelem.wrapping_sub(1 as libc::c_int as libc::c_uint) {
            vpath = xrealloc(
                vpath as *mut libc::c_void,
                (elem.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_ulong)
                    .wrapping_mul(
                        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                    ),
            ) as *mut *const libc::c_char;
        }
        let ref mut fresh3 = *vpath.offset(elem as isize);
        *fresh3 = 0 as *const libc::c_char;
        path_0 = xmalloc(::std::mem::size_of::<vpath>() as libc::c_ulong) as *mut vpath;
        (*path_0).searchpath = vpath;
        (*path_0).maxlen = maxvpath;
        (*path_0).next = vpaths;
        vpaths = path_0;
        (*path_0).pattern = strcache_add(pattern);
        (*path_0).patlen = strlen(pattern);
        (*path_0)
            .percent = if !percent.is_null() {
            ((*path_0).pattern)
                .offset(percent.offset_from(pattern) as libc::c_long as isize)
        } else {
            0 as *const libc::c_char
        };
    } else {
        free(vpath as *mut libc::c_void);
    };
}
pub unsafe extern "C" fn gpath_search(
    mut file: *const libc::c_char,
    mut len: size_t,
) -> libc::c_int {
    if !gpaths.is_null() && len <= (*gpaths).maxlen {
        let mut gp: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
        gp = (*gpaths).searchpath;
        while !(*gp).is_null() {
            if strncmp(*gp, file, len) == 0 as libc::c_int
                && *(*gp).offset(len as isize) as libc::c_int == '\0' as i32
            {
                return 1 as libc::c_int;
            }
            gp = gp.offset(1);
            gp;
        }
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn selective_vpath_search(
    mut path: *mut vpath,
    mut file: *const libc::c_char,
    mut mtime_ptr: *mut uintmax_t,
    mut path_index: *mut libc::c_uint,
) -> *const libc::c_char {
    let mut not_target: libc::c_int = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut n: *const libc::c_char = 0 as *const libc::c_char;
    let mut filename: *const libc::c_char = 0 as *const libc::c_char;
    let mut vpath: *mut *const libc::c_char = (*path).searchpath;
    let mut maxvpath: size_t = (*path).maxlen;
    let mut i: libc::c_uint = 0;
    let mut flen: size_t = 0;
    let mut name_dplen: size_t = 0;
    let mut exists: libc::c_int = 0 as libc::c_int;
    let mut f: *mut file = lookup_file(file);
    not_target = (f.is_null() || (*f).is_target() == 0) as libc::c_int;
    flen = strlen(file);
    n = strrchr(file, '/' as i32);
    name_dplen = (if !n.is_null() {
        n.offset_from(file) as libc::c_long
    } else {
        0 as libc::c_int as libc::c_long
    }) as size_t;
    filename = if name_dplen > 0 as libc::c_int as libc::c_ulong {
        n.offset(1 as libc::c_int as isize)
    } else {
        file
    };
    if name_dplen > 0 as libc::c_int as libc::c_ulong {
        flen = (flen as libc::c_ulong)
            .wrapping_sub(name_dplen.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as size_t as size_t;
    }
    let mut fresh4 = ::std::vec::from_elem(
        0,
        maxvpath
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(name_dplen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong)
            .wrapping_add(flen)
            .wrapping_add(1 as libc::c_int as libc::c_ulong) as usize,
    );
    name = fresh4.leak().as_mut_ptr() as *mut libc::c_char;
    let mut current_block_45: u64;
    i = 0 as libc::c_int as libc::c_uint;
    while !(*vpath.offset(i as isize)).is_null() {
        let mut exists_in_cache: libc::c_int = 0 as libc::c_int;
        let mut p: *mut libc::c_char = name;
        let mut vlen: size_t = strlen(*vpath.offset(i as isize));
        p = mempcpy(
            p as *mut libc::c_void,
            *vpath.offset(i as isize) as *const libc::c_void,
            vlen,
        ) as *mut libc::c_char;
        if name_dplen > 0 as libc::c_int as libc::c_ulong {
            let fresh5 = p;
            p = p.offset(1);
            *fresh5 = '/' as i32 as libc::c_char;
            p = mempcpy(p as *mut libc::c_void, file as *const libc::c_void, name_dplen)
                as *mut libc::c_char;
        }
        if p != name
            && *p.offset(-(1 as libc::c_int) as isize) as libc::c_int != '/' as i32
        {
            *p = '/' as i32 as libc::c_char;
            memcpy(
                p.offset(1 as libc::c_int as isize) as *mut libc::c_void,
                filename as *const libc::c_void,
                flen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        } else {
            memcpy(
                p as *mut libc::c_void,
                filename as *const libc::c_void,
                flen.wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        let mut f_0: *mut file = lookup_file(name);
        if !f_0.is_null() {
            exists = (not_target != 0 || (*f_0).is_target() as libc::c_int != 0)
                as libc::c_int;
            if exists != 0 && !mtime_ptr.is_null()
                && ((*f_0).last_mtime == 2 as libc::c_int as libc::c_ulong
                    || (*f_0).last_mtime
                        == (!(0 as libc::c_int as uintmax_t))
                            .wrapping_sub(
                                (if !(-(1 as libc::c_int) as uintmax_t
                                    <= 0 as libc::c_int as libc::c_ulong)
                                {
                                    0 as libc::c_int as uintmax_t
                                } else {
                                    !(0 as libc::c_int as uintmax_t)
                                        << (::std::mem::size_of::<uintmax_t>() as libc::c_ulong)
                                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                                            .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                }),
                            ))
            {
                *mtime_ptr = (*f_0).last_mtime;
                mtime_ptr = 0 as *mut uintmax_t;
            }
        }
        if exists == 0 {
            *p = '\0' as i32 as libc::c_char;
            exists = dir_file_exists_p(name, filename);
            exists_in_cache = exists;
        }
        if exists != 0 {
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
            *p = '/' as i32 as libc::c_char;
            if exists_in_cache != 0 {
                let mut e: libc::c_int = 0;
                loop {
                    e = stat(name, &mut st);
                    if !(e == -(1 as libc::c_int)
                        && *__errno_location() == 4 as libc::c_int)
                    {
                        break;
                    }
                }
                if e != 0 as libc::c_int {
                    exists = 0 as libc::c_int;
                    current_block_45 = 2868539653012386629;
                } else {
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = file_timestamp_cons(
                            name,
                            st.st_mtim.tv_sec,
                            st.st_mtim.tv_nsec,
                        );
                        mtime_ptr = 0 as *mut uintmax_t;
                    }
                    current_block_45 = 7427571413727699167;
                }
            } else {
                current_block_45 = 7427571413727699167;
            }
            match current_block_45 {
                2868539653012386629 => {}
                _ => {
                    if !mtime_ptr.is_null() {
                        *mtime_ptr = 0 as libc::c_int as uintmax_t;
                    }
                    if !path_index.is_null() {
                        *path_index = i;
                    }
                    return strcache_add_len(
                        name,
                        (p.offset(1 as libc::c_int as isize).offset_from(name)
                            as libc::c_long as libc::c_ulong)
                            .wrapping_add(flen),
                    );
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn vpath_search(
    mut file: *const libc::c_char,
    mut mtime_ptr: *mut uintmax_t,
    mut vpath_index: *mut libc::c_uint,
    mut path_index: *mut libc::c_uint,
) -> *const libc::c_char {
    let mut v: *mut vpath = 0 as *mut vpath;
    if *file.offset(0 as libc::c_int as isize) as libc::c_int == '/' as i32
        || vpaths.is_null() && general_vpath.is_null()
    {
        return 0 as *const libc::c_char;
    }
    if !vpath_index.is_null() {
        *vpath_index = 0 as libc::c_int as libc::c_uint;
        *path_index = 0 as libc::c_int as libc::c_uint;
    }
    v = vpaths;
    while !v.is_null() {
        if pattern_matches((*v).pattern, (*v).percent, file) != 0 {
            let mut p: *const libc::c_char = selective_vpath_search(
                v,
                file,
                mtime_ptr,
                path_index,
            );
            if !p.is_null() {
                return p;
            }
        }
        if !vpath_index.is_null() {
            *vpath_index = (*vpath_index).wrapping_add(1);
            *vpath_index;
        }
        v = (*v).next;
    }
    if !general_vpath.is_null() {
        let mut p_0: *const libc::c_char = selective_vpath_search(
            general_vpath,
            file,
            mtime_ptr,
            path_index,
        );
        if !p_0.is_null() {
            return p_0;
        }
    }
    return 0 as *const libc::c_char;
}
pub unsafe extern "C" fn print_vpath_data_base() {
    let mut nvpaths: libc::c_uint = 0;
    let mut v: *mut vpath = 0 as *mut vpath;
    puts(
        dcgettext(
            0 as *const libc::c_char,
            b"\n# VPATH Search Paths\n\0" as *const u8 as *const libc::c_char,
            5 as libc::c_int,
        ),
    );
    nvpaths = 0 as libc::c_int as libc::c_uint;
    v = vpaths;
    while !v.is_null() {
        let mut i: libc::c_uint = 0;
        nvpaths = nvpaths.wrapping_add(1);
        nvpaths;
        printf(b"vpath %s \0" as *const u8 as *const libc::c_char, (*v).pattern);
        i = 0 as libc::c_int as libc::c_uint;
        while !(*((*v).searchpath).offset(i as isize)).is_null() {
            printf(
                b"%s%c\0" as *const u8 as *const libc::c_char,
                *((*v).searchpath).offset(i as isize),
                if (*((*v).searchpath)
                    .offset(i.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
                    .is_null()
                {
                    '\n' as i32
                } else {
                    ':' as i32
                },
            );
            i = i.wrapping_add(1);
            i;
        }
        v = (*v).next;
    }
    if vpaths.is_null() {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"# No 'vpath' search paths.\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        printf(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# %u 'vpath' search paths.\n\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            nvpaths,
        );
    }
    if general_vpath.is_null() {
        puts(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# No general ('VPATH' variable) search path.\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
        );
    } else {
        let mut path: *mut *const libc::c_char = (*general_vpath).searchpath;
        let mut i_0: libc::c_uint = 0;
        fputs(
            dcgettext(
                0 as *const libc::c_char,
                b"\n# General ('VPATH' variable) search path:\n# \0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            stdout,
        );
        i_0 = 0 as libc::c_int as libc::c_uint;
        while !(*path.offset(i_0 as isize)).is_null() {
            printf(
                b"%s%c\0" as *const u8 as *const libc::c_char,
                *path.offset(i_0 as isize),
                if (*path
                    .offset(i_0.wrapping_add(1 as libc::c_int as libc::c_uint) as isize))
                    .is_null()
                {
                    '\n' as i32
                } else {
                    ':' as i32
                },
            );
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    };
}
