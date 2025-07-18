use ::libc;
extern "C" {
    pub type __dirstream;
    fn rpl_free(ptr: *mut libc::c_void);
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn qsort(
        __base: *mut libc::c_void,
        __nmemb: size_t,
        __size: size_t,
        __compar: __compar_fn_t,
    );
    fn __errno_location() -> *mut libc::c_int;
    fn opendir_safer(name: *const libc::c_char) -> *mut DIR;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn stpcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn ximalloc(s: idx_t) -> *mut libc::c_void;
    fn xirealloc(p: *mut libc::c_void, s: idx_t) -> *mut libc::c_void;
    fn xpalloc(
        pa: *mut libc::c_void,
        pn: *mut idx_t,
        n_incr_min: idx_t,
        n_max: ptrdiff_t,
        s: idx_t,
    ) -> *mut libc::c_void;
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
pub type __compar_fn_t = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type ptrdiff_t = libc::c_long;
pub type savedir_option = libc::c_uint;
pub const SAVEDIR_SORT_FASTREAD: savedir_option = 2;
pub const SAVEDIR_SORT_INODE: savedir_option = 2;
pub const SAVEDIR_SORT_NAME: savedir_option = 1;
pub const SAVEDIR_SORT_NONE: savedir_option = 0;
pub type idx_t = ptrdiff_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct direntry_t {
    pub name: *mut libc::c_char,
    pub ino: ino_t,
}
pub type comparison_function = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
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
    let mut allocated: idx_t = 0 as libc::c_int as idx_t;
    let mut entries: *mut direntry_t = 0 as *mut direntry_t;
    let mut entries_allocated: idx_t = 0 as libc::c_int as idx_t;
    let mut entries_used: idx_t = 0 as libc::c_int as idx_t;
    let mut used: idx_t = 0 as libc::c_int as idx_t;
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
            let mut entry_size: idx_t = (strlen(((*dp).d_name).as_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong) as idx_t;
            if cmp.is_some() {
                if entries_allocated == entries_used {
                    entries = xpalloc(
                        entries as *mut libc::c_void,
                        &mut entries_allocated,
                        1 as libc::c_int as idx_t,
                        -(1 as libc::c_int) as ptrdiff_t,
                        ::std::mem::size_of::<direntry_t>() as libc::c_ulong as idx_t,
                    ) as *mut direntry_t;
                }
                let ref mut fresh0 = (*entries.offset(entries_used as isize)).name;
                *fresh0 = xstrdup(entry);
                (*entries.offset(entries_used as isize)).ino = (*dp).d_ino;
                entries_used += 1;
                entries_used;
            } else {
                if allocated - used <= entry_size {
                    name_space = xpalloc(
                        name_space as *mut libc::c_void,
                        &mut allocated,
                        entry_size - (allocated - used),
                        9223372036854775807 as libc::c_long
                            - 1 as libc::c_int as libc::c_long,
                        ::std::mem::size_of::<libc::c_char>() as libc::c_ulong as idx_t,
                    ) as *mut libc::c_char;
                }
                memcpy(
                    name_space.offset(used as isize) as *mut libc::c_void,
                    entry as *const libc::c_void,
                    entry_size as libc::c_ulong,
                );
            }
            used += entry_size;
        }
    }
    if *__errno_location() != 0 as libc::c_int {
        rpl_free(entries as *mut libc::c_void);
        rpl_free(name_space as *mut libc::c_void);
        return 0 as *mut libc::c_char;
    }
    if cmp.is_some() {
        if entries_used != 0 {
            qsort(
                entries as *mut libc::c_void,
                entries_used as size_t,
                ::std::mem::size_of::<direntry_t>() as libc::c_ulong,
                cmp,
            );
        }
        name_space = ximalloc(used + 1 as libc::c_int as libc::c_long)
            as *mut libc::c_char;
        used = 0 as libc::c_int as idx_t;
        let mut i: idx_t = 0 as libc::c_int as idx_t;
        while i < entries_used {
            let mut dest: *mut libc::c_char = name_space.offset(used as isize);
            used
                += (stpcpy(dest, (*entries.offset(i as isize)).name)).offset_from(dest)
                    as libc::c_long + 1 as libc::c_int as libc::c_long;
            rpl_free((*entries.offset(i as isize)).name as *mut libc::c_void);
            i += 1;
            i;
        }
        rpl_free(entries as *mut libc::c_void);
    } else if used == allocated {
        name_space = xirealloc(
            name_space as *mut libc::c_void,
            used + 1 as libc::c_int as libc::c_long,
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
            rpl_free(name_space as *mut libc::c_void);
            return 0 as *mut libc::c_char;
        }
        return name_space;
    };
}
