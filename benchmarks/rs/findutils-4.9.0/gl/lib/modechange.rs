use ::libc;
extern "C" {
    fn rpl_free(ptr: *mut libc::c_void);
    fn __xstat(
        __ver: libc::c_int,
        __filename: *const libc::c_char,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
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
pub type __syscall_slong_t = libc::c_long;
pub type mode_t = __mode_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mode_change {
    pub op: libc::c_char,
    pub flag: libc::c_char,
    pub affected: mode_t,
    pub value: mode_t,
    pub mentioned: mode_t,
}
pub const MODE_DONE: C2RustUnnamed = 0;
pub const MODE_COPY_EXISTING: C2RustUnnamed = 3;
pub const MODE_X_IF_ANY_X: C2RustUnnamed = 2;
pub const MODE_ORDINARY_CHANGE: C2RustUnnamed = 1;
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
pub type C2RustUnnamed = libc::c_uint;
#[inline]
unsafe extern "C" fn stat(
    mut __path: *const libc::c_char,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __xstat(1 as libc::c_int, __path, __statbuf);
}
unsafe extern "C" fn octal_to_mode(mut octal: libc::c_uint) -> mode_t {
    return if 0o4000 as libc::c_int == 0o4000 as libc::c_int
        && 0o2000 as libc::c_int == 0o2000 as libc::c_int
        && 0o1000 as libc::c_int == 0o1000 as libc::c_int
        && 0o400 as libc::c_int == 0o400 as libc::c_int
        && 0o200 as libc::c_int == 0o200 as libc::c_int
        && 0o100 as libc::c_int == 0o100 as libc::c_int
        && 0o400 as libc::c_int >> 3 as libc::c_int == 0o40 as libc::c_int
        && 0o200 as libc::c_int >> 3 as libc::c_int == 0o20 as libc::c_int
        && 0o100 as libc::c_int >> 3 as libc::c_int == 0o10 as libc::c_int
        && 0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            == 0o4 as libc::c_int
        && 0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            == 0o2 as libc::c_int
        && 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            == 0o1 as libc::c_int
    {
        octal
    } else {
        ((if octal & 0o4000 as libc::c_int as libc::c_uint != 0 {
            0o4000 as libc::c_int
        } else {
            0 as libc::c_int
        })
            | (if octal & 0o2000 as libc::c_int as libc::c_uint != 0 {
                0o2000 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o1000 as libc::c_int as libc::c_uint != 0 {
                0o1000 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o400 as libc::c_int as libc::c_uint != 0 {
                0o400 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o200 as libc::c_int as libc::c_uint != 0 {
                0o200 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o100 as libc::c_int as libc::c_uint != 0 {
                0o100 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o40 as libc::c_int as libc::c_uint != 0 {
                0o400 as libc::c_int >> 3 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o20 as libc::c_int as libc::c_uint != 0 {
                0o200 as libc::c_int >> 3 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o10 as libc::c_int as libc::c_uint != 0 {
                0o100 as libc::c_int >> 3 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o4 as libc::c_int as libc::c_uint != 0 {
                0o400 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o2 as libc::c_int as libc::c_uint != 0 {
                0o200 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            } else {
                0 as libc::c_int
            })
            | (if octal & 0o1 as libc::c_int as libc::c_uint != 0 {
                0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int
            } else {
                0 as libc::c_int
            })) as mode_t
    };
}
unsafe extern "C" fn make_node_op_equals(
    mut new_mode: mode_t,
    mut mentioned: mode_t,
) -> *mut mode_change {
    let mut p: *mut mode_change = xmalloc(
        (2 as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<mode_change>() as libc::c_ulong),
    ) as *mut mode_change;
    (*p).op = '=' as i32 as libc::c_char;
    (*p).flag = MODE_ORDINARY_CHANGE as libc::c_int as libc::c_char;
    (*p)
        .affected = (0o4000 as libc::c_int | 0o2000 as libc::c_int
        | 0o1000 as libc::c_int
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            >> 3 as libc::c_int
        | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t;
    (*p).value = new_mode;
    (*p).mentioned = mentioned;
    (*p.offset(1 as libc::c_int as isize))
        .flag = MODE_DONE as libc::c_int as libc::c_char;
    return p;
}
pub unsafe extern "C" fn mode_compile(
    mut mode_string: *const libc::c_char,
) -> *mut mode_change {
    let mut current_block: u64;
    let mut mc: *mut mode_change = 0 as *mut mode_change;
    let mut used: size_t = 0 as libc::c_int as size_t;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    if '0' as i32 <= *mode_string as libc::c_int
        && (*mode_string as libc::c_int) < '8' as i32
    {
        let mut octal_mode: libc::c_uint = 0 as libc::c_int as libc::c_uint;
        let mut mode: mode_t = 0;
        let mut mentioned: mode_t = 0;
        p = mode_string;
        loop {
            let fresh0 = p;
            p = p.offset(1);
            octal_mode = (8 as libc::c_int as libc::c_uint)
                .wrapping_mul(octal_mode)
                .wrapping_add(*fresh0 as libc::c_uint)
                .wrapping_sub('0' as i32 as libc::c_uint);
            if (0o7777 as libc::c_int as libc::c_uint) < octal_mode {
                return 0 as *mut mode_change;
            }
            if !('0' as i32 <= *p as libc::c_int && (*p as libc::c_int) < '8' as i32) {
                break;
            }
        }
        if *p != 0 {
            return 0 as *mut mode_change;
        }
        mode = octal_to_mode(octal_mode);
        mentioned = if (p.offset_from(mode_string) as libc::c_long)
            < 5 as libc::c_int as libc::c_long
        {
            mode & (0o4000 as libc::c_int | 0o2000 as libc::c_int) as libc::c_uint
                | 0o1000 as libc::c_int as libc::c_uint
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int
                    | (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                    as libc::c_uint
        } else {
            (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int
                | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                    >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint
        };
        return make_node_op_equals(mode, mentioned);
    }
    let mut needed: size_t = 1 as libc::c_int as size_t;
    p = mode_string;
    while *p != 0 {
        needed = (needed as libc::c_ulong)
            .wrapping_add(
                (*p as libc::c_int == '=' as i32 || *p as libc::c_int == '+' as i32
                    || *p as libc::c_int == '-' as i32) as libc::c_int as libc::c_ulong,
            ) as size_t as size_t;
        p = p.offset(1);
        p;
    }
    mc = xnmalloc(needed, ::std::mem::size_of::<mode_change>() as libc::c_ulong)
        as *mut mode_change;
    p = mode_string;
    's_90: loop {
        let mut affected: mode_t = 0 as libc::c_int as mode_t;
        loop {
            match *p as libc::c_int {
                117 => {
                    affected
                        |= (0o4000 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int)) as libc::c_uint;
                }
                103 => {
                    affected
                        |= (0o2000 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int)
                            as libc::c_uint;
                }
                111 => {
                    affected
                        |= (0o1000 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint;
                }
                97 => {
                    affected
                        |= (0o4000 as libc::c_int | 0o2000 as libc::c_int
                            | 0o1000 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int)
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                            | (0o400 as libc::c_int | 0o200 as libc::c_int
                                | 0o100 as libc::c_int) >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint;
                }
                61 | 43 | 45 => {
                    break;
                }
                _ => {
                    current_block = 9015079295508301172;
                    break 's_90;
                }
            }
            p = p.offset(1);
            p;
        }
        loop {
            let fresh1 = p;
            p = p.offset(1);
            let mut op: libc::c_char = *fresh1;
            let mut value: mode_t = 0;
            let mut mentioned_0: mode_t = 0 as libc::c_int as mode_t;
            let mut flag: libc::c_char = MODE_COPY_EXISTING as libc::c_int
                as libc::c_char;
            let mut change: *mut mode_change = 0 as *mut mode_change;
            match *p as libc::c_int {
                48 | 49 | 50 | 51 | 52 | 53 | 54 | 55 => {
                    let mut octal_mode_0: libc::c_uint = 0 as libc::c_int
                        as libc::c_uint;
                    loop {
                        let fresh2 = p;
                        p = p.offset(1);
                        octal_mode_0 = (8 as libc::c_int as libc::c_uint)
                            .wrapping_mul(octal_mode_0)
                            .wrapping_add(*fresh2 as libc::c_uint)
                            .wrapping_sub('0' as i32 as libc::c_uint);
                        if (0o7777 as libc::c_int as libc::c_uint) < octal_mode_0 {
                            current_block = 9015079295508301172;
                            break 's_90;
                        }
                        if !('0' as i32 <= *p as libc::c_int
                            && (*p as libc::c_int) < '8' as i32)
                        {
                            break;
                        }
                    }
                    if affected != 0
                        || *p as libc::c_int != 0 && *p as libc::c_int != ',' as i32
                    {
                        current_block = 9015079295508301172;
                        break 's_90;
                    }
                    mentioned_0 = (0o4000 as libc::c_int | 0o2000 as libc::c_int
                        | 0o1000 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int)
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as mode_t;
                    affected = mentioned_0;
                    value = octal_to_mode(octal_mode_0);
                    flag = MODE_ORDINARY_CHANGE as libc::c_int as libc::c_char;
                }
                117 => {
                    value = (0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) as mode_t;
                    p = p.offset(1);
                    p;
                }
                103 => {
                    value = ((0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int) as mode_t;
                    p = p.offset(1);
                    p;
                }
                111 => {
                    value = ((0o400 as libc::c_int | 0o200 as libc::c_int
                        | 0o100 as libc::c_int) >> 3 as libc::c_int >> 3 as libc::c_int)
                        as mode_t;
                    p = p.offset(1);
                    p;
                }
                _ => {
                    value = 0 as libc::c_int as mode_t;
                    flag = MODE_ORDINARY_CHANGE as libc::c_int as libc::c_char;
                    loop {
                        match *p as libc::c_int {
                            114 => {
                                value
                                    |= (0o400 as libc::c_int
                                        | 0o400 as libc::c_int >> 3 as libc::c_int
                                        | 0o400 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int) as libc::c_uint;
                            }
                            119 => {
                                value
                                    |= (0o200 as libc::c_int
                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                        | 0o200 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int) as libc::c_uint;
                            }
                            120 => {
                                value
                                    |= (0o100 as libc::c_int
                                        | 0o100 as libc::c_int >> 3 as libc::c_int
                                        | 0o100 as libc::c_int >> 3 as libc::c_int
                                            >> 3 as libc::c_int) as libc::c_uint;
                            }
                            88 => {
                                flag = MODE_X_IF_ANY_X as libc::c_int as libc::c_char;
                            }
                            115 => {
                                value
                                    |= (0o4000 as libc::c_int | 0o2000 as libc::c_int)
                                        as libc::c_uint;
                            }
                            116 => {
                                value |= 0o1000 as libc::c_int as libc::c_uint;
                            }
                            _ => {
                                break;
                            }
                        }
                        p = p.offset(1);
                        p;
                    }
                }
            }
            let fresh3 = used;
            used = used.wrapping_add(1);
            change = &mut *mc.offset(fresh3 as isize) as *mut mode_change;
            (*change).op = op;
            (*change).flag = flag;
            (*change).affected = affected;
            (*change).value = value;
            (*change)
                .mentioned = if mentioned_0 != 0 {
                mentioned_0
            } else if affected != 0 {
                affected & value
            } else {
                value
            };
            if !(*p as libc::c_int == '=' as i32 || *p as libc::c_int == '+' as i32
                || *p as libc::c_int == '-' as i32)
            {
                break;
            }
        }
        if *p as libc::c_int != ',' as i32 {
            current_block = 16779030619667747692;
            break;
        }
        p = p.offset(1);
        p;
    }
    match current_block {
        16779030619667747692 => {
            if *p as libc::c_int == 0 as libc::c_int {
                (*mc.offset(used as isize))
                    .flag = MODE_DONE as libc::c_int as libc::c_char;
                return mc;
            }
        }
        _ => {}
    }
    rpl_free(mc as *mut libc::c_void);
    return 0 as *mut mode_change;
}
pub unsafe extern "C" fn mode_create_from_ref(
    mut ref_file: *const libc::c_char,
) -> *mut mode_change {
    let mut ref_stats: stat = stat {
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
    if stat(ref_file, &mut ref_stats) != 0 as libc::c_int {
        return 0 as *mut mode_change;
    }
    return make_node_op_equals(
        ref_stats.st_mode,
        (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as mode_t,
    );
}
pub unsafe extern "C" fn mode_adjust(
    mut oldmode: mode_t,
    mut dir: bool,
    mut umask_value: mode_t,
    mut changes: *const mode_change,
    mut pmode_bits: *mut mode_t,
) -> mode_t {
    let mut newmode: mode_t = oldmode
        & (0o4000 as libc::c_int | 0o2000 as libc::c_int | 0o1000 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int
            | (0o400 as libc::c_int | 0o200 as libc::c_int | 0o100 as libc::c_int)
                >> 3 as libc::c_int >> 3 as libc::c_int) as libc::c_uint;
    let mut mode_bits: mode_t = 0 as libc::c_int as mode_t;
    while (*changes).flag as libc::c_int != MODE_DONE as libc::c_int {
        let mut affected: mode_t = (*changes).affected;
        let mut omit_change: mode_t = (if dir as libc::c_int != 0 {
            0o4000 as libc::c_int | 0o2000 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint & !(*changes).mentioned;
        let mut value: mode_t = (*changes).value;
        match (*changes).flag as libc::c_int {
            3 => {
                value &= newmode;
                value
                    |= ((if value
                        & (0o400 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint != 0
                    {
                        0o400 as libc::c_int | 0o400 as libc::c_int >> 3 as libc::c_int
                            | 0o400 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int
                    } else {
                        0 as libc::c_int
                    })
                        | (if value
                            & (0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int) as libc::c_uint != 0
                        {
                            0o200 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                | 0o200 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })
                        | (if value
                            & (0o100 as libc::c_int
                                | 0o100 as libc::c_int >> 3 as libc::c_int
                                | 0o100 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int) as libc::c_uint != 0
                        {
                            0o100 as libc::c_int
                                | 0o100 as libc::c_int >> 3 as libc::c_int
                                | 0o100 as libc::c_int >> 3 as libc::c_int
                                    >> 3 as libc::c_int
                        } else {
                            0 as libc::c_int
                        })) as libc::c_uint;
            }
            2 => {
                if newmode
                    & (0o100 as libc::c_int | 0o100 as libc::c_int >> 3 as libc::c_int
                        | 0o100 as libc::c_int >> 3 as libc::c_int >> 3 as libc::c_int)
                        as libc::c_uint | dir as libc::c_uint != 0
                {
                    value
                        |= (0o100 as libc::c_int
                            | 0o100 as libc::c_int >> 3 as libc::c_int
                            | 0o100 as libc::c_int >> 3 as libc::c_int
                                >> 3 as libc::c_int) as libc::c_uint;
                }
            }
            1 | _ => {}
        }
        value &= (if affected != 0 { affected } else { !umask_value }) & !omit_change;
        match (*changes).op as libc::c_int {
            61 => {
                let mut preserved: mode_t = (if affected != 0 {
                    !affected
                } else {
                    0 as libc::c_int as libc::c_uint
                }) | omit_change;
                mode_bits
                    |= (0o4000 as libc::c_int | 0o2000 as libc::c_int
                        | 0o1000 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int)
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                        | (0o400 as libc::c_int | 0o200 as libc::c_int
                            | 0o100 as libc::c_int) >> 3 as libc::c_int
                            >> 3 as libc::c_int) as libc::c_uint & !preserved;
                newmode = newmode & preserved | value;
            }
            43 => {
                mode_bits |= value;
                newmode |= value;
            }
            45 => {
                mode_bits |= value;
                newmode &= !value;
            }
            _ => {}
        }
        changes = changes.offset(1);
        changes;
    }
    if !pmode_bits.is_null() {
        *pmode_bits = mode_bits;
    }
    return newmode;
}
