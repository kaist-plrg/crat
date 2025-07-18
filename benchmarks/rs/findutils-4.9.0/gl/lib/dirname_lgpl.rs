use ::libc;
extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn last_component(filename: *const libc::c_char) -> *mut libc::c_char;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn dir_len(mut file: *const libc::c_char) -> size_t {
    let mut prefix_length: size_t = 0 as libc::c_int as size_t;
    let mut length: size_t = 0;
    prefix_length = (prefix_length as libc::c_ulong)
        .wrapping_add(
            (if prefix_length != 0 as libc::c_int as libc::c_ulong {
                (0 as libc::c_int != 0
                    && *file.offset(prefix_length as isize) as libc::c_int == '/' as i32)
                    as libc::c_int
            } else if *file.offset(0 as libc::c_int as isize) as libc::c_int
                == '/' as i32
            {
                if 0 as libc::c_int != 0
                    && *file.offset(1 as libc::c_int as isize) as libc::c_int
                        == '/' as i32
                    && !(*file.offset(2 as libc::c_int as isize) as libc::c_int
                        == '/' as i32)
                {
                    2 as libc::c_int
                } else {
                    1 as libc::c_int
                }
            } else {
                0 as libc::c_int
            }) as libc::c_ulong,
        ) as size_t as size_t;
    length = (last_component(file)).offset_from(file) as libc::c_long as size_t;
    while prefix_length < length {
        if !(*file
            .offset(length.wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize)
            as libc::c_int == '/' as i32)
        {
            break;
        }
        length = length.wrapping_sub(1);
        length;
    }
    return length;
}
pub unsafe extern "C" fn mdir_name(mut file: *const libc::c_char) -> *mut libc::c_char {
    let mut length: size_t = dir_len(file);
    let mut append_dot: bool = length == 0 as libc::c_int as libc::c_ulong
        || 0 as libc::c_int != 0 && length == 0 as libc::c_int as libc::c_ulong
            && *file.offset(2 as libc::c_int as isize) as libc::c_int != '\0' as i32
            && !(*file.offset(2 as libc::c_int as isize) as libc::c_int == '/' as i32);
    let mut dir: *mut libc::c_char = malloc(
        length
            .wrapping_add(append_dot as libc::c_ulong)
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
    ) as *mut libc::c_char;
    if dir.is_null() {
        return 0 as *mut libc::c_char;
    }
    memcpy(dir as *mut libc::c_void, file as *const libc::c_void, length);
    if append_dot {
        let fresh0 = length;
        length = length.wrapping_add(1);
        *dir.offset(fresh0 as isize) = '.' as i32 as libc::c_char;
    }
    *dir.offset(length as isize) = '\0' as i32 as libc::c_char;
    return dir;
}
