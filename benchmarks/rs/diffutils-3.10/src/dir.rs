use ::libc;
extern "C" {
    pub type __dirstream;
    pub type exclude;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn perror_with_name(_: *const libc::c_char);
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn rpl_free(ptr: *mut libc::c_void);
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strcoll(__s1: *const libc::c_char, __s2: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn dcgettext(
        __domainname: *const libc::c_char,
        __msgid: *const libc::c_char,
        __category: libc::c_int,
    ) -> *mut libc::c_char;
    fn __errno_location() -> *mut libc::c_int;
    static mut ignore_file_name_case: bool;
    static mut starting_file: *const libc::c_char;
    static mut excluded: *mut exclude;
    fn error(
        __status: libc::c_int,
        __errnum: libc::c_int,
        __format: *const libc::c_char,
        _: ...
    );
    fn excluded_file_name(_: *const exclude, _: *const libc::c_char) -> bool;
    fn file_name_concat(
        dir: *const libc::c_char,
        base: *const libc::c_char,
        base_in_result: *mut *mut libc::c_char,
    ) -> *mut libc::c_char;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xnmalloc(n: size_t, s: size_t) -> *mut libc::c_void;
}
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
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
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
pub struct dirent {
    pub d_ino: __ino_t,
    pub d_off: __off_t,
    pub d_reclen: libc::c_ushort,
    pub d_type: libc::c_uchar,
    pub d_name: [libc::c_char; 256],
}
pub type DIR = __dirstream;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type lin = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct file_data {
    pub desc: libc::c_int,
    pub name: *const libc::c_char,
    pub stat: stat,
    pub buffer: *mut size_t,
    pub bufsize: size_t,
    pub buffered: size_t,
    pub linbuf: *mut *const libc::c_char,
    pub linbuf_base: lin,
    pub buffered_lines: lin,
    pub valid_lines: lin,
    pub alloc_lines: lin,
    pub prefix_end: *const libc::c_char,
    pub prefix_lines: lin,
    pub suffix_begin: *const libc::c_char,
    pub equivs: *mut lin,
    pub undiscarded: *mut lin,
    pub realindexes: *mut lin,
    pub nondiscarded_lines: lin,
    pub changed: *mut libc::c_char,
    pub missing_newline: bool,
    pub eof: bool,
    pub equiv_max: lin,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct comparison {
    pub file: [file_data; 2],
    pub parent: *const comparison,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct dirdata {
    pub nnames: size_t,
    pub names: *mut *const libc::c_char,
    pub data: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __jmp_buf_tag {
    pub __jmpbuf: __jmp_buf,
    pub __mask_was_saved: libc::c_int,
    pub __saved_mask: __sigset_t,
}
pub type __jmp_buf = [libc::c_long; 8];
pub type jmp_buf = [__jmp_buf_tag; 1];
static mut locale_specific_sorting: bool = false;
static mut failed_locale_specific_sorting: jmp_buf = [__jmp_buf_tag {
    __jmpbuf: [0; 8],
    __mask_was_saved: 0,
    __saved_mask: __sigset_t { __val: [0; 16] },
}; 1];
unsafe extern "C" fn dir_read(
    mut dir: *const file_data,
    mut dirdata: *mut dirdata,
) -> bool {
    let mut next: *mut dirent = 0 as *mut dirent;
    let mut i: size_t = 0;
    let mut names: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
    let mut nnames: size_t = 0;
    let mut data: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut data_alloc: size_t = 0;
    let mut data_used: size_t = 0;
    (*dirdata).names = 0 as *mut *const libc::c_char;
    (*dirdata).data = 0 as *mut libc::c_char;
    nnames = 0 as libc::c_int as size_t;
    data = 0 as *mut libc::c_char;
    if (*dir).desc != -(1 as libc::c_int) {
        let mut reading: *mut DIR = opendir((*dir).name);
        if reading.is_null() {
            return 0 as libc::c_int != 0;
        }
        data_alloc = 512 as libc::c_int as size_t;
        data_used = 0 as libc::c_int as size_t;
        data = xmalloc(data_alloc) as *mut libc::c_char;
        (*dirdata).data = data;
        loop {
            *__errno_location() = 0 as libc::c_int;
            next = readdir(reading);
            if !(next != 0 as *mut dirent) {
                break;
            }
            let mut d_name: *mut libc::c_char = ((*next).d_name).as_mut_ptr();
            let mut d_size: size_t = (strlen(((*next).d_name).as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if *d_name.offset(0 as libc::c_int as isize) as libc::c_int == '.' as i32
                && (*d_name.offset(1 as libc::c_int as isize) as libc::c_int
                    == 0 as libc::c_int
                    || *d_name.offset(1 as libc::c_int as isize) as libc::c_int
                        == '.' as i32
                        && *d_name.offset(2 as libc::c_int as isize) as libc::c_int
                            == 0 as libc::c_int)
            {
                continue;
            }
            if excluded_file_name(excluded, d_name) {
                continue;
            }
            while data_alloc < data_used.wrapping_add(d_size) {
                if (9223372036854775807 as libc::c_long
                    / 2 as libc::c_int as libc::c_long) as libc::c_ulong <= data_alloc
                {
                    xalloc_die();
                }
                data_alloc = (data_alloc as libc::c_ulong)
                    .wrapping_mul(2 as libc::c_int as libc::c_ulong) as size_t as size_t;
                data = xrealloc(data as *mut libc::c_void, data_alloc)
                    as *mut libc::c_char;
                (*dirdata).data = data;
            }
            memcpy(
                data.offset(data_used as isize) as *mut libc::c_void,
                d_name as *const libc::c_void,
                d_size,
            );
            data_used = (data_used as libc::c_ulong).wrapping_add(d_size) as size_t
                as size_t;
            nnames = nnames.wrapping_add(1);
            nnames;
        }
        if *__errno_location() != 0 {
            let mut e: libc::c_int = *__errno_location();
            closedir(reading);
            *__errno_location() = e;
            return 0 as libc::c_int != 0;
        }
        if closedir(reading) != 0 as libc::c_int {
            return 0 as libc::c_int != 0;
        }
    }
    names = xnmalloc(
        nnames.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
    ) as *mut *const libc::c_char;
    (*dirdata).names = names;
    (*dirdata).nnames = nnames;
    i = 0 as libc::c_int as size_t;
    while i < nnames {
        let ref mut fresh0 = *names.offset(i as isize);
        *fresh0 = data;
        data = data
            .offset(
                (strlen(data)).wrapping_add(1 as libc::c_int as libc::c_ulong) as isize,
            );
        i = i.wrapping_add(1);
        i;
    }
    let ref mut fresh1 = *names.offset(nnames as isize);
    *fresh1 = 0 as *const libc::c_char;
    return 1 as libc::c_int != 0;
}
unsafe extern "C" fn compare_collated(
    mut name1: *const libc::c_char,
    mut name2: *const libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    *__errno_location() = 0 as libc::c_int;
    if ignore_file_name_case {
        r = strcasecmp(name1, name2);
    } else {
        r = strcoll(name1, name2);
    }
    if *__errno_location() != 0 {
        error(
            0 as libc::c_int,
            *__errno_location(),
            dcgettext(
                0 as *const libc::c_char,
                b"cannot compare file names '%s' and '%s'\0" as *const u8
                    as *const libc::c_char,
                5 as libc::c_int,
            ),
            name1,
            name2,
        );
        longjmp(failed_locale_specific_sorting.as_mut_ptr(), 1 as libc::c_int);
    }
    return r;
}
unsafe extern "C" fn compare_names(
    mut name1: *const libc::c_char,
    mut name2: *const libc::c_char,
) -> libc::c_int {
    if locale_specific_sorting {
        let mut diff: libc::c_int = compare_collated(name1, name2);
        if diff != 0 || ignore_file_name_case as libc::c_int != 0 {
            return diff;
        }
    }
    return strcmp(name1, name2);
}
unsafe extern "C" fn compare_names_for_qsort(
    mut file1: *const libc::c_void,
    mut file2: *const libc::c_void,
) -> libc::c_int {
    let mut f1: *const *const libc::c_char = file1 as *const *const libc::c_char;
    let mut f2: *const *const libc::c_char = file2 as *const *const libc::c_char;
    let mut name1: *const libc::c_char = *f1;
    let mut name2: *const libc::c_char = *f2;
    if locale_specific_sorting {
        let mut diff: libc::c_int = compare_collated(name1, name2);
        if diff != 0 {
            return diff;
        }
    }
    return strcmp(name1, name2);
}
pub unsafe extern "C" fn diff_dirs(
    mut cmp: *const comparison,
    mut handle_file: Option::<
        unsafe extern "C" fn(
            *const comparison,
            *const libc::c_char,
            *const libc::c_char,
        ) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut dirdata: [dirdata; 2] = [dirdata {
        nnames: 0,
        names: 0 as *mut *const libc::c_char,
        data: 0 as *mut libc::c_char,
    }; 2];
    let mut val: libc::c_int = 0 as libc::c_int;
    let mut i: libc::c_int = 0;
    if ((*cmp).file[0 as libc::c_int as usize].desc == -(1 as libc::c_int)
        || dir_loop(cmp, 0 as libc::c_int) as libc::c_int != 0)
        && ((*cmp).file[1 as libc::c_int as usize].desc == -(1 as libc::c_int)
            || dir_loop(cmp, 1 as libc::c_int) as libc::c_int != 0)
    {
        error(
            0 as libc::c_int,
            0 as libc::c_int,
            dcgettext(
                0 as *const libc::c_char,
                b"%s: recursive directory loop\0" as *const u8 as *const libc::c_char,
                5 as libc::c_int,
            ),
            (*cmp)
                .file[((*cmp).file[0 as libc::c_int as usize].desc
                    == -(1 as libc::c_int)) as libc::c_int as usize]
                .name,
        );
        return 2 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        if !dir_read(
            &*((*cmp).file).as_ptr().offset(i as isize),
            &mut *dirdata.as_mut_ptr().offset(i as isize),
        ) {
            perror_with_name((*cmp).file[i as usize].name);
            ::std::ptr::write_volatile(&mut val as *mut libc::c_int, 2 as libc::c_int);
        }
        i += 1;
        i;
    }
    if val == 0 as libc::c_int {
        let mut names: [*mut *const libc::c_char; 2] = [0
            as *mut *const libc::c_char; 2];
        ::std::ptr::write_volatile(
            &mut names[0 as libc::c_int as usize] as *mut *mut *const libc::c_char,
            dirdata[0 as libc::c_int as usize].names,
        );
        ::std::ptr::write_volatile(
            &mut names[1 as libc::c_int as usize] as *mut *mut *const libc::c_char,
            dirdata[1 as libc::c_int as usize].names,
        );
        locale_specific_sorting = 1 as libc::c_int != 0;
        if _setjmp(failed_locale_specific_sorting.as_mut_ptr()) != 0 {
            locale_specific_sorting = 0 as libc::c_int != 0;
        }
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            qsort(
                names[i as usize] as *mut libc::c_void,
                dirdata[i as usize].nnames,
                ::std::mem::size_of::<*const libc::c_char>() as libc::c_ulong,
                Some(
                    compare_names_for_qsort
                        as unsafe extern "C" fn(
                            *const libc::c_void,
                            *const libc::c_void,
                        ) -> libc::c_int,
                ),
            );
            i += 1;
            i;
        }
        if !starting_file.is_null() && ((*cmp).parent).is_null() {
            while !(*names[0 as libc::c_int as usize]).is_null()
                && compare_names(*names[0 as libc::c_int as usize], starting_file)
                    < 0 as libc::c_int
            {
                ::std::ptr::write_volatile(
                    &mut names[0 as libc::c_int as usize]
                        as *mut *mut *const libc::c_char,
                    (::std::ptr::read_volatile::<
                        *mut *const libc::c_char,
                    >(
                        &names[0 as libc::c_int as usize]
                            as *const *mut *const libc::c_char,
                    ))
                        .offset(1),
                );
                ::std::ptr::read_volatile::<
                    *mut *const libc::c_char,
                >(&names[0 as libc::c_int as usize] as *const *mut *const libc::c_char);
            }
            while !(*names[1 as libc::c_int as usize]).is_null()
                && compare_names(*names[1 as libc::c_int as usize], starting_file)
                    < 0 as libc::c_int
            {
                ::std::ptr::write_volatile(
                    &mut names[1 as libc::c_int as usize]
                        as *mut *mut *const libc::c_char,
                    (::std::ptr::read_volatile::<
                        *mut *const libc::c_char,
                    >(
                        &names[1 as libc::c_int as usize]
                            as *const *mut *const libc::c_char,
                    ))
                        .offset(1),
                );
                ::std::ptr::read_volatile::<
                    *mut *const libc::c_char,
                >(&names[1 as libc::c_int as usize] as *const *mut *const libc::c_char);
            }
        }
        while !(*names[0 as libc::c_int as usize]).is_null()
            || !(*names[1 as libc::c_int as usize]).is_null()
        {
            let mut nameorder: libc::c_int = if (*names[0 as libc::c_int as usize])
                .is_null()
            {
                1 as libc::c_int
            } else if (*names[1 as libc::c_int as usize]).is_null() {
                -(1 as libc::c_int)
            } else {
                compare_names(
                    *names[0 as libc::c_int as usize],
                    *names[1 as libc::c_int as usize],
                )
            };
            if nameorder == 0 as libc::c_int && ignore_file_name_case as libc::c_int != 0
            {
                let mut raw_order: libc::c_int = strcmp(
                    *names[0 as libc::c_int as usize],
                    *names[1 as libc::c_int as usize],
                );
                if raw_order != 0 as libc::c_int {
                    let mut greater_side: libc::c_int = (raw_order < 0 as libc::c_int)
                        as libc::c_int;
                    let mut lesser_side: libc::c_int = 1 as libc::c_int - greater_side;
                    let mut lesser: *mut *const libc::c_char = names[lesser_side
                        as usize];
                    let mut greater_name: *const libc::c_char = *names[greater_side
                        as usize];
                    let mut p: *mut *const libc::c_char = 0 as *mut *const libc::c_char;
                    p = lesser.offset(1 as libc::c_int as isize);
                    while !(*p).is_null()
                        && compare_names(*p, greater_name) == 0 as libc::c_int
                    {
                        let mut c: libc::c_int = strcmp(*p, greater_name);
                        if 0 as libc::c_int <= c {
                            if c == 0 as libc::c_int {
                                memmove(
                                    lesser.offset(1 as libc::c_int as isize)
                                        as *mut libc::c_void,
                                    lesser as *const libc::c_void,
                                    (p as *mut libc::c_char)
                                        .offset_from(lesser as *mut libc::c_char) as libc::c_long
                                        as libc::c_ulong,
                                );
                                *lesser = greater_name;
                            }
                            break;
                        } else {
                            p = p.offset(1);
                            p;
                        }
                    }
                }
            }
            let mut v1: libc::c_int = (Some(handle_file.unwrap()))
                .unwrap()(
                cmp,
                if (0 as libc::c_int) < nameorder {
                    0 as *const libc::c_char
                } else {
                    let fresh2 = ::std::ptr::read_volatile::<
                        *mut *const libc::c_char,
                    >(
                        &names[0 as libc::c_int as usize]
                            as *const *mut *const libc::c_char,
                    );
                    ::std::ptr::write_volatile(
                        &mut names[0 as libc::c_int as usize]
                            as *mut *mut *const libc::c_char,
                        (::std::ptr::read_volatile::<
                            *mut *const libc::c_char,
                        >(
                            &names[0 as libc::c_int as usize]
                                as *const *mut *const libc::c_char,
                        ))
                            .offset(1),
                    );
                    *fresh2
                },
                if nameorder < 0 as libc::c_int {
                    0 as *const libc::c_char
                } else {
                    let fresh3 = ::std::ptr::read_volatile::<
                        *mut *const libc::c_char,
                    >(
                        &names[1 as libc::c_int as usize]
                            as *const *mut *const libc::c_char,
                    );
                    ::std::ptr::write_volatile(
                        &mut names[1 as libc::c_int as usize]
                            as *mut *mut *const libc::c_char,
                        (::std::ptr::read_volatile::<
                            *mut *const libc::c_char,
                        >(
                            &names[1 as libc::c_int as usize]
                                as *const *mut *const libc::c_char,
                        ))
                            .offset(1),
                    );
                    *fresh3
                },
            );
            if val < v1 {
                ::std::ptr::write_volatile(&mut val as *mut libc::c_int, v1);
            }
        }
    }
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int {
        rpl_free(dirdata[i as usize].names as *mut libc::c_void);
        rpl_free(dirdata[i as usize].data as *mut libc::c_void);
        i += 1;
        i;
    }
    return val;
}
unsafe extern "C" fn dir_loop(mut cmp: *const comparison, mut i: libc::c_int) -> bool {
    let mut p: *const comparison = cmp;
    loop {
        p = (*p).parent;
        if p.is_null() {
            break;
        }
        if (0 as libc::c_int)
            < ((*p).file[i as usize].stat.st_ino == (*cmp).file[i as usize].stat.st_ino
                && (*p).file[i as usize].stat.st_dev
                    == (*cmp).file[i as usize].stat.st_dev
                || ((*p).file[i as usize].stat.st_mode
                    & 0o170000 as libc::c_int as libc::c_uint
                    == 0o60000 as libc::c_int as libc::c_uint
                    && (*cmp).file[i as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o60000 as libc::c_int as libc::c_uint
                    || (*p).file[i as usize].stat.st_mode
                        & 0o170000 as libc::c_int as libc::c_uint
                        == 0o20000 as libc::c_int as libc::c_uint
                        && (*cmp).file[i as usize].stat.st_mode
                            & 0o170000 as libc::c_int as libc::c_uint
                            == 0o20000 as libc::c_int as libc::c_uint)
                    && (*p).file[i as usize].stat.st_rdev
                        == (*cmp).file[i as usize].stat.st_rdev) as libc::c_int
        {
            return 1 as libc::c_int != 0;
        }
    }
    return 0 as libc::c_int != 0;
}
pub unsafe extern "C" fn find_dir_file_pathname(
    mut dir: *const libc::c_char,
    mut file: *const libc::c_char,
) -> *mut libc::c_char {
    let mut match_0: *const libc::c_char = file;
    let mut val: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut dirdata: dirdata = dirdata {
        nnames: 0,
        names: 0 as *mut *const libc::c_char,
        data: 0 as *mut libc::c_char,
    };
    dirdata.names = 0 as *mut *const libc::c_char;
    dirdata.data = 0 as *mut libc::c_char;
    if ignore_file_name_case {
        let mut filedata: file_data = file_data {
            desc: 0,
            name: 0 as *const libc::c_char,
            stat: stat {
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
            },
            buffer: 0 as *mut size_t,
            bufsize: 0,
            buffered: 0,
            linbuf: 0 as *mut *const libc::c_char,
            linbuf_base: 0,
            buffered_lines: 0,
            valid_lines: 0,
            alloc_lines: 0,
            prefix_end: 0 as *const libc::c_char,
            prefix_lines: 0,
            suffix_begin: 0 as *const libc::c_char,
            equivs: 0 as *mut lin,
            undiscarded: 0 as *mut lin,
            realindexes: 0 as *mut lin,
            nondiscarded_lines: 0,
            changed: 0 as *mut libc::c_char,
            missing_newline: false,
            eof: false,
            equiv_max: 0,
        };
        filedata.name = dir;
        filedata.desc = 0 as libc::c_int;
        if dir_read(&mut filedata, &mut dirdata) {
            locale_specific_sorting = 1 as libc::c_int != 0;
            if _setjmp(failed_locale_specific_sorting.as_mut_ptr()) != 0 {
                match_0 = file;
            } else {
                let mut p: *mut *const libc::c_char = dirdata.names;
                while !(*p).is_null() {
                    if compare_names(*p, file) == 0 as libc::c_int {
                        if strcmp(*p, file) == 0 as libc::c_int {
                            match_0 = *p;
                            break;
                        } else if match_0 == file {
                            match_0 = *p;
                        }
                    }
                    p = p.offset(1);
                    p;
                }
            }
        }
    }
    val = file_name_concat(dir, match_0, 0 as *mut *mut libc::c_char);
    rpl_free(dirdata.names as *mut libc::c_void);
    rpl_free(dirdata.data as *mut libc::c_void);
    return val;
}
