use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn free(__ptr: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn save(s: *const libc::c_char) -> *mut libc::c_char;
    fn lstrtoi(
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn lstrtopos(
        _: *mut libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> POSITION;
    fn postoa(_: POSITION, _: *mut libc::c_char, _: libc::c_int);
    fn bell();
    fn ch_end_seek() -> libc::c_int;
    fn ch_tell() -> POSITION;
    fn ch_getflags() -> libc::c_int;
    fn edit_ifile(ifile: *mut libc::c_void) -> libc::c_int;
    fn lrealpath(path: *mut libc::c_char) -> *mut libc::c_char;
    fn prev_ifile(h: *mut libc::c_void) -> *mut libc::c_void;
    fn get_ifile(
        filename: *mut libc::c_char,
        prev: *mut libc::c_void,
    ) -> *mut libc::c_void;
    fn get_real_filename(ifile: *mut libc::c_void) -> *mut libc::c_char;
    fn jump_loc(pos: POSITION, sline: libc::c_int);
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn get_scrpos(scrpos: *mut scrpos, where_0: libc::c_int);
    static mut curr_ifile: *mut libc::c_void;
    static mut sc_height: libc::c_int;
    static mut jump_sline: libc::c_int;
    static mut perma_marks: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type off_t = __off_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type LWCHAR = libc::c_ulong;
pub type POSITION = off_t;
pub type LINENUM = off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct scrpos {
    pub pos: POSITION,
    pub ln: libc::c_int,
}
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
pub struct mark {
    pub m_letter: libc::c_char,
    pub m_ifile: *mut libc::c_void,
    pub m_filename: *mut libc::c_char,
    pub m_scrpos: scrpos,
}
static mut marks: [mark; 54] = [mark {
    m_letter: 0,
    m_ifile: 0 as *const libc::c_void as *mut libc::c_void,
    m_filename: 0 as *const libc::c_char as *mut libc::c_char,
    m_scrpos: scrpos { pos: 0, ln: 0 },
}; 54];
pub static mut marks_modified: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn cmark(
    mut m: *mut mark,
    mut ifile: *mut libc::c_void,
    mut pos: POSITION,
    mut ln: libc::c_int,
) {
    (*m).m_ifile = ifile;
    (*m).m_scrpos.pos = pos;
    (*m).m_scrpos.ln = ln;
    if !((*m).m_filename).is_null() {
        free((*m).m_filename as *mut libc::c_void);
    }
    (*m).m_filename = 0 as *mut libc::c_char;
}
pub unsafe extern "C" fn init_mark() {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int {
        let mut letter: libc::c_char = 0;
        match i {
            52 => {
                letter = '#' as i32 as libc::c_char;
            }
            53 => {
                letter = '\'' as i32 as libc::c_char;
            }
            _ => {
                letter = (if i < 26 as libc::c_int {
                    'a' as i32 + i
                } else {
                    'A' as i32 + i - 26 as libc::c_int
                }) as libc::c_char;
            }
        }
        marks[i as usize].m_letter = letter;
        cmark(
            &mut *marks.as_mut_ptr().offset(i as isize),
            0 as *mut libc::c_void,
            -(1 as libc::c_int) as POSITION,
            -(1 as libc::c_int),
        );
        i += 1;
        i;
    }
}
unsafe extern "C" fn mark_set_ifile(mut m: *mut mark, mut ifile: *mut libc::c_void) {
    (*m).m_ifile = ifile;
    free((*m).m_filename as *mut libc::c_void);
    (*m).m_filename = 0 as *mut libc::c_char;
}
unsafe extern "C" fn mark_get_ifile(mut m: *mut mark) {
    if (*m).m_ifile != 0 as *mut libc::c_void {
        return;
    }
    mark_set_ifile(m, get_ifile((*m).m_filename, prev_ifile(0 as *mut libc::c_void)));
}
unsafe extern "C" fn getumark(mut c: LWCHAR) -> *mut mark {
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    if c >= 'a' as i32 as libc::c_ulong && c <= 'z' as i32 as libc::c_ulong {
        return &mut *marks
            .as_mut_ptr()
            .offset(c.wrapping_sub('a' as i32 as libc::c_ulong) as isize) as *mut mark;
    }
    if c >= 'A' as i32 as libc::c_ulong && c <= 'Z' as i32 as libc::c_ulong {
        return &mut *marks
            .as_mut_ptr()
            .offset(
                c
                    .wrapping_sub('A' as i32 as libc::c_ulong)
                    .wrapping_add(26 as libc::c_int as libc::c_ulong) as isize,
            ) as *mut mark;
    }
    if c == '\'' as i32 as libc::c_ulong {
        return &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int
                    - 1 as libc::c_int) as isize,
            ) as *mut mark;
    }
    if c == '#' as i32 as libc::c_ulong {
        return &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int
                    - 2 as libc::c_int) as isize,
            ) as *mut mark;
    }
    parg.p_char = c as libc::c_char;
    error(
        b"Invalid mark letter %c\0" as *const u8 as *const libc::c_char
            as *mut libc::c_char,
        &mut parg,
    );
    return 0 as *mut mark;
}
unsafe extern "C" fn getmark(mut c: LWCHAR) -> *mut mark {
    let mut m: *mut mark = 0 as *mut mark;
    static mut sm: mark = mark {
        m_letter: 0,
        m_ifile: 0 as *const libc::c_void as *mut libc::c_void,
        m_filename: 0 as *const libc::c_char as *mut libc::c_char,
        m_scrpos: scrpos { pos: 0, ln: 0 },
    };
    match c {
        94 => {
            m = &mut sm;
            cmark(m, curr_ifile, 0 as libc::c_int as POSITION, 0 as libc::c_int);
        }
        36 => {
            if ch_end_seek() != 0 {
                error(
                    b"Cannot seek to end of file\0" as *const u8 as *const libc::c_char
                        as *mut libc::c_char,
                    0 as *mut libc::c_void as *mut PARG,
                );
                return 0 as *mut mark;
            }
            m = &mut sm;
            cmark(m, curr_ifile, ch_tell(), sc_height);
        }
        46 => {
            m = &mut sm;
            get_scrpos(&mut (*m).m_scrpos, 0 as libc::c_int);
            cmark(m, curr_ifile, (*m).m_scrpos.pos, (*m).m_scrpos.ln);
        }
        39 => {
            m = &mut *marks
                .as_mut_ptr()
                .offset(
                    (2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int
                        - 1 as libc::c_int) as isize,
                ) as *mut mark;
        }
        _ => {
            m = getumark(c);
            if !m.is_null() {
                if (*m).m_scrpos.pos == -(1 as libc::c_int) as POSITION {
                    error(
                        b"Mark not set\0" as *const u8 as *const libc::c_char
                            as *mut libc::c_char,
                        0 as *mut libc::c_void as *mut PARG,
                    );
                    return 0 as *mut mark;
                }
            }
        }
    }
    return m;
}
pub unsafe extern "C" fn badmark(mut c: LWCHAR) -> libc::c_int {
    return (getmark(c) == 0 as *mut libc::c_void as *mut mark) as libc::c_int;
}
pub unsafe extern "C" fn setmark(mut c: LWCHAR, mut where_0: libc::c_int) {
    let mut m: *mut mark = 0 as *mut mark;
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    m = getumark(c);
    if m.is_null() {
        return;
    }
    get_scrpos(&mut scrpos, where_0);
    if scrpos.pos == -(1 as libc::c_int) as POSITION {
        bell();
        return;
    }
    cmark(m, curr_ifile, scrpos.pos, scrpos.ln);
    marks_modified = 1 as libc::c_int;
}
pub unsafe extern "C" fn clrmark(mut c: LWCHAR) {
    let mut m: *mut mark = 0 as *mut mark;
    m = getumark(c);
    if m.is_null() {
        return;
    }
    if (*m).m_scrpos.pos == -(1 as libc::c_int) as POSITION {
        bell();
        return;
    }
    (*m).m_scrpos.pos = -(1 as libc::c_int) as POSITION;
    marks_modified = 1 as libc::c_int;
}
pub unsafe extern "C" fn lastmark() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    if ch_getflags() & 0o10 as libc::c_int != 0 {
        return;
    }
    get_scrpos(&mut scrpos, 0 as libc::c_int);
    if scrpos.pos == -(1 as libc::c_int) as POSITION {
        return;
    }
    cmark(
        &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int
                    - 1 as libc::c_int) as isize,
            ),
        curr_ifile,
        scrpos.pos,
        scrpos.ln,
    );
    marks_modified = 1 as libc::c_int;
}
pub unsafe extern "C" fn gomark(mut c: LWCHAR) {
    let mut m: *mut mark = 0 as *mut mark;
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    m = getmark(c);
    if m.is_null() {
        return;
    }
    if m
        == &mut *marks
            .as_mut_ptr()
            .offset(
                (2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int
                    - 1 as libc::c_int) as isize,
            ) as *mut mark && (*m).m_scrpos.pos == -(1 as libc::c_int) as POSITION
    {
        cmark(m, curr_ifile, 0 as libc::c_int as POSITION, jump_sline);
    }
    mark_get_ifile(m);
    scrpos = (*m).m_scrpos;
    if (*m).m_ifile != curr_ifile {
        if edit_ifile((*m).m_ifile) != 0 {
            return;
        }
    }
    jump_loc(scrpos.pos, scrpos.ln);
}
pub unsafe extern "C" fn markpos(mut c: LWCHAR) -> POSITION {
    let mut m: *mut mark = 0 as *mut mark;
    m = getmark(c);
    if m.is_null() {
        return -(1 as libc::c_int) as POSITION;
    }
    if (*m).m_ifile != curr_ifile {
        error(
            b"Mark not in current file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return -(1 as libc::c_int) as POSITION;
    }
    return (*m).m_scrpos.pos;
}
pub unsafe extern "C" fn posmark(mut pos: POSITION) -> libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * 26 as libc::c_int + 1 as libc::c_int {
        if marks[i as usize].m_ifile == curr_ifile
            && marks[i as usize].m_scrpos.pos == pos
        {
            if i < 26 as libc::c_int {
                return ('a' as i32 + i) as libc::c_char;
            }
            if i < 26 as libc::c_int * 2 as libc::c_int {
                return ('A' as i32 + (i - 26 as libc::c_int)) as libc::c_char;
            }
            return '#' as i32 as libc::c_char;
        }
        i += 1;
        i;
    }
    return 0 as libc::c_int as libc::c_char;
}
pub unsafe extern "C" fn unmark(mut ifile: *mut libc::c_void) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int {
        if marks[i as usize].m_ifile == ifile {
            marks[i as usize].m_scrpos.pos = -(1 as libc::c_int) as POSITION;
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn mark_check_ifile(mut ifile: *mut libc::c_void) {
    let mut i: libc::c_int = 0;
    let mut filename: *mut libc::c_char = get_real_filename(ifile);
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int {
        let mut m: *mut mark = &mut *marks.as_mut_ptr().offset(i as isize) as *mut mark;
        let mut mark_filename: *mut libc::c_char = (*m).m_filename;
        if !mark_filename.is_null() {
            mark_filename = lrealpath(mark_filename);
            if strcmp(filename, mark_filename) == 0 as libc::c_int {
                mark_set_ifile(m, ifile);
            }
            free(mark_filename as *mut libc::c_void);
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn save_marks(mut fout: *mut FILE, mut hdr: *mut libc::c_char) {
    let mut i: libc::c_int = 0;
    if perma_marks == 0 {
        return;
    }
    fprintf(fout, b"%s\n\0" as *const u8 as *const libc::c_char, hdr);
    i = 0 as libc::c_int;
    while i < 2 as libc::c_int * 26 as libc::c_int + 2 as libc::c_int {
        let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
        let mut m: *mut mark = &mut *marks.as_mut_ptr().offset(i as isize) as *mut mark;
        let mut pos_str: [libc::c_char; 23] = [0; 23];
        if !((*m).m_scrpos.pos == -(1 as libc::c_int) as POSITION) {
            postoa((*m).m_scrpos.pos, pos_str.as_mut_ptr(), 10 as libc::c_int);
            filename = (*m).m_filename;
            if filename.is_null() {
                filename = get_real_filename((*m).m_ifile);
            }
            if strcmp(filename, b"-\0" as *const u8 as *const libc::c_char)
                != 0 as libc::c_int
            {
                fprintf(
                    fout,
                    b"m %c %d %s %s\n\0" as *const u8 as *const libc::c_char,
                    (*m).m_letter as libc::c_int,
                    (*m).m_scrpos.ln,
                    pos_str.as_mut_ptr(),
                    filename,
                );
            }
        }
        i += 1;
        i;
    }
}
pub unsafe extern "C" fn restore_mark(mut line: *mut libc::c_char) {
    let mut m: *mut mark = 0 as *mut mark;
    let mut ln: libc::c_int = 0;
    let mut pos: POSITION = 0;
    let fresh0 = line;
    line = line.offset(1);
    if *fresh0 as libc::c_int != 'm' as i32 {
        return;
    }
    while *line as libc::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    let fresh1 = line;
    line = line.offset(1);
    m = getumark(*fresh1 as LWCHAR);
    if m.is_null() {
        return;
    }
    while *line as libc::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    ln = lstrtoi(line, &mut line, 10 as libc::c_int);
    if ln < 0 as libc::c_int {
        return;
    }
    if ln < 1 as libc::c_int {
        ln = 1 as libc::c_int;
    }
    if ln > sc_height {
        ln = sc_height;
    }
    while *line as libc::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    pos = lstrtopos(line, &mut line, 10 as libc::c_int);
    if pos < 0 as libc::c_int as libc::c_long {
        return;
    }
    while *line as libc::c_int == ' ' as i32 {
        line = line.offset(1);
        line;
    }
    cmark(m, 0 as *mut libc::c_void, pos, ln);
    (*m).m_filename = save(line);
}
