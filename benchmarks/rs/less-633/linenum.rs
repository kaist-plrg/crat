use ::libc;
extern "C" {
    fn ch_seek(pos: POSITION) -> libc::c_int;
    fn ch_length() -> POSITION;
    fn forw_raw_line(
        curr_pos: POSITION,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> POSITION;
    fn back_raw_line(
        curr_pos: POSITION,
        linep: *mut *mut libc::c_char,
        line_lenp: *mut libc::c_int,
    ) -> POSITION;
    fn position(sindex: libc::c_int) -> POSITION;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn get_time() -> time_t;
    fn ierror(fmt: *mut libc::c_char, parg: *mut PARG);
    static mut linenums: libc::c_int;
    static mut sigs: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut screen_trashed: libc::c_int;
    static mut header_lines: libc::c_int;
    static mut nonum_headers: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union parg {
    pub p_string: *mut libc::c_char,
    pub p_int: libc::c_int,
    pub p_linenum: LINENUM,
    pub p_char: libc::c_char,
}
pub type PARG = parg;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct linenum_info {
    pub next: *mut linenum_info,
    pub prev: *mut linenum_info,
    pub pos: POSITION,
    pub gap: POSITION,
    pub line: LINENUM,
}
static mut anchor: linenum_info = linenum_info {
    next: 0 as *const linenum_info as *mut linenum_info,
    prev: 0 as *const linenum_info as *mut linenum_info,
    pos: 0,
    gap: 0,
    line: 0,
};
static mut freelist: *mut linenum_info = 0 as *const linenum_info as *mut linenum_info;
static mut pool: [linenum_info; 200] = [linenum_info {
    next: 0 as *const linenum_info as *mut linenum_info,
    prev: 0 as *const linenum_info as *mut linenum_info,
    pos: 0,
    gap: 0,
    line: 0,
}; 200];
static mut spare: *mut linenum_info = 0 as *const linenum_info as *mut linenum_info;
pub static mut scanning_eof: libc::c_int = 0 as libc::c_int;
pub unsafe extern "C" fn clr_linenum() {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    p = pool.as_mut_ptr();
    while p
        < &mut *pool
            .as_mut_ptr()
            .offset((200 as libc::c_int - 2 as libc::c_int) as isize)
            as *mut linenum_info
    {
        (*p).next = p.offset(1 as libc::c_int as isize);
        p = p.offset(1);
        p;
    }
    pool[(200 as libc::c_int - 2 as libc::c_int) as usize].next = 0 as *mut linenum_info;
    freelist = pool.as_mut_ptr();
    spare = &mut *pool
        .as_mut_ptr()
        .offset((200 as libc::c_int - 1 as libc::c_int) as isize) as *mut linenum_info;
    anchor.prev = &mut anchor;
    anchor.next = anchor.prev;
    anchor.gap = 0 as libc::c_int as POSITION;
    anchor.pos = 0 as libc::c_int as POSITION;
    anchor.line = 1 as libc::c_int as LINENUM;
}
unsafe extern "C" fn calcgap(mut p: *mut linenum_info) {
    if p == &mut anchor as *mut linenum_info
        || (*p).next == &mut anchor as *mut linenum_info
    {
        return;
    }
    (*p).gap = (*(*p).next).pos - (*(*p).prev).pos;
}
pub unsafe extern "C" fn add_lnum(mut linenum: LINENUM, mut pos: POSITION) {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    let mut new: *mut linenum_info = 0 as *mut linenum_info;
    let mut nextp: *mut linenum_info = 0 as *mut linenum_info;
    let mut prevp: *mut linenum_info = 0 as *mut linenum_info;
    let mut mingap: POSITION = 0;
    p = anchor.next;
    while p != &mut anchor as *mut linenum_info && (*p).pos < pos {
        if (*p).line == linenum {
            return;
        }
        p = (*p).next;
    }
    nextp = p;
    prevp = (*p).prev;
    if !freelist.is_null() {
        new = freelist;
        freelist = (*freelist).next;
    } else {
        new = spare;
        spare = 0 as *mut linenum_info;
    }
    (*new).next = nextp;
    (*new).prev = prevp;
    (*new).pos = pos;
    (*new).line = linenum;
    (*nextp).prev = new;
    (*prevp).next = new;
    calcgap(new);
    calcgap(nextp);
    calcgap(prevp);
    if spare.is_null() {
        mingap = (*anchor.next).gap;
        p = anchor.next;
        while (*p).next != &mut anchor as *mut linenum_info {
            if (*p).gap <= mingap {
                spare = p;
                mingap = (*p).gap;
            }
            p = (*p).next;
        }
        (*(*spare).next).prev = (*spare).prev;
        (*(*spare).prev).next = (*spare).next;
    }
}
unsafe extern "C" fn longloopmessage() {
    ierror(
        b"Calculating line numbers\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void as *mut PARG,
    );
}
static mut loopcount: libc::c_int = 0;
static mut startime: time_t = 0;
unsafe extern "C" fn longish() {
    if loopcount >= 0 as libc::c_int
        && {
            loopcount += 1;
            loopcount > 100 as libc::c_int
        }
    {
        loopcount = 0 as libc::c_int;
        if get_time() >= startime + 2 as libc::c_int as libc::c_long {
            longloopmessage();
            loopcount = -(1 as libc::c_int);
        }
    }
}
unsafe extern "C" fn abort_long() {
    if loopcount >= 0 as libc::c_int {
        return;
    }
    if linenums == 2 as libc::c_int {
        screen_trashed = 1 as libc::c_int;
    }
    linenums = 0 as libc::c_int;
    error(
        b"Line numbers turned off\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void as *mut PARG,
    );
}
pub unsafe extern "C" fn find_linenum(mut pos: POSITION) -> LINENUM {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    let mut linenum: LINENUM = 0;
    let mut cpos: POSITION = 0;
    if linenums == 0 {
        return 0 as libc::c_int as LINENUM;
    }
    if pos == -(1 as libc::c_int) as POSITION {
        return 0 as libc::c_int as LINENUM;
    }
    if pos <= 0 as libc::c_int as POSITION {
        return 1 as libc::c_int as LINENUM;
    }
    p = anchor.next;
    while p != &mut anchor as *mut linenum_info && (*p).pos < pos {
        p = (*p).next;
    }
    if (*p).pos == pos {
        return (*p).line;
    }
    startime = get_time();
    loopcount = 0 as libc::c_int;
    if p == &mut anchor as *mut linenum_info || pos - (*(*p).prev).pos < (*p).pos - pos {
        p = (*p).prev;
        if ch_seek((*p).pos) != 0 {
            return 0 as libc::c_int as LINENUM;
        }
        linenum = (*p).line;
        cpos = (*p).pos;
        while cpos < pos {
            cpos = forw_raw_line(
                cpos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                abort_long();
                return 0 as libc::c_int as LINENUM;
            }
            if cpos == -(1 as libc::c_int) as POSITION {
                return 0 as libc::c_int as LINENUM;
            }
            longish();
            linenum += 1;
            linenum;
        }
        add_lnum(linenum, cpos);
        if cpos > pos {
            linenum -= 1;
            linenum;
        }
    } else {
        if ch_seek((*p).pos) != 0 {
            return 0 as libc::c_int as LINENUM;
        }
        linenum = (*p).line;
        cpos = (*p).pos;
        while cpos > pos {
            cpos = back_raw_line(
                cpos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                abort_long();
                return 0 as libc::c_int as LINENUM;
            }
            if cpos == -(1 as libc::c_int) as POSITION {
                return 0 as libc::c_int as LINENUM;
            }
            longish();
            linenum -= 1;
            linenum;
        }
        add_lnum(linenum, cpos);
    }
    loopcount = 0 as libc::c_int;
    return linenum;
}
pub unsafe extern "C" fn find_pos(mut linenum: LINENUM) -> POSITION {
    let mut p: *mut linenum_info = 0 as *mut linenum_info;
    let mut cpos: POSITION = 0;
    let mut clinenum: LINENUM = 0;
    if linenum <= 1 as libc::c_int as libc::c_long {
        return 0 as libc::c_int as POSITION;
    }
    p = anchor.next;
    while p != &mut anchor as *mut linenum_info && (*p).line < linenum {
        p = (*p).next;
    }
    if (*p).line == linenum {
        return (*p).pos;
    }
    if p == &mut anchor as *mut linenum_info
        || linenum - (*(*p).prev).line < (*p).line - linenum
    {
        p = (*p).prev;
        if ch_seek((*p).pos) != 0 {
            return -(1 as libc::c_int) as POSITION;
        }
        clinenum = (*p).line;
        cpos = (*p).pos;
        while clinenum < linenum {
            cpos = forw_raw_line(
                cpos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                return -(1 as libc::c_int) as POSITION;
            }
            if cpos == -(1 as libc::c_int) as POSITION {
                return -(1 as libc::c_int) as POSITION;
            }
            clinenum += 1;
            clinenum;
        }
    } else {
        if ch_seek((*p).pos) != 0 {
            return -(1 as libc::c_int) as POSITION;
        }
        clinenum = (*p).line;
        cpos = (*p).pos;
        while clinenum > linenum {
            cpos = back_raw_line(
                cpos,
                0 as *mut libc::c_void as *mut *mut libc::c_char,
                0 as *mut libc::c_void as *mut libc::c_int,
            );
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                return -(1 as libc::c_int) as POSITION;
            }
            if cpos == -(1 as libc::c_int) as POSITION {
                return -(1 as libc::c_int) as POSITION;
            }
            clinenum -= 1;
            clinenum;
        }
    }
    add_lnum(clinenum, cpos);
    return cpos;
}
pub unsafe extern "C" fn currline(mut where_0: libc::c_int) -> LINENUM {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    let mut linenum: LINENUM = 0;
    pos = position(where_0);
    len = ch_length();
    while pos == -(1 as libc::c_int) as POSITION && where_0 >= 0 as libc::c_int
        && where_0 < sc_height
    {
        where_0 += 1;
        pos = position(where_0);
    }
    if pos == -(1 as libc::c_int) as POSITION {
        pos = len;
    }
    linenum = find_linenum(pos);
    if pos == len {
        linenum -= 1;
        linenum;
    }
    return linenum;
}
pub unsafe extern "C" fn scan_eof() {
    let mut pos: POSITION = 0 as libc::c_int as POSITION;
    let mut linenum: LINENUM = 0 as libc::c_int as LINENUM;
    if ch_seek(0 as libc::c_int as POSITION) != 0 {
        return;
    }
    ierror(
        b"Determining length of file\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        0 as *mut libc::c_void as *mut PARG,
    );
    scanning_eof = 1 as libc::c_int;
    while pos != -(1 as libc::c_int) as POSITION {
        let fresh0 = linenum;
        linenum = linenum + 1;
        if fresh0 % 256 as libc::c_int as libc::c_long
            == 0 as libc::c_int as libc::c_long
        {
            add_lnum(linenum, pos);
        }
        pos = forw_raw_line(
            pos,
            0 as *mut libc::c_void as *mut *mut libc::c_char,
            0 as *mut libc::c_void as *mut libc::c_int,
        );
        if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
            break;
        }
    }
    scanning_eof = 0 as libc::c_int;
}
pub unsafe extern "C" fn vlinenum(mut linenum: LINENUM) -> LINENUM {
    if nonum_headers != 0 {
        linenum = if linenum < header_lines as libc::c_long {
            0 as libc::c_int as libc::c_long
        } else {
            linenum - header_lines as libc::c_long
        };
    }
    return linenum;
}
