use ::libc;
extern "C" {
    pub type __dirstream;
    fn opendir(__name: *const libc::c_char) -> *mut DIR;
    fn closedir(__dirp: *mut DIR) -> libc::c_int;
    fn readdir(__dirp: *mut DIR) -> *mut dirent;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type __ino_t = libc::c_ulong;
pub type __off_t = libc::c_long;
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
pub type filter_fp = Option::<
    unsafe extern "C" fn(
        *const libc::c_char,
        *const dirent,
        *mut libc::c_void,
    ) -> libc::c_int,
>;
pub unsafe extern "C" fn ag_scandir(
    mut dirname: *const libc::c_char,
    mut namelist: *mut *mut *mut dirent,
    mut filter: filter_fp,
    mut baton: *mut libc::c_void,
) -> libc::c_int {
    let mut current_block: u64;
    let mut dirp: *mut DIR = 0 as *mut DIR;
    let mut names: *mut *mut dirent = 0 as *mut *mut dirent;
    let mut entry: *mut dirent = 0 as *mut dirent;
    let mut d: *mut dirent = 0 as *mut dirent;
    let mut names_len: libc::c_int = 32 as libc::c_int;
    let mut results_len: libc::c_int = 0 as libc::c_int;
    dirp = opendir(dirname);
    if !dirp.is_null() {
        names = malloc(
            (::std::mem::size_of::<*mut dirent>() as libc::c_ulong)
                .wrapping_mul(names_len as libc::c_ulong),
        ) as *mut *mut dirent;
        if !names.is_null() {
            loop {
                entry = readdir(dirp);
                if entry.is_null() {
                    current_block = 17833034027772472439;
                    break;
                }
                if (Some(filter.unwrap())).unwrap()(dirname, entry, baton)
                    == 0 as libc::c_int
                {
                    continue;
                }
                if results_len >= names_len {
                    let mut tmp_names: *mut *mut dirent = names;
                    names_len *= 2 as libc::c_int;
                    names = realloc(
                        names as *mut libc::c_void,
                        (::std::mem::size_of::<*mut dirent>() as libc::c_ulong)
                            .wrapping_mul(names_len as libc::c_ulong),
                    ) as *mut *mut dirent;
                    if names.is_null() {
                        free(tmp_names as *mut libc::c_void);
                        current_block = 5678437015762115809;
                        break;
                    }
                }
                d = malloc((*entry).d_reclen as libc::c_ulong) as *mut dirent;
                if d.is_null() {
                    current_block = 5678437015762115809;
                    break;
                }
                memcpy(
                    d as *mut libc::c_void,
                    entry as *const libc::c_void,
                    (*entry).d_reclen as libc::c_ulong,
                );
                let ref mut fresh0 = *names.offset(results_len as isize);
                *fresh0 = d;
                results_len += 1;
                results_len;
            }
            match current_block {
                5678437015762115809 => {}
                _ => {
                    closedir(dirp);
                    *namelist = names;
                    return results_len;
                }
            }
        }
    }
    if !dirp.is_null() {
        closedir(dirp);
    }
    if !names.is_null() {
        let mut i: libc::c_int = 0;
        i = 0 as libc::c_int;
        while i < results_len {
            free(*names.offset(i as isize) as *mut libc::c_void);
            i += 1;
            i;
        }
        free(names as *mut libc::c_void);
    }
    return -(1 as libc::c_int);
}
