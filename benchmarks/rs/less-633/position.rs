use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    static mut sc_height: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type POSITION = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: libc::c_int,
}
static mut table: *mut POSITION = 0 as *const POSITION as *mut POSITION;
static mut table_size: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn position(mut sindex: libc::c_int) -> POSITION {
    match sindex {
        -1 => {
            sindex = sc_height - 2 as libc::c_int;
        }
        -2 => {
            sindex = sc_height - 1 as libc::c_int;
        }
        -3 => {
            sindex = (sc_height - 1 as libc::c_int) / 2 as libc::c_int;
        }
        _ => {}
    }
    return *table.offset(sindex as isize);
}
pub unsafe extern "C" fn add_forw_pos(mut pos: POSITION) {
    let mut i: libc::c_int = 0;
    i = 1 as libc::c_int;
    while i < sc_height {
        *table.offset((i - 1 as libc::c_int) as isize) = *table.offset(i as isize);
        i += 1;
        i;
    }
    *table.offset((sc_height - 1 as libc::c_int) as isize) = pos;
}
pub unsafe extern "C" fn add_back_pos(mut pos: POSITION) {
    let mut i: libc::c_int = 0;
    i = sc_height - 1 as libc::c_int;
    while i > 0 as libc::c_int {
        *table.offset(i as isize) = *table.offset((i - 1 as libc::c_int) as isize);
        i -= 1;
        i;
    }
    *table.offset(0 as libc::c_int as isize) = pos;
}
pub unsafe extern "C" fn pos_clear() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < sc_height {
        *table.offset(i as isize) = -(1 as libc::c_int) as POSITION;
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn pos_init() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if sc_height <= table_size {
        return;
    }
    if !table.is_null() {
        get_scrpos(&mut scrpos, 0 as libc::c_int);
        free(table as *mut libc::c_char as *mut libc::c_void);
    } else {
        scrpos.pos = -(1 as libc::c_int) as POSITION;
    }
    table = ecalloc(
        sc_height,
        ::std::mem::size_of::<POSITION>() as libc::c_ulong as libc::c_uint,
    ) as *mut POSITION;
    table_size = sc_height;
    pos_clear();
    if scrpos.pos != -(1 as libc::c_int) as POSITION {
        *table.offset((scrpos.ln - 1 as libc::c_int) as isize) = scrpos.pos;
    }
}
pub unsafe extern "C" fn onscreen(mut pos: POSITION) -> libc::c_int {
    let mut i: libc::c_int = 0;
    if pos < *table.offset(0 as libc::c_int as isize) {
        return -(1 as libc::c_int);
    }
    i = 1 as libc::c_int;
    while i < sc_height {
        if pos < *table.offset(i as isize) {
            return i - 1 as libc::c_int;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
pub unsafe extern "C" fn empty_screen() -> libc::c_int {
    return empty_lines(0 as libc::c_int, sc_height - 1 as libc::c_int);
}
pub unsafe extern "C" fn empty_lines(
    mut s: libc::c_int,
    mut e: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = s;
    while i <= e {
        if *table.offset(i as isize) != -(1 as libc::c_int) as POSITION
            && *table.offset(i as isize) != 0 as libc::c_int as libc::c_long
        {
            return 0 as libc::c_int;
        }
        i += 1;
        i;
    }
    return 1 as libc::c_int;
}
pub unsafe extern "C" fn get_scrpos(mut scrpos: *mut scrpos, mut where_0: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut dir: libc::c_int = 0;
    let mut last: libc::c_int = 0;
    match where_0 {
        0 => {
            i = 0 as libc::c_int;
            dir = 1 as libc::c_int;
            last = sc_height - 2 as libc::c_int;
        }
        -1 | -2 => {
            i = sc_height - 2 as libc::c_int;
            dir = -(1 as libc::c_int);
            last = 0 as libc::c_int;
        }
        _ => {
            i = where_0;
            if *table.offset(i as isize) == -(1 as libc::c_int) as POSITION {
                (*scrpos).pos = -(1 as libc::c_int) as POSITION;
                return;
            }
        }
    }
    loop {
        if *table.offset(i as isize) != -(1 as libc::c_int) as POSITION {
            (*scrpos).ln = i + 1 as libc::c_int;
            (*scrpos).pos = *table.offset(i as isize);
            return;
        }
        if i == last {
            break;
        }
        i += dir;
    }
    (*scrpos).pos = -(1 as libc::c_int) as POSITION;
}
pub unsafe extern "C" fn sindex_from_sline(mut sline: libc::c_int) -> libc::c_int {
    if sline < 0 as libc::c_int {
        sline += sc_height;
    }
    if sline <= 0 as libc::c_int {
        sline = 1 as libc::c_int;
    }
    if sline > sc_height {
        sline = sc_height;
    }
    return sline - 1 as libc::c_int;
}
