use ::libc;
extern "C" {
    pub type __dirstream;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn __errno_location() -> *mut libc::c_int;
    fn opendir_safer(name: *const libc::c_char) -> *mut DIR;
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
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xrealloc(p: *mut libc::c_void, s: size_t) -> *mut libc::c_void;
    fn xalloc_die();
    fn xmalloc(s: size_t) -> *mut libc::c_void;
    fn xstrdup(str: *const libc::c_char) -> *mut libc::c_char;
}
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type ino_t = __ino_t;
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
pub type size_t = libc::c_ulong;
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub name: *mut libc::c_char,
    pub ino: ino_t,
}
pub type comparison_function = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub const DEFAULT_MXFAST: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
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
unsafe extern "C" fn direntry_cmp_name(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut dea: *const direntry_t = a as *const direntry_t;
    let mut deb: *const direntry_t = b as *const direntry_t;
    return strcmp((*dea).name, (*deb).name);
}
unsafe extern "C" fn direntry_cmp_inode(
    mut a: *const libc::c_void,
    mut b: *const libc::c_void,
) -> libc::c_int {
    let mut dea: *const direntry_t = a as *const direntry_t;
    let mut deb: *const direntry_t = b as *const direntry_t;
    return ((*dea).ino > (*deb).ino) as libc::c_int
        - ((*dea).ino < (*deb).ino) as libc::c_int;
}
static mut comparison_function_table: [comparison_function; 3] = unsafe {
    [
        None,
        Some(
            direntry_cmp_name
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
        Some(
            direntry_cmp_inode
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        ),
    ]
};
pub unsafe extern "C" fn streamsavedir(
    mut dirp: *mut DIR,
    mut option: savedir_option,
) -> *mut libc::c_char {
    let mut name_space: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut allocated: size_t = 0 as libc::c_int as size_t;
    let mut entries: *mut direntry_t = 0 as *mut direntry_t;
    let mut entries_allocated: size_t = 0 as libc::c_int as size_t;
    let mut entries_used: size_t = 0 as libc::c_int as size_t;
    let mut used: size_t = 0 as libc::c_int as size_t;
    let mut readdir_errno: libc::c_int = 0;
    let mut cmp: comparison_function = comparison_function_table[option as usize];
    if dirp.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        let mut dp: *const dirent = 0 as *const dirent;
        let mut entry: *const libc::c_char = 0 as *const libc::c_char;
        *__errno_location() = 0 as libc::c_int;
        dp = readdir(dirp);
        if dp.is_null() {
            break;
        }
        entry = ((*dp).d_name).as_ptr();
        if *entry
            .offset(
                (if *entry.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32
                {
                    0 as libc::c_int
                } else {
                    if *entry.offset(1 as libc::c_int as isize) as libc::c_int
                        != '.' as i32
                    {
                        1 as libc::c_int
                    } else {
                        2 as libc::c_int
                    }
                }) as isize,
            ) as libc::c_int != '\0' as i32
        {
            let mut entry_size: size_t = (strlen(((*dp).d_name).as_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong);
            if cmp.is_some() {
                if entries_allocated == entries_used {
                    let mut n: size_t = entries_allocated;
                    entries = x2nrealloc(
                        entries as *mut libc::c_void,
                        &mut n,
                        ::std::mem::size_of::<direntry_t>() as libc::c_ulong,
                    ) as *mut direntry_t;
                    entries_allocated = n;
                }
                let ref mut fresh0 = (*entries.offset(entries_used as isize)).name;
                *fresh0 = xstrdup(entry);
                (*entries.offset(entries_used as isize)).ino = (*dp).d_ino;
                entries_used = entries_used.wrapping_add(1);
                entries_used;
            } else {
                if allocated.wrapping_sub(used) <= entry_size {
                    let mut n_0: size_t = used.wrapping_add(entry_size);
                    if n_0 < used {
                        xalloc_die();
                    }
                    name_space = x2nrealloc(
                        name_space as *mut libc::c_void,
                        &mut n_0,
                        1 as libc::c_int as size_t,
                    ) as *mut libc::c_char;
                    allocated = n_0;
                }
                memcpy(
                    name_space.offset(used as isize) as *mut libc::c_void,
                    entry as *const libc::c_void,
                    entry_size,
                );
            }
            used = (used as libc::c_ulong).wrapping_add(entry_size) as size_t as size_t;
        }
    }
    readdir_errno = *__errno_location();
    if readdir_errno != 0 as libc::c_int {
        rpl_free(entries as *mut libc::c_void);
        rpl_free(name_space as *mut libc::c_void);
        *__errno_location() = readdir_errno;
        return 0 as *mut libc::c_char;
    }
    if cmp.is_some() {
        let mut i: size_t = 0;
        if entries_used != 0 {
            qsort(
                entries as *mut libc::c_void,
                entries_used,
                ::std::mem::size_of::<direntry_t>() as libc::c_ulong,
                cmp,
            );
        }
        name_space = xmalloc(used.wrapping_add(1 as libc::c_int as libc::c_ulong))
            as *mut libc::c_char;
        used = 0 as libc::c_int as size_t;
        i = 0 as libc::c_int as size_t;
        while i < entries_used {
            let mut dest: *mut libc::c_char = name_space.offset(used as isize);
            used = (used as libc::c_ulong)
                .wrapping_add(
                    ((stpcpy(dest, (*entries.offset(i as isize)).name)).offset_from(dest)
                        as libc::c_long + 1 as libc::c_int as libc::c_long)
                        as libc::c_ulong,
                ) as size_t as size_t;
            rpl_free((*entries.offset(i as isize)).name as *mut libc::c_void);
            i = i.wrapping_add(1);
            i;
        }
        rpl_free(entries as *mut libc::c_void);
    } else if used == allocated {
        name_space = xrealloc(
            name_space as *mut libc::c_void,
            used.wrapping_add(1 as libc::c_int as libc::c_ulong),
        ) as *mut libc::c_char;
    }
    *name_space.offset(used as isize) = '\0' as i32 as libc::c_char;
    return name_space;
}
pub unsafe extern "C" fn savedir(
    mut dir: *const libc::c_char,
    mut option: savedir_option,
) -> *mut libc::c_char {
    let mut dirp: *mut DIR = opendir_safer(dir);
    if dirp.is_null() {
        return 0 as *mut libc::c_char
    } else {
        let mut name_space: *mut libc::c_char = streamsavedir(dirp, option);
        if closedir(dirp) != 0 as libc::c_int {
            let mut closedir_errno: libc::c_int = *__errno_location();
            rpl_free(name_space as *mut libc::c_void);
            *__errno_location() = closedir_errno;
            return 0 as *mut libc::c_char;
        }
        return name_space;
    };
}
