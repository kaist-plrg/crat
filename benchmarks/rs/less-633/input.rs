use ::libc;
extern "C" {
    fn ch_seek(pos: POSITION) -> libc::c_int;
    fn ch_tell() -> POSITION;
    fn ch_forw_get() -> libc::c_int;
    fn ch_back_get() -> libc::c_int;
    fn null_line();
    fn line_position() -> POSITION;
    fn set_status_col(c: libc::c_char, attr: libc::c_int);
    fn is_hilited_attr(
        pos: POSITION,
        epos: POSITION,
        nohide: libc::c_int,
        p_matches: *mut libc::c_int,
    ) -> libc::c_int;
    fn chop_line() -> libc::c_int;
    fn is_filtered(pos: POSITION) -> libc::c_int;
    fn pdone(endline: libc::c_int, chopped: libc::c_int, forw: libc::c_int);
    fn pappend(c: libc::c_int, pos: POSITION) -> libc::c_int;
    fn savec();
    fn loadc();
    fn pflushmbc() -> libc::c_int;
    fn pshift_all();
    fn plinestart(pos: POSITION);
    fn prewind();
    fn next_unfiltered(pos: POSITION) -> POSITION;
    fn prep_hilite(spos: POSITION, epos: POSITION, maxlines: libc::c_int);
    fn is_filtering() -> libc::c_int;
    static mut squeeze: libc::c_int;
    static mut hshift: libc::c_int;
    static mut quit_if_one_screen: libc::c_int;
    static mut sigs: libc::c_int;
    static mut ignore_eoi: libc::c_int;
    static mut status_col: libc::c_int;
    static mut wordwrap: libc::c_int;
    static mut start_attnpos: POSITION;
    static mut end_attnpos: POSITION;
    static mut hilite_search: libc::c_int;
    static mut size_linebuf: libc::c_int;
    static mut show_attn: libc::c_int;
}
pub type __off_t = libc::c_long;
pub type off_t = __off_t;
pub type POSITION = off_t;
unsafe extern "C" fn init_status_col(
    mut base_pos: POSITION,
    mut disp_pos: POSITION,
    mut edisp_pos: POSITION,
    mut eol_pos: POSITION,
) {
    let mut hl_before: libc::c_int = if chop_line() != 0
        && disp_pos != -(1 as libc::c_int) as POSITION
    {
        is_hilited_attr(base_pos, disp_pos, 1 as libc::c_int, 0 as *mut libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut hl_after: libc::c_int = if chop_line() != 0 {
        is_hilited_attr(edisp_pos, eol_pos, 1 as libc::c_int, 0 as *mut libc::c_int)
    } else {
        0 as libc::c_int
    };
    let mut attr: libc::c_int = 0;
    let mut ch: libc::c_char = 0;
    if hl_before != 0 && hl_after != 0 {
        attr = hl_after;
        ch = '=' as i32 as libc::c_char;
    } else if hl_before != 0 {
        attr = hl_before;
        ch = '<' as i32 as libc::c_char;
    } else if hl_after != 0 {
        attr = hl_after;
        ch = '>' as i32 as libc::c_char;
    } else {
        attr = is_hilited_attr(
            base_pos,
            eol_pos,
            1 as libc::c_int,
            0 as *mut libc::c_int,
        );
        ch = '*' as i32 as libc::c_char;
    }
    if attr != 0 {
        set_status_col(ch, attr);
    }
}
pub unsafe extern "C" fn forw_line_seg(
    mut curr_pos: POSITION,
    mut skipeol: libc::c_int,
    mut rscroll: libc::c_int,
    mut nochop: libc::c_int,
) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut c: libc::c_int = 0;
    let mut blankline: libc::c_int = 0;
    let mut endline: libc::c_int = 0;
    let mut chopped: libc::c_int = 0;
    let mut backchars: libc::c_int = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: libc::c_int = 0;
    loop {
        if curr_pos == -(1 as libc::c_int) as POSITION {
            null_line();
            return -(1 as libc::c_int) as POSITION;
        }
        if hilite_search == 2 as libc::c_int || is_filtering() != 0 || status_col != 0 {
            prep_hilite(
                curr_pos,
                curr_pos + (3 as libc::c_int * size_linebuf) as libc::c_long,
                if ignore_eoi != 0 { 1 as libc::c_int } else { -(1 as libc::c_int) },
            );
            curr_pos = next_unfiltered(curr_pos);
        }
        if ch_seek(curr_pos) != 0 {
            null_line();
            return -(1 as libc::c_int) as POSITION;
        }
        base_pos = curr_pos;
        loop {
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                null_line();
                return -(1 as libc::c_int) as POSITION;
            }
            c = ch_back_get();
            if c == -(1 as libc::c_int) {
                break;
            }
            if c == '\n' as i32 {
                ch_forw_get();
                break;
            } else {
                base_pos -= 1;
                base_pos;
            }
        }
        prewind();
        plinestart(base_pos);
        ch_seek(base_pos);
        new_pos = base_pos;
        while new_pos < curr_pos {
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                null_line();
                return -(1 as libc::c_int) as POSITION;
            }
            c = ch_forw_get();
            backchars = pappend(c, new_pos);
            new_pos += 1;
            new_pos;
            if backchars > 0 as libc::c_int {
                pshift_all();
                if wordwrap != 0 && (c == ' ' as i32 || c == '\t' as i32) {
                    loop {
                        new_pos += 1;
                        new_pos;
                        c = ch_forw_get();
                        if !(c == ' ' as i32 || c == '\t' as i32) {
                            break;
                        }
                    }
                    backchars = 1 as libc::c_int;
                }
                new_pos -= backchars as libc::c_long;
                loop {
                    backchars -= 1;
                    if !(backchars >= 0 as libc::c_int) {
                        break;
                    }
                    ch_back_get();
                }
            }
        }
        pflushmbc();
        pshift_all();
        c = ch_forw_get();
        if c == -(1 as libc::c_int) {
            null_line();
            return -(1 as libc::c_int) as POSITION;
        }
        blankline = (c == '\n' as i32 || c == '\r' as i32) as libc::c_int;
        wrap_pos = -(1 as libc::c_int) as POSITION;
        skipped_leading = 0 as libc::c_int;
        chopped = 0 as libc::c_int;
        loop {
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                null_line();
                return -(1 as libc::c_int) as POSITION;
            }
            if c == '\n' as i32 || c == -(1 as libc::c_int) {
                backchars = pflushmbc();
                new_pos = ch_tell();
                if backchars > 0 as libc::c_int && (nochop != 0 || chop_line() == 0)
                    && hshift == 0 as libc::c_int
                {
                    new_pos -= (backchars + 1 as libc::c_int) as libc::c_long;
                    endline = 0 as libc::c_int;
                } else {
                    endline = 1 as libc::c_int;
                }
                edisp_pos = new_pos;
                break;
            } else {
                if c != '\r' as i32 {
                    blankline = 0 as libc::c_int;
                }
                backchars = pappend(c, ch_tell() - 1 as libc::c_int as libc::c_long);
                if backchars > 0 as libc::c_int {
                    if skipeol != 0 {
                        edisp_pos = ch_tell();
                        loop {
                            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                                null_line();
                                return -(1 as libc::c_int) as POSITION;
                            }
                            c = ch_forw_get();
                            if !(c != '\n' as i32 && c != -(1 as libc::c_int)) {
                                break;
                            }
                        }
                        new_pos = ch_tell();
                        endline = 1 as libc::c_int;
                        quit_if_one_screen = 0 as libc::c_int;
                        chopped = 1 as libc::c_int;
                    } else {
                        if wordwrap == 0 {
                            new_pos = ch_tell() - backchars as libc::c_long;
                        } else if c == ' ' as i32 || c == '\t' as i32 {
                            loop {
                                new_pos = ch_tell();
                                c = ch_forw_get();
                                if !(c == ' ' as i32 || c == '\t' as i32) {
                                    break;
                                }
                            }
                            if c == '\r' as i32 {
                                c = ch_forw_get();
                            }
                            if c == '\n' as i32 {
                                new_pos = ch_tell();
                            }
                        } else if wrap_pos == -(1 as libc::c_int) as POSITION {
                            new_pos = ch_tell() - backchars as libc::c_long;
                        } else {
                            new_pos = wrap_pos;
                            loadc();
                        }
                        endline = 0 as libc::c_int;
                    }
                    break;
                } else {
                    if wordwrap != 0 {
                        if c == ' ' as i32 || c == '\t' as i32 {
                            if skipped_leading != 0 {
                                wrap_pos = ch_tell();
                                savec();
                            }
                        } else {
                            skipped_leading = 1 as libc::c_int;
                        }
                    }
                    c = ch_forw_get();
                }
            }
        }
        if blankline != 0 && show_attn != 0 {
            pappend(' ' as i32, ch_tell() - 1 as libc::c_int as libc::c_long);
        }
        pdone(endline, (rscroll != 0 && chopped != 0) as libc::c_int, 1 as libc::c_int);
        if !(is_filtered(base_pos) != 0) {
            break;
        }
        curr_pos = new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    if squeeze != 0 && blankline != 0 {
        loop {
            c = ch_forw_get();
            if !(c == '\n' as i32 || c == '\r' as i32) {
                break;
            }
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                null_line();
                return -(1 as libc::c_int) as POSITION;
            }
        }
        if c != -(1 as libc::c_int) {
            ch_back_get();
        }
        new_pos = ch_tell();
    }
    return new_pos;
}
pub unsafe extern "C" fn forw_line(mut curr_pos: POSITION) -> POSITION {
    return forw_line_seg(
        curr_pos,
        (chop_line() != 0 || hshift > 0 as libc::c_int) as libc::c_int,
        1 as libc::c_int,
        0 as libc::c_int,
    );
}
pub unsafe extern "C" fn back_line(mut curr_pos: POSITION) -> POSITION {
    let mut base_pos: POSITION = 0;
    let mut new_pos: POSITION = 0;
    let mut edisp_pos: POSITION = 0;
    let mut begin_new_pos: POSITION = 0;
    let mut c: libc::c_int = 0;
    let mut endline: libc::c_int = 0;
    let mut chopped: libc::c_int = 0;
    let mut backchars: libc::c_int = 0;
    let mut wrap_pos: POSITION = 0;
    let mut skipped_leading: libc::c_int = 0;
    loop {
        if curr_pos == -(1 as libc::c_int) as POSITION
            || curr_pos <= 0 as libc::c_int as POSITION
        {
            null_line();
            return -(1 as libc::c_int) as POSITION;
        }
        if hilite_search == 2 as libc::c_int || is_filtering() != 0 || status_col != 0 {
            prep_hilite(
                if curr_pos < (3 as libc::c_int * size_linebuf) as libc::c_long {
                    0 as libc::c_int as libc::c_long
                } else {
                    curr_pos - (3 as libc::c_int * size_linebuf) as libc::c_long
                },
                curr_pos,
                -(1 as libc::c_int),
            );
        }
        if ch_seek(curr_pos - 1 as libc::c_int as libc::c_long) != 0 {
            null_line();
            return -(1 as libc::c_int) as POSITION;
        }
        if squeeze != 0 {
            ch_forw_get();
            c = ch_forw_get();
            ch_back_get();
            ch_back_get();
            if c == '\n' as i32 || c == '\r' as i32 {
                loop {
                    c = ch_back_get();
                    if !(c == '\n' as i32 || c == '\r' as i32) {
                        break;
                    }
                    if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                        null_line();
                        return -(1 as libc::c_int) as POSITION;
                    }
                }
                if c == -(1 as libc::c_int) {
                    null_line();
                    return -(1 as libc::c_int) as POSITION;
                }
                ch_forw_get();
            }
        }
        loop {
            if sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0 {
                null_line();
                return -(1 as libc::c_int) as POSITION;
            }
            c = ch_back_get();
            if c == '\n' as i32 {
                base_pos = ch_tell() + 1 as libc::c_int as libc::c_long;
                break;
            } else {
                if !(c == -(1 as libc::c_int)) {
                    continue;
                }
                base_pos = ch_tell();
                break;
            }
        }
        new_pos = base_pos;
        if ch_seek(new_pos) != 0 {
            null_line();
            return -(1 as libc::c_int) as POSITION;
        }
        endline = 0 as libc::c_int;
        prewind();
        plinestart(new_pos);
        '_loop: loop {
            wrap_pos = -(1 as libc::c_int) as POSITION;
            skipped_leading = 0 as libc::c_int;
            begin_new_pos = new_pos;
            ch_seek(new_pos);
            chopped = 0 as libc::c_int;
            loop {
                c = ch_forw_get();
                if c == -(1 as libc::c_int)
                    || sigs & (0o1 as libc::c_int | 0o2 as libc::c_int) != 0
                {
                    null_line();
                    return -(1 as libc::c_int) as POSITION;
                }
                new_pos += 1;
                new_pos;
                if c == '\n' as i32 {
                    backchars = pflushmbc();
                    if backchars > 0 as libc::c_int && chop_line() == 0
                        && hshift == 0 as libc::c_int
                    {
                        backchars += 1;
                        backchars;
                    } else {
                        endline = 1 as libc::c_int;
                        edisp_pos = new_pos;
                        break '_loop;
                    }
                } else {
                    backchars = pappend(c, ch_tell() - 1 as libc::c_int as libc::c_long);
                    if backchars > 0 as libc::c_int {
                        if chop_line() != 0 || hshift > 0 as libc::c_int {
                            endline = 1 as libc::c_int;
                            chopped = 1 as libc::c_int;
                            quit_if_one_screen = 0 as libc::c_int;
                            edisp_pos = new_pos;
                            break '_loop;
                        }
                    } else {
                        if wordwrap != 0 {
                            if c == ' ' as i32 || c == '\t' as i32 {
                                if skipped_leading != 0 {
                                    wrap_pos = new_pos;
                                }
                            } else {
                                skipped_leading = 1 as libc::c_int;
                            }
                        }
                        if !(new_pos >= curr_pos) {
                            continue;
                        }
                        edisp_pos = new_pos;
                        break '_loop;
                    }
                }
                if wordwrap == 0 {
                    pshift_all();
                    new_pos -= backchars as libc::c_long;
                    break;
                } else if c == ' ' as i32 || c == '\t' as i32 {
                    loop {
                        c = ch_forw_get();
                        if c == ' ' as i32 || c == '\t' as i32 {
                            new_pos += 1;
                            new_pos;
                        } else {
                            if c == '\r' as i32 {
                                c = ch_forw_get();
                                if c == '\n' as i32 {
                                    new_pos += 1;
                                    new_pos;
                                }
                            }
                            if c == '\n' as i32 {
                                new_pos += 1;
                                new_pos;
                            }
                            break;
                        }
                    }
                    if new_pos >= curr_pos {
                        break '_loop;
                    }
                    pshift_all();
                    break;
                } else {
                    pshift_all();
                    if wrap_pos == -(1 as libc::c_int) as POSITION {
                        new_pos -= backchars as libc::c_long;
                    } else {
                        new_pos = wrap_pos;
                    }
                    break;
                }
            }
        }
        pdone(endline, chopped, 0 as libc::c_int);
        if !(is_filtered(base_pos) != 0) {
            break;
        }
        curr_pos = begin_new_pos;
    }
    if status_col != 0 {
        init_status_col(base_pos, line_position(), edisp_pos, new_pos);
    }
    return begin_new_pos;
}
pub unsafe extern "C" fn set_attnpos(mut pos: POSITION) {
    let mut c: libc::c_int = 0;
    if pos != -(1 as libc::c_int) as POSITION {
        if ch_seek(pos) != 0 {
            return;
        }
        loop {
            c = ch_forw_get();
            if c == -(1 as libc::c_int) {
                break;
            }
            if c == '\n' as i32 || c == '\r' as i32 {
                ch_back_get();
                break;
            } else {
                pos += 1;
                pos;
            }
        }
        end_attnpos = pos;
        loop {
            c = ch_back_get();
            if c == -(1 as libc::c_int) || c == '\n' as i32 || c == '\r' as i32 {
                break;
            }
            pos -= 1;
            pos;
        }
    }
    start_attnpos = pos;
}
