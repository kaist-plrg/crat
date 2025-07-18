use ::libc;
extern "C" {
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn ecalloc(count: libc::c_int, size: libc::c_uint) -> *mut libc::c_void;
    fn lrealpath(path: *mut libc::c_char) -> *mut libc::c_char;
    fn unmark(ifile: *mut libc::c_void);
    fn mark_check_ifile(ifile: *mut libc::c_void);
    static mut curr_ifile: *mut libc::c_void;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ifile {
    pub h_next: *mut ifile,
    pub h_prev: *mut ifile,
    pub h_filename: *mut libc::c_char,
    pub h_rfilename: *mut libc::c_char,
    pub h_filestate: *mut libc::c_void,
    pub h_index: libc::c_int,
    pub h_hold: libc::c_int,
    pub h_opened: libc::c_char,
    pub h_scrpos: scrpos,
    pub h_altpipe: *mut libc::c_void,
    pub h_altfilename: *mut libc::c_char,
}
static mut anchor: ifile = unsafe {
    {
        let mut init = ifile {
            h_next: &anchor as *const ifile as *mut ifile,
            h_prev: &anchor as *const ifile as *mut ifile,
            h_filename: 0 as *const libc::c_char as *mut libc::c_char,
            h_rfilename: 0 as *const libc::c_char as *mut libc::c_char,
            h_filestate: 0 as *const libc::c_void as *mut libc::c_void,
            h_index: 0 as libc::c_int,
            h_hold: 0 as libc::c_int,
            h_opened: '\0' as i32 as libc::c_char,
            h_scrpos: {
                let mut init = scrpos {
                    pos: -(1 as libc::c_int) as POSITION,
                    ln: 0 as libc::c_int,
                };
                init
            },
            h_altpipe: 0 as *const libc::c_void as *mut libc::c_void,
            h_altfilename: 0 as *const libc::c_char as *mut libc::c_char,
        };
        init
    }
};
static mut ifiles: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn incr_index(mut p: *mut ifile, mut incr: libc::c_int) {
    while p != &mut anchor as *mut ifile {
        (*p).h_index += incr;
        p = (*p).h_next;
    }
}
unsafe extern "C" fn link_ifile(mut p: *mut ifile, mut prev: *mut ifile) {
    if prev.is_null() {
        prev = &mut anchor;
    }
    (*p).h_next = (*prev).h_next;
    (*p).h_prev = prev;
    (*(*prev).h_next).h_prev = p;
    (*prev).h_next = p;
    (*p).h_index = (*prev).h_index + 1 as libc::c_int;
    incr_index((*p).h_next, 1 as libc::c_int);
    ifiles += 1;
    ifiles;
}
unsafe extern "C" fn unlink_ifile(mut p: *mut ifile) {
    (*(*p).h_next).h_prev = (*p).h_prev;
    (*(*p).h_prev).h_next = (*p).h_next;
    incr_index((*p).h_next, -(1 as libc::c_int));
    ifiles -= 1;
    ifiles;
}
unsafe extern "C" fn new_ifile(
    mut filename: *mut libc::c_char,
    mut prev: *mut ifile,
) -> *mut ifile {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = ecalloc(
        1 as libc::c_int,
        ::std::mem::size_of::<ifile>() as libc::c_ulong as libc::c_uint,
    ) as *mut ifile;
    (*p).h_filename = save(filename);
    (*p).h_rfilename = lrealpath(filename);
    (*p).h_scrpos.pos = -(1 as libc::c_int) as POSITION;
    (*p).h_opened = 0 as libc::c_int as libc::c_char;
    (*p).h_hold = 0 as libc::c_int;
    (*p).h_filestate = 0 as *mut libc::c_void;
    (*p).h_altfilename = 0 as *mut libc::c_char;
    (*p).h_altpipe = 0 as *mut libc::c_void;
    link_ifile(p, prev);
    mark_check_ifile(p as *mut libc::c_void);
    return p;
}
pub unsafe extern "C" fn del_ifile(mut h: *mut libc::c_void) {
    let mut p: *mut ifile = 0 as *mut ifile;
    if h == 0 as *mut libc::c_void {
        return;
    }
    unmark(h);
    if h == curr_ifile {
        curr_ifile = getoff_ifile(curr_ifile);
    }
    p = h as *mut ifile;
    unlink_ifile(p);
    free((*p).h_rfilename as *mut libc::c_void);
    free((*p).h_filename as *mut libc::c_void);
    free(p as *mut libc::c_void);
}
pub unsafe extern "C" fn next_ifile(mut h: *mut libc::c_void) -> *mut libc::c_void {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = if h == 0 as *mut libc::c_void { &mut anchor } else { h as *mut ifile };
    if (*p).h_next == &mut anchor as *mut ifile {
        return 0 as *mut libc::c_void;
    }
    return (*p).h_next as *mut libc::c_void;
}
pub unsafe extern "C" fn prev_ifile(mut h: *mut libc::c_void) -> *mut libc::c_void {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = if h == 0 as *mut libc::c_void { &mut anchor } else { h as *mut ifile };
    if (*p).h_prev == &mut anchor as *mut ifile {
        return 0 as *mut libc::c_void;
    }
    return (*p).h_prev as *mut libc::c_void;
}
pub unsafe extern "C" fn getoff_ifile(
    mut ifile: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut newifile: *mut libc::c_void = 0 as *mut libc::c_void;
    newifile = prev_ifile(ifile);
    if newifile != 0 as *mut libc::c_void {
        return newifile;
    }
    newifile = next_ifile(ifile);
    if newifile != 0 as *mut libc::c_void {
        return newifile;
    }
    return 0 as *mut libc::c_void;
}
pub unsafe extern "C" fn nifile() -> libc::c_int {
    return ifiles;
}
unsafe extern "C" fn find_ifile(mut filename: *mut libc::c_char) -> *mut ifile {
    let mut p: *mut ifile = 0 as *mut ifile;
    let mut rfilename: *mut libc::c_char = lrealpath(filename);
    p = anchor.h_next;
    while p != &mut anchor as *mut ifile {
        if strcmp(rfilename, (*p).h_rfilename) == 0 as libc::c_int {
            if strlen(filename) < strlen((*p).h_filename) {
                free((*p).h_filename as *mut libc::c_void);
                (*p).h_filename = save(filename);
            }
            break;
        } else {
            p = (*p).h_next;
        }
    }
    free(rfilename as *mut libc::c_void);
    if p == &mut anchor as *mut ifile {
        p = 0 as *mut ifile;
    }
    return p;
}
pub unsafe extern "C" fn get_ifile(
    mut filename: *mut libc::c_char,
    mut prev: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut p: *mut ifile = 0 as *mut ifile;
    p = find_ifile(filename);
    if p.is_null() {
        p = new_ifile(filename, prev as *mut ifile);
    }
    return p as *mut libc::c_void;
}
pub unsafe extern "C" fn get_filename(
    mut ifile: *mut libc::c_void,
) -> *mut libc::c_char {
    if ifile.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*(ifile as *mut ifile)).h_filename;
}
pub unsafe extern "C" fn get_real_filename(
    mut ifile: *mut libc::c_void,
) -> *mut libc::c_char {
    if ifile.is_null() {
        return 0 as *mut libc::c_char;
    }
    return (*(ifile as *mut ifile)).h_rfilename;
}
pub unsafe extern "C" fn get_index(mut ifile: *mut libc::c_void) -> libc::c_int {
    return (*(ifile as *mut ifile)).h_index;
}
pub unsafe extern "C" fn store_pos(
    mut ifile: *mut libc::c_void,
    mut scrpos: *mut scrpos,
) {
    (*(ifile as *mut ifile)).h_scrpos = *scrpos;
}
pub unsafe extern "C" fn get_pos(mut ifile: *mut libc::c_void, mut scrpos: *mut scrpos) {
    *scrpos = (*(ifile as *mut ifile)).h_scrpos;
}
pub unsafe extern "C" fn set_open(mut ifile: *mut libc::c_void) {
    (*(ifile as *mut ifile)).h_opened = 1 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn opened(mut ifile: *mut libc::c_void) -> libc::c_int {
    return (*(ifile as *mut ifile)).h_opened as libc::c_int;
}
pub unsafe extern "C" fn hold_ifile(
    mut ifile: *mut libc::c_void,
    mut incr: libc::c_int,
) {
    (*(ifile as *mut ifile)).h_hold += incr;
}
pub unsafe extern "C" fn held_ifile(mut ifile: *mut libc::c_void) -> libc::c_int {
    return (*(ifile as *mut ifile)).h_hold;
}
pub unsafe extern "C" fn get_filestate(
    mut ifile: *mut libc::c_void,
) -> *mut libc::c_void {
    return (*(ifile as *mut ifile)).h_filestate;
}
pub unsafe extern "C" fn set_filestate(
    mut ifile: *mut libc::c_void,
    mut filestate: *mut libc::c_void,
) {
    let ref mut fresh0 = (*(ifile as *mut ifile)).h_filestate;
    *fresh0 = filestate;
}
pub unsafe extern "C" fn set_altpipe(
    mut ifile: *mut libc::c_void,
    mut p: *mut libc::c_void,
) {
    let ref mut fresh1 = (*(ifile as *mut ifile)).h_altpipe;
    *fresh1 = p;
}
pub unsafe extern "C" fn get_altpipe(mut ifile: *mut libc::c_void) -> *mut libc::c_void {
    return (*(ifile as *mut ifile)).h_altpipe;
}
pub unsafe extern "C" fn set_altfilename(
    mut ifile: *mut libc::c_void,
    mut altfilename: *mut libc::c_char,
) {
    let mut p: *mut ifile = ifile as *mut ifile;
    if !((*p).h_altfilename).is_null() && (*p).h_altfilename != altfilename {
        free((*p).h_altfilename as *mut libc::c_void);
    }
    (*p).h_altfilename = altfilename;
}
pub unsafe extern "C" fn get_altfilename(
    mut ifile: *mut libc::c_void,
) -> *mut libc::c_char {
    return (*(ifile as *mut ifile)).h_altfilename;
}
