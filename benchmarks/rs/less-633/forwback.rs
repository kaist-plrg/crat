use ::libc;
extern "C" {
    fn quit(status: libc::c_int);
    fn home();
    fn add_line();
    fn lower_left();
    fn vbell();
    fn bell();
    fn clear();
    fn clear_eol();
    fn ch_length() -> POSITION;
    fn edit_next(n: libc::c_int) -> libc::c_int;
    fn repaint();
    fn set_attr_line(a: libc::c_int);
    fn forw_line_seg(
        curr_pos: POSITION,
        skipeol: libc::c_int,
        rscroll: libc::c_int,
        nochop: libc::c_int,
    ) -> POSITION;
    fn line_pfx_width() -> libc::c_int;
    fn forw_line(curr_pos: POSITION) -> POSITION;
    fn currline(where_0: libc::c_int) -> LINENUM;
    fn jump_loc(pos: POSITION, sline: libc::c_int);
    fn back_line(curr_pos: POSITION) -> POSITION;
    fn ch_getflags() -> libc::c_int;
    fn get_quit_at_eof() -> libc::c_int;
    fn get_time() -> time_t;
    fn position(sindex: libc::c_int) -> POSITION;
    fn put_line();
    fn add_back_pos(pos: POSITION);
    fn empty_screen() -> libc::c_int;
    fn pos_clear();
    fn putstr(s: *const libc::c_char);
    fn empty_lines(s: libc::c_int, e: libc::c_int) -> libc::c_int;
    fn add_forw_pos(pos: POSITION);
    fn onscreen(pos: POSITION) -> libc::c_int;
    fn putchr(c: libc::c_int) -> libc::c_int;
    fn next_unfiltered(pos: POSITION) -> POSITION;
    fn is_filtering() -> libc::c_int;
    fn prep_hilite(spos: POSITION, epos: POSITION, maxlines: libc::c_int);
    fn prev_unfiltered(pos: POSITION) -> POSITION;
    static mut top_scroll: libc::c_int;
    static mut quiet: libc::c_int;
    static mut sc_width: libc::c_int;
    static mut sc_height: libc::c_int;
    static mut hshift: libc::c_int;
    static mut auto_wrap: libc::c_int;
    static mut plusoption: libc::c_int;
    static mut forw_scroll: libc::c_int;
    static mut back_scroll: libc::c_int;
    static mut ignore_eoi: libc::c_int;
    static mut header_lines: libc::c_int;
    static mut header_cols: libc::c_int;
    static mut full_screen: libc::c_int;
    static mut size_linebuf: libc::c_int;
    static mut hilite_search: libc::c_int;
    static mut status_col: libc::c_int;
    static mut tagoption: *mut libc::c_char;
}
pub type __off_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type off_t = __off_t;
pub type time_t = __time_t;
pub type POSITION = off_t;
pub type LINENUM = off_t;
pub static mut screen_trashed: libc::c_int = 0;
pub static mut squished: libc::c_int = 0;
pub static mut no_back_scroll: libc::c_int = 0 as libc::c_int;
pub static mut forw_prompt: libc::c_int = 0;
pub static mut first_time: libc::c_int = 1 as libc::c_int;
pub unsafe extern "C" fn eof_bell() {
    static mut last_eof_bell: time_t = 0 as libc::c_int as time_t;
    let mut now: time_t = get_time();
    if now == last_eof_bell {
        return;
    }
    last_eof_bell = now;
    if quiet == 0 as libc::c_int {
        bell();
    } else {
        vbell();
    };
}
pub unsafe extern "C" fn eof_displayed() -> libc::c_int {
    let mut pos: POSITION = 0;
    if ignore_eoi != 0 {
        return 0 as libc::c_int;
    }
    if ch_length() == -(1 as libc::c_int) as POSITION {
        return 0 as libc::c_int;
    }
    pos = position(-(2 as libc::c_int));
    return (pos == -(1 as libc::c_int) as POSITION || pos == ch_length()) as libc::c_int;
}
pub unsafe extern "C" fn entire_file_displayed() -> libc::c_int {
    let mut pos: POSITION = 0;
    if eof_displayed() == 0 {
        return 0 as libc::c_int;
    }
    pos = position(0 as libc::c_int);
    return (pos == -(1 as libc::c_int) as POSITION
        || pos == 0 as libc::c_int as libc::c_long) as libc::c_int;
}
pub unsafe extern "C" fn squish_check() {
    if squished == 0 {
        return;
    }
    squished = 0 as libc::c_int;
    repaint();
}
unsafe extern "C" fn forw_line_pfx(
    mut pos: POSITION,
    mut pfx: libc::c_int,
    mut skipeol: libc::c_int,
) -> POSITION {
    let mut save_sc_width: libc::c_int = sc_width;
    let mut save_auto_wrap: libc::c_int = auto_wrap;
    let mut save_hshift: libc::c_int = hshift;
    sc_width = pfx + line_pfx_width();
    auto_wrap = 0 as libc::c_int;
    hshift = 0 as libc::c_int;
    pos = forw_line_seg(pos, skipeol, 0 as libc::c_int, 0 as libc::c_int);
    sc_width = save_sc_width;
    auto_wrap = save_auto_wrap;
    hshift = save_hshift;
    return pos;
}
unsafe extern "C" fn set_attr_header(mut ln: libc::c_int) {
    set_attr_line((9 as libc::c_int) << 8 as libc::c_int);
    if ln + 1 as libc::c_int == header_lines
        && position(0 as libc::c_int) != 0 as libc::c_int as POSITION
    {
        set_attr_line((1 as libc::c_int) << 0 as libc::c_int);
    }
}
pub unsafe extern "C" fn overlay_header() -> libc::c_int {
    let mut pos: POSITION = 0 as libc::c_int as POSITION;
    let mut ln: libc::c_int = 0;
    let mut moved: libc::c_int = 0 as libc::c_int;
    if header_lines > 0 as libc::c_int {
        home();
        ln = 0 as libc::c_int;
        while ln < header_lines {
            pos = forw_line(pos);
            set_attr_header(ln);
            clear_eol();
            put_line();
            ln += 1;
            ln;
        }
        moved = 1 as libc::c_int;
    }
    if header_cols > 0 as libc::c_int {
        home();
        pos = 0 as libc::c_int as POSITION;
        ln = 0 as libc::c_int;
        while ln < sc_height - 1 as libc::c_int {
            if ln >= header_lines {
                pos = position(ln);
            }
            if pos == -(1 as libc::c_int) as POSITION {
                putchr('\n' as i32);
            } else {
                pos = forw_line_pfx(
                    pos,
                    header_cols,
                    ((ln + 1 as libc::c_int) < header_lines) as libc::c_int,
                );
                set_attr_header(ln);
                put_line();
            }
            ln += 1;
            ln;
        }
        moved = 1 as libc::c_int;
    }
    if moved != 0 {
        lower_left();
    }
    return moved;
}
pub unsafe extern "C" fn forw(
    mut n: libc::c_int,
    mut pos: POSITION,
    mut force: libc::c_int,
    mut only_last: libc::c_int,
    mut nblank: libc::c_int,
) {
    let mut nlines: libc::c_int = 0 as libc::c_int;
    let mut do_repaint: libc::c_int = 0;
    squish_check();
    do_repaint = (only_last != 0 && n > sc_height - 1 as libc::c_int
        || forw_scroll >= 0 as libc::c_int && n > forw_scroll
            && n != sc_height - 1 as libc::c_int) as libc::c_int;
    if hilite_search == 2 as libc::c_int || is_filtering() != 0 || status_col != 0 {
        prep_hilite(
            pos,
            pos + (4 as libc::c_int * size_linebuf) as libc::c_long,
            if ignore_eoi != 0 { 1 as libc::c_int } else { -(1 as libc::c_int) },
        );
        pos = next_unfiltered(pos);
    }
    if do_repaint == 0 {
        if top_scroll != 0 && n >= sc_height - 1 as libc::c_int && pos != ch_length() {
            pos_clear();
            add_forw_pos(pos);
            force = 1 as libc::c_int;
            clear();
            home();
        }
        if pos != position(-(2 as libc::c_int)) || empty_screen() != 0 {
            pos_clear();
            add_forw_pos(pos);
            force = 1 as libc::c_int;
            if top_scroll != 0 {
                clear();
                home();
            } else if first_time == 0 && is_filtering() == 0 && full_screen != 0 {
                putstr(b"...skipping...\n\0" as *const u8 as *const libc::c_char);
            }
        }
    }
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        if nblank > 0 as libc::c_int {
            nblank -= 1;
            if nblank == 0 as libc::c_int {
                pos = 0 as libc::c_int as POSITION;
            }
        } else {
            pos = forw_line(pos);
            pos = next_unfiltered(pos);
            if pos == -(1 as libc::c_int) as POSITION {
                if force == 0
                    && position(0 as libc::c_int) != -(1 as libc::c_int) as POSITION
                {
                    break;
                }
                if empty_lines(0 as libc::c_int, 0 as libc::c_int) == 0
                    && empty_lines(1 as libc::c_int, 1 as libc::c_int) == 0
                    && empty_lines(2 as libc::c_int, sc_height - 1 as libc::c_int) != 0
                {
                    break;
                }
            }
        }
        add_forw_pos(pos);
        nlines += 1;
        nlines;
        if do_repaint != 0 {
            continue;
        }
        if first_time != 0 && pos == -(1 as libc::c_int) as POSITION && top_scroll == 0
            && header_lines == 0 as libc::c_int && header_cols == 0 as libc::c_int
            && tagoption.is_null() && plusoption == 0
        {
            squished = 1 as libc::c_int;
        } else {
            put_line();
            forw_prompt = 1 as libc::c_int;
        }
    }
    if header_lines > 0 as libc::c_int {
        if onscreen(0 as libc::c_int as POSITION) > 0 as libc::c_int {
            jump_loc(0 as libc::c_int as POSITION, 0 as libc::c_int);
            return;
        }
    }
    if nlines == 0 as libc::c_int && ignore_eoi == 0 {
        eof_bell();
    } else if do_repaint != 0 {
        repaint();
    } else {
        overlay_header();
    }
    first_time = 0 as libc::c_int;
    currline(-(1 as libc::c_int));
}
pub unsafe extern "C" fn back(
    mut n: libc::c_int,
    mut pos: POSITION,
    mut force: libc::c_int,
    mut only_last: libc::c_int,
) {
    let mut nlines: libc::c_int = 0 as libc::c_int;
    let mut do_repaint: libc::c_int = 0;
    squish_check();
    do_repaint = (n > get_back_scroll()
        || only_last != 0 && n > sc_height - 1 as libc::c_int
        || header_lines > 0 as libc::c_int) as libc::c_int;
    if hilite_search == 2 as libc::c_int || is_filtering() != 0 || status_col != 0 {
        prep_hilite(
            if pos < (3 as libc::c_int * size_linebuf) as libc::c_long {
                0 as libc::c_int as libc::c_long
            } else {
                pos - (3 as libc::c_int * size_linebuf) as libc::c_long
            },
            pos,
            -(1 as libc::c_int),
        );
    }
    loop {
        n -= 1;
        if !(n >= 0 as libc::c_int) {
            break;
        }
        pos = prev_unfiltered(pos);
        pos = back_line(pos);
        if pos == -(1 as libc::c_int) as POSITION {
            if force == 0 {
                break;
            }
        }
        add_back_pos(pos);
        nlines += 1;
        nlines;
        if do_repaint == 0 {
            home();
            add_line();
            put_line();
        }
    }
    if nlines == 0 as libc::c_int {
        eof_bell();
    } else if do_repaint != 0 {
        repaint();
    } else {
        overlay_header();
        lower_left();
    }
    currline(-(1 as libc::c_int));
}
pub unsafe extern "C" fn forward(
    mut n: libc::c_int,
    mut force: libc::c_int,
    mut only_last: libc::c_int,
) {
    let mut pos: POSITION = 0;
    if get_quit_at_eof() != 0 && eof_displayed() != 0
        && ch_getflags() & 0o10 as libc::c_int == 0
    {
        if edit_next(1 as libc::c_int) != 0 {
            quit(0 as libc::c_int);
        }
        return;
    }
    pos = position(-(2 as libc::c_int));
    if pos == -(1 as libc::c_int) as POSITION
        && (force == 0
            || empty_lines(2 as libc::c_int, sc_height - 1 as libc::c_int) != 0)
    {
        if ignore_eoi != 0 {
            if empty_screen() != 0 {
                pos = 0 as libc::c_int as POSITION;
            } else {
                loop {
                    back(
                        1 as libc::c_int,
                        position(0 as libc::c_int),
                        1 as libc::c_int,
                        0 as libc::c_int,
                    );
                    pos = position(-(2 as libc::c_int));
                    if !(pos == -(1 as libc::c_int) as POSITION) {
                        break;
                    }
                }
            }
        } else {
            eof_bell();
            return;
        }
    }
    forw(n, pos, force, only_last, 0 as libc::c_int);
}
pub unsafe extern "C" fn backward(
    mut n: libc::c_int,
    mut force: libc::c_int,
    mut only_last: libc::c_int,
) {
    let mut pos: POSITION = 0;
    pos = position(0 as libc::c_int);
    if pos == -(1 as libc::c_int) as POSITION
        && (force == 0
            || position(-(1 as libc::c_int)) == 0 as libc::c_int as libc::c_long)
    {
        eof_bell();
        return;
    }
    back(n, pos, force, only_last);
}
pub unsafe extern "C" fn get_back_scroll() -> libc::c_int {
    if no_back_scroll != 0 {
        return 0 as libc::c_int;
    }
    if back_scroll >= 0 as libc::c_int {
        return back_scroll;
    }
    if top_scroll != 0 {
        return sc_height - 2 as libc::c_int;
    }
    return 10000 as libc::c_int;
}
pub unsafe extern "C" fn get_one_screen() -> libc::c_int {
    let mut nlines: libc::c_int = 0;
    let mut pos: POSITION = 0 as libc::c_int as POSITION;
    nlines = 0 as libc::c_int;
    while nlines < sc_height {
        pos = forw_line(pos);
        if pos == -(1 as libc::c_int) as POSITION {
            break;
        }
        nlines += 1;
        nlines;
    }
    return (nlines < sc_height) as libc::c_int;
}
