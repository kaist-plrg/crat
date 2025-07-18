use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn __fxstat(
        __ver: libc::c_int,
        __fildes: libc::c_int,
        __stat_buf: *mut stat,
    ) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn lseek(__fd: libc::c_int, __offset: __off_t, __whence: libc::c_int) -> __off_t;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn fatal(flocp: *const floc, length: size_t, fmt: *const libc::c_char, _: ...) -> !;
    fn make_toui(_: *const libc::c_char, _: *mut *const libc::c_char) -> libc::c_uint;
    fn writebuf(_: libc::c_int, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn readbuf(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __intmax_t = libc::c_long;
pub type __uintmax_t = libc::c_ulong;
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
pub type off_t = __off_t;
pub type ssize_t = __ssize_t;
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
pub type intmax_t = __intmax_t;
pub type uintmax_t = __uintmax_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct floc {
    pub filenm: *const libc::c_char,
    pub lineno: libc::c_ulong,
    pub offset: libc::c_ulong,
}
pub type ar_member_func_t = Option::<
    unsafe extern "C" fn(
        libc::c_int,
        *const libc::c_char,
        libc::c_int,
        libc::c_long,
        libc::c_long,
        libc::c_long,
        intmax_t,
        libc::c_int,
        libc::c_int,
        libc::c_uint,
        *const libc::c_void,
    ) -> intmax_t,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ar_hdr {
    pub ar_name: [libc::c_char; 16],
    pub ar_date: [libc::c_char; 12],
    pub ar_uid: [libc::c_char; 6],
    pub ar_gid: [libc::c_char; 6],
    pub ar_mode: [libc::c_char; 8],
    pub ar_size: [libc::c_char; 10],
    pub ar_fmag: [libc::c_char; 2],
}
#[inline]
unsafe extern "C" fn fstat(
    mut __fd: libc::c_int,
    mut __statbuf: *mut stat,
) -> libc::c_int {
    return __fxstat(1 as libc::c_int, __fd, __statbuf);
}
unsafe extern "C" fn parse_int(
    mut ptr: *const libc::c_char,
    len: size_t,
    base: libc::c_int,
    mut max: uintmax_t,
    mut type_0: *const libc::c_char,
    mut archive: *const libc::c_char,
    mut name: *const libc::c_char,
) -> uintmax_t {
    let ep: *const libc::c_char = ptr.offset(len as isize);
    let maxchar: libc::c_int = '0' as i32 + base - 1 as libc::c_int;
    let mut val: uintmax_t = 0 as libc::c_int as uintmax_t;
    while ptr < ep && *ptr as libc::c_int == ' ' as i32 {
        ptr = ptr.offset(1);
        ptr;
    }
    while ptr < ep && *ptr as libc::c_int != ' ' as i32 {
        let mut nv: uintmax_t = 0;
        if (*ptr as libc::c_int) < '0' as i32 || *ptr as libc::c_int > maxchar {
            fatal(
                0 as *mut floc,
                (strlen(type_0))
                    .wrapping_add(strlen(archive))
                    .wrapping_add(strlen(name)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid %s for archive %s member %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                type_0,
                archive,
                name,
            );
        }
        nv = val
            .wrapping_mul(base as libc::c_ulong)
            .wrapping_add((*ptr as libc::c_int - '0' as i32) as libc::c_ulong);
        if nv < val || nv > max {
            fatal(
                0 as *mut floc,
                (strlen(type_0))
                    .wrapping_add(strlen(archive))
                    .wrapping_add(strlen(name)),
                dcgettext(
                    0 as *const libc::c_char,
                    b"Invalid %s for archive %s member %s\0" as *const u8
                        as *const libc::c_char,
                    5 as libc::c_int,
                ),
                type_0,
                archive,
                name,
            );
        }
        val = nv;
        ptr = ptr.offset(1);
        ptr;
    }
    return val;
}
pub unsafe extern "C" fn ar_scan(
    mut archive: *const libc::c_char,
    mut function: ar_member_func_t,
    mut arg: *const libc::c_void,
) -> intmax_t {
    let mut current_block: u64;
    let mut namemap: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut namemap_size: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut desc: libc::c_int = open(archive, 0 as libc::c_int, 0 as libc::c_int);
    if desc < 0 as libc::c_int {
        return -(1 as libc::c_int) as intmax_t;
    }
    let mut buf: [libc::c_char; 8] = [0; 8];
    let mut nread: libc::c_int = 0;
    nread = readbuf(
        desc,
        buf.as_mut_ptr() as *mut libc::c_void,
        8 as libc::c_int as size_t,
    ) as libc::c_int;
    if !(nread != 8 as libc::c_int
        || memcmp(
            buf.as_mut_ptr() as *const libc::c_void,
            b"!<arch>\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            8 as libc::c_int as libc::c_ulong,
        ) != 0)
    {
        let mut member_offset: libc::c_long = 8 as libc::c_int as libc::c_long;
        loop {
            let mut nread_0: ssize_t = 0;
            let mut member_header: ar_hdr = ar_hdr {
                ar_name: [0; 16],
                ar_date: [0; 12],
                ar_uid: [0; 6],
                ar_gid: [0; 6],
                ar_mode: [0; 8],
                ar_size: [0; 10],
                ar_fmag: [0; 2],
            };
            let mut namebuf: [libc::c_char; 17] = [0; 17];
            let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
            let mut is_namemap: libc::c_int = 0;
            let mut long_name: libc::c_int = 0 as libc::c_int;
            let mut eltsize: libc::c_long = 0;
            let mut eltmode: libc::c_uint = 0;
            let mut eltdate: intmax_t = 0;
            let mut eltuid: libc::c_int = 0;
            let mut eltgid: libc::c_int = 0;
            let mut fnval: intmax_t = 0;
            let mut o: off_t = 0;
            memset(
                &mut member_header as *mut ar_hdr as *mut libc::c_void,
                '\0' as i32,
                ::std::mem::size_of::<ar_hdr>() as libc::c_ulong,
            );
            loop {
                o = lseek(desc, member_offset, 0 as libc::c_int);
                if !(o == -(1 as libc::c_int) as libc::c_long
                    && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if o < 0 as libc::c_int as libc::c_long {
                current_block = 1976520893141997835;
                break;
            }
            nread_0 = readbuf(
                desc,
                &mut member_header as *mut ar_hdr as *mut libc::c_void,
                ::std::mem::size_of::<ar_hdr>() as libc::c_ulong,
            );
            if nread_0 == 0 as libc::c_int as libc::c_long {
                current_block = 16203797167131938757;
                break;
            }
            if nread_0 as libc::c_ulong
                != ::std::mem::size_of::<ar_hdr>() as libc::c_ulong
                || memcmp(
                    (member_header.ar_fmag).as_mut_ptr() as *const libc::c_void,
                    b"`\n\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                    2 as libc::c_int as libc::c_ulong,
                ) != 0 && 1 as libc::c_int != 0
            {
                current_block = 1976520893141997835;
                break;
            }
            name = namebuf.as_mut_ptr();
            memcpy(
                name as *mut libc::c_void,
                (member_header.ar_name).as_mut_ptr() as *const libc::c_void,
                ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong,
            );
            let mut p: *mut libc::c_char = name
                .offset(
                    ::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong as isize,
                );
            loop {
                *p = '\0' as i32 as libc::c_char;
                if !(p > name
                    && {
                        p = p.offset(-1);
                        *p as libc::c_int == ' ' as i32
                    })
                {
                    break;
                }
            }
            is_namemap = (strcmp(name, b"//\0" as *const u8 as *const libc::c_char) == 0
                || strcmp(name, b"ARFILENAMES/\0" as *const u8 as *const libc::c_char)
                    == 0) as libc::c_int;
            if *p as libc::c_int == '/' as i32 {
                *p = '\0' as i32 as libc::c_char;
            }
            if is_namemap == 0
                && (*name.offset(0 as libc::c_int as isize) as libc::c_int == ' ' as i32
                    || *name.offset(0 as libc::c_int as isize) as libc::c_int
                        == '/' as i32) && !namemap.is_null()
            {
                let mut err: *const libc::c_char = 0 as *const libc::c_char;
                let mut name_off: libc::c_uint = make_toui(
                    name.offset(1 as libc::c_int as isize),
                    &mut err,
                );
                let mut name_len: size_t = 0;
                if !err.is_null() || name_off >= namemap_size {
                    current_block = 1976520893141997835;
                    break;
                }
                name = namemap.offset(name_off as isize);
                name_len = strlen(name);
                if name_len < 1 as libc::c_int as libc::c_ulong {
                    current_block = 1976520893141997835;
                    break;
                }
                long_name = 1 as libc::c_int;
            } else if *name.offset(0 as libc::c_int as isize) as libc::c_int
                == '#' as i32
                && *name.offset(1 as libc::c_int as isize) as libc::c_int == '1' as i32
                && *name.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32
            {
                let mut err_0: *const libc::c_char = 0 as *const libc::c_char;
                let mut name_len_0: libc::c_uint = make_toui(
                    name.offset(3 as libc::c_int as isize),
                    &mut err_0,
                );
                if !err_0.is_null() || name_len_0 == 0 as libc::c_int as libc::c_uint
                    || name_len_0
                        >= (if (4096 as libc::c_int) < 2147483647 as libc::c_int {
                            4096 as libc::c_int
                        } else {
                            2147483647 as libc::c_int
                        }) as libc::c_uint
                {
                    current_block = 1976520893141997835;
                    break;
                }
                let mut fresh0 = ::std::vec::from_elem(
                    0,
                    name_len_0.wrapping_add(1 as libc::c_int as libc::c_uint)
                        as libc::c_ulong as usize,
                );
                name = fresh0.leak().as_mut_ptr() as *mut libc::c_char;
                nread_0 = readbuf(desc, name as *mut libc::c_void, name_len_0 as size_t);
                if nread_0 < 0 as libc::c_int as libc::c_long
                    || nread_0 as libc::c_uint != name_len_0
                {
                    current_block = 1976520893141997835;
                    break;
                }
                *name.offset(name_len_0 as isize) = '\0' as i32 as libc::c_char;
                long_name = 1 as libc::c_int;
            }
            eltmode = parse_int(
                (member_header.ar_mode).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 8]>() as libc::c_ulong,
                8 as libc::c_int,
                (if (0 as libc::c_int as libc::c_uint)
                    < -(1 as libc::c_int) as libc::c_uint
                {
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
                b"mode\0" as *const u8 as *const libc::c_char,
                archive,
                name,
            ) as libc::c_uint;
            eltsize = parse_int(
                (member_header.ar_size).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                10 as libc::c_int,
                (if (0 as libc::c_int as libc::c_long)
                    < -(1 as libc::c_int) as libc::c_long
                {
                    -(1 as libc::c_int) as libc::c_long
                } else {
                    (((1 as libc::c_int as libc::c_long)
                        << (::std::mem::size_of::<libc::c_long>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as uintmax_t,
                b"size\0" as *const u8 as *const libc::c_char,
                archive,
                name,
            ) as libc::c_long;
            eltdate = parse_int(
                (member_header.ar_date).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                10 as libc::c_int,
                (if (0 as libc::c_int as intmax_t) < -(1 as libc::c_int) as intmax_t {
                    -(1 as libc::c_int) as intmax_t
                } else {
                    (((1 as libc::c_int as intmax_t)
                        << (::std::mem::size_of::<intmax_t>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int as libc::c_long)
                        * 2 as libc::c_int as libc::c_long
                        + 1 as libc::c_int as libc::c_long
                }) as uintmax_t,
                b"date\0" as *const u8 as *const libc::c_char,
                archive,
                name,
            ) as intmax_t;
            eltuid = parse_int(
                (member_header.ar_uid).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                10 as libc::c_int,
                (if (0 as libc::c_int) < -(1 as libc::c_int) {
                    -(1 as libc::c_int)
                } else {
                    (((1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                }) as uintmax_t,
                b"uid\0" as *const u8 as *const libc::c_char,
                archive,
                name,
            ) as libc::c_int;
            eltgid = parse_int(
                (member_header.ar_gid).as_mut_ptr(),
                ::std::mem::size_of::<[libc::c_char; 6]>() as libc::c_ulong,
                10 as libc::c_int,
                (if (0 as libc::c_int) < -(1 as libc::c_int) {
                    -(1 as libc::c_int)
                } else {
                    (((1 as libc::c_int)
                        << (::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
                            .wrapping_mul(8 as libc::c_int as libc::c_ulong)
                            .wrapping_sub(2 as libc::c_int as libc::c_ulong))
                        - 1 as libc::c_int) * 2 as libc::c_int + 1 as libc::c_int
                }) as uintmax_t,
                b"gid\0" as *const u8 as *const libc::c_char,
                archive,
                name,
            ) as libc::c_int;
            fnval = (Some(function.unwrap()))
                .unwrap()(
                desc,
                name,
                (long_name == 0) as libc::c_int,
                member_offset,
                (member_offset as libc::c_ulong)
                    .wrapping_add(::std::mem::size_of::<ar_hdr>() as libc::c_ulong)
                    as libc::c_long,
                eltsize,
                eltdate,
                eltuid,
                eltgid,
                eltmode,
                arg,
            );
            if fnval != 0 {
                close(desc);
                return fnval;
            }
            if is_namemap != 0 {
                let mut clear: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut limit: *mut libc::c_char = 0 as *mut libc::c_char;
                if eltsize > 2147483647 as libc::c_int as libc::c_long {
                    current_block = 1976520893141997835;
                    break;
                }
                let mut fresh1 = ::std::vec::from_elem(
                    0,
                    (eltsize + 1 as libc::c_int as libc::c_long) as libc::c_ulong
                        as usize,
                );
                namemap = fresh1.leak().as_mut_ptr() as *mut libc::c_char;
                nread_0 = readbuf(desc, namemap as *mut libc::c_void, eltsize as size_t);
                if nread_0 != eltsize {
                    current_block = 1976520893141997835;
                    break;
                }
                namemap_size = eltsize as libc::c_uint;
                limit = namemap.offset(eltsize as isize);
                clear = namemap;
                while clear < limit {
                    if *clear as libc::c_int == '\n' as i32 {
                        *clear = '\0' as i32 as libc::c_char;
                        if *clear.offset(-(1 as libc::c_int) as isize) as libc::c_int
                            == '/' as i32
                        {
                            *clear
                                .offset(
                                    -(1 as libc::c_int) as isize,
                                ) = '\0' as i32 as libc::c_char;
                        }
                    }
                    clear = clear.offset(1);
                    clear;
                }
                *limit = '\0' as i32 as libc::c_char;
                is_namemap = 0 as libc::c_int;
            }
            member_offset = (member_offset as libc::c_ulong)
                .wrapping_add(
                    (::std::mem::size_of::<ar_hdr>() as libc::c_ulong)
                        .wrapping_add(eltsize as libc::c_ulong),
                ) as libc::c_long as libc::c_long;
            if member_offset % 2 as libc::c_int as libc::c_long
                != 0 as libc::c_int as libc::c_long
            {
                member_offset += 1;
                member_offset;
            }
        }
        match current_block {
            1976520893141997835 => {}
            _ => {
                close(desc);
                return 0 as libc::c_int as intmax_t;
            }
        }
    }
    close(desc);
    return -(2 as libc::c_int) as intmax_t;
}
pub unsafe extern "C" fn ar_name_equal(
    mut name: *const libc::c_char,
    mut mem: *const libc::c_char,
    mut truncated: libc::c_int,
) -> libc::c_int {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    p = strrchr(name, '/' as i32);
    if !p.is_null() {
        name = p.offset(1 as libc::c_int as isize);
    }
    if truncated != 0 {
        let mut hdr: ar_hdr = ar_hdr {
            ar_name: [0; 16],
            ar_date: [0; 12],
            ar_uid: [0; 6],
            ar_gid: [0; 6],
            ar_mode: [0; 8],
            ar_size: [0; 10],
            ar_fmag: [0; 2],
        };
        return (strncmp(
            name,
            mem,
            (::std::mem::size_of::<[libc::c_char; 16]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        ) == 0 as libc::c_int) as libc::c_int;
    }
    return (strcmp(name, mem) == 0) as libc::c_int;
}
unsafe extern "C" fn ar_member_pos(
    mut desc: libc::c_int,
    mut mem: *const libc::c_char,
    mut truncated: libc::c_int,
    mut hdrpos: libc::c_long,
    mut datapos: libc::c_long,
    mut size: libc::c_long,
    mut date: intmax_t,
    mut uid: libc::c_int,
    mut gid: libc::c_int,
    mut mode: libc::c_uint,
    mut name: *const libc::c_void,
) -> intmax_t {
    if ar_name_equal(name as *const libc::c_char, mem, truncated) == 0 {
        return 0 as libc::c_int as intmax_t;
    }
    return hdrpos;
}
pub unsafe extern "C" fn ar_member_touch(
    mut arname: *const libc::c_char,
    mut memname: *const libc::c_char,
) -> libc::c_int {
    let mut pos: intmax_t = ar_scan(
        arname,
        Some(
            ar_member_pos
                as unsafe extern "C" fn(
                    libc::c_int,
                    *const libc::c_char,
                    libc::c_int,
                    libc::c_long,
                    libc::c_long,
                    libc::c_long,
                    intmax_t,
                    libc::c_int,
                    libc::c_int,
                    libc::c_uint,
                    *const libc::c_void,
                ) -> intmax_t,
        ),
        memname as *const libc::c_void,
    );
    let mut opos: off_t = 0;
    let mut fd: libc::c_int = 0;
    let mut ar_hdr: ar_hdr = ar_hdr {
        ar_name: [0; 16],
        ar_date: [0; 12],
        ar_uid: [0; 6],
        ar_gid: [0; 6],
        ar_mode: [0; 8],
        ar_size: [0; 10],
        ar_fmag: [0; 2],
    };
    let mut o: off_t = 0;
    let mut r: libc::c_int = 0;
    let mut datelen: libc::c_int = 0;
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
    if pos < 0 as libc::c_int as libc::c_long {
        return pos as libc::c_int;
    }
    if pos == 0 {
        return 1 as libc::c_int;
    }
    opos = pos;
    loop {
        fd = open(arname, 0o2 as libc::c_int, 0o666 as libc::c_int);
        if !(fd == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int) {
            break;
        }
    }
    if fd < 0 as libc::c_int {
        return -(3 as libc::c_int);
    }
    loop {
        o = lseek(fd, opos, 0 as libc::c_int);
        if !(o == -(1 as libc::c_int) as libc::c_long
            && *__errno_location() == 4 as libc::c_int)
        {
            break;
        }
    }
    if !(o < 0 as libc::c_int as libc::c_long) {
        r = readbuf(
            fd,
            &mut ar_hdr as *mut ar_hdr as *mut libc::c_void,
            ::std::mem::size_of::<ar_hdr>() as libc::c_ulong,
        ) as libc::c_int;
        if !(r as libc::c_ulong != ::std::mem::size_of::<ar_hdr>() as libc::c_ulong) {
            loop {
                r = fstat(fd, &mut statbuf);
                if !(r == -(1 as libc::c_int) && *__errno_location() == 4 as libc::c_int)
                {
                    break;
                }
            }
            if !(r < 0 as libc::c_int) {
                datelen = snprintf(
                    (ar_hdr.ar_date).as_mut_ptr(),
                    ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong,
                    b"%ld\0" as *const u8 as *const libc::c_char,
                    statbuf.st_mtim.tv_sec,
                );
                if 0 as libc::c_int <= datelen
                    && datelen
                        < ::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong
                            as libc::c_int
                {
                    memset(
                        (ar_hdr.ar_date).as_mut_ptr().offset(datelen as isize)
                            as *mut libc::c_void,
                        ' ' as i32,
                        (::std::mem::size_of::<[libc::c_char; 12]>() as libc::c_ulong)
                            .wrapping_sub(datelen as libc::c_ulong),
                    );
                    loop {
                        o = lseek(fd, opos, 0 as libc::c_int);
                        if !(o == -(1 as libc::c_int) as libc::c_long
                            && *__errno_location() == 4 as libc::c_int)
                        {
                            break;
                        }
                    }
                    if !(o < 0 as libc::c_int as libc::c_long) {
                        r = writebuf(
                            fd,
                            &mut ar_hdr as *mut ar_hdr as *const libc::c_void,
                            ::std::mem::size_of::<ar_hdr>() as libc::c_ulong,
                        ) as libc::c_int;
                        if !(r as libc::c_ulong
                            != ::std::mem::size_of::<ar_hdr>() as libc::c_ulong)
                        {
                            close(fd);
                            return 0 as libc::c_int;
                        }
                    }
                }
            }
        }
    }
    r = *__errno_location();
    close(fd);
    *__errno_location() = r;
    return -(3 as libc::c_int);
}
