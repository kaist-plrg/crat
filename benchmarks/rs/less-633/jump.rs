use ::libc;
extern "C" {
    fn home();
    fn clear();
    fn ch_seek(pos: POSITION) -> libc::c_int;
    fn ch_end_seek() -> libc::c_int;
    fn ch_end_buffer_seek() -> libc::c_int;
    fn ch_beg_seek() -> libc::c_int;
    fn ch_length() -> POSITION;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> libc::c_int;
    fn ch_back_get() -> libc::c_int;
    fn eof_bell();
    fn forw(
        n: libc::c_int,
        pos: POSITION,
        force: libc::c_int,
        only_last: libc::c_int,
        nblank: libc::c_int,
    );
    fn back(n: libc::c_int, pos: POSITION, force: libc::c_int, only_last: libc::c_int);
    fn forw_line(curr_pos: POSITION) -> POSITION;
    fn back_line(curr_pos: POSITION) -> POSITION;
    fn set_attnpos(pos: POSITION);
    fn add_back_pos(pos: POSITION);
    fn lastmark();
    fn repaint_hilite(on: libc::c_int);
    fn next_unfiltered(pos: POSITION) -> POSITION;
    fn position(sindex: libc::c_int) -> POSITION;
    fn error(fmt: *mut libc::c_char, parg: *mut PARG);
    fn onscreen(pos: POSITION) -> libc::c_int;
    fn sindex_from_sline(sline: libc::c_int) -> libc::c_int;
    fn pos_clear();
    fn get_scrpos(scrpos: *mut scrpos, where_0: libc::c_int);
    fn find_pos(linenum: LINENUM) -> POSITION;
    fn percent_pos(
        pos: POSITION,
        percent: libc::c_int,
        fraction: libc::c_long,
    ) -> POSITION;
    fn ierror(fmt: *mut libc::c_char, parg: *mut PARG);
    static mut jump_sline: libc::c_int;
    static mut squished: libc::c_int;
    static mut screen_trashed: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut show_attn: libc::c_int;
    static mut top_scroll: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
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
pub unsafe extern "C" fn jump_forw() {
    let mut pos: POSITION = 0;
    let mut end_pos: POSITION = 0;
    if ch_end_seek() != 0 {
        error(
            b"Cannot seek to end of file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    end_pos = ch_tell();
    if position(sc_height - 1 as libc::c_int) == end_pos {
        eof_bell();
        return;
    }
    lastmark();
    pos_clear();
    pos = back_line(end_pos);
    if pos == -(1 as libc::c_int) as POSITION {
        jump_loc(0 as libc::c_int as POSITION, sc_height - 1 as libc::c_int);
    } else {
        jump_loc(pos, sc_height - 1 as libc::c_int);
        if position(sc_height - 1 as libc::c_int) != end_pos {
            repaint();
        }
    };
}
pub unsafe extern "C" fn jump_forw_buffered() {
    let mut end: POSITION = 0;
    if ch_end_buffer_seek() != 0 {
        error(
            b"Cannot seek to end of buffers\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    end = ch_tell();
    if end != -(1 as libc::c_int) as POSITION && end > 0 as libc::c_int as libc::c_long {
        jump_line_loc(
            end - 1 as libc::c_int as libc::c_long,
            sc_height - 1 as libc::c_int,
        );
    }
}
pub unsafe extern "C" fn jump_back(mut linenum: LINENUM) {
    let mut pos: POSITION = 0;
    let mut parg: PARG = parg {
        p_string: 0 as *mut libc::c_char,
    };
    pos = find_pos(linenum);
    if pos != -(1 as libc::c_int) as POSITION && ch_seek(pos) == 0 as libc::c_int {
        if show_attn != 0 {
            set_attnpos(pos);
        }
        jump_loc(pos, jump_sline);
    } else if linenum <= 1 as libc::c_int as libc::c_long
        && ch_beg_seek() == 0 as libc::c_int
    {
        jump_loc(ch_tell(), jump_sline);
        error(
            b"Cannot seek to beginning of file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
    } else {
        parg.p_linenum = linenum;
        error(
            b"Cannot seek to line number %n\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            &mut parg,
        );
    };
}
pub unsafe extern "C" fn repaint() {
    let mut scrpos: scrpos = scrpos { pos: 0, ln: 0 };
    get_scrpos(&mut scrpos, 0 as libc::c_int);
    pos_clear();
    if scrpos.pos == -(1 as libc::c_int) as POSITION {
        jump_loc(0 as libc::c_int as POSITION, 1 as libc::c_int);
    } else {
        jump_loc(scrpos.pos, scrpos.ln);
    };
}
pub unsafe extern "C" fn jump_percent(
    mut percent: libc::c_int,
    mut fraction: libc::c_long,
) {
    let mut pos: POSITION = 0;
    let mut len: POSITION = 0;
    len = ch_length();
    if len == -(1 as libc::c_int) as POSITION {
        ierror(
            b"Determining length of file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        ch_end_seek();
    }
    len = ch_length();
    if len == -(1 as libc::c_int) as POSITION {
        error(
            b"Don't know length of file\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    pos = percent_pos(len, percent, fraction);
    if pos >= len {
        pos = len - 1 as libc::c_int as libc::c_long;
    }
    jump_line_loc(pos, jump_sline);
}
pub unsafe extern "C" fn jump_line_loc(mut pos: POSITION, mut sline: libc::c_int) {
    let mut c: libc::c_int = 0;
    if ch_seek(pos) == 0 as libc::c_int {
        loop {
            c = ch_back_get();
            if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
                break;
            }
        }
        if c == '\n' as i32 {
            ch_forw_get();
        }
        pos = ch_tell();
    }
    if show_attn != 0 {
        set_attnpos(pos);
    }
    jump_loc(pos, sline);
}
pub unsafe extern "C" fn jump_loc(mut pos: POSITION, mut sline: libc::c_int) {
    let mut nline: libc::c_int = 0;
    let mut sindex: libc::c_int = 0;
    let mut tpos: POSITION = 0;
    let mut bpos: POSITION = 0;
    sindex = sindex_from_sline(sline);
    nline = onscreen(pos);
    if nline >= 0 as libc::c_int {
        nline -= sindex;
        if nline > 0 as libc::c_int {
            forw(
                nline,
                position(-(2 as libc::c_int)),
                1 as libc::c_int,
                0 as libc::c_int,
                0 as libc::c_int,
            );
        } else {
            back(-nline, position(0 as libc::c_int), 1 as libc::c_int, 0 as libc::c_int);
        }
        if show_attn != 0 {
            repaint_hilite(1 as libc::c_int);
        }
        return;
    }
    if ch_seek(pos) != 0 {
        error(
            b"Cannot seek to that file position\0" as *const u8 as *const libc::c_char
                as *mut libc::c_char,
            0 as *mut libc::c_void as *mut PARG,
        );
        return;
    }
    tpos = position(0 as libc::c_int);
    bpos = position(-(2 as libc::c_int));
    if tpos == -(1 as libc::c_int) as POSITION || pos >= tpos {
        nline = 0 as libc::c_int;
        while nline < sindex {
            if bpos != -(1 as libc::c_int) as POSITION && pos <= bpos {
                forw(
                    sc_height - sindex + nline - 1 as libc::c_int,
                    bpos,
                    1 as libc::c_int,
                    0 as libc::c_int,
                    0 as libc::c_int,
                );
                if show_attn != 0 {
                    repaint_hilite(1 as libc::c_int);
                }
                return;
            }
            pos = back_line(pos);
            if pos == -(1 as libc::c_int) as POSITION {
                break;
            }
            nline += 1;
            nline;
        }
        lastmark();
        squished = 0 as libc::c_int;
        screen_trashed = 0 as libc::c_int;
        forw(
            sc_height - 1 as libc::c_int,
            pos,
            1 as libc::c_int,
            0 as libc::c_int,
            sindex - nline,
        );
    } else {
        nline = sindex;
        while nline < sc_height - 1 as libc::c_int {
            pos = forw_line(pos);
            if pos == -(1 as libc::c_int) as POSITION {
                break;
            }
            pos = next_unfiltered(pos);
            if pos >= tpos {
                back(nline + 1 as libc::c_int, tpos, 1 as libc::c_int, 0 as libc::c_int);
                if show_attn != 0 {
                    repaint_hilite(1 as libc::c_int);
                }
                return;
            }
            nline += 1;
            nline;
        }
        lastmark();
        if top_scroll == 0 {
            clear();
        } else {
            home();
        }
        screen_trashed = 0 as libc::c_int;
        add_back_pos(pos);
        back(sc_height - 1 as libc::c_int, pos, 1 as libc::c_int, 0 as libc::c_int);
    };
}
