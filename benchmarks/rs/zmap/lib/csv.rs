use ::libc;
extern "C" {
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strndup(_: *const libc::c_char, _: libc::c_ulong) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
}
pub type size_t = libc::c_ulong;
pub unsafe extern "C" fn csv_find_index(
    mut header: *mut libc::c_char,
    mut names: *mut *const libc::c_char,
    mut names_len: size_t,
) -> libc::c_int {
    let mut split: *mut libc::c_char = header;
    let mut idx: libc::c_int = 0 as libc::c_int;
    while !split.is_null() {
        let mut front: *mut libc::c_char = if idx == 0 as libc::c_int {
            split
        } else {
            split.offset(1 as libc::c_int as isize)
        };
        let mut i: size_t = 0 as libc::c_int as size_t;
        while i < names_len {
            if strncmp(
                front,
                *names.offset(i as isize),
                strlen(*names.offset(i as isize)),
            ) == 0 as libc::c_int
            {
                return idx;
            }
            i = i.wrapping_add(1);
            i;
        }
        split = strchr(front, ',' as i32);
        idx += 1;
        idx;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn csv_get_index(
    mut row: *mut libc::c_char,
    mut idx: size_t,
) -> *mut libc::c_char {
    let mut split: *mut libc::c_char = row;
    let mut i: size_t = 0 as libc::c_int as size_t;
    while i < idx {
        split = strchr(split.offset(1 as libc::c_int as isize), ',' as i32);
        if split.is_null() {
            return 0 as *mut libc::c_char;
        }
        i = i.wrapping_add(1);
        i;
    }
    let mut entry: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut start: *mut libc::c_char = if idx == 0 as libc::c_int as libc::c_ulong {
        split
    } else {
        split.offset(1 as libc::c_int as isize)
    };
    let mut end: *mut libc::c_char = strchr(start, ',' as i32);
    if !end.is_null() {
        entry = strndup(start, end.offset_from(start) as libc::c_long as libc::c_ulong);
    } else {
        entry = strdup(start);
    }
    return entry;
}
